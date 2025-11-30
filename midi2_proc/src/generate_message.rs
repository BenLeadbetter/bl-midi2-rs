use crate::common;
use darling::FromMeta;
use midi2_message_schema as schema;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct Args {
    message: String,
}

fn generic_buffer_constraint(repr: common::Representation) -> TokenStream {
    use common::Representation::*;
    match repr {
        UmpOrBytes => quote! { crate::buffer::Buffer },
        Bytes => quote! { crate::buffer::Bytes },
        Ump => quote! { crate::buffer::Ump },
    }
}

fn root_ident(message: &schema::Message) -> syn::Ident {
    use heck::ToPascalCase;
    syn::Ident::new(
        &message.name.to_pascal_case(),
        proc_macro2::Span::call_site(),
    )
}

fn representation(message: &schema::Message) -> Option<common::Representation> {
    match (&message.ump, &message.bytes) {
        (&Some(_), &Some(_)) => Some(common::Representation::UmpOrBytes),
        (None, &Some(_)) => Some(common::Representation::Bytes),
        (&Some(_), None) => Some(common::Representation::Ump),
        (None, None) => None,
    }
}

fn message(
    message: &schema::Message,
    attributes: &[syn::Attribute],
    repr: common::Representation,
) -> TokenStream {
    let constraint = generic_buffer_constraint(repr);
    let root_ident = root_ident(message);

    let mut doc_attributes = TokenStream::new();
    for attribute in attributes.iter() {
        if let syn::Meta::NameValue(syn::MetaNameValue { path, .. }) = &attribute.meta {
            if let Some(syn::PathSegment { ident, .. }) = path.segments.last() {
                if ident == "doc" {
                    doc_attributes.extend(quote! { #attribute });
                }
            }
        }
    }

    quote! {
        #[derive(PartialEq, Eq, Clone, Copy, midi2_proc::Debug)]
        #doc_attributes
        pub struct #root_ident<B: #constraint>(B);
    }
}

fn buffer_access_impl(schema: &schema::Message, repr: common::Representation) -> TokenStream {
    let constraint = generic_buffer_constraint(repr);
    let root_ident = root_ident(schema);
    quote! {
        impl<B: #constraint> crate::traits::BufferAccess<B> for #root_ident<B> {
            fn buffer_access(&self) -> &B {
                &self.0
            }
            fn buffer_access_mut(&mut self) -> &mut B
            where
                B: crate::buffer::BufferMut
            {
                &mut self.0
            }
        }
    }
}

fn min_size_impl(schema: &schema::Message, repr: common::Representation) -> TokenStream {
    let body = match (&schema.ump, &schema.bytes) {
        (
            &Some(schema::Ump {
                packet_size,
                min_packets,
                ..
            }),
            &Some(schema::Bytes { min_size, .. }),
        ) => quote! {
            match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
                crate::buffer::UNIT_ID_U32 => #min_packets * #packet_size,
                crate::buffer::UNIT_ID_U8 => #min_size,
                _ => unreachable!(),
            }
        },
        (None, &Some(schema::Bytes { min_size, .. })) => quote! { #min_size },
        (
            &Some(schema::Ump {
                packet_size,
                min_packets,
                ..
            }),
            None,
        ) => quote! { #packet_size * #min_packets },
        (None, None) => {
            quote! { compile_error!("Message has neither UMP nor Bytes representation") }
        }
    };
    let constraint = generic_buffer_constraint(repr);
    let root_ident = root_ident(schema);
    quote! {
        impl<B: #constraint> crate::traits::MinSize<B> for #root_ident<B> {
            const MIN_SIZE: usize = #body;
        }
    }
}

fn new_impl(schema: &schema::Message, repr: common::Representation) -> TokenStream {
    let constraint = generic_buffer_constraint(repr);
    let root_ident = root_ident(schema);
    // let initialise_properties = initialise_property_statements(properties, quote! {B});
    quote! {
        impl<B: #constraint
                    + crate::buffer::BufferMut
                    + crate::buffer::BufferDefault
                    + crate::buffer::BufferResize
        > #root_ident<B>
        {
            /// Create a new message backed by a resizable buffer.
            pub fn new() -> #root_ident<B>
            {
                let mut buffer = <B as crate::buffer::BufferDefault>::default();
                let buffer_ref_mut = &mut buffer;
                buffer_ref_mut.resize(<Self as crate::traits::MinSize<B>>::MIN_SIZE);
                // TODO:
                // #initialise_properties
                #root_ident::<B>(buffer)
            }
        }
    }
}

fn new_array_impl(schema: &schema::Message, repr: common::Representation) -> TokenStream {
    use common::Representation::*;
    let generics = match repr {
        UmpOrBytes => quote! { , U: crate::buffer::Unit },
        _ => TokenStream::new(),
    };
    let unit_type = match repr {
        Ump => quote! { u32 },
        Bytes => quote! { u8 },
        UmpOrBytes => quote! { U },
    };
    let buffer_type = quote! { [#unit_type; SIZE] };
    let root_ident = root_ident(schema);
    // let initialise_properties = initialise_property_statements(properties, quote! { #buffer_type });
    quote! {
        impl<const SIZE: usize #generics> #root_ident<#buffer_type>
        {
            /// Create a new message backed by a simple array type buffer.
            ///
            /// Note: this constructor will fail to compile for `SIZE` values
            /// which are smaller than the minimum representable message size.
            pub fn new() -> #root_ident<#buffer_type>
            {
                let _valid = <Self as crate::traits::ArraySizeValid<SIZE, #buffer_type>>::VALID;
                let mut buffer = [<#unit_type as crate::buffer::Unit>::zero(); SIZE];
                let buffer_ref_mut = &mut buffer;
                // #initialise_properties
                #root_ident(buffer)
            }
        }
    }
}

// pub fn initialise_property_statements(
//     properties: &[Property],
//     buffer_type: TokenStream,
// ) -> TokenStream {
//     let mut initialise_properties = TokenStream::new();
//     for property in properties.iter().filter(|p| !p.readonly) {
//         let meta_type = &property.meta_type;
//         let std_only_attribute = common::std_only_attribute(property.std);
//
//         initialise_properties.extend(quote! {
//             #std_only_attribute
//             <#meta_type as crate::detail::property::WriteProperty<#buffer_type>>::write(
//                 buffer_ref_mut,
//                 <#meta_type as crate::detail::property::WriteProperty<#buffer_type>>::default(),
//             );
//         });
//     }
//     initialise_properties
// }

pub fn generate_message(args: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let args: Args = match syn::parse(args) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };
    let input = syn::parse_macro_input!(item as syn::ItemConst);

    let Some(schema) = midi2_message_schema::SCHEMA.messages.get(&args.message) else {
        let err = format!("Message not found in schema: {}", args.message);
        return syn::Error::new_spanned(args.message, err)
            .to_compile_error()
            .into();
    };
    let Some(repr) = representation(schema) else {
        let err = format!(
            concat!(
                "Couldn't deduce buffer type for: {}.",
                "This probably means the schema is missing",
                "either ump and/or bytes information"
            ),
            args.message
        );
        return syn::Error::new_spanned(args.message, err)
            .to_compile_error()
            .into();
    };

    let message = message(schema, &input.attrs, repr);
    let buffer_access_impl = buffer_access_impl(schema, repr);
    let min_size_impl = min_size_impl(schema, repr);
    let new_impl = new_impl(schema, repr);
    let new_array_impl = new_array_impl(schema, repr);

    quote! {
        #message
        #buffer_access_impl
        #min_size_impl
        #new_impl
        #new_array_impl
    }
    .into()
}

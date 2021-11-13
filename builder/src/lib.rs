use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, FieldsNamed, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let builder_name = Ident::new(&format!("{}Builder", name), Span::call_site());

    let fields = builder_fields(&input.data);
    let fields_init = builder_init(&input.data);

    let expanded = quote! {
        pub struct #builder_name {
            #fields
        }

        impl #name {
            fn builder() -> #builder_name {
                #builder_name {
                    #fields_init
                }
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

fn builder_fields(data: &Data) -> TokenStream {
    let fields = extract_fields(data);
    let optionateds = fields.named.iter().map(|f| {
        let ty = &f.ty;
        let ident = &f.ident;
        quote! {
            #ident: std::option::Option<#ty>
        }
    });
    quote! {
        #(#optionateds),*
    }
}

fn builder_init(data: &Data) -> TokenStream {
    let fields = extract_fields(data);
    let inits = fields.named.iter().map(|f| {
        let ident = &f.ident;
        quote! {
            #ident: std::option::Option::None
        }
    });
    quote! {
        #(#inits),*
    }
}

fn extract_fields(data: &Data) -> &FieldsNamed {
    match *data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields_named) => fields_named,
            _ => panic!("expected all fields which named"),
        },
        _ => panic!("Struct expected!"),
    }
}

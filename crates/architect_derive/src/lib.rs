use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields};

#[proc_macro_derive(Module, attributes(input, output))]
pub fn derive_module(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let module_name = input.ident;

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("only non-tuple structs are supported as modules"),
        },
        _ => panic!("only structs are supported as modules"),
    };

    let input_fields = fields.iter().filter(|field| {
        field.attrs.iter().any(|attr| {
            attr.path.segments.len() == 1
                && attr.path.segments[0].arguments.is_empty()
                && attr.path.segments[0].ident == "input"
        })
    });

    let output_fields = fields.iter().filter(|field| {
        field.attrs.iter().any(|attr| {
            attr.path.segments.len() == 1
                && attr.path.segments[0].arguments.is_empty()
                && attr.path.segments[0].ident == "output"
        })
    });

    let interface_inputs = input_fields.map(|field| {
        let name = &field.ident;
        quote_spanned!(field.span() => (stringify!(#name), self.#name.get_type(context)))
    });

    let interface_outputs = output_fields.map(|field| {
        let name = &field.ident;
        quote_spanned!(field.span() => (stringify!(#name), self.#name.get_type(context)))
    });

    let expanded = quote! {
        impl ::architect::Module for #module_name {
            fn interface(&self, context: &mut Context) -> ModuleInterface {
                use ::architect::LogicType;

                let inputs = vec![
                    #(#interface_inputs,)*
                ];

                let outputs = vec![
                    #(#interface_outputs,)*
                ];

                ModuleInterface {
                    name: stringify!(#module_name),
                    inputs,
                    outputs,
                }
            }
        }
    };

    TokenStream::from(expanded)
}

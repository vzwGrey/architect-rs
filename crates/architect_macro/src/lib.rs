use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Error, Field, Fields, ItemStruct};

#[proc_macro_attribute]
pub fn entity(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let entity_name = input.ident;

    let fields = match input.fields {
        Fields::Named(ref fields) => &fields.named,
        _ => {
            return Error::new(
                entity_name.span(),
                "only non-tuple structs are supported as entities",
            )
            .to_compile_error()
            .into()
        }
    };

    let input_fields = fields.iter().filter(|field| {
        field.attrs.iter().any(|attr| {
            attr.path.segments.len() == 1
                && attr.path.segments[0].arguments.is_empty()
                && attr.path.segments[0].ident == "input"
        })
    });
    let input_signal_names = input_fields.clone().map(|field| &field.ident);
    let input_signal_getters = input_fields
        .clone()
        .map(|field| signal_getter(field, false));
    let input_signal_to_type_name = input_fields.map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote_spanned!(field.span() => stringify!(#name) => <#ty>::name())
    });

    let output_fields = fields.iter().filter(|field| {
        field.attrs.iter().any(|attr| {
            attr.path.segments.len() == 1
                && attr.path.segments[0].arguments.is_empty()
                && attr.path.segments[0].ident == "output"
        })
    });
    let output_signal_names = output_fields.clone().map(|field| &field.ident);
    let output_signal_getters = output_fields
        .clone()
        .map(|field| signal_getter(field, true));
    let output_signal_to_type_name = output_fields.map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote_spanned!(field.span() => stringify!(#name) => <#ty>::name())
    });

    let expanded = quote! {
        struct #entity_name {
            context: ::std::rc::Rc<::architect::Context>,
        }

        impl #entity_name {
            #(#input_signal_getters)*
            #(#output_signal_getters)*
        }

        impl ::architect::Entity for #entity_name {
            fn create(context: ::std::rc::Rc<::architect::Context>) -> Self {
                Self {
                    context
                }
            }

            fn name(&self) -> &'static str {
                stringify!(#entity_name)
            }

            fn inputs(&self) -> Vec<&'static str> {
                vec![
                    #(stringify!(#input_signal_names),)*
                ]
            }

            fn outputs(&self) -> Vec<&'static str> {
                vec![
                    #(stringify!(#output_signal_names),)*
                ]
            }

            fn get_type_name_for_signal(&self, name: &'static str) -> String {
                use ::architect::LogicType;
                match name {
                    #(#input_signal_to_type_name,)*
                    #(#output_signal_to_type_name,)*
                    _ => panic!("invalid signal name")
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn signal_getter(field: &Field, is_output: bool) -> Box<dyn ToTokens> {
    let name = &field.ident;
    let ty = &field.ty;
    let in_out = if is_output {
        quote!(::architect::OutputSignal)
    } else {
        quote!(::architect::InputSignal)
    };

    Box::new(quote_spanned!(field.span() =>
        pub fn #name (&self) -> ::architect::Signal<#ty, #in_out> {
            ::architect::Signal::with_name(stringify!(#name))
        }
    ))
}

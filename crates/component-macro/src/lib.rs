extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Token, ImplItem, ItemImpl, ImplItemMethod, Signature};

#[proc_macro]
pub fn component(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemImpl);

    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let self_ty = &ast.self_ty;
    let items = &ast.items;

    let mut methods = vec![];

    for item in items.iter() {
        match item {
            ImplItem::Method(method) => {
                if format!("{}", &method.sig.ident) == "render" {
                    methods.push(expand_render_method(&method));
                } else {
                    methods.push(quote! { #method });
                }
            }
            any => {
                methods.push(quote! { #any })
            }
        }
    }

    TokenStream::from(quote! {
        impl #impl_generics Component for #self_ty #ty_generics #where_clause {
            #( #methods )*
        }
    })
}

fn expand_render_method(method: &ImplItemMethod) -> proc_macro2::TokenStream {
    let Signature { ident, generics, inputs, .. } = &method.sig;
    let block = &method.block;

    quote! {
        fn #ident #generics (#inputs, context: Context) -> VirtualNode {
            Wrapper::with_current_wrapper(|_current| {
                _current.enter_child(context, || #block)
            })
        }
    }
}

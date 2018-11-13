extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[cfg_attr(feature = "cargo-clippy", allow(clippy::needless_pass_by_value))]
#[proc_macro_attribute]
pub fn trace_info(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed: syn::ItemFn = syn::parse2(input.clone().into()).unwrap();
    let block = parsed.block;
    let vis = parsed.vis;
    let ident = parsed.ident;
    let decl = parsed.decl;
    let inputs = decl.clone().inputs;
    let output = decl.clone().output;

    let expanded = quote! {
        #vis fn #ident(#inputs) #output {
            println!("入==>  `{}` [{}#line={}]", stringify!(#ident), file!(), line!()+1);
            let inner = ||{
                #block
            };
            let result = inner();
            println!("<==出  `{}` [{}#line={}]", stringify!(#ident), file!(), line!()+1);
            return result;
        }
    };

    expanded.into()
}

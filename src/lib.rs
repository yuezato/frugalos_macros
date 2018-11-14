#![recursion_limit = "128"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate regex;
use regex::Regex;

use proc_macro::TokenStream;

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[proc_macro_attribute]
pub fn trace_info(args: TokenStream, input: TokenStream) -> TokenStream {
    let args_str = args.to_string();
    let re = Regex::new(r#"^comment\s*=\s*"(.*?)"$"#).unwrap();
    let mut comment: Option<&str> = None;
    if let Some(cap) = re.captures(&args_str) {
        comment = Some(cap.get(1).unwrap().as_str());
    }

    let parsed: syn::ItemFn = syn::parse2(input.clone().into()).unwrap();
    let block = parsed.block;
    let vis = parsed.vis;
    let ident = parsed.ident;
    let decl = parsed.decl;
    let inputs = decl.clone().inputs;
    let output = decl.clone().output;

    let expanded = if let Some(comment) = comment {
        quote! {
            #vis fn #ident(#inputs) #output {
                let dir = env!("CARGO_MANIFEST_DIR");
                let mut file_name = String::from(file!());
                if !file_name.starts_with("/") {
                    file_name = format!("{}/{}", dir, file_name);
                }

                println!("入==> `{}` [ {}:{} ]", stringify!(#ident), file_name, line!()+1);
                println!("      {}", #comment);
                let inner = ||{
                    #block
                };
                let result = inner();
                    println!("<==出 `{}` [ {}:{} ]", stringify!(#ident), file_name, line!()+1);
                return result;
            }
        }
    } else {
        quote! {
            #vis fn #ident(#inputs) #output {
                let dir = env!("CARGO_MANIFEST_DIR");
                let mut file_name = String::from(file!());
                if !file_name.starts_with("/") {
                    file_name = format!("{}/{}", dir, file_name);
                }

                println!("入==> `{}` [ {}:{} ]", stringify!(#ident), file_name, line!()+1);

                let inner = ||{
                    #block
                };
                let result = inner();
                println!("<==出 `{}` [ {}:{} ]", stringify!(#ident), file_name, line!()+1);
                return result;
            }
        }
    };

    expanded.into()
}

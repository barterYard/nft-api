use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    spanned::Spanned,
    DeriveInput, ItemFn, ItemStruct, Stmt,
};

mod kw {
    use syn::custom_keyword;
    custom_keyword!(scope);
    custom_keyword!(services);
}
#[derive(Default, Debug, Clone)]
struct TaskAttributes {
    scope: String,
    services: Vec<String>,
}

impl Parse for TaskAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut sc = Self::default();

        let _: kw::scope = input.parse()?;
        let _: syn::Token![=] = input.parse()?;
        let name = input.parse::<proc_macro2::Literal>()?;

        sc.scope = name.to_string();

        let _: kw::services = input.parse()?;
        let _: syn::Token![=] = input.parse()?;
        let vis = input.parse::<proc_macro2::Literal>()?;
        sc.services = vis
            .to_string()
            .replace("\"", "")
            .split(",")
            .map(|x| x.to_string())
            .collect();

        return Ok(sc);
    }
}
#[proc_macro_attribute]
pub fn endpoint(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsed_item = parse_macro_input!(item as ItemStruct);
    let parsed_args = parse_macro_input!(attr as TaskAttributes);

    let struct_name = parsed_item.clone().ident;

    let scope: proc_macro2::TokenStream = parsed_args.scope.parse().unwrap();

    let services: proc_macro2::TokenStream = parsed_args
        .services
        .into_iter()
        .fold("".to_string(), |acc, x| format!("{}.service({})", acc, x))
        .parse()
        .unwrap();

    let k: proc_macro2::TokenStream = quote!(
        pub struct #struct_name {}

        impl Endpoint for #struct_name {
            fn services() -> actix_web::Scope {
                web::scope(#scope)#services
            }
        }

    );
    TokenStream::from(k)
}

#[proc_macro_attribute]
pub fn authenticated(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut parsed_item = parse_macro_input!(item as ItemFn);

    let arg = parsed_item
        .clone()
        .sig
        .inputs
        .into_iter()
        .find(|arg| match arg {
            syn::FnArg::Typed(syn::PatType { ty, .. }) => {
                ty.to_token_stream().to_string().contains("Identity")
            }
            _ => false,
        });

    if arg.is_none() {
        parsed_item
            .sig
            .inputs
            .push(parse_quote!(user: Option<Identity>));
    }
    let arg_name = match arg.is_none() {
        true => "user".to_string(),
        false => arg.span().source_text().unwrap_or("user".to_string()),
    };
    let identity_name = Some(format_ident!("{}", arg_name));

    let start_itm: Stmt = parse_quote! {
        if #identity_name.is_none() {
            return (None, http::StatusCode::UNAUTHORIZED);
        };
    };

    parsed_item.block.stmts.insert(0, start_itm);
    TokenStream::from(quote!(#parsed_item))
}

#[proc_macro_derive(ModelCollection)]
pub fn model_collection(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let col_name: proc_macro2::TokenStream = format!("\"{}s\"", ident.to_string().to_lowercase())
        .parse()
        .unwrap();

    let db_name: proc_macro2::TokenStream = quote! {

        let r: Vec<String> = file!().split("/").map(|x| x.to_string()).collect();
        let x = r[r.len() - 2].clone();
        let mut folder = x.chars();
        let mut name = match folder.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + folder.as_str(),
        };
        if let Some(idx) = name.find("_") {
            name.remove(idx);

            name.replace_range(idx..idx+1, name.get(idx..idx+1).unwrap().to_uppercase().as_str())
        }
        name
    };

    let item = quote! (
    impl ModelCollection for #ident {
        fn get_db_name() -> String {
            #db_name
        }
        fn get_col_name() -> String {
            #col_name.to_string()
        }
    });

    TokenStream::from(item)
}

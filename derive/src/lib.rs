use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemFn, Ident, Block, punctuated::Punctuated, Token, parse::Parser};

#[proc_macro_attribute]
pub fn entrypoint(initializer_type: TokenStream, input: TokenStream) -> TokenStream {
    let initializer_type: Punctuated<Ident, Token![,]> = Punctuated::<Ident, Token![,]>::parse_terminated.parse(initializer_type).unwrap();
    assert!(initializer_type.len() == 1, "You must specify the type of initializer to use when using the `entrypoint()` macro");
    let initializer_type: String = initializer_type.get(0).unwrap().to_string();
    match initializer_type.as_str() {
        "ModInitializer" | "ClientModInitializer" | "DedicatedServerModInitializer" => {},
        str => panic!("{:?} is not a valid initializer type.\nValid types: ModInitializer, ClientModInitializer, DedicatedServerInitilizer", str)
    }

    let input: ItemFn = parse_macro_input!(input as ItemFn);
    let fn_name: &Ident = &input.sig.ident;
    let fn_block: &Box<Block> = &input.block;

    let java_package: String = std::env::var("JAVA_PACKAGE")
        .expect("JAVA_PACKAGE not set; add `oxidizemc_fabric::build::configure()` to your build.rs");
    let jni_fn_name: Ident = format_ident!("Java_{}_Natives_{}_1init", java_package.replace(".", "_"), initializer_type);

    let expanded: proc_macro2::TokenStream = quote! {
        #[unsafe(no_mangle)]
        extern "system" fn #jni_fn_name(
            env: ::oxidizemc_fabric::sys::__Env,
            _this: *mut (),
        ) {
            ::oxidizemc_fabric::__java_entrypoint(env, #fn_name);
        }

        fn #fn_name() {
            #fn_block
        }
    };

    TokenStream::from(expanded)
}

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{ReturnType, ItemFn};

fn check_lua_function(input: &mut ItemFn) {
	assert!(input.sig.asyncness.is_none(), "Cannot be async");
	assert!(input.sig.constness.is_none(), "Cannot be const");
	assert!(input.sig.inputs.len() == 1, "There can only be one argument, and it should be a pointer to the Lua state (gmod::lua::State)");
	assert!(matches!(&input.sig.output, ReturnType::Type(_, r#type) if r#type.to_token_stream().to_string() == "i32"), "The output must be an i32, representing the number of return values of the function");
	assert!(input.sig.abi.is_none() || input.sig.abi.as_ref().and_then(|abi| abi.name.as_ref()).map(|abi| abi.value() == "C-unwind").unwrap_or(true), "Do not specify an ABI");
	input.sig.abi = Some(syn::parse_quote!(extern "C-unwind"));
}

#[proc_macro_attribute]
pub fn gmod13_open(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
	let mut input = parse_macro_input!(tokens as ItemFn);
	check_lua_function(&mut input);
	input.block.stmts.insert(0, syn::parse2(quote!(#[allow(unused_unsafe)] unsafe { ::gmod::lua::load() })).unwrap());
	input.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn gmod13_close(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
	let mut input = parse_macro_input!(tokens as ItemFn);
	check_lua_function(&mut input);
	input.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn lua_function(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
	let mut input = parse_macro_input!(tokens as ItemFn);
	check_lua_function(&mut input);
	input.into_token_stream().into()
}
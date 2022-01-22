#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::ItemFn;

fn check_lua_function(input: &mut ItemFn) {
	assert!(input.sig.asyncness.is_none(), "Cannot be async");
	assert!(input.sig.constness.is_none(), "Cannot be const");
	assert!(input.sig.inputs.len() == 1, "There can only be one argument, and it should be a pointer to the Lua state (gmod::lua::State)");
	assert!(input.sig.abi.is_none() || input.sig.abi.as_ref().and_then(|abi| abi.name.as_ref()).map(|abi| abi.value() == "C-unwind").unwrap_or(true), "Do not specify an ABI");
	input.sig.abi = Some(syn::parse_quote!(extern "C-unwind"));
}

fn genericify_return(item_fn: &mut ItemFn) {
	let stmts = std::mem::take(&mut item_fn.block.stmts);
	let output = std::mem::replace(&mut item_fn.sig.output, parse_quote!(-> i32));
	item_fn.block.stmts = vec![syn::parse2(quote!({::gmod::lua::ValuesReturned::from((|| #output {#(#stmts);*})()).into()})).unwrap()];
}

#[proc_macro_attribute]
pub fn gmod13_open(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
	let mut input = parse_macro_input!(tokens as ItemFn);

	let lua_ident = format_ident!("{}", match &input.sig.inputs[0] {
		syn::FnArg::Typed(arg) => arg.pat.to_token_stream().to_string(),
		_ => unreachable!(),
	});

	// Capture the Lua state
	input.block.stmts.insert(0, syn::parse2(quote!(::gmod::lua::__set_state__internal(#lua_ident);)).unwrap());

	// Load lua_shared
	input.block.stmts.insert(0, syn::parse2(quote!(#[allow(unused_unsafe)] unsafe { ::gmod::lua::load() })).unwrap());

	// Make sure it's valid
	check_lua_function(&mut input);

	// No mangling
	input.attrs.push(parse_quote!(#[no_mangle]));

	// Make the return type nice and dynamic
	genericify_return(&mut input);

	input.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn gmod13_close(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
	let mut input = parse_macro_input!(tokens as ItemFn);

	// Make sure it's valid
	check_lua_function(&mut input);

	// No mangling
	input.attrs.push(parse_quote!(#[no_mangle]));

	// Shutdown gmcl thread if it's running
	#[cfg(feature = "gmcl")] {
		let stmts = std::mem::take(&mut input.block.stmts);
		input.block.stmts = vec![syn::parse2(quote!({
			let ret = (|| {#(#stmts);*})();
			::gmod::gmcl::restore_stdout();
			ret
		})).unwrap()];
	}

	// Make the return type nice and dynamic
	genericify_return(&mut input);

	input.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn lua_function(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
	let mut input = parse_macro_input!(tokens as ItemFn);

	// Make sure it's valid
	check_lua_function(&mut input);

	// Make the return type nice and dynamic
	genericify_return(&mut input);

	input.into_token_stream().into()
}
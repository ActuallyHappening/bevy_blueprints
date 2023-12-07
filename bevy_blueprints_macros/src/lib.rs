use quote::quote;
use syn::{DeriveInput, parse_macro_input};

struct Config {
	generate_bundle: GenerateBundle,
}

struct GenerateBundle {
	
}

// #[bevy_blueprints] attribute macro
#[proc_macro_attribute]
pub fn bevy_blueprints(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	// let item: proc_macro2::TokenStream = item.into();
	let item = parse_macro_input!(item as DeriveInput);

	// let top_level_attributes = item.attrs;

	let ret = quote!{
		#item
	};

	ret.into()
}
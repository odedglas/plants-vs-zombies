mod behavior;
mod behavior_field;

extern crate proc_macro;

use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput};
use behavior_field::{get_derived_behavior_feilds, BehaviorMacroInput};
use behavior::{impl_base_bevhior_derive};

#[proc_macro_derive(BaseBehavior)]
pub fn base_bahvior_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);

    impl_base_bevhior_derive(parsed_input)
}

#[proc_macro_attribute]
pub fn derive_behavior_fields(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(attr as BehaviorMacroInput);

    get_derived_behavior_feilds(input, item)
}

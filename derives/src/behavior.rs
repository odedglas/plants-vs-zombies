use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub fn impl_base_bevhior_derive(parsed_input: DeriveInput) -> TokenStream {
    let struct_name = parsed_input.ident;

    match parsed_input.data {
        Data::Struct(_s) => {
            let tokens = quote! {
                use std::any::Any;
                use super::base::BehaviorState;

                impl BehaviorState for #struct_name {
                    fn start(&mut self, _now: f64) {
                        self.running = true;
                    }

                    fn stop(&mut self, _now: f64) {
                        self.running = false;
                    }

                    fn is_running(&self) -> bool {
                        self.running
                    }

                    fn clean_interaction(&mut self) {
                        self.interaction_active = false
                    }

                    fn set_sprite_id(&mut self, sprite_id: String) {
                        self.sprite_id = sprite_id;
                    }

                    fn as_any(&mut self) -> &mut dyn Any {
                        self
                    }
                }
            };

            proc_macro::TokenStream::from(tokens)
        }
        other => panic!("Cannot implement BehaviorState for: {:?}", other),
    }
}
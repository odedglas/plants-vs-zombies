use proc_macro::{Delimiter, Group, Ident, TokenStream, TokenTree};
use quote::quote;
use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
pub enum BehaviorDerivedType {
    DEFAULT
}

#[derive(Debug)]
pub struct BehaviorMacroInput {
    pub kind: BehaviorDerivedType,
}

impl BehaviorMacroInput {
    pub fn new(kind: String) -> Self {
        let derived_kind = match kind {
            _ => BehaviorDerivedType::DEFAULT
        };

        Self { kind: derived_kind }
    }
}

impl Parse for BehaviorMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let kind = input.parse::<syn::LitStr>()?;

        Ok(BehaviorMacroInput::new(kind.value()))
    }
}

impl BehaviorMacroInput {
    pub fn get_fields(&self) -> Vec<TokenStream> {
        match &self.kind {
            BehaviorDerivedType::DEFAULT => vec![
                quote!(running: bool,).into(),
                quote!(interaction_active: bool,).into(),
                quote!(sprite_id: String,).into(),
            ]
        }
    }
}

pub fn get_derived_behavior_feilds(input: BehaviorMacroInput, item: TokenStream) -> TokenStream {
    let mut found_struct = false;

    item.into_iter()
        .map(|token_tree| match &token_tree {
            &TokenTree::Ident(ref ident) if is_struct(ident) => {
                found_struct = true;
                token_tree
            }
            &TokenTree::Group(ref group) if is_brace(group.delimiter()) && found_struct => {
                let mut stream = TokenStream::new();
                let fields = input.get_fields();

                stream.extend(fields.into_iter());
                stream.extend(group.stream());

                TokenTree::Group(Group::new(Delimiter::Brace, stream))
            }
            _ => token_tree,
        })
        .collect()
}

fn is_struct(ident: &Ident) -> bool {
    ident.to_string() == "struct"
}

fn is_brace(delimiter: Delimiter) -> bool {
    matches!(delimiter, Delimiter::Brace)
}
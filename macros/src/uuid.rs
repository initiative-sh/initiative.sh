use proc_macro::TokenStream;
use quote::quote;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    if !input.is_empty() {
        return Err(format!("Expected no arguments, got `{}`.", input));
    }

    Ok(quote! {
        #[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
        #[serde(from = "uuid::Uuid", into = "uuid::Uuid")]
        pub struct Uuid(uuid::Uuid);

        impl Uuid {
            pub fn new() -> Self {
                Self(uuid::Uuid::new_v4())
            }

            pub fn as_ref(&self) -> &uuid::Uuid {
                &self.0
            }
        }

        impl std::fmt::Display for Uuid {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<uuid::Uuid> for Uuid {
            fn from(uuid: uuid::Uuid) -> Self {
                Self(uuid)
            }
        }

        impl From<Uuid> for uuid::Uuid {
            fn from(uuid: Uuid) -> Self {
                uuid.0
            }
        }
    }
    .into())
}

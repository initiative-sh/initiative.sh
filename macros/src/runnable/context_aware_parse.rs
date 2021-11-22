use super::CommandEnum;
use proc_macro2::TokenStream;
//use quote::quote;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let command_enum: CommandEnum = input.try_into()?;
    Err(format!("{:?}", command_enum))
}

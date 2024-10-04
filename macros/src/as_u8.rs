use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let mut input_iter = input.into_iter().peekable();
    let mut output_stream = TokenStream::new();

    let as_u8 = TokenTree::Group(Group::new(
        Delimiter::None,
        [
            TokenTree::Ident(Ident::new("as", Span::call_site())),
            TokenTree::Ident(Ident::new("u8", Span::call_site())),
        ]
        .into_iter()
        .collect(),
    ));

    while let Some(token) = input_iter.next() {
        if let TokenTree::Punct(punct) = &token {
            if punct.as_char() == ',' {
                output_stream.extend([as_u8.clone(), token]);
                continue;
            }
        }

        if input_iter.peek().is_none() {
            output_stream.extend([token, as_u8.clone()]);
        } else {
            output_stream.extend([token]);
        }
    }

    Ok(TokenTree::Group(Group::new(Delimiter::Bracket, output_stream)).into())
}

use cairo_lang_macro::{TextSpan, Token, TokenStream, TokenTree};

pub fn str_to_token_stream(s: &str) -> TokenStream {
    TokenStream::new(vec![create_single_token(s)])
}

pub fn create_single_token(content: impl AsRef<str>) -> TokenTree {
    TokenTree::Ident(Token::new(content, TextSpan::call_site()))
}

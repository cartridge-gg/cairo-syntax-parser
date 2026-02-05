use cairo_lang_macro::{TextSpan, Token, TokenStream, TokenTree};

pub fn str_to_token_stream(s: &str) -> TokenStream {
    TokenStream::new(vec![create_single_token(s)])
}

pub fn create_single_token(content: impl AsRef<str>) -> TokenTree {
    TokenTree::Ident(Token::new(content, TextSpan::call_site()))
}

pub trait Slice {
    type Element;
    fn elements(&self) -> &[Self::Element];
}

// Implementation for Vec and other types that Deref to slice
impl<E> Slice for Vec<E> {
    type Element = E;
    fn elements(&self) -> &[Self::Element] {
        self.as_slice()
    }
}

// Implementation for slices
impl<E> Slice for [E] {
    type Element = E;
    fn elements(&self) -> &[Self::Element] {
        self
    }
}

// Implementation for fixed-size arrays
impl<E, const N: usize> Slice for [E; N] {
    type Element = E;
    fn elements(&self) -> &[Self::Element] {
        self.as_slice()
    }
}

impl<E> Slice for &[E] {
    type Element = E;
    fn elements(&self) -> &[Self::Element] {
        self
    }
}

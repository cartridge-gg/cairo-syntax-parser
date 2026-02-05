use crate::cformat::formatter::Formatted;
use smallvec::SmallVec;
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult, Write};

pub trait FormatOp<'a, T: Display + ?Sized> {
    fn format<W: Write>(&self, obj: &T, buf: &mut W) -> FmtResult;
    fn formatted(self, obj: &'a T) -> Formatted<'a, T, Self>
    where
        T: Sized,
        Self: Sized,
    {
        Formatted::new(obj, self)
    }
}
pub struct Affixes<'a> {
    prefixes: SmallVec<[Token<'a>; 4]>,
    suffixes: SmallVec<[Token<'a>; 4]>,
}

pub struct Affix<'a> {
    prefix: Token<'a>,
    suffix: Token<'a>,
}

trait FmtWrite: Display {
    fn write<W: Write>(&self, buf: &mut W) -> FmtResult {
        write!(buf, "{}", self)
    }
}

impl<T: Display> FmtWrite for T {}

trait TokensTrait<'a> {
    fn write_tokens<W: Write>(&self, buf: &mut W) -> FmtResult;
    fn write_tokens_rev<W: Write>(&self, buf: &mut W) -> FmtResult;
}

impl TokensTrait<'_> for [Token<'_>] {
    fn write_tokens<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.iter().try_for_each(|token| token.write_token(buf))
    }
    fn write_tokens_rev<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.iter()
            .rev()
            .try_for_each(|token| token.write_token(buf))
    }
}

pub struct Prefix<'a>(pub Token<'a>);
pub struct Suffix<'a>(pub Token<'a>);

impl<'a, T> FormatOp<'_, T> for Affixes<'_>
where
    T: Display + Sized,
{
    fn format<W: Write>(&self, obj: &T, buf: &mut W) -> FmtResult {
        let Affixes { prefixes, suffixes } = self;
        prefixes.write_tokens_rev(buf)?;
        obj.write(buf)?;
        suffixes.write_tokens(buf)
    }
}

impl<'a, T> FormatOp<'_, T> for Affix<'_>
where
    T: Display + Sized,
{
    fn format<W: Write>(&self, obj: &T, buf: &mut W) -> FmtResult {
        let Affix { prefix, suffix } = self;
        write!(buf, "{prefix}{obj}{suffix}",)
    }
}

impl<'a, T> FormatOp<'_, T> for Prefix<'_>
where
    T: Display + Sized,
{
    fn format<W: Write>(&self, obj: &T, buf: &mut W) -> FmtResult {
        write!(buf, "{self}{obj}",)
    }
}

impl<'a, T> FormatOp<'_, T> for Suffix<'_>
where
    T: Display + Sized,
{
    fn format<W: Write>(&self, obj: &T, buf: &mut W) -> FmtResult {
        write!(buf, "{obj}{self}",)
    }
}

impl Display for Prefix<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        self.0.write_token(f)
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        self.write_token(f)
    }
}

impl Display for Suffix<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        self.0.write_token(f)
    }
}

pub enum Token<'a> {
    Char(char),
    Str(Cow<'a, str>), // Can be borrowed or owned
}

impl Token<'_> {
    pub fn write_token<W: Write>(&self, buf: &mut W) -> FmtResult {
        match self {
            Token::Str(s) => buf.write_str(s),
            Token::Char(c) => buf.write_char(*c),
        }
    }
}

impl<'a> From<char> for Token<'a> {
    fn from(c: char) -> Self {
        Token::Char(c)
    }
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(s: &'a str) -> Self {
        Token::Str(Cow::Borrowed(s))
    }
}

impl<'a> From<String> for Token<'a> {
    fn from(s: String) -> Self {
        Token::Str(Cow::Owned(s))
    }
}

pub trait MaybeTokenTrait<'a> {
    fn maybe_token(self) -> Option<Token<'a>>;
}

impl<'a> MaybeTokenTrait<'a> for () {
    fn maybe_token(self) -> Option<Token<'a>> {
        None
    }
}

impl<'a> MaybeTokenTrait<'a> for char {
    fn maybe_token(self) -> Option<Token<'a>> {
        Some(Token::Char(self))
    }
}

impl<'a> MaybeTokenTrait<'a> for &'a str {
    fn maybe_token(self) -> Option<Token<'a>> {
        Some(Token::Str(Cow::Borrowed(self)))
    }
}

impl<'a> MaybeTokenTrait<'a> for String {
    fn maybe_token(self) -> Option<Token<'a>> {
        Some(Token::Str(Cow::Owned(self)))
    }
}

pub trait TokensTrait<'a> {
    fn write_tokens<W: Write>(&self, buf: &mut W) -> FmtResult;
    fn write_tokens_rev<W: Write>(&self, buf: &mut W) -> FmtResult;
}

impl TokensTrait<'_> for [Token<'_>] {
    fn write_tokens<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.iter().try_for_each(|token| token.write_token(buf))
    }
    fn write_tokens_rev<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.iter()
            .rev()
            .try_for_each(|token| token.write_token(buf))
    }
}

impl Default for Affixes<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_enclosers(c: char) -> (char, char) {
    match c {
        '(' => ('(', ')'),
        '{' => ('{', '}'),
        '[' => ('[', ']'),
        '<' => ('<', '>'),
        _ => (c, c),
    }
}

impl<'a> Affixes<'a> {
    pub fn new() -> Self {
        Self {
            prefixes: Default::default(),
            suffixes: Default::default(),
        }
    }

    pub fn format_prefixes<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.prefixes.write_tokens_rev(buf)
    }
    pub fn format_suffixes<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.suffixes.write_tokens(buf)
    }
    pub fn format<T: Display + ?Sized, W: Write>(&self, obj: &T, buf: &mut W) -> FmtResult {
        self.format_prefixes(buf)?;
        write!(buf, "{}", obj)?;
        self.format_suffixes(buf)
    }
}

pub trait AffixesOps<'a>: Sized {
    fn prefix_token(&mut self, val: Token<'a>);
    fn suffix_token(&mut self, val: Token<'a>);
    fn prefix<T: Into<Token<'a>>>(&mut self, val: T) {
        self.prefix_token(val.into());
    }
    fn suffix<T: Into<Token<'a>>>(&mut self, val: T) {
        self.suffix_token(val.into());
    }
    fn wrap<T: Into<Token<'a>>, U: Into<Token<'a>>>(&mut self, prefix: T, suffix: U) {
        self.prefix(prefix);
        self.suffix(suffix);
    }
    fn enclose(&mut self, val: char) {
        let (open, close) = get_enclosers(val);
        self.wrap(open, close);
    }
    fn prefixed<T: Into<Token<'a>>>(mut self, val: T) -> Self {
        self.prefix(val);
        self
    }
    fn suffixed<T: Into<Token<'a>>>(mut self, val: T) -> Self {
        self.suffix(val);
        self
    }
    fn wrapped<T: Into<Token<'a>>, U: Into<Token<'a>>>(mut self, prefix: T, suffix: U) -> Self {
        self.wrap(prefix, suffix);
        self
    }
    fn enclosed(mut self, val: char) -> Self {
        self.enclose(val);
        self
    }
}

impl AffixesOps<'_> for Affixes<'_> {
    fn prefix_token(&mut self, val: Token<'_>) {
        self.prefixes.push(val);
    }
    fn suffix_token(&mut self, val: Token<'_>) {
        self.suffixes.push(val);
    }
}

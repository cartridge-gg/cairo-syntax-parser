use crate::Slice;
use smallvec::SmallVec;
use starknet_types_core::felt::Felt;
use std::borrow::Cow;
use std::fmt::{Display, Result as FmtResult, Write};

impl CWrite for [u8; 32] {
    fn cwrite<W: Write>(&self, buf: &mut W) -> FmtResult {
        buf.write_str("0x")?;
        for byte in self.iter() {
            write!(buf, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl CWrite for [u8; 31] {
    fn cwrite<W: Write>(&self, buf: &mut W) -> FmtResult {
        buf.write_str("0x00")?;
        for byte in self.iter() {
            write!(buf, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl CWrite for String {
    fn cwrite<W: Write>(&self, buf: &mut W) -> FmtResult {
        buf.write_str(self)
    }
}

impl CWrite for &str {
    fn cwrite<W: Write>(&self, buf: &mut W) -> FmtResult {
        buf.write_str(self)
    }
}

impl CWrite for Felt {
    fn cwrite<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.to_fixed_hex_string().cwrite(buf)
    }
}

impl<T> CWriteSlice for T
where
    T: Slice,
    T::Element: CWrite,
{
}

pub trait CWrite {
    fn cwrite<W: Write>(&self, buf: &mut W) -> FmtResult;
    fn size_hint(&self) -> usize {
        let mut sizer = Sizer::new();
        self.cwrite(&mut sizer).unwrap();
        sizer.size()
    }
    fn prefix<'a, T: TokenTrait<'a>>(&'a self, val: T) -> ElementWriter<'a, Self>
    where
        Self: Sized,
    {
        ElementWriter::new_empty(self).prefixed(val)
    }

    fn suffix<'a, T: TokenTrait<'a>>(&'a self, val: T) -> ElementWriter<'a, Self>
    where
        Self: Sized,
    {
        ElementWriter::new_empty(self).suffixed(val)
    }

    fn wrap<'a, T: TokenTrait<'a>, U: TokenTrait<'a>>(
        &'a self,
        prefix: T,
        suffix: U,
    ) -> ElementWriter<'a, Self>
    where
        Self: Sized,
    {
        ElementWriter::new_empty(self).wrapped(prefix, suffix)
    }

    fn enclose<'a>(&'a self, val: char) -> ElementWriter<'a, Self>
    where
        Self: Sized,
    {
        ElementWriter::new_empty(self).enclosed(val)
    }
}

pub trait CWriteSlice
where
    Self: Slice,
    Self::Element: CWrite,
{
    fn prefix<'a, T: TokenTrait<'a>>(&'a self, val: T) -> ElementWriter<'a, Self>
    where
        Self: Sized,
    {
        ElementWriter::new_empty(self).prefixed(val)
    }

    fn suffix<'a, T: TokenTrait<'a>>(&'a self, val: T) -> ElementWriter<'a, Self>
    where
        Self: Sized,
    {
        ElementWriter::new_empty(self).suffixed(val)
    }

    fn wrap<'a, T: TokenTrait<'a>, U: TokenTrait<'a>>(
        &'a self,
        prefix: T,
        suffix: U,
    ) -> ElementWriter<'a, Self>
    where
        Self: Sized,
    {
        ElementWriter::new_empty(self).wrapped(prefix, suffix)
    }

    fn enclose<'a>(&'a self, val: char) -> ElementWriter<'a, Self>
    where
        Self: Sized,
    {
        ElementWriter::new_empty(self).enclosed(val)
    }
    fn join<'a, T: MaybeTokenTrait<'a>>(&'a self, val: T) -> SliceWriter<'a, Self>
    where
        Self: Sized,
    {
        ElementWriter::new_empty(self).join(val)
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

pub trait TokenTrait<'a> {
    fn to_token(self) -> Token<'a>;
}

impl<'a> TokenTrait<'a> for char {
    fn to_token(self) -> Token<'a> {
        Token::Char(self)
    }
}

impl<'a> TokenTrait<'a> for &'a str {
    fn to_token(self) -> Token<'a> {
        Token::Str(Cow::Borrowed(self))
    }
}

impl<'a> TokenTrait<'a> for String {
    fn to_token(self) -> Token<'a> {
        Token::Str(Cow::Owned(self))
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

pub struct ElementOps<'a> {
    prefixes: SmallVec<[Token<'a>; 4]>,
    suffixes: SmallVec<[Token<'a>; 4]>,
}

pub struct SliceOps<'a> {
    element_ops: ElementOps<'a>,
    slice_ops: ElementOps<'a>,
    delimiter: Option<Token<'a>>,
}

pub struct ElementWriter<'a, T> {
    obj: &'a T,
    ops: ElementOps<'a>,
}

pub struct SliceWriter<'a, T>
where
    T: Slice,
    T::Element: CWrite,
{
    slice: &'a T,
    ops: SliceOps<'a>,
}

impl<'a, S> SliceWriter<'a, S>
where
    S: Slice,
    S::Element: CWrite,
{
    pub fn new(slice: &'a S, ops: SliceOps<'a>) -> Self {
        Self { slice, ops }
    }
    pub fn apply<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.ops.apply(self.slice, buf)
    }
    pub fn len(&self) -> usize {
        self.slice.elements().len()
    }
}

impl<'a, E> ElementWriter<'a, E> {
    pub fn new(obj: &'a E, ops: ElementOps<'a>) -> Self {
        Self { obj, ops }
    }
    pub fn new_empty(obj: &'a E) -> Self {
        Self {
            obj,
            ops: ElementOps::new(),
        }
    }

    pub fn apply<W: Write>(&self, buf: &mut W) -> FmtResult
    where
        E: CWrite,
    {
        self.ops.apply(self.obj, buf)
    }

    pub fn join<T: MaybeTokenTrait<'a>>(self, val: T) -> SliceWriter<'a, E>
    where
        E: Slice,
        E::Element: CWrite,
    {
        SliceWriter {
            slice: self.obj,
            ops: SliceOps {
                element_ops: self.ops,
                slice_ops: ElementOps::new(),
                delimiter: val.maybe_token(),
            },
        }
    }
    pub fn to_string(&self) -> String
    where
        E: CWrite,
    {
        let mut buf = String::new();
        self.apply(&mut buf).unwrap();
        buf
    }
}

impl<'a, E> Display for ElementWriter<'a, E>
where
    E: CWrite,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        self.apply(f)
    }
}

impl<'a, E> Display for SliceWriter<'a, E>
where
    E: Slice,
    E::Element: CWrite,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        self.apply(f)
    }
}

impl Default for ElementOps<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CWriter<'a> {
    obj: Box<dyn CWrite + 'a>,
    ops: ElementOps<'a>,
}

impl<'a> CWriter<'a> {
    pub fn new<T: CWrite + 'a>(obj: T) -> Self {
        Self {
            obj: Box::new(obj),
            ops: ElementOps::new(),
        }
    }
}

pub struct CSliceWriter<'a, T: CWrite> {
    elements: &'a [T],
    element_ops: ElementOps<'a>,
    delimiter: Option<Token<'a>>,
}

impl<'a> ElementOps<'a> {
    pub fn new() -> Self {
        Self {
            prefixes: Default::default(),
            suffixes: Default::default(),
        }
    }

    pub fn join<T: MaybeTokenTrait<'a>>(self, val: T) -> SliceOps<'a> {
        SliceOps::new(self, Default::default(), val)
    }

    pub fn apply_prefixes<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.prefixes.write_tokens_rev(buf)
    }
    pub fn apply_suffixes<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.suffixes.write_tokens(buf)
    }

    pub fn apply<T: CWrite, W: Write>(&self, obj: &T, buf: &mut W) -> FmtResult {
        self.apply_prefixes(buf)?;
        obj.cwrite(buf)?;
        self.apply_suffixes(buf)
    }
}

impl Default for SliceOps<'_> {
    fn default() -> Self {
        Self {
            element_ops: Default::default(),
            slice_ops: Default::default(),
            delimiter: Default::default(),
        }
    }
}

impl<'a> SliceOps<'a> {
    pub fn new<T: MaybeTokenTrait<'a>>(
        element_ops: ElementOps<'a>,
        slice_ops: ElementOps<'a>,
        delimiter: T,
    ) -> Self {
        Self {
            element_ops,
            slice_ops,
            delimiter: delimiter.maybe_token(),
        }
    }
    pub fn apply<T, W: Write>(&self, slice: &T, buf: &mut W) -> FmtResult
    where
        T: Slice,
        T::Element: CWrite,
    {
        let SliceOps {
            element_ops,
            slice_ops,
            delimiter: delimiters,
        } = &self;
        slice_ops.apply_prefixes(buf)?;
        let mut elements = slice.elements().iter();
        match elements.next() {
            None => {}
            Some(first) => {
                element_ops.apply(first, buf)?;
                for element in elements {
                    if let Some(delims) = delimiters {
                        delims.write_token(buf)?;
                    }
                    element_ops.apply(element, buf)?;
                }
            }
        }
        slice_ops.apply_suffixes(buf)
    }
}

pub trait AppendWriteOps<'a>: Sized {
    fn prefix_token(&mut self, val: Token<'a>);
    fn suffix_token(&mut self, val: Token<'a>);
    fn prefix<T: TokenTrait<'a>>(&mut self, val: T) {
        self.prefix_token(val.to_token());
    }
    fn suffix<T: TokenTrait<'a>>(&mut self, val: T) {
        self.suffix_token(val.to_token());
    }
    fn wrap<T: TokenTrait<'a>, U: TokenTrait<'a>>(&mut self, prefix: T, suffix: U) {
        self.prefix(prefix);
        self.suffix(suffix);
    }
    fn enclose(&mut self, val: char) {
        let (open, close) = get_enclosers(val);
        self.wrap(open, close);
    }
    fn prefixed<T: TokenTrait<'a>>(mut self, val: T) -> Self {
        self.prefix(val);
        self
    }
    fn suffixed<T: TokenTrait<'a>>(mut self, val: T) -> Self {
        self.suffix(val);
        self
    }
    fn wrapped<T: TokenTrait<'a>, U: TokenTrait<'a>>(mut self, prefix: T, suffix: U) -> Self {
        self.wrap(prefix, suffix);
        self
    }
    fn enclosed(mut self, val: char) -> Self {
        self.enclose(val);
        self
    }
}

impl<'a> AppendWriteOps<'a> for ElementOps<'a> {
    fn prefix_token(&mut self, val: Token<'a>) {
        self.prefixes.push(val);
    }
    fn suffix_token(&mut self, val: Token<'a>) {
        self.suffixes.push(val);
    }
}

impl<'a> AppendWriteOps<'a> for SliceOps<'a> {
    fn prefix_token(&mut self, val: Token<'a>) {
        self.slice_ops.prefix_token(val);
    }
    fn suffix_token(&mut self, val: Token<'a>) {
        self.slice_ops.suffix_token(val);
    }
}

impl<'a, T> AppendWriteOps<'a> for ElementWriter<'a, T> {
    fn prefix_token(&mut self, val: Token<'a>) {
        self.ops.prefix_token(val);
    }
    fn suffix_token(&mut self, val: Token<'a>) {
        self.ops.suffix_token(val);
    }
}

impl<'a, T> AppendWriteOps<'a> for SliceWriter<'a, T>
where
    T: Slice,
    T::Element: CWrite,
{
    fn prefix_token(&mut self, val: Token<'a>) {
        self.ops.prefix_token(val);
    }
    fn suffix_token(&mut self, val: Token<'a>) {
        self.ops.suffix_token(val);
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

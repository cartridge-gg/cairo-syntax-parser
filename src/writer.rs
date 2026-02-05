use smallvec::SmallVec;
use starknet_types_core::felt::Felt;
use std::borrow::Cow;
use std::fmt::{Display, Result as FmtResult, Write};
use super::FormatOp;


pub struct Formatted<'a, T: CWrite, Op: FormatOp<'a, T> = Affixes<'a>> {
    obj: &'a T,
    op: Op,
}

pub struct FormattedSeq<'a, T: CWrite, Op: FormatOp<'a, T> = Affixes<'a>> {
    elements: &'a [T],
    ops: Op,
}

pub trait Formatter<'a, T: CWrite, Op: FormatOp<'a, T>> {
    fn apply<W: Write>(&self, buf: &mut W) -> FmtResult;
    fn to_string(&self) -> String {
        let mut buf = String::new();
        self.apply(&mut buf).unwrap();
        buf
    }
}

pub trait SeqFormatter<'a, T: CWrite, Op: FormatOp<'a, T>> {
    type Element: CWrite;
    fn iter(&'a self) -> FormattedIter<'a, T, Op>;
    fn to_string(&self) -> String {
        let mut buf = String::new();
        self.apply(&mut buf).unwrap();
        buf
    }
}

pub trait FormatMany<'a, T: CWrite, Op: FormatOp<'a, T>> {
    fn elements(&self) -> &[T];

    fn iter(&'a self) -> FormattedIter<'a, T, Op> {
        FormattedIter {
            elements: self.elements(),
            ops: self.ops(),
            index: 0,
        }
    }
}

pub struct ValueOrTuple<'a, T: FormatMany>  {
    elements: T,
}

pub struct CDelimited<'a, T: CWrite, Op: FormatOp<'a, T> = Affixes<'a>> {
    elements_writer: CFormatEach<'a, T, Op>,
    delimiter: Option<Token<'a>>,
}

pub struct FormattedIter<'a, T: CWrite, Op: FormatOp<'a, T>> {
    elements: &'a [T],
    op: &'a Op,
    index: usize,
}




impl<'a, T: CWrite + ?Sized> FormatOp<'a, T> for Affixes<'a> {
    fn apply_op<W: Write>(&self, obj: &T, buf: &mut W) -> FmtResult {
        self.apply(obj, buf)
    }
}

impl<'a, T: CWrite, Op: FormatOp<'a, T>> FormattedIter<'a, T, Op> {
    pub fn next<W: Write>(&mut self, buf: &mut W) -> Option<FmtResult> {
        if self.index >= self.elements.len() {
            return None;
        }

        let result = self.ops.apply_op(&self.elements[self.index], buf);
        self.index += 1;
        Some(result)
    }

    pub fn remaining(&self) -> usize {
        self.elements.len().saturating_sub(self.index)
    }
}



pub trait CWrite {
    fn cwrite<W: Write>(&self, buf: &mut W) -> FmtResult;
}






impl<'a, T: CWrite> CFormater<'a, T, Affixes<'a>> {
    pub fn new(obj: &'a T) -> Self {
        Self {
            obj,
            op: Affixes::new(),
        }
    }
}

impl<'a, T: CWrite, Op: FormatOp<'a, T>> CFormater<'a, T, Op> {
    pub fn apply<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.op.apply_op(self.obj, buf)
    }

    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        self.apply(&mut buf).unwrap();
        buf
    }
}

impl<'a, T: CWrite, Op: FormatOp<'a, T>> CWrite for CFormater<'a, T, Op> {
    fn cwrite<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.apply(buf)
    }
}

impl<'a, T: CWrite, Op: FormatOp<'a, T>> Display for CFormater<'a, T, Op> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        self.apply(f)
    }
}

impl<'a, T: CWrite> AppendWriteOps<'a> for CFormater<'a, T, Affixes<'a>> {
    fn prefix_token(&mut self, val: Token<'a>) {
        self.op.prefix_token(val);
    }
    fn suffix_token(&mut self, val: Token<'a>) {
        self.op.suffix_token(val);
    }
}

impl<'a, T: CWrite, Op: FormatOp<'a, T>> CWrite for ValueOrTuple<'a, T, Op> {
    fn cwrite<W: Write>(&self, buf: &mut W) -> FmtResult {
        if self.elements.elements.len() == 1 {
            self.elements.elements[0].cwrite(buf)
        } else {
            buf.write_char('(')?;
            self.elements.apply(buf)?;
            buf.write_char(')')
        }
    }
}

impl<'a, T: CWrite> CFormatEach<'a, T, Affixes<'a>> {
    pub fn new(elements: &'a [T], ops: Affixes<'a>) -> Self {
        Self {
            elements,
            ops,
        }
    }

    pub fn new_empty(elements: &'a [T]) -> Self {
        Self {
            elements,
            ops: Affixes::new(),
        }
    }
}

impl<'a, T: CWrite, Op: FormatOp<'a, T>> CFormatEach<'a, T, Op> {
        CDelimited {
            elements_writer: self,
            delimiter: delimiter.maybe_token(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        self.apply(&mut buf).unwrap();
        buf
    }
}

impl<'a, T: CWrite, Op: FormatOp<'a, T>> FormatMany<'a, T, Op> for CFormatEach<'a, T, Op> {
    fn elements(&self) -> &[T] {
        self.elements
    }

    fn ops(&self) -> &Op {
        &self.ops
    }
}

impl<'a, T: CWrite> AppendWriteOps<'a> for CFormatEach<'a, T, Affixes<'a>> {
    fn prefix_token(&mut self, val: Token<'a>) {
        self.ops.prefix_token(val);
    }
    fn suffix_token(&mut self, val: Token<'a>) {
        self.ops.suffix_token(val);
    }
}

impl<'a, T: CWrite> CDelimited<'a, T, Affixes<'a>> {
    pub fn new(elements: &'a [T], element_ops: Affixes<'a>, delimiter: Option<Token<'a>>) -> Self {
        Self {
            elements_writer: CFormatEach::new(elements, element_ops),
            delimiter,
        }
    }
}

impl<'a, T: CWrite, Op: FormatOp<'a, T>> CDelimited<'a, T, Op> {
    pub fn apply<W: Write>(&self, buf: &mut W) -> FmtResult {
        let mut iter = self.elements_writer.elements.iter();
        if let Some(first) = iter.next() {
            self.elements_writer.ops.apply_op(first, buf)?;
            for elem in iter {
                if let Some(delim) = &self.delimiter {
                    delim.write_token(buf)?;
                }
                self.elements_writer.ops.apply_op(elem, buf)?;
            }
        }
        Ok(())
    }

    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        self.apply(&mut buf).unwrap();
        buf
    }
}

impl<'a, T: CWrite, Op: FormatOp<'a, T>> CWrite for CDelimited<'a, T, Op> {
    fn cwrite<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.apply(buf)
    }
}

impl<'a, T: CWrite, Op: FormatOp<'a, T>> Display for CDelimited<'a, T, Op> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        self.apply(f)
    }
}

impl<'a, T: CWrite> AppendWriteOps<'a> for CDelimited<'a, T, Affixes<'a>> {
    fn prefix_token(&mut self, val: Token<'a>) {
        self.elements_writer.ops.prefix_token(val);
    }
    fn suffix_token(&mut self, val: Token<'a>) {
        self.elements_writer.ops.suffix_token(val);
    }
}





impl<'a, T: CWrite, Op: FormatOp<'a, T>> CWrite for CFormatEach<'a, T, Op> {
    fn cwrite<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.apply(buf)
    }
}

impl<'a, T: CWrite, Op: FormatOp<'a, T>> Display for CFormatEach<'a, T, Op> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        self.apply(f)
    }
}

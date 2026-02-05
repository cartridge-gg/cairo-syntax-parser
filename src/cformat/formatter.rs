use super::FormatOp;
use crate::cformat::op::Token;
use std::fmt::{Display, Result as FmtResult, Write};

pub struct Formatted<'a, T: Display, Op: FormatOp<'a, T>> {
    obj: &'a T,
    op: Op,
}

pub struct FormattedSeq<'a, T: Display, Op: FormatOp<'a, T>> {
    elements: &'a [T],
    ops: Op,
}

pub struct Delimited<'a, S: SeqFormatter<'a>> {
    sequence: S,
    delimiter: Token<'a>,
}

pub struct FormattedIter<'a, T: Display, Op: FormatOp<'a, T>> {
    elements: &'a [T],
    op: &'a Op,
    index: usize,
}

// pub trait Formatter<'a> {
//     fn apply<W: Write>(&self, buf: &mut W) -> FmtResult;
//     fn to_string(&self) -> String {
//         let mut buf = String::new();
//         self.apply(&mut buf).unwrap();
//         buf
//     }
// }

pub trait SeqFormatter<'a> {
    type Element: Display;
    type Op: FormatOp<'a, Self::Element>;
    fn size(&self) -> usize;
    fn elements(&self) -> &[Self::Element];
    fn op(&self) -> &Self::Op;
    fn iter(&'a self) -> FormattedIter<'a, Self::Element, Self::Op>;
    fn fmt_all<W: Write>(&self, buf: &mut W) -> FmtResult {
        let op = self.op();
        self.elements()
            .iter()
            .try_for_each(|elem| op.format(elem, buf))
    }
    fn fmt_index<W: Write>(&self, buf: &mut W, index: usize) -> FmtResult {
        self.op().format(&self.elements()[index], buf)
    }
}

impl<'a, T, Op> Display for Formatted<'a, T, Op>
where
    T: Display,
    Op: FormatOp<'a, T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        self.op.format(self.obj, f)
    }
}

impl<'a, S> Display for Delimited<'a, S>
where
    S: SeqFormatter<'a>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        match self.sequence.elements().split_first() {
            None => Ok(()),
            Some((first, rest)) => {
                self.sequence.op().format(first, f)?;
                for elem in rest {
                    self.delimiter.write_token(f)?;
                    self.sequence.op().format(elem, f)?;
                }
                Ok(())
            }
        }
    }
}

pub struct ValueOrTuple<'a, T: SeqFormatter<'a>>(pub &'a T);

impl<'a, T> Display for ValueOrTuple<'a, T>
where
    T: SeqFormatter<'a>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let size = self.0.size();
        match size {
            0 => f.write_str("()"),
            1 => self.0.fmt_index(f, 0),
            _ => {
                f.write_char('(')?;
                self.0.fmt_index(f, 0)?;
                for i in 1..size {
                    f.write_char(',')?;
                    self.0.fmt_index(f, i)?;
                }
                f.write_char(')')
            }
        }
    }
}

impl<'a, T, Op> Formatted<'a, T, Op>
where
    T: Display,
    Op: FormatOp<'a, T>,
{
    pub fn new(obj: &'a T, op: Op) -> Self {
        Self { obj, op }
    }
}

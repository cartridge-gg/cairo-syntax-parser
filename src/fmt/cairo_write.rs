use crate::{Sizer, Slice};
use starknet_types_core::felt::Felt;
use std::fmt::{Result, Write};

pub trait CairoWrite {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result;
    fn size_hint(&self) -> usize {
        let mut sizer = Sizer::new();
        self.cwrite(&mut sizer).unwrap();
        sizer.size()
    }
    fn cwrite_suffixed_str<W: Write>(&self, buf: &mut W, suffix: &str) -> Result {
        self.cwrite(buf)?;
        buf.write_str(suffix)
    }
    fn cwrite_prefixed<W: Write>(&self, buf: &mut W, prefix: char) -> Result {
        buf.write_char(prefix)?;
        self.cwrite(buf)
    }
    fn cwrite_prefixed_str<W: Write>(&self, buf: &mut W, prefix: &str) -> Result {
        buf.write_str(prefix)?;
        self.cwrite(buf)
    }
    fn cwrite_suffixed<W: Write>(&self, buf: &mut W, suffix: char) -> Result {
        self.cwrite(buf)?;
        buf.write_char(suffix)
    }
    fn cwrite_wrapped<W: Write>(&self, buf: &mut W, prefix: char, suffix: char) -> Result {
        buf.write_char(prefix)?;
        self.cwrite(buf)?;
        buf.write_char(suffix)
    }
    fn cwrite_wrapped_str<W: Write>(&self, buf: &mut W, prefix: &str, suffix: &str) -> Result {
        buf.write_str(prefix)?;
        self.cwrite(buf)?;
        buf.write_str(suffix)
    }
    fn cwrite_parenthesized<W: Write>(&self, buf: &mut W) -> Result {
        self.cwrite_wrapped(buf, '(', ')')
    }
    fn cwrite_braced<W: Write>(&self, buf: &mut W) -> Result {
        self.cwrite_wrapped(buf, '{', '}')
    }
    fn cwrite_bracketed<W: Write>(&self, buf: &mut W) -> Result {
        self.cwrite_wrapped(buf, '[', ']')
    }
}

impl CairoWrite for [u8; 32] {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("0x")?;
        for byte in self.iter() {
            write!(buf, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl CairoWrite for [u8; 31] {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("0x00")?;
        for byte in self.iter() {
            write!(buf, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl CairoWrite for String {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str(self)
    }
}

impl CairoWrite for &str {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str(self)
    }
}

impl CairoWrite for Felt {
    fn cwrite<W: Write>(&self, buf: &mut W) -> Result {
        self.to_fixed_hex_string().cwrite(buf)
    }
}

pub trait CairoWriteSlice
where
    Self: Slice,
    Self::Element: CairoWrite,
{
    fn cwrite_join<W: Write>(&self, buf: &mut W, delimiter: &str) -> Result {
        let elements = self.elements();
        if let Some((first, rest)) = elements.split_first() {
            first.cwrite(buf)?;
            rest.iter()
                .map(|e| e.cwrite_prefixed_str(buf, delimiter))
                .collect::<Result>()?;
        }
        Ok(())
    }
    fn cwrite_delimited<W: Write>(&self, buf: &mut W, delimiter: char) -> Result {
        let elements = self.elements();
        if let Some((first, rest)) = elements.split_first() {
            first.cwrite(buf)?;
            rest.iter()
                .map(|e| e.cwrite_prefixed(buf, delimiter))
                .collect::<Result>()?;
        }
        Ok(())
    }
    fn cwrite_terminated<W: Write>(&self, buf: &mut W, terminator: char) -> Result {
        self.elements()
            .iter()
            .map(|e| e.cwrite_suffixed(buf, terminator))
            .collect::<Result>()?;
        Ok(())
    }
    fn cwrite_terminated_str<W: Write>(&self, buf: &mut W, terminator: &str) -> Result {
        self.elements()
            .iter()
            .map(|e| e.cwrite_suffixed_str(buf, terminator))
            .collect::<Result>()?;
        Ok(())
    }

    fn cwrite_concatenated<W: Write>(&self, buf: &mut W) -> Result {
        self.elements()
            .iter()
            .map(|e| e.cwrite(buf))
            .collect::<Result>()?;
        Ok(())
    }

    fn cwrite_concatenated_wrapped<W: Write>(
        &self,
        buf: &mut W,
        prefix: char,
        suffix: char,
    ) -> Result {
        buf.write_char(prefix)?;
        self.cwrite_concatenated(buf)?;
        buf.write_char(suffix)
    }

    fn cwrite_csv<W: Write>(&self, buf: &mut W) -> Result {
        self.cwrite_join(buf, ", ")
    }
    fn cwrite_block<W: Write>(&self, buf: &mut W) -> Result {
        let elements = self.elements();
        if !elements.is_empty() {
            buf.write_char('\n')?;
            elements.cwrite_terminated(buf, '\n')?;
        }
        Ok(())
    }
    fn cwrite_block_braced<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_char('{')?;
        self.cwrite_block(buf)?;
        buf.write_char('}')
    }
    fn cwrite_tuple<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_char('(')?;
        let elements = self.elements();
        match elements.len() {
            0 => {}
            1 => {
                elements[0].cwrite_suffixed_str(buf, ", ")?;
            }
            _ => self.cwrite_join(buf, ", ")?,
        }
        buf.write_char(')')
    }
    fn cwrite_csv_wrapped<W: Write>(&self, buf: &mut W, prefix: char, suffix: char) -> Result {
        buf.write_char(prefix)?;
        self.cwrite_csv(buf)?;
        buf.write_char(suffix)
    }
    fn cwrite_csv_wrapped_str<W: Write>(&self, buf: &mut W, prefix: &str, suffix: &str) -> Result {
        buf.write_str(prefix)?;
        self.cwrite_csv(buf)?;
        buf.write_str(suffix)
    }
    fn cwrite_csv_braced<W: Write>(&self, buf: &mut W) -> Result {
        self.cwrite_csv_wrapped(buf, '{', '}')
    }
    fn cwrite_csv_parenthesized<W: Write>(&self, buf: &mut W) -> Result {
        self.cwrite_csv_wrapped(buf, '(', ')')
    }
    fn cwrite_csv_bracketed<W: Write>(&self, buf: &mut W) -> Result {
        self.cwrite_csv_wrapped(buf, '[', ']')
    }
    fn cwrite_csv_angled<W: Write>(&self, buf: &mut W) -> Result {
        self.cwrite_csv_wrapped(buf, '<', '>')
    }
    fn cwrite_csv_barred<W: Write>(&self, buf: &mut W) -> Result {
        self.cwrite_csv_wrapped(buf, '|', '|')
    }
    fn cwrite_array<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_str("array![")?;
        self.cwrite_csv(buf)?;
        buf.write_char(']')
    }
    fn cwrite_span<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_char('[')?;
        self.cwrite_csv(buf)?;
        buf.write_str("].span()")
    }

    fn cwrite_fields<W: Write>(&self, buf: &mut W) -> Result {
        let elements = self.elements();
        if !elements.is_empty() {
            buf.write_char('\n')?;
            elements.cwrite_terminated_str(buf, ",\n")?;
        }
        Ok(())
    }
    fn cwrite_fields_braced<W: Write>(&self, buf: &mut W) -> Result {
        buf.write_char('{')?;
        self.cwrite_fields(buf)?;
        buf.write_char('}')
    }
}

impl<T> CairoWriteSlice for T
where
    T: Slice,
    T::Element: CairoWrite,
{
}

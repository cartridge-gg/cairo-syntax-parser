use crate::macros::{
    MacroDeclarationItem, MacroElement, MacroParam, MacroParamKind, MacroRepetition,
    MacroRepetitionOperator, MacroRule, TokenNode, WrappedMacro,
};
use crate::{CairoWrite, CairoWriteSlice};
impl CairoWrite for MacroDeclarationItem {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        self.attributes.cwrite(buf)?;
        self.visibility.cwrite(buf)?;
        buf.write_str("macro ")?;
        self.name.cwrite(buf)?;
        buf.write_str(" {\n")?;
        self.rules.cwrite_block(buf)?;
        buf.write_str("\n}")
    }
}

impl CairoWrite for MacroRule {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        self.lhs.cwrite(buf)?;
        buf.write_str(" => ")?;
        self.rhs.cwrite_concatenated_wrapped(buf, '{', '}')?;
        buf.write_char(';')
    }
}

impl CairoWrite for MacroParam {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        buf.write_char('$')?;
        self.name.cwrite(buf)?;
        if let Some(kind) = &self.kind {
            kind.cwrite_prefixed(buf, ':')?;
        }
        Ok(())
    }
}

impl CairoWrite for MacroParamKind {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        match self {
            MacroParamKind::Identifier(value) | MacroParamKind::Expr(value) => buf.write_str(value),
            MacroParamKind::Missing => Ok(()),
        }
    }
}

impl CairoWrite for MacroRepetition {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        buf.write_str("$")?;
        self.elements.cwrite_concatenated_wrapped(buf, '(', ')')?;
        if self.comma {
            buf.write_char(',')?;
        }
        self.operator.cwrite(buf)
    }
}

impl CairoWrite for MacroRepetitionOperator {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        match self {
            MacroRepetitionOperator::ZeroOrOne => buf.write_str("?"),
            MacroRepetitionOperator::ZeroOrMore => buf.write_str("*"),
            MacroRepetitionOperator::OneOrMore => buf.write_str("+"),
            MacroRepetitionOperator::Missing => Ok(()),
        }
    }
}

impl CairoWrite for WrappedMacro {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        match self {
            WrappedMacro::Parenthesized(elements) => {
                elements.cwrite_concatenated_wrapped(buf, '(', ')')
            }
            WrappedMacro::Braced(elements) => elements.cwrite_concatenated_wrapped(buf, '{', '}'),
            WrappedMacro::Bracketed(elements) => {
                elements.cwrite_concatenated_wrapped(buf, '[', ']')
            }
        }
    }
}

impl CairoWrite for MacroElement {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        match self {
            MacroElement::Token(t) => t.cwrite(buf),
            MacroElement::Param(p) => p.cwrite(buf),
            MacroElement::Subtree(r) => r.cwrite(buf),
            MacroElement::Repetition(w) => w.cwrite(buf),
        }
    }
}

impl CairoWrite for TokenNode {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        match self {
            TokenNode::Identifier(s)
            | TokenNode::LiteralNumber(s)
            | TokenNode::ShortString(s)
            | TokenNode::String(s) => buf.write_str(s),
            TokenNode::As => buf.write_str("as"),
            TokenNode::Const => buf.write_str("const"),
            TokenNode::Else => buf.write_str("else"),
            TokenNode::Enum => buf.write_str("enum"),
            TokenNode::Extern => buf.write_str("extern"),
            TokenNode::False => buf.write_str("false"),
            TokenNode::Function => buf.write_str("function"),
            TokenNode::If => buf.write_str("if"),
            TokenNode::While => buf.write_str("while"),
            TokenNode::For => buf.write_str("for"),
            TokenNode::Loop => buf.write_str("loop"),
            TokenNode::Impl => buf.write_str("impl"),
            TokenNode::Implicits => buf.write_str("implicits"),
            TokenNode::Let => buf.write_str("let"),
            TokenNode::Macro => buf.write_str("macro"),
            TokenNode::Match => buf.write_str("match"),
            TokenNode::Module => buf.write_str("module"),
            TokenNode::Mut => buf.write_str("mut"),
            TokenNode::NoPanic => buf.write_str("no_panic"),
            TokenNode::Of => buf.write_str("of"),
            TokenNode::Ref => buf.write_str("ref"),
            TokenNode::Continue => buf.write_str("continue"),
            TokenNode::Return => buf.write_str("return"),
            TokenNode::Break => buf.write_str("break"),
            TokenNode::Struct => buf.write_str("struct"),
            TokenNode::Trait => buf.write_str("trait"),
            TokenNode::True => buf.write_str("true"),
            TokenNode::Type => buf.write_str("type"),
            TokenNode::Use => buf.write_str("use"),
            TokenNode::Pub => buf.write_str("pub"),
            TokenNode::And => buf.write_str("&"),
            TokenNode::AndAnd => buf.write_str("&&"),
            TokenNode::Arrow => buf.write_str("->"),
            TokenNode::At => buf.write_str("@"),
            TokenNode::BadCharacters(s) => buf.write_str(s),
            TokenNode::Colon => buf.write_char(':'),
            TokenNode::ColonColon => buf.write_str("::"),
            TokenNode::Comma => buf.write_char(','),
            TokenNode::Div => buf.write_char('/'),
            TokenNode::DivEq => buf.write_char('/'),
            TokenNode::Dollar => buf.write_char('$'),
            TokenNode::Dot => buf.write_char('.'),
            TokenNode::DotDot => buf.write_str(".."),
            TokenNode::DotDotEq => buf.write_str("..="),
            TokenNode::EndOfFile => buf.write_str(""),
            TokenNode::Eq => buf.write_char('='),
            TokenNode::EqEq => buf.write_str("=="),
            TokenNode::GE => buf.write_str(">="),
            TokenNode::GT => buf.write_char('>'),
            TokenNode::Hash => buf.write_str("#"),
            TokenNode::LBrace => buf.write_char('{'),
            TokenNode::LBrack => buf.write_char('['),
            TokenNode::LE => buf.write_str("<="),
            TokenNode::LParen => buf.write_char('('),
            TokenNode::LT => buf.write_char('<'),
            TokenNode::MatchArrow => buf.write_str("=>"),
            TokenNode::Minus => buf.write_char('-'),
            TokenNode::MinusEq => buf.write_str("-="),
            TokenNode::Mod => buf.write_char('%'),
            TokenNode::ModEq => buf.write_str("%="),
            TokenNode::Mul => buf.write_char('*'),
            TokenNode::MulEq => buf.write_str("*="),
            TokenNode::Neq => buf.write_str("!="),
            TokenNode::Not => buf.write_str("!"),
            TokenNode::BitNot => buf.write_str("~"),
            TokenNode::Or => buf.write_str("|"),
            TokenNode::OrOr => buf.write_str("||"),
            TokenNode::Plus => buf.write_str("+"),
            TokenNode::PlusEq => buf.write_str("+="),
            TokenNode::QuestionMark => buf.write_str("?"),
            TokenNode::RBrace => buf.write_str("}"),
            TokenNode::RBrack => buf.write_str("]"),
            TokenNode::RParen => buf.write_str(")"),
            TokenNode::Semicolon => buf.write_str(";"),
            TokenNode::Underscore => buf.write_str("_"),
            TokenNode::Xor => buf.write_str("^"),
            TokenNode::Empty => buf.write_str(""),
        }
    }
}

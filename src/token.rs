use std::fmt;

#[derive(PartialEq)]
pub enum TokenType {
	// Single-character tokens.
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	Comma,
	Dot,
	Minus,
	Plus,
	Semicolon,
	Slash,
	Star,

	// One Or Two Character Tokens.
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

	// Literals.
	Identifier(String),
	String(String),
	Number(f64),

	// Keywords.
	And,
	Class,
	Else,
	False,
	Fun,
	For,
	If,
	Nil,
	Or,
	Print,
	Return,
	Super,
	This,
	True,
	Let,
	Const,
	While,

	Eof,
}

pub struct Token {
	pub byte_offset: usize,
	pub byte_length: usize,
	pub token_type: TokenType,
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}\tfrom: {};\tto: {};",
			self.token_type,
			self.byte_offset,
			self.byte_offset + self.byte_length,
		)
	}
}

impl fmt::Display for TokenType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			TokenType::Identifier(s) => write!(f, "{}", s),
			TokenType::String(s) => write!(f, "{:?}", s),
			TokenType::Number(n) => write!(f, "{:?}", n),
			TokenType::LeftParen => write!(f, "("),
			TokenType::RightParen => write!(f, ")"),
			TokenType::LeftBrace => write!(f, "{{"),
			TokenType::RightBrace => write!(f, "}}"),
			TokenType::Comma => write!(f, ","),
			TokenType::Dot => write!(f, "."),
			TokenType::Minus => write!(f, "-"),
			TokenType::Plus => write!(f, "+"),
			TokenType::Semicolon => write!(f, ";"),
			TokenType::Slash => write!(f, "/"),
			TokenType::Star => write!(f, "*"),
			TokenType::Bang => write!(f, "!"),
			TokenType::BangEqual => write!(f, "!="),
			TokenType::Equal => write!(f, "="),
			TokenType::EqualEqual => write!(f, "=="),
			TokenType::Greater => write!(f, ">"),
			TokenType::GreaterEqual => write!(f, ">="),
			TokenType::Less => write!(f, "<"),
			TokenType::LessEqual => write!(f, "<="),
			TokenType::And => write!(f, "and"),
			TokenType::Class => write!(f, "class"),
			TokenType::Else => write!(f, "else"),
			TokenType::False => write!(f, "false"),
			TokenType::Fun => write!(f, "fun"),
			TokenType::For => write!(f, "for"),
			TokenType::If => write!(f, "if"),
			TokenType::Nil => write!(f, "nil"),
			TokenType::Or => write!(f, "or"),
			TokenType::Print => write!(f, "print"),
			TokenType::Return => write!(f, "("),
			TokenType::Super => write!(f, "("),
			TokenType::This => write!(f, "this"),
			TokenType::True => write!(f, "true"),
			TokenType::Let => write!(f, "let"),
			TokenType::Const => write!(f, "const"),
			TokenType::While => write!(f, "while"),
			TokenType::Eof => write!(f, "EOF"),
		}
	}
}

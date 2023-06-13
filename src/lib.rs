#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(elided_lifetimes_in_paths, unsafe_code)]

#[cfg(test)]
mod tests;

use chumsky::{
	prelude::*,
	text::{ident, whitespace}
};
use std::{fmt::Debug, str::FromStr};

type PError = Simple<char>;

/// A parser for an unsigned integer.
fn uint<T>(radix: u32) -> impl Parser<char, T, Error = PError> + Clone
where
	T: FromStr,
	T::Err: Debug
{
	text::int(radix).map(|s: String| s.parse().unwrap())
}

#[derive(Debug, Eq, PartialEq)]
pub enum VarName {
	None,
	Index(usize),
	Ident(String)
}

impl VarName {
	fn parser() -> impl Parser<char, Self, Error = PError> + Clone {
		choice((
			uint(10).map(Self::Index),
			ident().map(Self::Ident),
			empty().map(|_| Self::None)
		))
	}
}

#[derive(Debug, Eq, PartialEq)]
pub enum Param {
	Const(usize),
	Dynamic(VarName)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Align {
	Left,
	Center,
	Right
}

impl Align {
	fn parser() -> impl Parser<char, Self, Error = PError> + Clone {
		choice((
			just("<").map(|_| Self::Left),
			just("^").map(|_| Self::Center),
			just(">").map(|_| Self::Right),
			empty().map(|_| Self::Left)
		))
		.debug("Align Parser")
	}
}

#[derive(Debug, Eq, PartialEq)]
pub enum Padding {
	/// Padding with zeroes. Usually for numbers.
	ZeroPadding { width: Param },

	/// Padding a custom char. Using spaces by default.
	TextPadding {
		ch: char,
		align: Align,
		width: Param
	}
}

impl Padding {
	fn parser() -> impl Parser<char, Self, Error = PError> + Clone {
		let width_parser = choice((
			uint(10)
				.then_ignore(just("$"))
				.map(|idx| Param::Dynamic(VarName::Index(idx))),
			ident()
				.then_ignore(just("$"))
				.map(|name| Param::Dynamic(VarName::Ident(name))),
			uint(10).map(Param::Const)
		));

		choice((
			just("0")
				.ignore_then(uint(10))
				.map(|width| Self::ZeroPadding {
					width: Param::Const(width)
				})
				.debug("Padding::ZeroPadding Parser"),
			Align::parser()
				.then(width_parser.clone())
				.map(|(align, width)| Self::TextPadding {
					ch: ' ',
					align,
					width
				}),
			any()
				.or(empty().map(|_| ' '))
				.then(Align::parser())
				.then(width_parser)
				.map(|((ch, align), width)| Self::TextPadding { ch, align, width })
				.debug("Padding::TextPadding Parser")
		))
		.debug("Padding Parser")
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Style {
	Display,
	Debug,
	LowerHex,
	UpperHex,
	Binary,
	Octal
}

impl Style {
	fn parser() -> impl Parser<char, Self, Error = PError> + Clone {
		choice((
			just("?").map(|_| Self::Debug),
			just("x").map(|_| Self::LowerHex),
			just("X").map(|_| Self::UpperHex),
			just("b").map(|_| Self::Binary),
			just("o").map(|_| Self::Octal),
			empty().map(|_| Self::Display)
		))
	}
}

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
	/// A text token. Double braces (`{{` and `}}`) have already been converted to single
	/// braces.
	Text(String),

	/// A variable/placeholder.
	Variable {
		/// The name of the variable. Can be empty, an index, or an ident.
		name: VarName,

		/// The padding/fill/alignment of the variable.
		padding: Option<Padding>,
		/// The precision of the argument.
		precision: Option<Param>,

		/// The style of the variable.
		style: Style,
		/// Whether to use pretty printing (`{:#}`).
		pretty: bool,
		/// Whether to print signs (`{:+}`).
		sign: bool
	}
}

impl Token {
	fn parser() -> impl Parser<char, Self, Error = PError> + Clone {
		choice((
			none_of("{}")
				.or(just("{{").map(|_| '{'))
				.or(just("}}").map(|_| '}'))
				.repeated()
				.at_least(1)
				.map(|text| Self::Text(text.into_iter().collect())),
			just("{")
				.ignore_then(VarName::parser())
				.then_ignore(whitespace())
				.then_ignore(just("}"))
				.map(|name| Self::Variable {
					name,
					padding: None,
					precision: None,
					style: Style::Display,
					pretty: false,
					sign: false
				}),
			just("{")
				.ignore_then(VarName::parser())
				.then_ignore(just(":"))
				.then(Padding::parser().map(Some).or(empty().map(|_| None)))
				.then(just("+").map(|_| true).or(empty().map(|_| false)))
				.then(just("#").map(|_| true).or(empty().map(|_| false)))
				.then(
					just(".")
						.ignore_then(choice((
							uint(10)
								.then_ignore(just("$"))
								.map(|idx| Param::Dynamic(VarName::Index(idx))),
							ident()
								.then_ignore(just("$"))
								.map(|ident| Param::Dynamic(VarName::Ident(ident))),
							just("*").map(|_| Param::Dynamic(VarName::None)),
							uint(10).map(Param::Const)
						)))
						.map(Some)
						.or(empty().map(|_| None))
				)
				.then(Style::parser())
				.then_ignore(whitespace())
				.then_ignore(just("}"))
				.map(|(((((name, padding), sign), pretty), precision), style)| {
					Self::Variable {
						name,
						padding,
						precision,
						style,
						pretty,
						sign
					}
				})
		))
	}
}

pub type Error = Vec<PError>;

pub fn parse(input: &str) -> Result<Vec<Token>, Error> {
	Token::parser().repeated().then_ignore(end()).parse(input)
}

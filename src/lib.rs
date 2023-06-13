#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(elided_lifetimes_in_paths, unsafe_code)]

pub enum VarName {
	None,
	Index(usize),
	Ident(String)
}

pub enum Param {
	Const(usize),
	Dynamic(VarName)
}

pub enum Align {
	Left,
	Center,
	Right
}

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

pub enum Style {
	Display,
	Debug,
	LowerHex,
	UpperHex,
	Binary,
	Octal
}

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

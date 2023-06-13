use crate::*;

fn assert(input: &str, expected: &[Token]) {
	assert_eq!(parse(input).unwrap(), expected);
}

macro_rules! text {
	($text:literal) => {
		Token::Text(String::from($text))
	};
}

macro_rules! var {
	() => {
		Token::Variable {
			name: VarName::None,
			padding: None,
			precision: None,
			style: Style::Display,
			pretty: false,
			sign: false
		}
	};

	($idx:literal) => {
		Token::Variable {
			name: VarName::Index($idx),
			padding: None,
			precision: None,
			style: Style::Display,
			pretty: false,
			sign: false
		}
	};

	($ident:ident) => {
		Token::Variable {
			name: VarName::Ident(String::from(stringify!($ident))),
			padding: None,
			precision: None,
			style: Style::Display,
			pretty: false,
			sign: false
		}
	};

	(: $width:literal) => {
		Token::Variable {
			name: VarName::None,
			padding: Some(Padding::TextPadding {
				ch: ' ',
				align: Align::Left,
				width: Param::Const($width)
			}),
			precision: None,
			style: Style::Display,
			pretty: false,
			sign: false
		}
	};

	(: ?) => {
		Token::Variable {
			name: VarName::None,
			padding: None,
			precision: None,
			style: Style::Debug,
			pretty: false,
			sign: false
		}
	};

	(: #?) => {
		Token::Variable {
			name: VarName::None,
			padding: None,
			precision: None,
			style: Style::Debug,
			pretty: true,
			sign: false
		}
	};

	(: 0 $width:literal) => {
		Token::Variable {
			name: VarName::None,
			padding: Some(Padding::ZeroPadding {
				width: Param::Const($width)
			}),
			precision: None,
			style: Style::Display,
			pretty: false,
			sign: false
		}
	};
}

mod std_fmt;

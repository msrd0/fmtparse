use crate::*;
use ariadne::{Label, Report, ReportKind, Source};
use chumsky::error::SimpleReason;

fn report_err(buf: &str, path_str: &str, err: Vec<Simple<char>>) {
	for e in err {
		let mut report = Report::build(ReportKind::Error, path_str, e.span().start);
		match (e.reason(), e.found()) {
			(SimpleReason::Unexpected, Some(found)) => {
				report.set_message("Unexpected token");
				report.add_label(
					Label::new((path_str, e.span()))
						.with_message(format!("Unexpected token {found}"))
				);
				if e.expected().len() > 0 {
					report.set_note(format!(
						"Expected {}",
						e.expected()
							.map(|ex| match ex {
								Some(ex) => ex.to_string(),
								None => "end of file".to_owned()
							})
							.collect::<Vec<_>>()
							.join(", ")
					));
				}
			},

			(SimpleReason::Unexpected, None) => {
				report.set_message("Unexpected end of file");
			},

			(SimpleReason::Unclosed { span, delimiter }, found) => {
				report.set_message("Unclosed delimiter");
				report.add_label(
					Label::new((path_str, span.clone()))
						.with_message(format!("Unclosed delimiter {delimiter}"))
				);
				if let Some(found) = found {
					report.add_label(
						Label::new((path_str, e.span()))
							.with_message(format!("Must be closed before this {found}"))
					);
				}
			},

			(SimpleReason::Custom(msg), _) => {
				report.set_message(msg);
				report.add_label(Label::new((path_str, e.span())).with_message(msg));
			}
		};

		let mut out = Vec::<u8>::new();
		report
			.finish()
			.write((path_str, Source::from(buf)), &mut out)
			.unwrap();
		eprintln!("{}", String::from_utf8(out).unwrap());
	}
}

#[track_caller]
fn assert(input: &str, expected: &[Token]) {
	println!("Input: {input}");
	let tokens = match parse(input) {
		Ok(tokens) => tokens,
		Err(err) => {
			report_err(input, "<input>", err);
			panic!("Failed to parse input");
		}
	};
	assert_eq!(tokens, expected);
}

macro_rules! text {
	($text:literal) => {
		Token::Text(String::from($text))
	};
}

macro_rules! var_name {
	() => {
		VarName::None
	};
	($idx:literal) => {
		VarName::Index($idx)
	};
	($ident:ident) => {
		VarName::Ident(String::from(stringify!($ident)))
	};
}

macro_rules! align {
	(<) => {
		Align::Left
	};
	(^) => {
		Align::Center
	};
	(>) => {
		Align::Right
	};
}

macro_rules! var {
	($($var_name:tt)?) => {
		Token::Variable {
			name: var_name!($($var_name)?),
			text_padding: None,
			precision: None,
			style: Style::Display,
			pretty: false,
			sign: false,
			zero_padding: None
		}
	};

	(: $width:literal) => {
		Token::Variable {
			name: VarName::None,
			text_padding: Some(TextPadding {
				ch: ' ',
				align: Align::Left,
				width: Param::Const($width)
			}),
			precision: None,
			style: Style::Display,
			pretty: false,
			sign: false,
			zero_padding: None
		}
	};

	(: $width:tt $) => {
		Token::Variable {
			name: VarName::None,
			text_padding: Some(TextPadding {
				ch: ' ',
				align: Align::Left,
				width: Param::Dynamic(var_name!($width))
			}),
			precision: None,
			style: Style::Display,
			pretty: false,
			sign: false,
			zero_padding: None
		}
	};

	(: $ch:literal $align:tt $width:literal) => {
		Token::Variable {
			name: VarName::None,
			text_padding: Some(TextPadding {
				ch: $ch,
				align: align!($align),
				width: Param::Const($width)
			}),
			precision: None,
			style: Style::Display,
			pretty: false,
			sign: false,
			zero_padding: None
		}
	};

	($var_name:tt : $width:tt $) => {
		Token::Variable {
			name: var_name!($var_name),
			text_padding: Some(TextPadding {
				ch: ' ',
				align: Align::Left,
				width: Param::Dynamic(var_name!($width))
			}),
			precision: None,
			style: Style::Display,
			pretty: false,
			sign: false,
			zero_padding: None
		}
	};

	(:+) => {
		Token::Variable {
			name: VarName::None,
			text_padding: None,
			precision: None,
			style: Style::Display,
			pretty: false,
			sign: true,
			zero_padding: None
		}
	};

	(: ?) => {
		Token::Variable {
			name: VarName::None,
			text_padding: None,
			precision: None,
			style: Style::Debug,
			pretty: false,
			sign: false,
			zero_padding: None
		}
	};

	(: #?) => {
		Token::Variable {
			name: VarName::None,
			text_padding: None,
			precision: None,
			style: Style::Debug,
			pretty: true,
			sign: false,
			zero_padding: None
		}
	};

	(: #x) => {
		Token::Variable {
			name: VarName::None,
			text_padding: None,
			precision: None,
			style: Style::LowerHex,
			pretty: true,
			sign: false,
			zero_padding: None
		}
	};

	(: 0 $width:literal) => {
		Token::Variable {
			name: VarName::None,
			text_padding: None,
			precision: None,
			style: Style::Display,
			pretty: false,
			sign: false,
			zero_padding: Some(ZeroPadding {
				width: $width
			})
		}
	};

	(: # 0 $width:literal x) => {
		Token::Variable {
			name: VarName::None,
			text_padding: None,
			precision: None,
			style: Style::LowerHex,
			pretty: true,
			sign: false,
			zero_padding: Some(ZeroPadding {
				width: $width
			})
		}
	}
}

#[test]
fn mix_vars_and_escaping() {
	assert("{{{x}}}", &[text!("{"), var!(x), text!("}")]);
}

mod std_fmt;
mod subparsers;

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
mod subparsers;

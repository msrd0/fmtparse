/// This file contains tests that test subparsers.
use super::*;

#[track_caller]
fn assert<P, T>(parser: P, input: &str, expected: T)
where
	P: Parser<char, T, Error = Simple<char>>,
	T: Debug + PartialEq
{
	let tokens = match parser.then_ignore(end()).parse_recovery_verbose(input) {
		(Some(tokens), err) if err.is_empty() => tokens,
		(_, err) => {
			report_err(input, "<input>", err);
			panic!("Failed to parse input");
		}
	};
	assert_eq!(tokens, expected);
}

#[test]
fn padding_can_parse_uint() {
	assert(Padding::parser(), "5", Padding::TextPadding {
		ch: ' ',
		align: Align::Left,
		width: Param::Const(5)
	});
}

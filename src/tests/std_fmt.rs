/// All tests taken from <https://doc.rust-lang.org/stable/std/fmt/index.html>.
use super::*;

#[test]
fn usage_1() {
	assert("Hello", &[text!("Hello")]);
}

#[test]
fn usage_2() {
	assert("Hello, {}!", &[text!("Hello, "), var!(), text!("!")]);
}

#[test]
fn usage_3() {
	assert("The number is {}", &[text!("The number is "), var!()]);
}

#[test]
fn usage_4() {
	assert("{:?}", &[var!(:?)]);
}

#[test]
fn usage_5() {
	assert("{value}", &[var!(value)]);
}

#[test]
fn usage_6() {
	assert("Hello {people}!", &[
		text!("Hello "),
		var!(people),
		text!("!")
	]);
}

#[test]
fn usage_7() {
	assert("{} {}", &[var!(), text!(" "), var!()]);
}

#[test]
fn usage_8() {
	assert("{:04}", &[var!(:0 4)]);
}

#[test]
fn usage_9() {
	assert("{:#?}", &[var!(:#?)]);
}

#[test]
fn positional_parameters() {
	assert("{1} {} {0} {}", &[
		var!(1),
		text!(" "),
		var!(),
		text!(" "),
		var!(0),
		text!(" "),
		var!()
	]);
}

#[test]
fn named_parameters_1() {
	assert("{argument}", &[var!(argument)]);
}

#[test]
fn named_parameters_2() {
	assert("{name} {}", &[var!(name), text!(" "), var!()]);
}

#[test]
fn named_parameters_3() {
	assert("{a} {c} {b}", &[
		var!(a),
		text!(" "),
		var!(c),
		text!(" "),
		var!(b)
	]);
}

#[test]
fn width_1() {
	assert("Hello {:5}!", &[text!("Hello "), var!(:5), text!("!")]);
}

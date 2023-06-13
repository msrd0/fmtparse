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

#[test]
fn width_2() {
	assert("Hello {:1$}!", &[text!("Hello "), var!(:1$), text!("!")]);
}

#[test]
fn width_3() {
	assert("Hello {1:0$}!", &[text!("Hello "), var!(1:0$), text!("!")]);
}

#[test]
fn width_4() {
	assert("Hello {:width$}!", &[
		text!("Hello "),
		var!(:width$),
		text!("!")
	]);
}

#[test]
fn fill_1() {
	assert("Hello {:<5}!", &[
		text!("Hello "),
		var!(: ' ' < 5),
		text!("!")
	]);
}

#[test]
fn fill_2() {
	assert("Hello {:-<5}!", &[
		text!("Hello "),
		var!(: '-' < 5),
		text!("!")
	]);
}

#[test]
fn fill_3() {
	assert("Hello {:^5}!", &[
		text!("Hello "),
		var!(: ' ' ^ 5),
		text!("!")
	]);
}

#[test]
fn fill_4() {
	assert("Hello {:>5}!", &[
		text!("Hello "),
		var!(: ' ' > 5),
		text!("!")
	]);
}

#[test]
fn sign_1() {
	assert("Hello {:+}!", &[text!("Hello "), var!(:+), text!("!")]);
}

#[test]
fn sign_2() {
	assert("{:#x}!", &[var!(:#x), text!("!")]);
}

#[test]
fn sign_3() {
	assert("Hello {:05}!", &[text!("Hello "), var!(:0 5), text!("!")]);
}

#[test]
fn sign_4() {
	assert("{:#010x}!", &[var!(:# 0 10 x), text!("!")]);
}

// more tests here ...

#[test]
fn escaping_1() {
	assert("Hello {{}}", &[text!("Hello {}")]);
}

#[test]
fn escaping_2() {
	assert("{{ Hello", &[text!("{ Hello")]);
}

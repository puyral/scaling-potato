use std::io;
use termion::{style, color};
use std::io::Write;

pub struct Printer<R> {
	pub result: R,
	pub description: Option<String>
}
pub fn res<R>(res: R, str: Option<String>) -> Printer<R>{
	Printer{
		result: res,
		description: str
	}
}

pub const NONE: Printer<()> = Printer{result: (), description: None};

pub fn only_r<R>(result: R) -> Printer<R>{
	res(result, None)
}

pub fn print_done<R, F: FnOnce() -> Printer<R>>(header: &str, fun: F) -> R {
	print!("{}", header);
	io::stdout().flush().ok().expect("Could not flush stdout");
	let r = fun();
	match r.description {
		None => println!(
			"{green}{bold}[DONE]{reset_c}{reset_s}",
			bold = style::Bold,
			green = color::Fg(color::Green),
			reset_c = color::Fg(color::Reset),
			reset_s = style::Reset
		),
		Some(str) => println!(
			"{green}{bold}[DONE]{reset_c}{reset_s} ({})",
			&str,
			bold = style::Bold,
			green = color::Fg(color::Green),
			reset_c = color::Fg(color::Reset),
			reset_s = style::Reset
		)
	}
	r.result
}
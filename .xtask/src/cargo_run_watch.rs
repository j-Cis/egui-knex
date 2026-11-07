use std::{
	env,
	process::{Command, ExitStatus},
};

/// Tryb `cargo x watch`: `check`, `clippy` i `run` pod `cargo watch`.
/// Bez `--example` uruchamia bin projektu (jak `cargo run`),
/// z `--example <nazwa>` uruchamia wskazany example.
pub fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
	let target = match parse_example(args) {
		Some(name) => format!("--example {name}"),
		None => String::new(),
	};

	let check_cmd = format!("check {target}").trim_end().to_string();
	let clippy_cmd = format!("clippy {target}").trim_end().to_string();
	let run_cmd = format!("run {target}").trim_end().to_string();

	let status: ExitStatus = Command::new(cargo_bin())
		.args(["watch", "-c", "-q", "-x", &check_cmd, "-x", &clippy_cmd, "-x", &run_cmd])
		.status()?;

	if !status.success() {
		std::process::exit(status.code().unwrap_or(1));
	}

	Ok(())
}

fn parse_example(args: &[String]) -> Option<String> {
	let mut iter = args.iter();
	let mut example = None;

	while let Some(arg) = iter.next() {
		if arg == "--example" {
			example = iter.next().cloned();
		} else if let Some(val) = arg.strip_prefix("--example=") {
			example = Some(val.to_string());
		}
	}

	example
}

fn cargo_bin() -> String { env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()) }

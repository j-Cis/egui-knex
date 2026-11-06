use std::{
	env,
	process::{Command, ExitStatus},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut args = env::args().skip(1);
	let task = args.next();

	match task.as_deref() {
		Some("watch") => {
			let mut example_name = None;

			while let Some(arg) = args.next() {
				if arg == "--example" {
					example_name = args.next();
				} else if let Some(val) = arg.strip_prefix("--example=") {
					example_name = Some(val.to_string());
				}
			}

			let Some(example) = example_name else {
				list_examples_and_exit();
			};

			run_watch(&example)?;
		}
		_ => print_help(),
	}

	Ok(())
}

fn cargo_bin() -> String { env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()) }

/// Uruchamia `cargo run --example` bez argumentu. Cargo sam zwraca błąd
/// z listą dostępnych examples, więc wynik jest identyczny z ręcznym
/// wywołaniem.
fn list_examples_and_exit() -> ! {
	match Command::new(cargo_bin()).args(["run", "--example"]).output() {
		Ok(out) => eprint!("{}", String::from_utf8_lossy(&out.stderr)),
		Err(e) => eprintln!("Błąd: nie udało się uruchomić cargo: {e}"),
	}
	std::process::exit(1);
}

fn run_watch(example: &str) -> Result<(), Box<dyn std::error::Error>> {
	// let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

	let check_cmd = format!("check --example {example}");
	let clippy_cmd = format!("clippy --example {example}");
	let run_cmd = format!("run --example {example}");

	let status: ExitStatus = Command::new(cargo_bin())
		.args(["watch", "-c", "-q", "-x", &check_cmd, "-x", &clippy_cmd, "-x", &run_cmd])
		.status()?;

	if !status.success() {
		std::process::exit(status.code().unwrap_or(1));
	}

	Ok(())
}

fn print_help() {
	eprintln!("Użycie: cargo xtask watch --example <nazwa_przykładu>");
}

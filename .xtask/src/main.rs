mod cargo_run_watch;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args: Vec<String> = env::args().skip(1).collect();

	match args.first().map(String::as_str) {
		Some("watch") => cargo_run_watch::run(&args[1..])?,
		_ => print_help(),
	}

	Ok(())
}

fn print_help() {
	eprintln!("Użycie:");
	eprintln!("  cargo x watch                        # bin projektu (jak cargo run)");
	eprintln!("  cargo x watch --example <nazwa>      # wskazany example");
}

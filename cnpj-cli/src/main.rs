use clap::{Parser, Subcommand};
use cnpj::{Error, UncheckedCNPJ, CNPJ};
use colored::Colorize;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Valida { cnpj: Vec<String> },
    Calcula { unchecked_cnpj: Vec<String> },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Valida { cnpj } => {
            handle_inputs(cnpj, |cnpj: CNPJ| {
                println!("{} ({cnpj})", "Válido".green());
            });
        }
        Command::Calcula { unchecked_cnpj } => {
            handle_inputs(unchecked_cnpj, |unchecked_cnpj: UncheckedCNPJ| {
                println!(
                    "{} ({})",
                    unchecked_cnpj.calculate_check_digits(),
                    unchecked_cnpj.with_check_digits()
                );
            });
        }
    }
}

fn handle_inputs<T: TryFrom<String, Error = Error>>(inputs: Vec<String>, f: fn(T)) {
    for input in inputs {
        print!("{input}: ");

        match T::try_from(input) {
            Ok(input) => {
                f(input);
            }
            Err(error) => {
                println!("{} {error}", "Inválido".red());
            }
        }
    }
}

use std::fmt::Display;

use clap::{Parser, Subcommand};
use colored::Colorize;
use cpf_cnpj::{Error, UncheckedCNPJ, UncheckedCPF, CNPJ, CPF};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    ValidaCPF { cpf: Vec<String> },
    ValidaCNPJ { cnpj: Vec<String> },
    CalculaCPF { unchecked_cpf: Vec<String> },
    CalculaCNPJ { unchecked_cnpj: Vec<String> },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::ValidaCPF { cpf } => handle_inputs::<CPF>(cpf),
        Command::ValidaCNPJ { cnpj } => handle_inputs::<CNPJ>(cnpj),
        Command::CalculaCPF { unchecked_cpf } => handle_inputs::<UncheckedCPF>(unchecked_cpf),
        Command::CalculaCNPJ { unchecked_cnpj } => handle_inputs::<UncheckedCNPJ>(unchecked_cnpj),
    }
}

fn handle_inputs<T: TryFrom<String, Error = Error> + Display>(inputs: Vec<String>) {
    for input in inputs {
        print!("{input}: ");

        match T::try_from(input) {
            Ok(input) => {
                println!("{} ({input})", "Válido".green());
            }
            Err(error) => {
                println!("{} {error}", "Inválido".red());
            }
        }
    }
}

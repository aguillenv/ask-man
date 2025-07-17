use clap::Parser;

/// CLI Arguments
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Natural language prompt to generate a command
    prompt: String,
}

fn main() {
    let args = Cli::parse();

    println!("{}", &args.prompt);
}

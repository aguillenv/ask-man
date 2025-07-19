use clap::Parser;

mod openai;

/// CLI Arguments
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Natural language prompt to generate a command
    prompt: String,

    /// Option --explain to explain the command
    #[arg(long)]
    explain: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let args = Cli::parse();

    let command: String = openai::get_command_sync(&args.prompt, &args.explain)?;

    println!("Prompt: {}\n", &args.prompt);
    println!("{}", command);

    Ok(())
}

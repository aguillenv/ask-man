use clap::Parser;

mod openai;

/// CLI Arguments
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Natural language prompt to generate a command
    prompt: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let args = Cli::parse();

    let command = openai::get_command_sync(&args.prompt)?;

    println!("Prompt: {}\n", &args.prompt);
    println!("{}", command);

    Ok(())
}

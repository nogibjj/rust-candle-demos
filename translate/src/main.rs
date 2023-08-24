use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "A Hugging Face Translation Tool in Rust"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Translate {
        #[clap(short, long)]
        path: String,
    },
    Print {
        #[clap(short, long)]
        path: String,
    },
}
// create main function that uses lib.rs
fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Translate { path }) => {
            translate::translate_file(path)?;
        }
        Some(Commands::Print { path }) => {
            println!("{}", translate::read_file(path)?);
        }
        None => {
            println!("No command given");
        }
    }
    Ok(())
}

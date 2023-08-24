/* Invoke CLI */
//A command-line tool to invoke the run function
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Noah Gift", about = "Run MNIST on GPU")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Train {
        #[clap(short, long, default_value = "data/")]
        path: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Train { path }) => {
            println!("Training MNIST on GPU");
            mnist_cli_gpu::run(&path);
        }
        None => println!("No command was used"),
    }
}
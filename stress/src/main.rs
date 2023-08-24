use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "A Stress Test for PyTorch CPU and GPU.  There are three subcommands: cpu, gpu, and tgpu. The tgpu subcommand uses Rayon to send the load to the GPU."
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Cpu {},
    Gpu {},
    Tgpu {},
}

//build the main function and import stress:: namespace
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Cpu {}) => {
            println!("Running CPU Stress Test");
            stress::cpu_load_test();
        }
        Some(Commands::Gpu {}) => {
            println!("Running GPU Stress Test");
            stress::gpu_load_test();
        }
        Some(Commands::Tgpu {}) => {
            println!("Running GPU Stress Test with Rayon");
            stress::gpu_load_test_rayon();
        }

        None => {
            println!("Please specify a subcommand");
        }
    }
}

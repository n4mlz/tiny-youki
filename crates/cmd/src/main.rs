use clap::{Parser, Subcommand};
use libcontainer::ContainerBuilder;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(
    name = "tiny-youki",
    about = "tiny-youki (WIP) - Open Container Initiative runtime",
    author = "n4mlz",
    arg_required_else_help = true
)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[clap(name = "create", about = "create a container")]
    Create {
        /// container id
        container_id: String,

        /// bundle path
        #[clap(short = 'b', long, value_name = "PATH", required = true)]
        bundle: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        SubCommands::Create {
            container_id,
            bundle,
        } => {
            ContainerBuilder::new(bundle)
                .unwrap()
                .create(container_id)
                .unwrap();
        }
    }
}

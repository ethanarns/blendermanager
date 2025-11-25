use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version = "0.1.0")]
#[command(name = "Blender Manager")]
#[command(about = "Manage Blender versions", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// List installed Blender versions
    List(ListArgs),
    /// Install Blender version
    Install(InstallArgs),
    /// List available Blender versions
    Available(AvailableArgs),
    /// Uninstall a Blender version
    Remove(RemoveArgs),
    /// Manage install location
    Location(LocationArgs)
}

#[derive(Args)]
struct ListArgs {

}

#[derive(Args)]
struct InstallArgs {

}

#[derive(Args)]
struct AvailableArgs {
    
}

#[derive(Args)]
struct RemoveArgs {

}

#[derive(Args)]
struct LocationArgs {

}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::List(list_args) => {

        },
        Commands::Install(install_args) => {

        },
        Commands::Available(available_args) => {

        },
        Commands::Remove(remove_args) => {

        },
        Commands::Location(location_args) => {
            
        },
    }
}
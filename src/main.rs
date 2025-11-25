use clap::{Args, Parser, Subcommand};
use tokio;

const RELEASE_LIST_URL: &str = "https://download.blender.org/release/";

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

async fn get_major_versions() -> Result<Vec<String>,Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    println!("Querying URL: {}",RELEASE_LIST_URL);
    let res = client.get(RELEASE_LIST_URL).send().await?;
    println!("Got response code '{}'",res.status());
    let res_text = res.text().await?;
    let document = scraper::Html::parse_document(&res_text);
    let versions: Vec<String> = document.select(&scraper::Selector::parse("a")?).map(|x| x.text().collect()).collect();
    let versions_filtered: Vec<String> = versions.into_iter().filter(|x|
        x.starts_with("Blender") && !x.contains("Benchmark")
    ).collect();
    Ok(versions_filtered)
}

async fn get_minor_version_releases(major_version: &str) -> Result<Vec<String>,Box<dyn std::error::Error>> {
    let mut end_url = major_version.to_owned();
    if !end_url.ends_with("/") {
        end_url.push_str("/");
    }
    let releases_url = RELEASE_LIST_URL.to_owned() + major_version;
    let client = reqwest::Client::new();
    println!("Querying URL: {}",releases_url);
    let res = client.get(releases_url).send().await?;
    let code = res.status().to_string();
    println!("Got response code '{}'",code);
    let res_text = res.text().await?;
    let document = scraper::Html::parse_document(&res_text);
    let releases: Vec<String> = document.select(&scraper::Selector::parse("a")?).map(|x| x.text().collect()).collect();
    let releases_filtered: Vec<String> = releases.into_iter().filter(|x| x.starts_with("blender")).collect();
    Ok(releases_filtered)
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::List(_list_args) => {

        },
        Commands::Install(_install_args) => {

        },
        Commands::Available(_available_args) => {
            let versions = get_major_versions().await;
            if versions.is_err() {
                return;
            }
            let versions = versions.unwrap();
            for version in versions {
                println!("Version: {}",version);
            }
            // Next
            let releases = get_minor_version_releases("Blender4.5").await;
            if releases.is_err() {
                println!("Error: {:?}",releases.err());
                return;
            }
            let releases = releases.unwrap();
            for release in releases {
                println!("Release: {}",release);
            }
        },
        Commands::Remove(_remove_args) => {

        },
        Commands::Location(_location_args) => {

        },
    }
}
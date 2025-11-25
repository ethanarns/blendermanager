use std::{fs::{self, File}, io::{BufReader, Seek, Write}, path::Path};

use clap::{Args, Parser, Subcommand};
use tar::Archive;
use flate2::read::GzDecoder;
use xz2::bufread::XzDecoder;

const RELEASE_LIST_URL: &str = "https://download.blender.org/release/";
const SYSTEM_EXT: &str = "linux-x64.tar.xz";
const DEFAULT_INSTALL_LOC_LINUX: &str = "/home/ethan/.local/share/blender_versions/";

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
    /// Versions formated such as "4.5", "Blender2.58", or "5.0"
    version: String
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

fn clean_up_major_version(input: &str) -> String {
    let mut ret = input.to_owned();
    if !ret.starts_with("Blender") {
        ret = format!("Blender{}",ret);
    }
    if !ret.ends_with("/") {
        ret.push('/');
    }
    ret
}

fn create_version_folder(cleaned_version: &str) -> Result<String,Box<dyn std::error::Error>> {
    let base_path = DEFAULT_INSTALL_LOC_LINUX.to_owned();
    if !Path::new(&base_path).exists() {
        println!("Base path '{}' does not exist, creating",&base_path);
        let create_base_dir_result = fs::create_dir_all(&base_path);
        if create_base_dir_result.is_err() {
            let err = create_base_dir_result.unwrap_err();
            println!("Error creating base dir: {}",&err);
            return Err(Box::new(err));
        }
    }
    Ok(base_path)
}

fn get_major_versions() -> Result<Vec<String>,Box<dyn std::error::Error>> {
    println!("Querying URL: {}",RELEASE_LIST_URL);
    let res = reqwest::blocking::get(RELEASE_LIST_URL)?;
    println!("Got response code '{}'",res.status());
    let res_text = res.text()?;
    let document = scraper::Html::parse_document(&res_text);
    let versions: Vec<String> = document.select(&scraper::Selector::parse("a")?).map(|x| x.text().collect()).collect();
    let versions_filtered: Vec<String> = versions.into_iter().filter(|x|
        x.starts_with("Blender") && !x.contains("Benchmark")
    ).collect();
    Ok(versions_filtered)
}

fn get_minor_version_releases(major_version: &str) -> Result<Vec<String>,Box<dyn std::error::Error>> {
    let mut end_url = major_version.to_owned();
    if !end_url.ends_with("/") {
        end_url.push('/');
    }
    let releases_url = RELEASE_LIST_URL.to_owned() + major_version;
    println!("Querying URL: {}",releases_url);
    let res = reqwest::blocking::get(releases_url)?;
    let code = res.status().to_string();
    println!("Got response code '{}'",code);
    let res_text = res.text()?;
    let document = scraper::Html::parse_document(&res_text);
    let releases: Vec<String> = document.select(&scraper::Selector::parse("a")?).map(|x| x.text().collect()).collect();
    let releases_filtered: Vec<String> = releases.into_iter().filter(|x| x.starts_with("blender")).collect();
    Ok(releases_filtered)
}

fn get_latest_release_url(major_version: &str) -> Result<String,Box<dyn std::error::Error>> {
    let releases = get_minor_version_releases(major_version)?;
    let releases: Vec<String> = releases.into_iter().filter(|x| x.contains(&SYSTEM_EXT.to_owned())).collect();
    let last_release = releases.last();
    if last_release.is_none() {
        return Ok("Shit, not found".to_string());
    }
    let mut ret = "https://download.blender.org/release/".to_owned();
    ret.push_str(major_version);
    if !ret.ends_with("/") {
        ret.push('/');
    }
    ret.push_str(last_release.unwrap());
    Ok(ret)
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::List(_list_args) => {

        },
        Commands::Install(install_args) => {
            let version = clean_up_major_version(&install_args.version);
            println!("Installing version '{}'",version);
            let latest_release_url = get_latest_release_url(&version);
            if latest_release_url.is_err() {
                println!("Error getting version URL: {}",latest_release_url.unwrap_err());
                return;
            }
            let install_folder = create_version_folder(&version).expect("Install folder to be created");
            let latest_release_url = latest_release_url.unwrap();
            println!("Connecting to '{}'",latest_release_url);
            let dl_resp = reqwest::blocking::get(&latest_release_url).expect("Found URL to be parsed right");
            println!("Got response code '{}'",dl_resp.status());
            // Create the file
            let filename = latest_release_url.split("/").last().expect("Split to work properly");
            let install_file = format!("{}{}", install_folder, filename);
            let mut out = File::create(&install_file).expect("Create file");

            println!("Downloading...");
            let bytes_res = dl_resp.bytes().expect("Get body right");
            println!("Copying data...");
            let write_res = out.write_all(&bytes_res);
            if write_res.is_err() {
                println!("Error saving data: '{}'",write_res.unwrap_err());
                return;
            }
            // Now unpack
            println!("Creating new XzDecoder");
            let tar_xz = File::open(&install_file).expect("Opens right");
            let tar_xz_buffer = BufReader::new(tar_xz);
            let tar = XzDecoder::new(tar_xz_buffer);
            println!("Creating new Archive");
            let mut archive = Archive::new(tar);
            let unpack_res = archive.unpack(install_folder);
            if unpack_res.is_err() {
                println!("Error unpacking: {}",unpack_res.unwrap_err());
            } else {
                println!("Unpacked successfully");
            }
        },
        Commands::Available(_available_args) => {
            let versions = get_major_versions();
            if versions.is_err() {
                return;
            }
            let versions = versions.unwrap();
            for version in versions {
                println!("Version: {}",version);
            }
            // Next
            let releases = get_minor_version_releases("Blender4.5");
            if releases.is_err() {
                println!("Error: {:?}",releases.err());
                return;
            }
            let releases = releases.unwrap();
            for release in releases {
                println!("Release: {}",release);
            }

            let latest_release = get_latest_release_url("Blender4.5");
            if latest_release.is_err() {
                println!("Error: {:?}",latest_release.err());
                return;
            }
            let latest_release = latest_release.unwrap();
            println!("Latest: {}",latest_release);
        },
        Commands::Remove(_remove_args) => {

        },
        Commands::Location(_location_args) => {

        },
    }
}
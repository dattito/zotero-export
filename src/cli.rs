use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Clone, Debug)]
pub struct Cli {
    #[arg(short, long, default_value=default_config_path().into_os_string())]
    pub config_path: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    Login,
    Export {
        #[arg(short, long)]
        collection_name: String,

        #[arg(short, long, default_value=default_output_path().into_os_string())]
        output_path: PathBuf,

        #[arg(short, long, default_value="biblatex")]
        format: String,
    },
}

fn default_config_path() -> PathBuf {
    let mut home_dir =
        dirs::home_dir().expect("No home directory found. Please specify the config path manually");

    home_dir.push(".zotero-export.toml");

    home_dir
}

fn default_output_path() -> PathBuf {
    let mut p = PathBuf::new();
    p.push("./bibliography.bib");

    p
}

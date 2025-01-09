use std::{
    fs,
    io::{self, Write},
};

mod cli;
mod credentials;
mod zotero_client;

use clap::Parser;
use cli::Cli;
use credentials::Credentials;
use eyre::OptionExt;
use zotero_client::ZoteroClient;

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Login => {
            let mut username = String::new();

            print!("Enter user id: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut username)?;
            let username = username.trim().to_string();

            print!("Enter API key: ");
            io::stdout().flush()?;
            let api_key = rpassword::read_password()?;

            let credentials = Credentials::new(username, api_key);

            credentials.save_to_file(cli.config_path)?;

            println!("Success");
        }
        cli::Commands::Export {
            collection_name,
            output_path,
        } => {
            let cred = Credentials::read_from_file(cli.config_path)?;

            let zotero_client = ZoteroClient::new(cred);

            let collections = zotero_client.get_collections()?;

            let collection_key = collections
                .into_iter()
                .find(|c| c.data.name == collection_name)
                .ok_or_eyre("no collection with the given name was found")?
                .key;

            let file_content = zotero_client.get_bibliography(collection_key)?;

            fs::write(&output_path, file_content)?;

            println!(
                "Successfully written bibtex to {}",
                output_path.to_str().unwrap_or("<path cannot be displayed>")
            );
        }
    };

    Ok(())
}

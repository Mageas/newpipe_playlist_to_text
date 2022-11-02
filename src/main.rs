use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

use anyhow::{Context, Result};

use clap::Parser;

mod error;
use error::*;

mod database;
use database::*;

mod args;
use args::*;

fn main() -> Result<()> {
    let args = Cli::parse();

    let current_directory =
        std::env::current_dir().context("Unable to get the current directory from the env")?;

    let output = match &args.output {
        Some(output) => PathBuf::from(output),
        None => current_directory,
    };

    let database = Database::new(&args.database).context("Unable to open the database")?;
    let playlists = database.query().context("Unable to query the database")?;

    for playlist in playlists {
        let mut output = output.clone();
        output.push(&playlist.name);

        if output.exists() && !args.overwrite {
            println!("Skip {}, the playlist already exists!", &playlist.name);
            continue;
        }

        let output = output.to_str().context(format!(
            "Unbale to generate the output path for {}",
            &playlist.name
        ))?;

        let mut file =
            fs::File::create(&output).context(format!("Unable to create the file {}", &output))?;
        file.write_all(playlist.urls.join("\n").as_bytes())
            .context(format!("Unable to write the file {}", &output))?;

        println!("Wrote {}!", playlist.name);
    }

    Ok(())
}

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Local};
use clap::Parser;
use regex::Regex;
use std::fs::{read_dir, remove_file};
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
    /// path to search for files
    #[arg(short, long)]
    path: String,

    /// regex to match files on
    #[arg(short, long)]
    glob: String,

    /// Number days to keep
    #[arg(short, long)]
    days: u64,

    /// test run (don't delete files)
    #[arg(short, long)]
    test: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path: &Path = Path::new(&args.path);

    if !path.is_dir() {
        return Err(anyhow!("Path is not a dir: {:?}", path));
    }

    // convert the user defined days to seconds for easy comparisons
    let rotate_seconds = (args.days * 86400) as i64;

    let re =
        Regex::new(&args.glob).with_context(|| format!("Invalid regex pattern: {}", args.glob))?;

    // loop over each file in the user defined directory
    for file_res in read_dir(path)? {
        let file = file_res?;
        let name = &file.file_name();

        if !re.is_match(name.to_str().unwrap()) {
            println!("Invalid glob match: {:?}", name);
            continue;
        }

        // let timestamp = file
        //     .metadata()
        //     .with_context(|| format!("couldn't get metadata on file: {:?}", file))?
        //     .modified()
        //     .with_context(|| {
        //         format!(
        //             "couldn't extract SystemTime from metadata on file: {:?}",
        //             file
        //         )
        //     })?;

        // better to use timestamp in the filename than modified time
        let backup_time: i64 = name
            .to_str()
            .context("Could not convert timestamp from OsString to str")?
            .split("-")
            .last()
            .context("Could not get last split element from '-'")?
            .replace(".7z", "")
            .parse()
            .context("Could not parse timestamp string to integer")?;

        let backup_time = DateTime::from_timestamp(backup_time, 0).unwrap();
        let current_time: DateTime<Local> = Local::now();

        let seconds_since_backup = current_time.timestamp() - backup_time.timestamp();

        if seconds_since_backup > rotate_seconds {
            println!(
                "Deleting: {:?}, Created: {}",
                name,
                backup_time.to_rfc3339()
            );
            if !args.test {
                let removed = remove_file(file.path());
                if let Err(err) = removed {
                    println!("Failed to remove file: {:?} - {}", name, err);
                }
            }
        }
    }

    Ok(())
}

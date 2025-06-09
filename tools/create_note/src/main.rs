use std::fs;
use std::path::PathBuf;
use clap::{Parser, ValueEnum};

/// Simple tool to create notes or folders
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to create the note or folder
    #[arg(short, long)]
    path: PathBuf,

    /// Title for the note or folder
    #[arg(short, long)]
    title: String,

    /// Item type: note or folder
    #[arg(short = 'k', long)]
    kind: ItemKind,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ItemKind {
    Note,
    Folder,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut target = args.path;
    target.push(&args.title);

    match args.kind {
        ItemKind::Note => {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&target, "")?;
        }
        ItemKind::Folder => {
            fs::create_dir_all(&target)?;
        }
    }

    println!("Created {:?}", target);
    Ok(())
}


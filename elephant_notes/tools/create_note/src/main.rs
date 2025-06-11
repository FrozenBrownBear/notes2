use std::fs;
use std::path::PathBuf;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
    #[arg(short, long)]
    title: String,
    #[arg(short='k', long)]
    kind: ItemKind,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum ItemKind { Note, Folder }

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut target = args.path;
    target.push(&args.title);
    match args.kind {
        ItemKind::Note => {
            if let Some(parent) = target.parent() { fs::create_dir_all(parent)?; }
            fs::write(&target, "")?;
        }
        ItemKind::Folder => { fs::create_dir_all(&target)?; }
    }
    println!("Created {:?}", target);
    Ok(())
}

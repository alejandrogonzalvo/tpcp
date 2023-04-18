use clap::Parser;
use std::{
    io,
    path::{Path, PathBuf},
    process::{exit, Command, Output},
};

#[derive(Parser)]
struct Cli {
    source: PathBuf,
    destination: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let source = &args.source;
    let destination = &args.destination;
    if destination.exists() {
        println!("Destion folder already exists!");
        exit(1);
    }

    copy_dir(source.as_path(), destination.as_path()).expect("failed to copy dir");

    let project_name = destination.file_name().unwrap().to_str().unwrap();

    find_and_replace(
        "template-project",
        project_name,
        destination.join("CMakeLists.txt").as_path(),
    )
    .expect("failed to find and replace");

    find_and_replace(
        "template-project",
        project_name,
        destination.join("build").join("build.sh").as_path(),
    )
    .expect("failed to find and replace");
}

fn copy_dir(src: &Path, dst: &Path) -> io::Result<Output> {
    Command::new("cp").arg("-r").arg(src).arg(dst).output()
}

fn find_and_replace(previous: &str, new: &str, path: &Path) -> io::Result<Output> {
    Command::new("sed")
        .arg("-i")
        .arg(format!(
            "s/{previous}/{new}/g",
            previous = previous,
            new = new
        ))
        .arg(path)
        .output()
}

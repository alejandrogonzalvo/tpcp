use clap::Parser;
use std::{
    io,
    path::{Path, PathBuf},
    process::{exit, Command, Output},
};

#[derive(Parser)]
/// Easily create a new project from a template
///
/// This tool will copy a template project, changing the project name (the destination folder name by default)
struct TPCP {
    /// Source folder, must exists
    source: PathBuf,
    /// Destination folder, must not exists
    destination: PathBuf,

    #[arg(short, long, default_value = "template-project")]
    /// The placeholder of the project name on the template
    placeholder: String,

    #[arg(short, long)]
    /// The new project name, defaults to the destination folder name
    name: Option<String>,
}

fn main() {
    let args = TPCP::parse();

    if args.destination.exists() {
        eprintln!("destion folder already exists");
        exit(1);
    } else if !args.source.exists() {
        eprintln!("source folder does not exists");
        exit(1);
    }

    handle_output(
        copy_dir(args.source.as_path(), args.destination.as_path()),
        "failed to copy files template project",
    );

    let project_name = match args.name {
        Some(name) => name,
        None => args
            .destination
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    };

    handle_output(
        find_and_replace(
            "template-project",
            project_name.as_str(),
            args.destination.join("CMakeLists.txt").as_path(),
        ),
        "failed to update CMakeLists.txt",
    );

    handle_output(
        find_and_replace(
            "template-project",
            project_name.as_str(),
            args.destination.join("build").join("build.sh").as_path(),
        ),
        "failed to update build.sh",
    );
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

fn handle_output(output: io::Result<Output>, error_message: &str) {
    match output {
        Ok(output) if !output.status.success() => {
            eprintln!("{} ({})", error_message, output.status);
            exit(1);
        }
        Err(err) => {
            eprintln!("{} ({})", error_message, err);
            exit(1);
        }
        _ => (),
    }
}

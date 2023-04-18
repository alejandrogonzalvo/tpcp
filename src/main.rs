use std::{path::PathBuf, process::{exit, Command}, fmt::format};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    source: PathBuf,
    destination: PathBuf 
}

fn main() {
    let args = Cli::parse();
    let source = &args.source;
    let destination = &args.destination;
    if destination.exists() {
        println!("Destion folder already exists!");
        exit(1);
    }

    Command::new("cp").arg("-r").arg(source).arg(destination).output();

    let project_name = destination.file_name().unwrap().to_str().unwrap();
    let str_path = destination.display();
    let replace_pattern = format!("s/template-project/{project_name}/g");
    let cmake_path = format!("{str_path}/CMakeLists.txt");
    let build_path = format!("{str_path}/build/build.sh"); 
    Command::new("sed").arg("-i").arg(replace_pattern.clone()).arg(cmake_path).output();
    Command::new("sed").arg("-i").arg(replace_pattern.clone()).arg(build_path).output();

}

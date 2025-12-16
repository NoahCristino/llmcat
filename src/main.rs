use clap::Parser;
use glob::glob;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "llmprint")]
#[command(about = "Bundle source files into LLM-ready context")]
struct Args {
    /// Files, directories, or glob patterns
    inputs: Vec<String>,

    /// Max file size in bytes
    #[arg(long, default_value = "200000")]
    max_bytes: u64,
}

fn collect_paths(inputs: &[String]) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();

    for input in inputs {
        let p = Path::new(input);

        if p.is_dir() {
            for entry in WalkDir::new(p)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                paths.push(entry.path().to_path_buf());
            }
        } else if input.contains('*') {
            if let Ok(entries) = glob(input) {
                for entry in entries.flatten() {
                    if entry.is_file() {
                        paths.push(entry);
                    }
                }
            }
        } else if p.is_file() {
            paths.push(p.to_path_buf());
        }
    }

    paths.sort();
    paths.dedup();
    paths
}

fn is_binary(path: &Path) -> bool {
    match fs::read(path) {
        Ok(bytes) => bytes.iter().take(1024).any(|b| *b == 0),
        Err(_) => true,
    }
}

fn main() {
    let args = Args::parse();

    if args.inputs.is_empty() {
        return;
    }

    let files = collect_paths(&args.inputs);

    for path in files {
        let metadata = match fs::metadata(&path) {
            Ok(m) => m,
            Err(_) => continue,
        };

        if metadata.len() > args.max_bytes {
            continue;
        }

        if is_binary(&path) {
            continue;
        }

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        println!("\n===== FILE: {} =====", path.display());
        print!("{content}");
    }
}

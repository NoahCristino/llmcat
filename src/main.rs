use clap::Parser;
use glob::Pattern;
use glob::glob;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "llmprint")]
#[command(about = "Bundle source files into LLM-ready context")]
struct Args {
    inputs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    settings: Option<Settings>,
    paths: Option<Paths>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    remove_comments: Option<bool>,
    remove_whitespace: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct Paths {
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
}

fn find_config() -> Option<PathBuf> {
    let mut dir = env::current_dir().ok()?;

    loop {
        let candidate = dir.join(".llmcat.toml");
        if candidate.exists() {
            println!("Using config: {}", candidate.display());
            return Some(candidate);
        }

        if !dir.pop() {
            break;
        }
    }

    None
}

fn load_config(path: &Path) -> Option<Config> {
    let content = fs::read_to_string(path).ok()?;
    toml::from_str(&content).ok()
}

fn resolve_inputs(args: &Args, config: &Option<Config>) -> Vec<String> {
    if let Some(cfg) = config {
        if let Some(paths) = &cfg.paths {
            if let Some(include) = &paths.include {
                return include.clone();
            }
        }
    }
    args.inputs.clone()
}

fn collect_paths(inputs: &[String]) -> Vec<PathBuf> {
    let mut paths = Vec::new();

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

fn apply_excludes(paths: Vec<PathBuf>, config: &Option<Config>) -> Vec<PathBuf> {
    // always exclude the config file itself
    let mut patterns = vec![Pattern::new(".llmcat.toml").unwrap()];

    if let Some(cfg) = config {
        if let Some(paths_cfg) = &cfg.paths {
            if let Some(exclude_patterns) = &paths_cfg.exclude {
                for pat in exclude_patterns {
                    if let Ok(p) = Pattern::new(pat) {
                        patterns.push(p);
                    }
                }
            }
        }
    }

    paths
        .into_iter()
        .filter(|path| !patterns.iter().any(|pat| pat.matches_path(path)))
        .collect()
}

fn is_binary(path: &Path) -> bool {
    match fs::read(path) {
        Ok(bytes) => bytes.iter().take(1024).any(|b| *b == 0),
        Err(_) => true,
    }
}

fn main() {
    let args = Args::parse();

    let config = find_config().and_then(|p| load_config(&p));

    let inputs = resolve_inputs(&args, &config);

    if inputs.is_empty() {
        return;
    }

    let files = collect_paths(&inputs);
    let files = apply_excludes(files, &config);

    for path in files {
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

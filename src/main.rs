use arboard::Clipboard;
use clap::Parser;
use glob::{Pattern, glob};
use inline_colorization::*;
use regex::Regex;
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(
    name = "llmprint",
    about = "Bundle source files into LLM-ready context"
)]
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
    #[allow(dead_code)]
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

fn collect_files(inputs: &[String]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for input in inputs {
        let path = Path::new(input);
        if path.is_dir() {
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                files.push(entry.path().to_path_buf());
            }
        } else if input.contains('*') {
            if let Ok(entries) = glob(input) {
                for entry in entries.flatten() {
                    if entry.is_file() {
                        files.push(entry);
                    }
                }
            }
        } else if path.is_file() {
            files.push(path.to_path_buf());
        }
    }
    files.sort();
    files.dedup();
    files
}

fn strip_comments(content: &str) -> String {
    let block_comment = Regex::new(r"(?s)/\*.*?\*/").unwrap();
    let no_block = block_comment.replace_all(content, "");
    let slash_comment = Regex::new(r"//\s.*").unwrap();
    let no_slash = slash_comment.replace_all(&no_block, "");
    let hash_comment = Regex::new(r"(?m)(^|\s)#(\s.*|$)").unwrap();
    let result = hash_comment.replace_all(&no_slash, "$1");

    result.to_string()
}

fn apply_excludes(files: Vec<PathBuf>, config: &Option<Config>) -> Vec<PathBuf> {
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
    files
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

fn read_input(full_output: &str) {
    loop {
        print!("{color_yellow}:{color_reset} ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "c" => {
                // Detect Wayland
                let is_wayland = std::env::var("WAYLAND_DISPLAY").is_ok();

                if is_wayland {
                    // Use wl-copy for persistence
                    match Command::new("wl-copy").stdin(Stdio::piped()).spawn() {
                        Ok(mut child) => {
                            if let Some(mut stdin) = child.stdin.take() {
                                let _ = stdin.write_all(full_output.as_bytes());
                            }
                            println!("{color_green}Copied to clipboard!{color_reset}");
                        }
                        Err(e) => {
                            println!("wl-copy failed: {}", e);
                            println!("Falling back to arboard...");

                            // fallback
                            if let Ok(mut clipboard) = Clipboard::new() {
                                let _ = clipboard.set_text(full_output.to_string());
                            } else {
                                println!("{}", full_output);
                            }
                        }
                    }
                } else {
                    // Use arboard everywhere else
                    match Clipboard::new() {
                        Ok(mut clipboard) => {
                            if let Err(e) = clipboard.set_text(full_output.to_string()) {
                                println!("Failed to copy: {}", e);
                                println!("{}", full_output);
                            } else {
                                println!("{color_green}Copied to clipboard!{color_reset}");
                            }
                        }
                        Err(e) => {
                            println!("Clipboard not available: {}", e);
                            println!("{}", full_output);
                        }
                    }
                }
            }
            "q" => {
                println!("Exiting.");
                break;
            }
            _ => println!("Unknown command. Type 'c' to copy or 'q' to quit."),
        }
    }
}

fn main() {
    let args = Args::parse();
    let config = find_config().and_then(|p| load_config(&p));
    let inputs = resolve_inputs(&args, &config);

    if inputs.is_empty() {
        println!("LLMCat\n/\\_/\\\n( o.o )\n> ^ <  LLMCat\nv. 1.2.0");
        return;
    }

    let mut full_output = String::new();
    let files = apply_excludes(collect_files(&inputs), &config);
    let remove_comments = config
        .as_ref()
        .and_then(|c| c.settings.as_ref())
        .and_then(|s| s.remove_comments)
        .unwrap_or(false);

    for path in files {
        if is_binary(&path) {
            continue;
        }
        let raw_content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let content = if remove_comments {
            strip_comments(&raw_content)
        } else {
            raw_content
        };
        let line_count = content.lines().count();

        println!(
            "\n{color_red}===== FILE: {color_reset}{} [{color_blue}{} lines{color_red}] ====={color_reset}",
            path.display(),
            line_count
        );

        let preview: String = content.lines().take(10).collect::<Vec<&str>>().join("\n");
        println!("{color_green}{preview}{color_reset}");
        if line_count > 10 {
            println!("...");
        }

        full_output.push_str(&format!("\n===== FILE: {} =====\n", path.display()));
        full_output.push_str(&content);
        if !content.ends_with('\n') {
            full_output.push_str("\n");
        }
    }

    println!("{color_yellow}c{color_reset} - copy | {color_yellow}q{color_reset} - quit");
    read_input(&full_output);
}

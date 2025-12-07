use chrono::{Datelike, Local};
use clap::Parser;
use colored::*;
use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

#[derive(Parser)] // Turn Args into a command line parser, register --help and --version
#[command(author, version, about, long_about = None)]

struct Args {
    dir: Option<String>,
    #[arg(long)]
    public: bool,
}

const TEMPLATE_REPO: &str = "darrenkuro/repo-template";
const COMMIT_MESSAGE: &str = "Initial commit";

fn main() {
    if let Err(err) = git_init() {
        eprintln!("{} {}", "Error:".red(), err.to_string().red());
        std::process::exit(1);
    }
}

fn run_cmd(cmd: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
    let status = Command::new(cmd).args(args).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("Command `{}` failed", cmd).into())
    }
}

fn get_repo_name(dir: &str) -> Result<String, Box<dyn Error>> {
    if dir == "." {
        let path = env::current_dir()?;
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or("Invalid dir")?;
        Ok(name.to_string())
    } else {
        Ok(Path::new(dir)
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or("Invalid dir")?
            .to_string())
    }
}

fn is_inside_git_repo() -> bool {
    let mut dir = env::current_dir().ok();
    while let Some(path) = dir {
        if path.join(".git").is_dir() {
            return true;
        }
        dir = path.parent().map(Path::to_path_buf);
    }
    false
}

fn hyphen_to_title(name: &str) -> String {
    name.replace('-', " ")
        .split_whitespace()
        .map(|w| {
            let mut c = w.chars();
            c.next()
                .map(|f| f.to_uppercase().collect::<String>())
                .unwrap_or_default()
                + c.as_str()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn replace_placeholders(file: &str, replacements: &[(&str, &str)]) -> Result<(), Box<dyn Error>> {
    if !Path::new(file).exists() {
        eprintln!("{} file not found: {}", "Warning:".yellow(), file);
        return Ok(());
    }

    let contents = fs::read_to_string(file)?;
    let updated = replacements
        .iter()
        .fold(contents, |acc, (k, v)| acc.replace(k, v));
    fs::write(file, updated)?;
    Ok(())
}

fn git_init() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let dir = args.dir.unwrap_or(".".to_string());

    if is_inside_git_repo() {
        return Err("Already a git directory!".into());
    }

    let visibility = if args.public { "--public" } else { "--private" };
    let repo_name = get_repo_name(&dir)?;
    let project_title = hyphen_to_title(&repo_name);
    let year = Local::now().year().to_string();
    // let repo_name = get_current_dir()?;
    run_cmd(
        "gh",
        &[
            "repo",
            "create",
            &repo_name,
            "--template",
            TEMPLATE_REPO,
            visibility,
            "--clone",
        ],
    )?;

    env::set_current_dir(&dir)?;

    // Replace in-place
    replace_placeholders(
        "README.md",
        &[
            ("{{REPO_NAME}}", &repo_name),
            ("{{PROJECT_NAME}}", &project_title),
        ],
    )?;
    replace_placeholders("LICENSE", &[("{{YEAR}}", &year)])?;

    run_cmd("git", &["add", "."])?;
    run_cmd("git", &["commit", "-m", COMMIT_MESSAGE])?;
    run_cmd("git", &["push", "-u", "origin", "main"])?;
    Ok(())
}

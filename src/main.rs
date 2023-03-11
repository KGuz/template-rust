use std::fs;
use std::io::Result;
use std::process::Command;

struct Config {
    author: &'static str,
    project: &'static str,
    year: &'static str,
    url: &'static str,
}

impl Config {
    fn replace(path: &str, f: impl Fn(String) -> String) -> Result<()> {
        let contents = fs::read_to_string(path)?;
        fs::write(path, f(contents))
    }

    fn update_git(&self) -> Result<()> {
        fs::remove_dir_all(".git")?;
        Command::new("git").args(["init"]).output()?;

        let remote = ["remote", "add", "origin", &format!("{}.git", self.url)];
        Command::new("git").args(remote).output()?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let config = Config {
        author: "Kamil Guz",
        project: "template-rust",
        year: "2023",
        url: "https://github.com/KGuz/template-rust",
    };

    Config::replace("Cargo.toml", |contents| {
        let name = format!("name = \"{}\"", config.project);
        contents.replace("name = \"template-rust\"", &name)
    })?;

    Config::replace("Contributing.md", |contents| {
        contents.replace("https://github.com/KGuz/template-rust", config.url)
    })?;

    Config::replace("License.md", |contents| {
        let header = format!("{} {}", config.year, config.author);
        contents.replace("2023 Kamil Guz", &header)
    })?;

    Config::replace("Readme.md", |contents| {
        let header = format!("<h1>{}</h1>", config.project);
        contents
            .replace("<h1>template-rust</h1>", &header)
            .replace("https://github.com/KGuz/template-rust", config.url)
    })?;

    config.update_git()
}

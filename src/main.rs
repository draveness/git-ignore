use std::fs::File;
use std::io::copy;
use std::path::Path;
use clap::{Arg, App, SubCommand};
use colored::*;

fn main() {
    let gitignore_file = ".gitignore";

    let matches = App::new("git-ignore")
        .version("0.1.0")
        .author("Draveness <i@draveness.me>")
        .about("Manages gitignore files")
        .subcommand(SubCommand::with_name("init")
                    .about("Downloads gitignore file from GitHub")
                    .arg(Arg::with_name("language")
                         .required(true)
                         .index(1))
                    .help("Programming language specified gitignore file"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        if Path::new(gitignore_file).exists() {
            println!("{}: .gitignore already exists", "warning".bold().red());
            return
        }

        let lang = matches.value_of("language").unwrap();
        let target = format!("https://raw.githubusercontent.com/github/gitignore/master/{}.gitignore", lang);
        let mut response = reqwest::get(target.as_str()).unwrap();

        if response.status().is_success() {
            let mut dest = {
                let fname = ".gitignore";
                File::create(fname).unwrap()
            };
            let _ = copy(&mut response, &mut dest);
            println!("{}: .gitignore file for {} initialized", "succeed".bold().green(), lang.bold())
        } else {
            println!("{}: {}.gitignore not found on GitHub", "warning".bold().red(), lang.bold());
        }
    }
}

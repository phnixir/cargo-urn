use std::{env, fs, io};
use chrono::prelude::*;

fn is_dir_present(dir: &str) -> io::Result<bool> {
    let mut path = env::current_dir()?;
    path.push(dir);
    let metadata = fs::metadata(path)?;
    Ok(metadata.is_dir())
}

fn is_file_present(file: &str) -> io::Result<bool> {
    let mut path = env::current_dir()?;
    path.push(file);
    let metadata = fs::metadata(path)?;
    Ok(metadata.is_file())
}

fn main() {
    let cur_year = Utc::today().format("%Y");
    let mut pkgname: String = "".to_string();

    match is_file_present("Cargo.toml") {
        Ok(_) => {
            let tomlfile = cargo_toml::Manifest::from_path("Cargo.toml").unwrap();
            let pkg = tomlfile.package.unwrap();
            pkgname = pkg.name.clone();
        },
        Err(err) => {
            println!("Error occured {:?}", err);
            println!("Are you in a cargo project?");
            std::process::exit(1);
        }
    }

    match is_dir_present("target") {
        Ok(_) => {
            println!("You cremated your program alive.");
            fs::remove_dir_all("target").unwrap();
            println!(r#"    ______
   (______)
     )  (
   ,'<\o=`.
  (  <_\)  )
   `."/| .'
     )\\(
    /____\"#);
            println!("Rest in pieces, {}", pkgname);
            println!("{} to {}", cur_year, cur_year)
        },
        Err(err) => {
            println!("Error occured: {:?}", err);
            println!("You may need to run/build your program first.");
            std::process::exit(1);
        }
    }
}

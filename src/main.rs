use colored::*;
use std::collections::HashSet;
use std::fs::{File, OpenOptions, metadata, read_to_string, write};
use std::io::{BufRead, BufReader, Write};
use reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let banner = r#"
    
                        ____        _ _        
                        |  _ \ __  _(_) |_ _ __ 
                        | |_) |\ \/ / | __| '__|
                        |  _ <  >  <| | |_| |   
                        |_| \_\/_/\_\_|\__|_|   

  "#;
    println!("{}", banner.red());
    println!("          [ Simple TokenChecker by github.com/snakextdev ]\n\n");

    let file_metadata = metadata("tokens.txt");
    if let Ok(metadata) = file_metadata {
        if metadata.len() == 0 {
            println!("{}El archivo tokens.txt está vacío", "    Error  ".red());
            return Ok(());
        }
    } else {
        println!("{}No se puede leer el archivo tokens.txt: {}", "    Error  ".red(), file_metadata.unwrap_err());
        return Ok(());
    }

    let file = File::open("tokens.txt")?;

    let tokens = read_to_string("tokens.txt").unwrap();
    let unique_tokens: HashSet<_> = tokens.lines().collect();

    write("tokens.txt", unique_tokens.into_iter().map(|s| s.to_owned() + "\n").collect::<String>()).unwrap();
    
    let mut valid_tokens = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("valid_tokens.txt")?;


    let reader = BufReader::new(file);
    for line in reader.lines() {
        let token = line?;
        let mut is_valid = true;

        let valid_reader = BufReader::new(File::open("valid_tokens.txt")?);
        for valid_line in valid_reader.lines() {
            if valid_line? == token {
                is_valid = false;
                break;
            }
        }
        
        let client = reqwest::blocking::Client::new();
        let response = client.get("https://discord.com/api/v10/users/@me")
            .header("Authorization", format!("{}", token))
            .send()?;

        if response.status().is_success() {
            println!(" {}  Valid Token -> {}", response.status().as_str().green(), token);
            if !is_valid {
                continue;
            }
            writeln!(valid_tokens, "{}", token)?;
        } else {
            println!(" {}  Invalid Token -> {}", response.status().as_str().red(), token);
        }
    }
    Ok(())
}
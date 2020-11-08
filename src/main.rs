use anyhow::{anyhow,Result};
use rustyline::{config::Configurer, error::ReadlineError};
use rustyline::{Editor,ColorMode};
use proctitle::set_title;
use std::process::Command;
use std::env;
use std::ffi::OsString;

#[tokio::main]
async fn main() -> Result<()> {
    set_title("Testcmd");
    let mut rl = Editor::<()>::new();
    rl.set_color_mode(ColorMode::Enabled);
    loop {
        let readline = rl.readline("command > ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                let split = command_processer(&(line.trim().to_string()))?;
                println!("{:#?}",split);
                if let Some(bin) = split.get(0) {
                    if bin != "" {
                        let path = {
                            format!("{};{}",env::var("Path")?,{
                                if let Some(v) = env::current_dir()?.to_str() {
                                    v
                                } else {
                                    ""
                                }
                            })
                        };
                        let data = Command::new(bin).args(&split[1..]).env("PATH", OsString::from(&path)).spawn();
                        println!("\n");
                        if let Err(v) = data {
                            println!("{:#?}",v);
                        }
                    }
                    
                }
                
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    Ok(())
}


fn command_processer(input: &String) -> Result<Vec<String>> {
    let mut out: Vec<String> = vec![];
    let sp: Vec<&str> = input.split(" ").collect();
    let mut i: usize = 0;
    while i < sp.len() {
        if let Some(_) = sp[i].find("\"") {
            let mut found = false;
            let mut full = vec![];
            for l in i..sp.len() {
                let ss = sp[l].to_string();
                let sc: Vec<char> = ss.chars().collect();
                if let Some(v) = sc.get(sc.len() - 1) {
                    if v != &'"' {
                        full.push(ss);
                        i += 1;
                    } else {
                        found = true;
                        full.push(ss);
                        i += 1;
                        break;
                    }
                }

            }
            if found {
                let full_string = full.join(" ");
                out.push(full_string.trim().to_string());
            } else {
                return Err(anyhow!("Unclosed quotes!"));
            };
        } else {
            out.push(sp[i].to_string());
            i += 1;
        }
    }
    Ok(out)
}
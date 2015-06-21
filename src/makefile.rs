use std::fs::File;
use std::io;
use regex::Regex;

enum Error { OpeningFile, ParsingTargets }

struct Makefile {
    targets: Vec<String>,
    path: String,
}


fn open(path: &str) -> Result<Makefile, Error> {
    match File::open(path) {
        Ok(f) => {
            match read_targets(&mut io::BufReader::new(f)) {
                Ok(targets) => Ok(Makefile{ targets: targets, path: path.to_string() }),
                Err(e) => Err(e)
            }
        },
        Err(_) => Err(Error::OpeningFile)
    }
}

fn read_targets(buff: &mut io::BufRead) -> Result<Vec<String>, Error> {
    loop {
        let ctn =  &mut String::new();
        match buff.read_line(ctn) {
            Ok(s) => match s {
                0 => break,
                _ => {
                    println!("read {0}", s);
                    break;
                }
            },
            Err(_) => {
                return Err(Error::ParsingTargets);
            }
        }
    }

    Ok(vec![])
}

fn get_targets_from_line(l: &String) -> Option<Vec<String>> {
    let parts: Vec<&str> = l.splitn(2, ':').collect();

    if parts.len() > 1 {
       let targets = split_targets(parts[0]);
       if targets.len() > 0 {
           Some(targets);
       }
    }

    None
}

fn split_targets(t: &str) -> Vec<String> {
    let mut targets = vec![];
    let re = Regex::new(r"\s?(\S+)\s?").unwrap();

    for cap in re.captures_iter(t) {
        match cap.at(0) {
            Some(t) => targets.push(t.to_string()),
            None => continue
        }
    }

    targets
}


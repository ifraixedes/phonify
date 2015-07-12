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
    let phony_targets: Vec<String> = Vec::new();

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

    Ok(phony_targets)
}

fn is_phoniphy_macro(line: &str) -> bool {
    let re = Regex::new(r"^\s*#\s*\[phoniphy\]").unwrap();
    return re.is_match(line);
}

fn get_targets_from_line(l: &str) -> Option<Vec<&str>> {
    let parts: Vec<&str> = l.splitn(2, ':').collect();

    if parts.len() > 1 {
       let targets = split_targets(parts[0]);
       if targets.len() > 0 {
           return Some(targets);
       }
    }

    None
}

fn split_targets(t: &str) -> Vec<&str> {
    let mut targets = vec![];
    let re = Regex::new(r"\S+").unwrap();

    for cap in re.captures_iter(t) {
        match cap.at(0) {
            Some(t) => {
                match t.to_string().find('%') {
                    Some(_) => continue,
                    None => targets.push(t)
                }
            }
            None => continue
        }
    }

    targets
}

#[cfg(test)]
mod test_is_phoniphy_macro {
    use super::is_phoniphy_macro;

    #[test]
    fn it_retunrs_true_for_perfect_macro_syntax() {
        assert_eq!(is_phoniphy_macro("#[phoniphy]"), true);
    }

    #[test]
    fn it_returns_true_if_macro_is_prefixed_with_blanks() {
        assert_eq!(is_phoniphy_macro("\t #[phoniphy]"), true);
    }

    #[test]
    fn it_returns_true_if_macro_has_trailing_blanks() {
        assert_eq!(is_phoniphy_macro("#[phoniphy]    "), true);
    }

    #[test]
    fn it_returns_true_if_macro_has_blanks_betweeen_comment_and_macro() {
        assert_eq!(is_phoniphy_macro("# \t   [phoniphy]"), true);
    }

    #[test]
    fn it_returns_true_if_macro_has_blanks_everywhere() {
        assert_eq!(is_phoniphy_macro("   #    [phoniphy] \t   "), true);
    }

    #[test]
    fn it_returns_false_if_not_surrounded_by_square_brackets() {
        assert_eq!(is_phoniphy_macro("#phoniphy]"), false);
    }

    #[test]
    fn it_returns_false_if_line_is_not_makefile_comment() {
        assert_eq!(is_phoniphy_macro("[phoniphy]"), false);
    }

    #[test]
    fn it_returns_false_if_macro_is_not_phoniphy() {
        assert_eq!(is_phoniphy_macro("[phoni]"), false);
    }
}

#[cfg(test)]
mod test_target_from_line {
    use super::get_targets_from_line;

    #[test]
    fn it_returns_targets_for_target_line() {
        assert_eq!(get_targets_from_line("start: dev-env  "), Some(vec!["start"]));
    }

    #[test]
    fn it_returns_empty_for_no_target_line() {
        assert_eq!(get_targets_from_line("  $(CC) main.c"), None);
    }
}

#[cfg(test)]
mod test_split_targets {
    use super::split_targets;

    #[test]
    fn it_split_one_targets() {
        assert_eq!(split_targets("run"), vec!["run"])
    }

    #[test]
    fn it_split_one_targets_removing_blanks() {
        assert_eq!(split_targets("  dev "), vec!["dev"])
    }

    #[test]
    fn it_split_two_targets() {
        assert_eq!(split_targets("dev start"), vec!["dev", "start"])
    }

    #[test]
    fn it_split_three_targets_with_more_than_one_space() {
        assert_eq!(split_targets("hey   you   !"), vec!["hey", "you", "!"])
    }

    #[test]
    fn it_consider_targets_which_are_variables() {
        assert_eq!(split_targets("$(list)"), vec!["$(list)"])
    }

    #[test]
    fn it_does_not_consider_targets_which_are_patterns() {
        let empty: Vec<String> = vec![];

        assert_eq!(split_targets("%.o"), empty);
    }
}

extern crate url;

use std::old_io as io;
use std::string::String;

enum ParseOption {
    HostName,
    Port,
    Protocol,
    Username,
    Password,
    Path,
    Fragment,
    Query
}

fn fail(message: Option<&str>) {
    std::env::set_exit_status(1);
    match message {
        Some(m) => println!("{}", m),
        None    => {}
    };
}

fn print_help() {
    println!("Usage: urlp OPTION URL");
    println!("");
    println!("Options:");
    println!("    --host                  Hostname");
    println!("    --port                  Port");
    println!("    --protocol, --scheme    Protocol");
    println!("    --password              Password");
    println!("    --username              Password");
    println!("    --path                  Path");
    println!("    --fragment              Fragment");
    println!("    --query                 Query string");
}

fn get_parse_option(arg: &str) -> ParseOption {
    let option_prefix_chars: &[char] = &['-'];
    let trimmed_arg = arg.trim_matches(option_prefix_chars);

    match trimmed_arg {
        "host"     => ParseOption::HostName,
        "port"     => ParseOption::Port,
        "protocol" => ParseOption::Protocol,
        "password" => ParseOption::Password,
        "username" => ParseOption::Username,
        "path"     => ParseOption::Path,
        "fragment" => ParseOption::Fragment,
        "query"    => ParseOption::Query,
        "scheme"   => ParseOption::Protocol,
        _          => panic!("Invalid parsing option")
    }
}

fn get_url_from_stdin() -> Result<String, String> {
    let mut stdin     = io::stdin();
    let mut stdin_url = String::new();

    // only read the first line
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => {
                if stdin_url.is_empty() {
                    stdin_url.push_str(&l);
                }
            }
            Err(_) => {}
        }
    }

    if stdin_url.is_empty() {
        return Err("Stdin read failed".to_string());
    }

    let newline_chars: &[char] = &['\n'];
    let stdin_slice = &stdin_url.trim_matches(newline_chars);

    return Ok(stdin_slice.to_string());
}

fn execute_from_stdin(option: &str) {
    match get_url_from_stdin() {
        Ok(url) => {
            execute(option, &url);
        },
        _ => {
            fail(Some("Failed to read from stdin"));
        }
    };
}


fn parse_component<T: ToString>(option: Option<T>, description: &str) -> Result<String, String> {
    return match option {
        Some(x) => Ok(x.to_string()),
        None    => Err(format!("No {} found", description))
    }
}

fn execute(option: &str, url: &str) {
    let result = url::Url::parse(url);

    match result {
        Err(e) => {
            fail(Some(&format!("Parse error: {}", e)));
            return;
        },
        _ => {}
    };

    let parsed = result.unwrap();

    let value = match get_parse_option(option) {
        ParseOption::HostName => parse_component(parsed.domain(), "hostname"),
        ParseOption::Port     => parse_component(parsed.port(), "port"),
        ParseOption::Protocol => parse_component(Some(parsed.scheme), "scheme"),
        ParseOption::Username => parse_component(parsed.username(), "username"),
        ParseOption::Password => parse_component(parsed.password(), "password"),
        ParseOption::Path     => parse_component(parsed.serialize_path(), "path"),
        ParseOption::Fragment => parse_component(parsed.fragment, "fragment"),
        ParseOption::Query    => parse_component(parsed.query, "query")
    };

    match value {
        Ok(v) => {
            println!("{}", v);
        }
        Err(e) => {
            fail(Some(&e));
        }
    };
}

fn optional_print_help(option: &str) -> bool {
    match option.as_slice() {
        "-h" => {
            print_help();
            return true;
        },
        "--help" => {
            print_help();
            return true;
        }
        _ => {
            return false;
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.as_slice() {
        [_, ref option]  => match optional_print_help(option.as_slice()) {
            false => execute_from_stdin(option),
            _ => {}
        },

        [_, ref option, ref url] => match optional_print_help(option.as_slice()) {
            false => execute(option, url),
            _ => execute(option, url)
        },

        _  => {
            print_help();
            fail(None);
        }
    }
}

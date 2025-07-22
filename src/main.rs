use ccl_rs::key_val::KeyVal;
use ccl_rs::monoid::Monoid;
use ccl_rs::parser::CCL;
use clap::Parser;
use std::fs;
use std::io::{self, Read};
use std::process;

#[derive(Parser)]
#[command(name = "cclq")]
#[command(about = "Merge CCL files and query")]
#[command(
    long_about = "Merge CCL files and query. Queries are single keys."
)]
struct Args {
    /// Input files to query (default: read from stdin)
    #[arg(short, long, num_args = 1..)]
    file: Vec<String>,

    /// Query key (empty for print all)
    #[arg(short, long, num_args = 1..)]
    query: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let file = if args.file.is_empty() {
        vec!["/dev/stdin".to_string()]
    } else {
        args.file
    };

    // Load and merge all CCL files
    let ccl = load_files(file);

    // Execute query
    if args.query.is_empty() {
        print!("{}", ccl.pretty());
    } else {
        for query in args.query {
            match execute_query(&query, &ccl) {
                Ok(result) => {
                    print!("{}", result.pretty());
                    println!();
                }
                Err(e) => {
                    eprintln!("Query failed: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}

fn load_files(files: Vec<String>) -> CCL {
    let mut ccls = Vec::new();

    for file_path in files {
        let content = if file_path == "/dev/stdin" {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap_or_else(|e| {
                eprintln!("Failed to read from stdin: {}", e);
                process::exit(1);
            });
            buffer
        } else {
            fs::read_to_string(&file_path).unwrap_or_else(|e| {
                eprintln!("Failed to read file '{}': {}", file_path, e);
                process::exit(1);
            })
        };

        match KeyVal::parse(&content) {
            Ok(key_vals) => {
                let ccl = CCL::parse(key_vals);
                ccls.push(ccl);
            }
            Err(e) => {
                eprintln!("Failed to parse file '{}': {}", file_path, e);
                process::exit(1);
            }
        }
    }

    CCL::aggregate(ccls)
}

fn execute_query(query_key: &str, ccl: &CCL) -> Result<CCL, String> {
    let nested_keys = query_key.split('=').collect::<Vec<&str>>();
    let mut ccl = ccl.clone();
    for key in nested_keys {
        ccl = query_single_key(key, &ccl)?;
    }
    Ok(ccl)
}

fn query_single_key(key: &str, ccl: &CCL) -> Result<CCL, String> {
    let CCL(map) = ccl;
    match map.get(key) {
        Some(ccl) => Ok(ccl.clone()),
        None => Err(format!("Key '{}' not found", key)),
    }
}

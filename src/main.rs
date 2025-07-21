use ccl_rs::key_val::KeyVal;
use ccl_rs::parser::CCL;
use ccl_rs::monoid::Monoid;
use clap::Parser;
use std::fs;
use std::io::{self, Read};
use std::process;

#[derive(Parser)]
#[command(name = "cclq")]
#[command(about = "Query values in CCL files")]
#[command(long_about = "Query values in CCL files. Queries are single keys.")]
struct Args {
    /// Input files to query (default: read from stdin)
    files: Vec<String>,
    
    /// Query key (empty for print all)
    #[arg(short, long)]
    query: Option<String>,
}

fn main() {
    let args = Args::parse();
    
    let files = if args.files.is_empty() {
        vec!["/dev/stdin".to_string()]
    } else {
        args.files
    };
    
    // Load and merge all CCL files
    let ccl = load_files(files);
    
    // Execute query
    match execute_query(args.query.as_deref(), &ccl) {
        Ok(result) => {
            print!("{}", result.pretty());
        }
        Err(e) => {
            eprintln!("Query failed: {}", e);
            process::exit(1);
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

fn execute_query(query_key: Option<&str>, ccl: &CCL) -> Result<CCL, String> {
    match query_key {
        None => {
            // No query: return all data
            Ok(ccl.clone())
        }
        Some(key) => {
            // Single key query
            query_single_key(key, ccl)
        }
    }
}

fn query_single_key(key: &str, ccl: &CCL) -> Result<CCL, String> {
    // Need to access the internal map of CCL to query a specific key
    // This would require adding a method to CCL or making the field public
    // For now, return an error indicating this needs to be implemented
    Err("Single key querying not yet implemented - need to add query method to CCL".to_string())
}

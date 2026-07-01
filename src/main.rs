use std::io::{self, BufRead};

use clap::Parser;
use rsomics_eigenvector_centrality::{
    EigenvectorError, eigenvector_centrality, format_scientific, parse_edgelist,
};
use serde_json::json;

#[derive(Parser)]
#[command(
    name = "rsomics-eigenvector-centrality",
    about = "Eigenvector centrality of undirected graphs (power-iteration, value-exact vs networkx)"
)]
struct Cli {
    #[arg(long, default_value_t = 100, help = "Maximum power-iteration steps")]
    max_iter: usize,

    #[arg(long, default_value_t = 1e-6, help = "L1 convergence tolerance")]
    tol: f64,

    #[arg(long, help = "Emit JSON {node: value, ...} instead of TSV")]
    json: bool,
}

fn main() {
    let cli = Cli::parse();

    let stdin = io::stdin();
    let mut buf = String::new();
    for line in stdin.lock().lines() {
        let line = line.expect("stdin read error");
        buf.push_str(&line);
        buf.push('\n');
    }

    let g = parse_edgelist(&buf);
    if g.node_count() == 0 {
        eprintln!("error: empty graph");
        std::process::exit(1);
    }

    match eigenvector_centrality(&g, cli.max_iter, cli.tol) {
        Ok(centrality) => {
            let names = g.node_names();

            if cli.json {
                let mut map = serde_json::Map::new();
                let mut pairs: Vec<(&str, f64)> = names
                    .iter()
                    .enumerate()
                    .map(|(i, n)| (n.as_str(), centrality[i]))
                    .collect();
                pairs.sort_by_key(|(n, _)| *n);
                for (name, val) in pairs {
                    map.insert(name.to_owned(), json!(val));
                }
                println!("{}", serde_json::Value::Object(map));
            } else {
                let mut pairs: Vec<(&str, f64)> = names
                    .iter()
                    .enumerate()
                    .map(|(i, n)| (n.as_str(), centrality[i]))
                    .collect();
                pairs.sort_by_key(|(n, _)| *n);
                for (name, val) in pairs {
                    println!("{name}\t{}", format_scientific(val));
                }
            }
        }
        Err(EigenvectorError::EmptyGraph) => {
            eprintln!("error: empty graph");
            std::process::exit(1);
        }
        Err(EigenvectorError::FailedConvergence { max_iter }) => {
            eprintln!("error: power iteration failed to converge within {max_iter} iterations");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cli_debug_assert() {
        use clap::CommandFactory;
        crate::Cli::command().debug_assert();
    }
}

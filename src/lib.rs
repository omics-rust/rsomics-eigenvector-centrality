use std::collections::HashMap;

/// Adjacency list preserving node and neighbor insertion order,
/// mirroring `nx.read_edgelist → Graph` semantics.
///
/// Parallel edges are silently deduplicated (undirected-set).
pub struct Graph {
    node_ids: HashMap<String, usize>,
    nodes: Vec<String>,
    /// `adj[u]` = neighbour ids in insertion order (deduped).
    adj: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            node_ids: HashMap::new(),
            nodes: Vec::new(),
            adj: Vec::new(),
        }
    }

    fn intern(&mut self, name: &str) -> usize {
        if let Some(&id) = self.node_ids.get(name) {
            return id;
        }
        let id = self.nodes.len();
        self.nodes.push(name.to_owned());
        self.node_ids.insert(name.to_owned(), id);
        self.adj.push(Vec::new());
        id
    }

    pub fn add_edge(&mut self, u: &str, v: &str) {
        let uid = self.intern(u);
        let vid = self.intern(v);
        if uid == vid {
            // self-loops don't participate in undirected eigenvector centrality
            return;
        }
        if !self.adj[uid].contains(&vid) {
            self.adj[uid].push(vid);
        }
        if !self.adj[vid].contains(&uid) {
            self.adj[vid].push(uid);
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn node_names(&self) -> &[String] {
        &self.nodes
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum EigenvectorError {
    EmptyGraph,
    FailedConvergence { max_iter: usize },
}

impl std::fmt::Display for EigenvectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyGraph => write!(f, "cannot compute centrality for the null graph"),
            Self::FailedConvergence { max_iter } => write!(
                f,
                "power iteration failed to converge within {max_iter} iterations"
            ),
        }
    }
}

impl std::error::Error for EigenvectorError {}

/// Power-iteration eigenvector centrality matching networkx semantics exactly.
///
/// Mirrors `networkx.eigenvector_centrality` (BSD-3-Clause):
/// - init: x[v] = 1/N  (all-ones vector normalised by its sum)
/// - each iter: xlast = x; x = xlast.copy() [implements A+I shift];
///   for n in x: for nbr in G[n]: x[nbr] += xlast[n]
/// - normalise: norm = hypot(x.values()) or 1; x /= norm
/// - converge: L1(x − xlast) < N × tol
pub fn eigenvector_centrality(
    g: &Graph,
    max_iter: usize,
    tol: f64,
) -> Result<Vec<f64>, EigenvectorError> {
    let n = g.node_count();
    if n == 0 {
        return Err(EigenvectorError::EmptyGraph);
    }

    let init = 1.0_f64 / n as f64;
    let mut x: Vec<f64> = vec![init; n];

    for _ in 0..max_iter {
        let xlast = x.clone();
        x.clone_from(&xlast);

        for (node, &xn) in xlast.iter().enumerate() {
            for &nbr in &g.adj[node] {
                x[nbr] += xn;
            }
        }

        let norm: f64 = x.iter().map(|v| v * v).sum::<f64>().sqrt();
        let norm = if norm == 0.0 { 1.0 } else { norm };
        for v in &mut x {
            *v /= norm;
        }

        let l1: f64 = x.iter().zip(xlast.iter()).map(|(a, b)| (a - b).abs()).sum();
        if l1 < n as f64 * tol {
            return Ok(x);
        }
    }

    Err(EigenvectorError::FailedConvergence { max_iter })
}

/// Format a float as Python's `{:.17e}`: 17 fractional digits and a
/// sign-prefixed, minimum-two-digit exponent (`5.77...e-01`, not `e-1`).
/// Rust's `{:.17e}` omits the exponent sign and zero-padding, so the raw
/// output would diverge from the networkx reference text.
pub fn format_scientific(v: f64) -> String {
    let raw = format!("{v:.17e}");
    let (mantissa, exp) = raw.split_once('e').expect("scientific format has 'e'");
    let (sign, digits) = match exp.strip_prefix('-') {
        Some(rest) => ('-', rest),
        None => ('+', exp),
    };
    format!("{mantissa}e{sign}{digits:0>2}")
}

/// Parse an edge-list text into a Graph (# comments + blank lines skipped).
pub fn parse_edgelist(s: &str) -> Graph {
    let mut g = Graph::new();
    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let Some(u) = parts.next() else { continue };
        let Some(v) = parts.next() else { continue };
        g.add_edge(u, v);
    }
    g
}

#[cfg(test)]
mod tests {
    use super::format_scientific;

    #[test]
    fn scientific_matches_python() {
        assert_eq!(
            format_scientific(0.5773502691896258),
            "5.77350269189625842e-01"
        );
        assert_eq!(format_scientific(0.5), "5.00000000000000000e-01");
        assert_eq!(format_scientific(1.0), "1.00000000000000000e+00");
        assert_eq!(format_scientific(12.5), "1.25000000000000000e+01");
    }
}

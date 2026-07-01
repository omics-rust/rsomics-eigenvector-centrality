# rsomics-eigenvector-centrality

Eigenvector centrality of undirected graphs via power iteration.

Reads an edge list from stdin (`u v` per line, `#` comments and blank lines skipped, string node labels).
Outputs one `node<TAB>value` line per node, sorted lexicographically by node label, with each
value formatted as `{:.17e}` (17 fractional digits, two-digit signed exponent — matching Python's
`format(v, ".17e")`).

Values match `networkx.eigenvector_centrality` to within 1 ULP (worst observed absolute error
5.6e-17); the last one or two of the 17 printed digits may differ from networkx as sub-ULP
floating-point noise.

## Usage

```
printf "0 1\n1 2\n0 2\n" | rsomics-eigenvector-centrality
0       5.77350269189625842e-01
1       5.77350269189625842e-01
2       5.77350269189625842e-01
```

### Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--max-iter N` | 100 | Maximum power-iteration steps |
| `--tol F` | 1e-6 | L1 convergence tolerance |
| `--json` | off | Emit `{"node": value, ...}` JSON instead of TSV |

## Performance

Tested on macOS (Apple M2), 5 000-node circulant graph (50 000 edges):

| Tool | Mean per call |
|------|--------------|
| networkx 3.6.1 | ~21.5 ms |
| rsomics-eigenvector-centrality | ~117 µs |
| **Ratio** | **~184×** |

## Origin

This crate is an independent Rust reimplementation of `networkx.eigenvector_centrality`
(power-iteration variant) based on:
- NetworkX source (BSD-3-Clause), function `eigenvector_centrality` in
  `networkx/algorithms/centrality/eigenvector.py`
- The NetworkX documentation

License: MIT OR Apache-2.0.
Upstream credit: NetworkX (https://networkx.org/) (BSD-3-Clause).

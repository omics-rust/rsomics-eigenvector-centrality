use criterion::{Criterion, criterion_group, criterion_main};
use rsomics_eigenvector_centrality::{eigenvector_centrality, parse_edgelist};

/// Build a dense connected graph on 5 000 nodes.
///
/// Each node i connects to (i + k) mod n for k in 1..=ring_k (circulant graph).
/// A circulant with ring_k = 10 has spectral gap large enough for the
/// power iteration to converge within 100 iterations.
fn make_bench_graph() -> rsomics_eigenvector_centrality::Graph {
    let n = 5_000_u32;
    let ring_k = 10_u32;
    let mut edges = String::new();
    for i in 0..n {
        for k in 1..=ring_k {
            let j = (i + k) % n;
            edges.push_str(&format!("{i} {j}\n"));
        }
    }
    parse_edgelist(&edges)
}

fn bench_eigenvector(c: &mut Criterion) {
    let g = make_bench_graph();
    // Verify the fixture converges before benchmarking.
    eigenvector_centrality(&g, 1000, 1e-6).expect("bench graph must converge");

    c.bench_function("eigenvector_5k", |b| {
        b.iter(|| {
            eigenvector_centrality(&g, 1000, 1e-6).unwrap();
        })
    });
}

criterion_group!(benches, bench_eigenvector);
criterion_main!(benches);

#![allow(clippy::excessive_precision)]
/// Compatibility tests against real networkx 3.6.1 output.
///
/// Source: `networkx/algorithms/centrality/eigenvector.py`.
/// Goldens obtained by running `networkx.eigenvector_centrality` with
/// max_iter=100, tol=1e-6, nstart=None (all-ones) on each graph and
/// printing `{v:.17e}` for each node in lexicographic order.
/// Oracle: networkx 3.6.1 (miniforge rs-up env).
use rsomics_eigenvector_centrality::{eigenvector_centrality, parse_edgelist};

fn worst_abs_err(got: &[f64], expected: &[f64]) -> f64 {
    got.iter()
        .zip(expected.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0_f64, f64::max)
}

/// Triangle K3.
/// networkx output (lex order 0,1,2):
///   0  5.77350269189625842e-01
///   1  5.77350269189625842e-01
///   2  5.77350269189625842e-01
#[test]
fn triangle_k3() {
    let g = parse_edgelist("0 1\n1 2\n0 2\n");
    let c = eigenvector_centrality(&g, 100, 1e-6).unwrap();

    // Nodes are inserted in edge-order: 0, 1, 2
    let expected = [
        5.77350269189625842e-01_f64,
        5.77350269189625842e-01_f64,
        5.77350269189625842e-01_f64,
    ];
    let err = worst_abs_err(&c, &expected);
    assert!(
        err < 1e-12,
        "K3 worst abs err {err:.3e} exceeds 1e-12\ngot:      {c:?}\nexpected: {expected:?}"
    );
}

/// Path graph P5 (nodes 0–4).
/// networkx output (lex order 0,1,2,3,4):
///   0  2.88676032028529372e-01
///   1  4.99999995082353044e-01
///   2  5.77349380271444335e-01
///   3  4.99999995082352933e-01
///   4  2.88676032028529372e-01
#[test]
fn path_p5() {
    let g = parse_edgelist("0 1\n1 2\n2 3\n3 4\n");
    let c = eigenvector_centrality(&g, 100, 1e-6).unwrap();

    // Nodes inserted in order: 0,1,2,3,4
    let expected = [
        2.88676032028529372e-01_f64,
        4.99999995082353044e-01_f64,
        5.77349380271444335e-01_f64,
        4.99999995082352933e-01_f64,
        2.88676032028529372e-01_f64,
    ];
    let err = worst_abs_err(&c, &expected);
    assert!(
        err < 1e-12,
        "P5 worst abs err {err:.3e} exceeds 1e-12\ngot:      {c:?}\nexpected: {expected:?}"
    );
}

/// Complete graph K4.
/// networkx output (lex order 0,1,2,3):
///   0  5.00000000000000000e-01
///   1  5.00000000000000000e-01
///   2  5.00000000000000000e-01
///   3  5.00000000000000000e-01
#[test]
fn complete_k4() {
    let g = parse_edgelist("0 1\n0 2\n0 3\n1 2\n1 3\n2 3\n");
    let c = eigenvector_centrality(&g, 100, 1e-6).unwrap();

    let expected = [0.5_f64, 0.5_f64, 0.5_f64, 0.5_f64];
    let err = worst_abs_err(&c, &expected);
    assert!(
        err < 1e-12,
        "K4 worst abs err {err:.3e} exceeds 1e-12\ngot:      {c:?}\nexpected: {expected:?}"
    );
}

/// Random connected graph G(20, 50) seed=42 (networkx gnm_random_graph).
///
/// Edge list (from nx.edges() with seed=42):
///   0 2 / 0 3 / 0 8 / 0 17
///   1 7 / 1 8 / 1 10 / 1 13
///   2 6 / 2 8 / 2 9 / 2 12 / 2 17 / 2 18
///   3 7 / 3 10 / 3 11 / 3 12 / 3 17
///   4 6 / 4 7 / 4 8 / 4 14
///   5 11 / 5 13 / 5 14 / 5 19
///   6 7 / 6 10 / 6 11 / 6 17 / 6 18
///   7 8 / 7 11 / 7 13 / 7 17
///   8 10 / 8 12 / 8 17
///   9 19
///   10 12
///   11 14 / 11 18 / 11 19
///   12 15 / 12 18
///   13 18
///   14 17 / 14 18
///   16 19
///
/// networkx output (lex-sorted by str key):
///   0   2.03246456207534620e-01
///   1   1.74395645532165994e-01
///   10  2.21659954029404305e-01
///   11  2.57779850747924255e-01
///   12  2.32869005728698508e-01
///   13  1.45298331091919197e-01
///   14  1.90210594859967996e-01
///   15  3.91412642736780128e-02
///   16  1.25250800434968564e-02
///   17  3.24383382740939641e-01
///   18  2.39818752355785814e-01
///   19  7.45148937517248200e-02
///   2   2.86976164534223144e-01
///   3   2.65221116312849281e-01
///   4   1.97406130223171855e-01
///   5   1.12248201299187264e-01
///   6   3.13644130095900475e-01
///   7   3.37974400818587872e-01
///   8   3.32621675931507044e-01
///   9   6.07609136469801428e-02
#[test]
fn random_g20_50_seed42() {
    let edgelist = "\
0 2\n0 3\n0 8\n0 17\n\
1 7\n1 8\n1 10\n1 13\n\
2 6\n2 8\n2 9\n2 12\n2 17\n2 18\n\
3 7\n3 10\n3 11\n3 12\n3 17\n\
4 6\n4 7\n4 8\n4 14\n\
5 11\n5 13\n5 14\n5 19\n\
6 7\n6 10\n6 11\n6 17\n6 18\n\
7 8\n7 11\n7 13\n7 17\n\
8 10\n8 12\n8 17\n\
9 19\n\
10 12\n\
11 14\n11 18\n11 19\n\
12 15\n12 18\n\
13 18\n\
14 17\n14 18\n\
16 19\n";

    let g = parse_edgelist(edgelist);
    assert_eq!(g.node_count(), 20);
    let c = eigenvector_centrality(&g, 100, 1e-6).unwrap();

    // Networkx lex-sorted output; node string order matches id order for integer names,
    // but networkx sorts as strings: "0","1","10","11",...
    // Our Vec is indexed by insertion order (0..19 numerically).
    // Build a name→value map for comparison.
    let names = g.node_names();
    let got: std::collections::HashMap<&str, f64> = names
        .iter()
        .enumerate()
        .map(|(i, n)| (n.as_str(), c[i]))
        .collect();

    let golden: &[(&str, f64)] = &[
        ("0", 2.03246456207534620e-01),
        ("1", 1.74395645532165994e-01),
        ("10", 2.21659954029404305e-01),
        ("11", 2.57779850747924255e-01),
        ("12", 2.32869005728698508e-01),
        ("13", 1.45298331091919197e-01),
        ("14", 1.90210594859967996e-01),
        ("15", 3.91412642736780128e-02),
        ("16", 1.25250800434968564e-02),
        ("17", 3.24383382740939641e-01),
        ("18", 2.39818752355785814e-01),
        ("19", 7.45148937517248200e-02),
        ("2", 2.86976164534223144e-01),
        ("3", 2.65221116312849281e-01),
        ("4", 1.97406130223171855e-01),
        ("5", 1.12248201299187264e-01),
        ("6", 3.13644130095900475e-01),
        ("7", 3.37974400818587872e-01),
        ("8", 3.32621675931507044e-01),
        ("9", 6.07609136469801428e-02),
    ];

    let mut worst = 0.0_f64;
    for &(name, exp) in golden {
        let got_val = *got
            .get(name)
            .unwrap_or_else(|| panic!("missing node {name}"));
        let err = (got_val - exp).abs();
        worst = worst.max(err);
    }
    assert!(worst < 1e-12, "G20 worst abs err {worst:.3e} exceeds 1e-12");
}

/// Hub self-loop: node 0 links 1 and 2 and carries a self-loop.
/// networkx keeps the self-loop (`G[0]` includes 0), so it adds a diagonal
/// term A[0][0]=1 — the hub is boosted relative to the no-self-loop star.
/// networkx output (lex order 0,1,2):
///   0  8.16496580927726145e-01
///   1  4.08248290463862962e-01
///   2  4.08248290463862962e-01
#[test]
fn hub_self_loop() {
    let g = parse_edgelist("0 1\n0 2\n0 0\n");
    let c = eigenvector_centrality(&g, 100, 1e-6).unwrap();

    // Nodes inserted in order: 0,1,2
    let expected = [
        8.16496580927726145e-01_f64,
        4.08248290463862962e-01_f64,
        4.08248290463862962e-01_f64,
    ];
    let err = worst_abs_err(&c, &expected);
    assert!(
        err < 1e-12,
        "hub self-loop worst abs err {err:.3e} exceeds 1e-12\ngot:      {c:?}\nexpected: {expected:?}"
    );
}

/// Leaf self-loop: path 0-1-2 with a self-loop on the terminal node 2.
/// The self-loop lifts the leaf above its unweighted path value.
/// networkx output (lex order 0,1,2):
///   0  3.27986682781174543e-01
///   1  5.91009673867220697e-01
///   2  7.36975102234507462e-01
#[test]
fn leaf_self_loop() {
    let g = parse_edgelist("0 1\n1 2\n2 2\n");
    let c = eigenvector_centrality(&g, 100, 1e-6).unwrap();

    // Nodes inserted in order: 0,1,2
    let expected = [
        3.27986682781174543e-01_f64,
        5.91009673867220697e-01_f64,
        7.36975102234507462e-01_f64,
    ];
    let err = worst_abs_err(&c, &expected);
    assert!(
        err < 1e-12,
        "leaf self-loop worst abs err {err:.3e} exceeds 1e-12\ngot:      {c:?}\nexpected: {expected:?}"
    );
}

/// A repeated self-loop edge collapses to a single self-neighbour.
#[test]
fn self_loop_dedup() {
    let g1 = parse_edgelist("0 1\n0 2\n0 0\n0 0\n");
    let g2 = parse_edgelist("0 1\n0 2\n0 0\n");
    let c1 = eigenvector_centrality(&g1, 100, 1e-6).unwrap();
    let c2 = eigenvector_centrality(&g2, 100, 1e-6).unwrap();
    let err = worst_abs_err(&c1, &c2);
    assert!(err < 1e-15, "self-loop dedup err {err:.3e}");
}

/// Verify comment and blank line skipping in the parser.
#[test]
fn parser_skips_comments_and_blanks() {
    let input = "# header\n\n0 1\n# comment\n1 2\n\n";
    let g = parse_edgelist(input);
    assert_eq!(g.node_count(), 3);
}

/// Parallel edges must be deduplicated (same result as single edge).
#[test]
fn parallel_edge_dedup() {
    let g1 = parse_edgelist("0 1\n0 1\n1 2\n");
    let g2 = parse_edgelist("0 1\n1 2\n");
    let c1 = eigenvector_centrality(&g1, 100, 1e-6).unwrap();
    let c2 = eigenvector_centrality(&g2, 100, 1e-6).unwrap();
    let err = worst_abs_err(&c1, &c2);
    assert!(err < 1e-15, "parallel dedup err {err:.3e}");
}

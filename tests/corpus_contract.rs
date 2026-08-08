use std::fs;
use std::path::PathBuf;

fn corpus_file(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("corpus")
        .join("vfp")
        .join(name);
    fs::read_to_string(path).unwrap().to_ascii_lowercase()
}

#[test]
fn libfunct_fixture_contains_corrected_contracts() {
    let source = corpus_file("libfunct_updated.prg");
    assert!(source.contains("function nextlat"));
    assert!(source.contains("dtor(normalizedegrees(tndirection))"));
    assert!(source.contains("function nextlong"));
    assert!(source.contains("function arclength"));
    assert!(source.contains("function turnradius"));
    assert!(!source.contains("cos(direction)*distance"));
}

#[test]
fn matchprg_fixture_uses_global_shortest_path_state() {
    let source = corpus_file("matchprg_updated.prg");
    assert!(source.contains("function shortestpath"));
    assert!(source.contains("ladistance"));
    assert!(source.contains("laprevious"));
    assert!(source.contains("global minimum"));
}

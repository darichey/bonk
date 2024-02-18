use std::path::PathBuf;

use bonk_workspace::Workspace;

#[test]
fn test() {
    let cfg_path = PathBuf::from("example/Bonk.toml");
    let workspace = Workspace::from_cfg(&cfg_path).unwrap();

    let mut paths: Vec<PathBuf> = workspace.included_paths().collect();

    paths.sort();

    assert_eq!(
        paths,
        vec![
            PathBuf::from("example/bar.bonk"),
            PathBuf::from("example/foo.bonk"),
        ]
    )
}

use std::path::PathBuf;

use bonk_db::create_db;

#[test]
fn test() {
    let cfg_path = PathBuf::from("example/Bonk.toml");
    let db_path = PathBuf::from(":memory:"); // FIXME should this really be a path

    create_db(cfg_path, db_path).unwrap();

    // TODO run some test queries
}

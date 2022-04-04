use purpur::get_paths;


#[test]
fn test_get_paths() -> Result<(), std::io::Error> {
    let paths = get_paths(".")?;
    assert_eq!(format!("{:?}", vec!["./.git", "./src", "./target", "./tests"]), format!("{:?}", paths.0));

    Ok(())
}
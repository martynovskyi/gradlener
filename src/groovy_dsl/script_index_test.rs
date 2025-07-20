use super::*;

#[cfg(test)]


#[test]
fn single_block_index() {
    let script = "application { }";
    let result = script_index::parse(script);

    assert!(result.contains_key("application"));
    assert_eq!(result.len(), 1);
}

#[test]
fn single_block_with_inner_index() {
    let script = "application{ 
block {} 
}";
    let result = script_index::parse(script);

    assert!(result.contains_key("application"));
    assert_eq!(result.len(), 1);
    let (start, end) = result.get("application").unwrap();
    assert_eq!(*start, 0usize);
    assert_eq!(*end, 24usize);
}

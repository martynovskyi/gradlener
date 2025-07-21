use super::*;

#[cfg(test)]


#[test]
fn single_block_index() {
    let script = "application { }";
    let result = script_index::parse(script);

    assert!(result.contains_key("application"));
    assert_eq!(result.len(), 1); 
    let (start, end) = result.get("application").unwrap();
    assert_eq!(&script[*start..*end], script);
}

#[test]
fn several_empty_blocks_index() {
    let script = "application { } block{} end { } ";
    let result = script_index::parse(script);

    assert!(result.contains_key("application"));
    assert!(result.contains_key("block"));
    assert!(result.contains_key("end"));
    assert_eq!(result.len(), 3);
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
    assert_eq!(&script[*start..*end], script);
}


#[test]
fn single_block_with_comment_index() {
    let script = 
    "//comment above block
application { }";
    let result = script_index::parse(script);

    assert!(result.contains_key("application"));
    assert_eq!(result.len(), 1);
    let (start, end) = result.get("application").unwrap();
    assert_eq!(&script[*start..*end], "application { }");
}

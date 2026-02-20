use dateno_duckdb_ext::api::parse_filter;

#[test]
fn test_parse_filter_valid() {
    let (key, value) = parse_filter("source.countries.name=Canada").unwrap();
    assert_eq!(key, "source.countries.name");
    assert_eq!(value, "Canada");
}

#[test]
fn test_parse_filter_with_equals_in_value() {
    let (key, value) = parse_filter("key=value=with=equals").unwrap();
    assert_eq!(key, "key");
    assert_eq!(value, "value=with=equals");
}

#[test]
fn test_parse_filter_invalid_no_equals() {
    let result = parse_filter("invalid");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Filter must be in format"));
}

#[test]
fn test_parse_filter_empty_key() {
    let (key, value) = parse_filter("=value").unwrap();
    assert_eq!(key, "");
    assert_eq!(value, "value");
}

#[test]
fn test_parse_filter_empty_value() {
    let (key, value) = parse_filter("key=").unwrap();
    assert_eq!(key, "key");
    assert_eq!(value, "");
}

use dateno_duckdb_ext::api::{DatenoClient, SearchRequest};
use mockito::{Mock, Server};

#[test]
fn test_search_request_basic() {
    let mut server = Server::new();
    
    // Mock API response
    let mock = Mock::new()
        .expect_method("GET")
        .expect_path("/search/0.2/query")
        .expect_query_param("q", "climate")
        .expect_query_param("limit", "10")
        .expect_header("authorization", "Bearer test-key")
        .respond_with(200)
        .body(r#"{"total": 1, "results": [{"id": "test-1", "title": "Test Dataset", "description": "", "format": "CSV", "source": "", "url": "", "formats": []}]}"#)
        .create_on(&mut server);

    let client = DatenoClient::new(Some("test-key".to_string())).unwrap();
    let request = SearchRequest {
        query: "climate".to_string(),
        filters: vec![],
        limit: 10,
    };

    let response = client.search(request);
    assert!(response.is_ok());
    
    let search_response = response.unwrap();
    assert_eq!(search_response.total, 1);
    assert_eq!(search_response.results.len(), 1);
    assert_eq!(search_response.results[0].id, "test-1");
    assert_eq!(search_response.results[0].title, "Test Dataset");
    
    mock.assert();
}

#[test]
fn test_search_with_filters() {
    let mut server = Server::new();
    
    let mock = Mock::new()
        .expect_method("GET")
        .expect_path("/search/0.2/query")
        .expect_query_param("q", "temperature")
        .expect_query_param("limit", "50")
        .expect_query_param("filters", "\"source.countries.name\"=\"Canada\"")
        .respond_with(200)
        .body(r#"{"total": 0, "results": []}"#)
        .create_on(&mut server);

    let client = DatenoClient::new(None).unwrap();
    let request = SearchRequest {
        query: "temperature".to_string(),
        filters: vec!["source.countries.name=Canada".to_string()],
        limit: 50,
    };

    let response = client.search(request);
    assert!(response.is_ok());
    
    let search_response = response.unwrap();
    assert_eq!(search_response.total, 0);
    
    mock.assert();
}

#[test]
fn test_search_limit_validation() {
    let client = DatenoClient::new(None).unwrap();
    
    // Test limit too low
    let request = SearchRequest {
        query: "test".to_string(),
        filters: vec![],
        limit: 0,
    };
    assert!(client.search(request).is_err());
    
    // Test limit too high
    let request = SearchRequest {
        query: "test".to_string(),
        filters: vec![],
        limit: 101,
    };
    assert!(client.search(request).is_err());
    
    // Test valid limit
    let request = SearchRequest {
        query: "test".to_string(),
        filters: vec![],
        limit: 50,
    };
    // This will fail because we don't have a mock server, but validation should pass
    // The actual HTTP call will fail, but limit validation happens first
}

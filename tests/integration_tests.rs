#[cfg(feature = "integration-tests")]
mod integration {
    use dateno_duckdb_ext::api::{DatenoClient, SearchRequest};
    use std::env;

    #[test]
    #[ignore = "requires DATENO_API_KEY environment variable"]
    fn test_live_search() {
        let api_key = env::var("DATENO_API_KEY")
            .expect("DATENO_API_KEY environment variable must be set");
        
        let client = DatenoClient::new(Some(api_key)).unwrap();
        let request = SearchRequest {
            query: "climate".to_string(),
            filters: vec![],
            limit: 10,
        };

        let response = client.search(request);
        assert!(response.is_ok(), "Search should succeed");
        
        let search_response = response.unwrap();
        assert!(search_response.total >= 0, "Total should be non-negative");
        assert_eq!(search_response.results.len(), search_response.total.min(10));
    }

    #[test]
    #[ignore = "requires DATENO_API_KEY environment variable"]
    fn test_live_search_with_filters() {
        let api_key = env::var("DATENO_API_KEY")
            .expect("DATENO_API_KEY environment variable must be set");
        
        let client = DatenoClient::new(Some(api_key)).unwrap();
        let request = SearchRequest {
            query: "temperature".to_string(),
            filters: vec!["format=CSV".to_string()],
            limit: 5,
        };

        let response = client.search(request);
        assert!(response.is_ok(), "Search with filters should succeed");
    }
}

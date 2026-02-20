use crate::api::{DatenoClient, Dataset, SearchRequest};
use crate::error::DatenoError;
use duckdb::{
    types::{LogicalType, Value},
    vtab::{BindInfo, DataChunkHandle, InitInfo, TableFunctionInfo, VTab},
};
use std::error::Error;
use std::sync::Arc;

/// Bind data for the table function (read-only, shared across threads)
#[derive(Clone)]
pub struct DatenoSearchBindData {
    pub query: String,
    pub filters: Vec<String>,
    pub limit: i64,
}

/// Init data for the table function (mutable state)
pub struct DatenoSearchInitData {
    pub client: Arc<DatenoClient>,
    pub results: Vec<Dataset>,
    pub current_index: usize,
}

/// Table function implementation
pub struct DatenoSearchFunction;

impl VTab for DatenoSearchFunction {
    type InitData = DatenoSearchInitData;
    type BindData = DatenoSearchBindData;

    fn parameters() -> Option<Vec<duckdb::types::LogicalTypeHandle>> {
        Some(vec![
            LogicalType::Varchar.into(),                    // query (required)
            LogicalType::List(Box::new(LogicalType::Varchar)).into(), // filters (optional)
            LogicalType::Bigint.into(),                    // limit (optional)
        ])
    }

    fn bind(bind: &BindInfo) -> Result<Self::BindData, Box<dyn Error>> {
        let param_count = bind.get_parameter_count();

        // Get query parameter (required, first parameter)
        if param_count == 0 {
            bind.set_error("query parameter is required");
            return Err("query parameter is required".into());
        }

        let query_value = bind.get_parameter(0);
        let query = match query_value {
            Value::Text(s) => s,
            _ => {
                bind.set_error("query parameter must be a string");
                return Err("query parameter must be a string".into());
            }
        };

        // Get filters parameter (optional, second parameter)
        let mut filters = Vec::new();
        if param_count >= 2 {
            let filters_value = bind.get_parameter(1);
            if let Value::List(list) = filters_value {
                for item in list {
                    if let Value::Text(s) = item {
                        filters.push(s);
                    }
                }
            }
        }

        // Get limit parameter (optional, third parameter)
        let mut limit = 10i64; // default
        if param_count >= 3 {
            let limit_value = bind.get_parameter(2);
            if let Value::BigInt(n) = limit_value {
                limit = n;
            }
        }

        // Validate limit
        if limit < 1 || limit > 100 {
            bind.set_error("limit must be between 1 and 100");
            return Err("limit must be between 1 and 100".into());
        }

        Ok(DatenoSearchBindData {
            query,
            filters,
            limit,
        })
    }

    fn init(init: &InitInfo) -> Result<Self::InitData, Box<dyn Error>> {
        // Get bind data (read-only) - this is set during bind phase
        let bind_data_ptr = init.get_bind_data::<DatenoSearchBindData>();
        if bind_data_ptr.is_null() {
            init.set_error("Failed to get bind data");
            return Err("Failed to get bind data".into());
        }

        // SAFETY: bind_data is set during bind phase and remains valid until function completes
        let bind_data = unsafe { &*bind_data_ptr };

        // Get API key from environment
        let api_key = std::env::var("DATENO_API_KEY").ok();
        let client = Arc::new(
            DatenoClient::new(api_key)
                .map_err(|e| format!("Failed to create client: {}", e))?,
        );

        // Execute search
        let request = SearchRequest {
            query: bind_data.query.clone(),
            filters: bind_data.filters.clone(),
            limit: bind_data.limit,
        };

        let response = client
            .search(request)
            .map_err(|e| format!("Search failed: {}", e))?;

        Ok(DatenoSearchInitData {
            client,
            results: response.results,
            current_index: 0,
        })
    }

    fn func(
        func: &TableFunctionInfo<Self>,
        output: &mut DataChunkHandle,
    ) -> Result<(), Box<dyn Error>> {
        use duckdb::DataChunk;
        
        // Get init data
        let init_data = func
            .get_init_data::<Self::InitData>()
            .ok_or("Failed to get init data")?;

        let results = &init_data.results;

        if results.is_empty() {
            // No rows to output
            return Ok(());
        }

        // Create a DataChunk with all results
        let mut chunk = DataChunk::new(9); // 9 columns

        for dataset in results {
            let mut row = Vec::new();

            // id
            row.push(Value::Text(dataset.id.clone()));

            // title
            row.push(Value::Text(dataset.title.clone()));

            // description
            row.push(Value::Text(dataset.description.clone()));

            // format
            row.push(Value::Text(dataset.format.clone()));

            // source
            row.push(Value::Text(dataset.source.clone()));

            // url
            row.push(Value::Text(dataset.url.clone()));

            // created_at
            row.push(
                dataset
                    .created_at
                    .map(|dt| {
                        Value::Timestamp(duckdb::types::Timestamp::from_micros(
                            dt.timestamp_micros(),
                        ))
                    })
                    .unwrap_or(Value::Null),
            );

            // updated_at
            row.push(
                dataset
                    .updated_at
                    .map(|dt| {
                        Value::Timestamp(duckdb::types::Timestamp::from_micros(
                            dt.timestamp_micros(),
                        ))
                    })
                    .unwrap_or(Value::Null),
            );

            // formats
            let formats: Vec<Value> = dataset
                .formats
                .iter()
                .map(|f| Value::Text(f.clone()))
                .collect();
            row.push(Value::List(formats));

            chunk.append_row(&row)?;
        }

        // Write chunk to output
        // Note: This API may need adjustment based on actual duckdb-rs implementation
        output.append_data_chunk(&chunk)?;

        Ok(())
    }
}

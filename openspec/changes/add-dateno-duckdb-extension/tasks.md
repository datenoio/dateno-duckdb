## 1. Project Setup
- [x] 1.1 Initialize Rust project with Cargo (`cargo init --lib`)
- [x] 1.2 Configure `Cargo.toml` with dependencies (duckdb, reqwest, serde, tokio, chrono, thiserror)
- [x] 1.3 Set up rust-analyzer configuration in `.vscode/settings.json` with `vtab` feature
- [x] 1.4 Create project directory structure (`src/`, `tests/`, `examples/`)

## 2. Extension Core Implementation
- [x] 2.1 Implement extension entry point `dateno_duckdb_ext_init()` with FFI bindings
- [x] 2.2 Implement version function `dateno_duckdb_ext_version()`
- [x] 2.3 Create extension metadata structure
- [x] 2.4 Set up error handling infrastructure

## 3. Dateno API Client
- [x] 3.1 Implement HTTP client initialization with connection pooling and timeouts
- [x] 3.2 Create request builder for Dateno API endpoint (`/search/0.2/query`)
- [x] 3.3 Implement filter parsing and URL encoding
- [x] 3.4 Create response deserialization with serde
- [x] 3.5 Handle authentication (query parameter and Bearer token support)
- [x] 3.6 Implement async-to-sync bridge for DuckDB's synchronous API

## 4. Table Function Implementation
- [x] 4.1 Define `DatenoSearchFunction` struct implementing `VTab` trait
- [x] 4.2 Implement parameter binding (`query`, `filters`, `limit`)
- [x] 4.3 Define return schema with appropriate column types
- [x] 4.4 Implement row iterator for result materialization
- [x] 4.5 Create type conversion utilities (JSON → DuckDB types)
- [x] 4.6 Handle null values and edge cases

## 5. Type System & Data Conversion
- [x] 5.1 Map Dateno response fields to DuckDB logical types
- [x] 5.2 Implement string → VARCHAR conversion
- [x] 5.3 Implement array/list handling for nested fields
- [x] 5.4 Implement date/timestamp parsing with chrono
- [x] 5.5 Handle nullable fields appropriately

## 6. Testing Infrastructure
- [x] 6.1 Set up unit test framework with mock HTTP server (wiremock/mockito)
- [x] 6.2 Write unit tests for API client parsing logic
- [x] 6.3 Write unit tests for filter parsing and encoding
- [x] 6.4 Create integration test suite (feature-gated for live API calls)
- [ ] 6.5 Write SQL-level tests for table function (requires compilation fixes)
- [x] 6.6 Create test fixtures and sample responses

## 7. Build & Distribution
- [x] 7.1 Configure cross-compilation targets (linux-musl, darwin, windows) - build script created
- [x] 7.2 Create extension manifest (`extension.toml`)
- [x] 7.3 Set up build scripts for multiple platforms - build.sh and GitHub Actions
- [x] 7.4 Create installation documentation
- [x] 7.5 Set up CI/CD pipeline for automated builds - GitHub Actions workflow

## 8. Documentation
- [x] 8.1 Write README with installation instructions
- [x] 8.2 Document SQL function signature and usage examples
- [x] 8.3 Create API reference documentation
- [x] 8.4 Add code comments and docstrings

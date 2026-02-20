# Change: Add DuckDB Extension for Dateno Dataset Search

## Why

Enable SQL-native access to Dateno's dataset catalog directly within DuckDB, eliminating context switching between data discovery and analysis tools. This extension bridges DuckDB's analytical capabilities with external dataset catalogs, allowing users to discover, evaluate, and query dataset metadata without leaving their SQL workflow.

The extension transforms procedural, tool-chaining approaches into declarative, integrated data discovery where the boundary between "finding data" and "using data" becomes permeable.

## What Changes

- **NEW**: DuckDB extension written in Rust using `duckdb-rs` crate
- **NEW**: Table function `dateno_search()` that exposes Dateno API as a SQL queryable table
- **NEW**: HTTP client integration with Dateno API (`api.dateno.io`)
- **NEW**: Type mapping system for converting JSON responses to DuckDB types
- **NEW**: Parameter binding and validation for search queries and filters
- **NEW**: Testing infrastructure (unit, integration, SQL-level tests)
- **NEW**: Cross-compilation support for multiple platforms
- **NEW**: Extension packaging and distribution mechanism

## Impact

- Affected specs: 
  - `extension-core` (new capability)
  - `dateno-api` (new capability)
  - `table-function` (new capability)
  - `testing` (new capability)
  - `build-distribution` (new capability)
- Affected code: 
  - New Rust project structure (`src/`, `Cargo.toml`)
  - Extension entry points (`dateno_duckdb_ext_init`, `dateno_duckdb_ext_version`)
  - HTTP client module
  - Table function implementation
  - Type conversion utilities
  - Test suites
  - Build configuration and CI/CD

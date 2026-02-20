# Project Context

## Purpose
Build a DuckDB extension in Rust that enables SQL-native access to Dateno's dataset catalog. The extension bridges DuckDB's analytical capabilities with external dataset catalogs, allowing users to discover, evaluate, and query dataset metadata without leaving their SQL workflow.

## Tech Stack
- **Rust** - Primary implementation language
- **duckdb-rs** - DuckDB extension framework (with `vtab` feature)
- **reqwest** - HTTP client for API calls
- **serde/serde_json** - JSON serialization/deserialization
- **tokio** - Async runtime for bridging async HTTP with sync DuckDB API
- **chrono** - Date/time parsing
- **thiserror** - Error handling

## Project Conventions

### Code Style
- Follow Rust standard formatting (`rustfmt`)
- Use `clippy` for linting
- Snake_case for function names (matching DuckDB conventions)
- Clear, descriptive names for types and functions
- Comprehensive error handling with descriptive messages

### Architecture Patterns
- Extension follows DuckDB's table function pattern using `VTab` trait
- Separation of concerns: API client, type conversion, table function implementation
- Async-to-sync bridging using Tokio runtime
- Stateless design (no caching, each query is independent)

### Testing Strategy
- Unit tests with HTTP mocking (wiremock/mockito)
- Integration tests feature-gated for live API calls
- SQL-level tests for end-to-end verification
- Test fixtures for consistent test data

### Git Workflow
- Main branch for stable releases
- Feature branches for development
- OpenSpec-driven development (proposals → implementation → archive)

## Domain Context
- **DuckDB**: In-process analytical database with extension API
- **Dateno**: Dataset catalog aggregator indexing datasets from government portals, research institutions, and international organizations
- **Table Functions**: DuckDB mechanism for exposing external data sources as queryable tables
- **Extension API**: C-compatible FFI interface for registering extensions

## Important Constraints
- DuckDB extension API is synchronous, requiring async-to-sync bridge
- Must support cross-platform deployment (Linux, macOS, Windows)
- Network operations must handle timeouts and failures gracefully
- Type conversion must be robust for various JSON response formats
- Extension must be stateless (no caching or local storage)

## External Dependencies
- **Dateno API**: `https://api.dateno.io/search/0.2/query`
  - Requires API key (query parameter or Bearer token)
  - Supports full-text search (`q` parameter)
  - Supports metadata filters (`filters` parameter)
  - Returns JSON with `total` and `results` fields
- **DuckDB**: Version >=0.10.0 required
- **Rust toolchain**: Latest stable Rust with cross-compilation support

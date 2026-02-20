# Design: DuckDB Extension for Dateno Dataset Search

## Context

This extension enables SQL-native access to Dateno's dataset catalog within DuckDB. The core challenge is bridging Rust's async HTTP client ecosystem with DuckDB's synchronous extension API while maintaining clean SQL ergonomics and type safety.

**Key Constraints:**
- DuckDB extensions use synchronous C API
- Rust HTTP clients (reqwest) are async-first
- Must support multiple platforms (Linux, macOS, Windows)
- Must handle network failures gracefully
- Type conversion from JSON to DuckDB types must be robust

**Stakeholders:**
- Data analysts using DuckDB for analytics
- Data engineers building data pipelines
- Developers integrating dataset discovery into workflows

## Goals / Non-Goals

### Goals
- Provide SQL-native interface to Dateno search API
- Support full-text search and metadata filtering
- Handle network errors gracefully with clear error messages
- Support cross-platform deployment (Linux, macOS, Windows)
- Maintain type safety throughout the Rust implementation
- Enable composability with other DuckDB operations (JOINs, CTEs, etc.)

### Non-Goals
- Direct dataset ingestion (only metadata search)
- Caching or local dataset storage
- Authentication token management (users provide API keys)
- Dataset format conversion or parsing

## Decisions

### Decision: Use `duckdb-rs` with `vtab` feature
**Rationale:** The `duckdb-rs` crate provides idiomatic Rust bindings for DuckDB's C extension API, including table function support via the `vtab` feature. This avoids manual FFI work and provides type safety.

**Alternatives considered:**
- Direct C FFI bindings: Too low-level, error-prone
- Other DuckDB Rust crates: Less mature, fewer features

### Decision: Use `reqwest` for HTTP client
**Rationale:** `reqwest` is the de facto standard HTTP client for Rust, offering connection pooling, middleware, and robust JSON handling. It's well-maintained and widely used.

**Alternatives considered:**
- `ureq`: Synchronous but less feature-rich
- `hyper`: Lower-level, requires more boilerplate

### Decision: Bridge async HTTP with sync DuckDB API using Tokio runtime
**Rationale:** DuckDB's extension API is synchronous, but Rust HTTP clients are async. We'll use Tokio's `Handle::block_on()` or spawn a runtime thread to bridge this gap.

**Alternatives considered:**
- Synchronous HTTP client (`ureq`): Simpler but less robust
- Background thread pool: More complex, potential resource issues

### Decision: Use serde for JSON parsing
**Rationale:** Serde provides compile-time verified parsing with derive macros, reducing runtime errors and improving maintainability.

**Alternatives considered:**
- Manual JSON parsing: Error-prone, harder to maintain
- Other serialization libraries: Less ecosystem support

### Decision: Support both query parameter and Bearer token authentication
**Rationale:** Query parameter auth is simpler for development/testing, while Bearer tokens are more secure for production. Supporting both provides flexibility.

**Alternatives considered:**
- Bearer token only: Less flexible for development
- Query parameter only: Less secure for production

### Decision: Return schema includes common dataset metadata fields
**Rationale:** Users need key metadata fields (title, description, format, source, etc.) to evaluate datasets. Including these in the return schema enables filtering and analysis without additional API calls.

**Alternatives considered:**
- Minimal schema (ID only): Requires follow-up API calls, less useful
- Full nested schema: Too complex, harder to query

## Risks / Trade-offs

### Risk: Async-to-sync bridging may cause deadlocks
**Mitigation:** Use Tokio's `Handle::block_on()` carefully, ensure no nested blocking calls, test thoroughly with concurrent queries.

### Risk: Network timeouts may block DuckDB queries
**Mitigation:** Configure reasonable timeouts (10s connect, 60s request), provide clear error messages, consider async cancellation if DuckDB supports it.

### Risk: Type conversion errors from malformed API responses
**Mitigation:** Use serde's `#[serde(default)]` and `Option<T>` for optional fields, validate required fields, provide descriptive error messages.

### Risk: Cross-compilation complexity
**Mitigation:** Use `cross` tool for Linux targets, test on native platforms, document platform-specific requirements.

### Trade-off: Simplicity vs. Feature Completeness
**Decision:** Start with core search functionality, add advanced features (pagination, sorting) in future iterations based on user feedback.

## Migration Plan

### Phase 1: Core Extension (Weeks 1-2)
- Set up project structure
- Implement basic extension entry points
- Create minimal table function skeleton

### Phase 2: API Integration (Weeks 2-3)
- Implement HTTP client
- Add request/response parsing
- Handle authentication

### Phase 3: Table Function (Weeks 3-4)
- Implement parameter binding
- Create result materialization
- Add type conversion

### Phase 4: Testing & Polish (Weeks 4-5)
- Write comprehensive tests
- Fix edge cases
- Improve error messages

### Phase 5: Distribution (Week 5-6)
- Set up cross-compilation
- Create packaging
- Write documentation

**Rollback:** If issues arise, users can simply not load the extension. No data migration needed.

## Open Questions

- Should we support pagination for large result sets? (Deferred to future iteration)
- How should we handle rate limiting from Dateno API? (Return clear error, let users handle retries)
- Should we cache API responses? (No, keep stateless for simplicity)
- What's the maximum result limit we should support? (Start with API's limit of 100, make configurable later)

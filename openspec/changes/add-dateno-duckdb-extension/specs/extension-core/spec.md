## ADDED Requirements

### Requirement: Extension Entry Point
The extension SHALL provide a C-compatible entry point function `dateno_duckdb_ext_init()` that registers the table function with DuckDB.

#### Scenario: Extension loads successfully
- **WHEN** DuckDB calls `dateno_duckdb_ext_init()` during `LOAD` or `INSTALL`
- **THEN** the function registers `dateno_search` table function and returns success status

#### Scenario: Extension version reporting
- **WHEN** DuckDB queries extension version via `dateno_duckdb_ext_version()`
- **THEN** the function returns a null-terminated version string (e.g., "0.1.0")

### Requirement: Extension Metadata
The extension SHALL provide metadata including name, version, and DuckDB version compatibility.

#### Scenario: Extension metadata is accessible
- **WHEN** extension is loaded
- **THEN** DuckDB can query extension name, version, and required DuckDB version

### Requirement: Error Handling
The extension SHALL handle initialization errors gracefully and report them to DuckDB.

#### Scenario: Initialization failure reporting
- **WHEN** extension initialization fails (e.g., missing dependencies)
- **THEN** error message is written to DuckDB's error buffer and initialization returns failure status

## ADDED Requirements

### Requirement: Table Function Registration
The extension SHALL register a table function named `dateno_search` that can be called from SQL.

#### Scenario: Function is callable from SQL
- **WHEN** extension is loaded and user executes `SELECT * FROM dateno_search('query', ARRAY[], 10)`
- **THEN** function executes and returns results

#### Scenario: Function name follows DuckDB conventions
- **WHEN** function is registered
- **THEN** function name is `dateno_search` (snake_case, descriptive)

### Requirement: Parameter Binding
The extension SHALL accept and validate function parameters: query string, filters array, and optional limit.

#### Scenario: Required query parameter
- **WHEN** user calls `dateno_search()` without query parameter
- **THEN** DuckDB returns parameter binding error

#### Scenario: Query parameter binding
- **WHEN** user provides query "climate data"
- **THEN** parameter is bound as VARCHAR type and passed to API client

#### Scenario: Filters array parameter
- **WHEN** user provides `ARRAY['source.countries.name=Canada']`
- **THEN** parameter is bound as LIST(VARCHAR) and parsed into filter strings

#### Scenario: Optional limit parameter
- **WHEN** user provides limit 50
- **THEN** parameter is bound as BIGINT and used in API request

#### Scenario: Default limit when omitted
- **WHEN** user omits limit parameter
- **THEN** default limit of 10 is used

#### Scenario: Limit validation
- **WHEN** user provides limit outside valid range (1-100)
- **THEN** error is returned indicating valid range

### Requirement: Return Schema Definition
The extension SHALL define a return schema with columns for dataset metadata fields.

#### Scenario: Schema includes core fields
- **WHEN** function is described
- **THEN** schema includes columns: `id` (VARCHAR), `title` (VARCHAR), `description` (VARCHAR), `format` (VARCHAR), `source` (VARCHAR), `url` (VARCHAR), `created_at` (TIMESTAMP), `updated_at` (TIMESTAMP)

#### Scenario: Schema column types are appropriate
- **WHEN** schema is inspected
- **THEN** each column has appropriate DuckDB logical type matching the data

### Requirement: Result Materialization
The extension SHALL materialize API response results into DuckDB DataChunks.

#### Scenario: Single result row
- **WHEN** API returns one dataset result
- **THEN** one row is produced with columns matching return schema

#### Scenario: Multiple result rows
- **WHEN** API returns multiple dataset results
- **THEN** multiple rows are produced, one per dataset

#### Scenario: Empty result set
- **WHEN** API returns no results
- **THEN** zero rows are produced without error

#### Scenario: Chunked production
- **WHEN** large result set is materialized
- **THEN** rows are produced in chunks compatible with DuckDB's execution model

### Requirement: Type Conversion
The extension SHALL convert JSON response fields to appropriate DuckDB types.

#### Scenario: String field conversion
- **WHEN** API returns string field "Climate Data"
- **THEN** field is converted to VARCHAR type

#### Scenario: Null field handling
- **WHEN** API returns null for optional field
- **THEN** DuckDB NULL value is produced for that column

#### Scenario: Timestamp conversion
- **WHEN** API returns ISO 8601 timestamp string
- **THEN** field is parsed and converted to TIMESTAMP type

#### Scenario: Array field handling
- **WHEN** API returns array field (e.g., formats)
- **THEN** field is converted to LIST(VARCHAR) type

### Requirement: SQL Composability
The extension SHALL enable composition with other DuckDB SQL operations.

#### Scenario: Function in CTE
- **WHEN** user uses function in WITH clause
- **THEN** results can be referenced in subsequent query parts

#### Scenario: Function with WHERE clause
- **WHEN** user applies WHERE clause to function results
- **THEN** filtering is applied correctly

#### Scenario: Function with JOIN
- **WHEN** user joins function results with another table
- **THEN** join executes successfully

#### Scenario: Function with aggregation
- **WHEN** user applies GROUP BY or aggregation functions
- **THEN** aggregation executes correctly on result set

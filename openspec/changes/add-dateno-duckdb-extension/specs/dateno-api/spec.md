## ADDED Requirements

### Requirement: HTTP Client Configuration
The extension SHALL initialize an HTTP client with appropriate timeouts, connection pooling, and user agent.

#### Scenario: Client initialization with defaults
- **WHEN** extension initializes HTTP client
- **THEN** client has connection timeout of 10 seconds, request timeout of 60 seconds, and user agent "dateno-duckdb-ext/{version}"

#### Scenario: Connection pooling
- **WHEN** multiple requests are made
- **THEN** client reuses connections with maximum 10 idle connections per host

### Requirement: API Request Construction
The extension SHALL construct valid HTTP requests to Dateno API endpoint with query parameters.

#### Scenario: Basic search request
- **WHEN** user provides query "climate data"
- **THEN** request is sent to `https://api.dateno.io/search/0.2/query?q=climate+data`

#### Scenario: Request with limit parameter
- **WHEN** user provides query "temperature" and limit 50
- **THEN** request includes `limit=50` query parameter

#### Scenario: Request with filters
- **WHEN** user provides filters `["source.countries.name=Canada", "source.langs.id=FR"]`
- **THEN** request includes properly encoded `filters` parameters: `filters="source.countries.name"="Canada"` and `filters="source.langs.id"="FR"`

### Requirement: Filter Parsing and Encoding
The extension SHALL parse filter strings and encode them according to Dateno API format.

#### Scenario: Filter encoding format
- **WHEN** filter string is `source.countries.name=Canada`
- **THEN** it is encoded as `filters="source.countries.name"="Canada"` with proper URL encoding

#### Scenario: Multiple filters
- **WHEN** multiple filters are provided
- **THEN** each filter is added as a separate `filters` query parameter (AND logic)

### Requirement: Authentication Support
The extension SHALL support both query parameter and Bearer token authentication methods.

#### Scenario: Query parameter authentication
- **WHEN** API key is provided as query parameter
- **THEN** request includes `apikey={key}` query parameter

#### Scenario: Bearer token authentication
- **WHEN** API key is provided as Bearer token
- **THEN** request includes `Authorization: Bearer {token}` header

### Requirement: Response Parsing
The extension SHALL parse JSON responses from Dateno API into structured Rust types.

#### Scenario: Successful response parsing
- **WHEN** API returns valid JSON with `total` and `results` fields
- **THEN** response is deserialized into `SearchResponse` struct with typed fields

#### Scenario: Missing optional fields
- **WHEN** API response omits optional fields
- **THEN** those fields are set to default values (empty strings, empty arrays, null) without error

#### Scenario: Malformed JSON handling
- **WHEN** API returns invalid JSON
- **THEN** error is returned with descriptive message indicating parsing failure

### Requirement: Async-to-Sync Bridge
The extension SHALL bridge async HTTP client calls to DuckDB's synchronous extension API.

#### Scenario: Blocking HTTP call
- **WHEN** table function executes HTTP request
- **THEN** async HTTP call is executed synchronously using Tokio runtime without blocking DuckDB thread

#### Scenario: Concurrent requests
- **WHEN** multiple queries execute simultaneously
- **THEN** each query uses its own async context without deadlocks

### Requirement: Error Handling
The extension SHALL handle network errors, timeouts, and API errors gracefully.

#### Scenario: Network timeout
- **WHEN** API request exceeds timeout duration
- **THEN** error is returned with message indicating timeout

#### Scenario: HTTP error status
- **WHEN** API returns 4xx or 5xx status code
- **THEN** error is returned with status code and error message from response body

#### Scenario: Connection failure
- **WHEN** network connection cannot be established
- **THEN** error is returned with message indicating connection failure

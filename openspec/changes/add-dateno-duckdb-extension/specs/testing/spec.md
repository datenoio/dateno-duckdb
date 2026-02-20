## ADDED Requirements

### Requirement: Unit Test Framework
The extension SHALL include unit tests for core functionality using a mocking framework.

#### Scenario: HTTP client mocking
- **WHEN** unit test runs
- **THEN** HTTP requests are intercepted by mock server (wiremock/mockito) without external API calls

#### Scenario: Response parsing tests
- **WHEN** test provides mock JSON response
- **THEN** parsing logic is tested without network dependency

#### Scenario: Filter encoding tests
- **WHEN** test provides filter strings
- **THEN** encoding logic is verified with expected URL format

### Requirement: Integration Test Suite
The extension SHALL include integration tests that exercise live API calls (feature-gated).

#### Scenario: Live API test execution
- **WHEN** integration test feature is enabled and API key is provided
- **THEN** tests make actual HTTP requests to Dateno API

#### Scenario: Integration test isolation
- **WHEN** integration tests run
- **THEN** tests are marked with `#[ignore]` by default and require explicit feature flag

#### Scenario: API key management
- **WHEN** integration test requires API key
- **THEN** API key is read from environment variable `DATENO_API_KEY`

### Requirement: SQL-Level Testing
The extension SHALL include tests that verify SQL function behavior end-to-end.

#### Scenario: Function loading test
- **WHEN** test loads extension
- **THEN** `dateno_search` function is available for use

#### Scenario: Basic query test
- **WHEN** test executes `SELECT * FROM dateno_search('test', ARRAY[]) LIMIT 1`
- **THEN** function returns at least zero rows without error

#### Scenario: Schema verification test
- **WHEN** test describes function schema
- **THEN** schema matches expected column names and types

#### Scenario: Parameter validation test
- **WHEN** test provides invalid parameters
- **THEN** appropriate error is returned

#### Scenario: Composability test
- **WHEN** test uses function in CTE or JOIN
- **THEN** query executes successfully

### Requirement: Test Fixtures
The extension SHALL provide test fixtures and sample responses for consistent testing.

#### Scenario: Sample API responses
- **WHEN** tests need mock data
- **THEN** sample JSON responses are available in test fixtures directory

#### Scenario: Edge case fixtures
- **WHEN** tests need to verify edge cases
- **THEN** fixtures include empty results, null fields, malformed data examples

### Requirement: Test Coverage
The extension SHALL maintain test coverage for critical paths including error handling.

#### Scenario: Error path testing
- **WHEN** tests run
- **THEN** network errors, timeouts, and API errors are tested

#### Scenario: Type conversion testing
- **WHEN** tests run
- **THEN** all type conversion paths are covered with various input formats

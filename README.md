# Dateno DuckDB Extension

A DuckDB extension written in Rust that enables SQL-native access to Dateno's dataset catalog, allowing you to discover and query dataset metadata directly within DuckDB.

## Features

- **SQL-native interface**: Query Dateno's dataset catalog using standard SQL
- **Full-text search**: Search datasets by keywords
- **Metadata filtering**: Filter datasets by source, format, country, language, and more
- **Type-safe**: Built with Rust for reliability and performance
- **Cross-platform**: Supports Linux, macOS, and Windows

## Installation

### Prerequisites

- DuckDB >= 0.10.0
- Rust toolchain (for building from source)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/commondataio/dateno-duckdb.git
cd dateno-duckdb

# Build the extension
cargo build --release

# The extension library will be in target/release/
```

### Installing the Extension

```sql
-- Load the extension
LOAD 'path/to/libdateno_duckdb_ext.so';  -- Linux
LOAD 'path/to/libdateno_duckdb_ext.dylib';  -- macOS
LOAD 'path/to/dateno_duckdb_ext.dll';  -- Windows
```

Or use DuckDB's `INSTALL` command:

```sql
INSTALL 'dateno' FROM 'https://your-repo.com/extensions';
LOAD 'dateno';
```

## Configuration

Set your Dateno API key as an environment variable:

```bash
export DATENO_API_KEY="your-api-key-here"
```

The extension will automatically use this key for authentication when making API requests.

## Usage

### Basic Search

```sql
SELECT * FROM dateno_search('climate data', ARRAY[], 10);
```

### Search with Filters

```sql
SELECT * FROM dateno_search(
    'temperature',
    ARRAY['source.countries.name=Canada', 'format=CSV'],
    50
);
```

### Using in CTEs and JOINs

```sql
WITH climate_datasets AS (
    SELECT * FROM dateno_search('climate', ARRAY[], 100)
    WHERE format = 'CSV'
)
SELECT format, COUNT(*) as count
FROM climate_datasets
GROUP BY format;
```

## Function Signature

```sql
dateno_search(
    query VARCHAR,           -- Search query string (required)
    filters LIST[VARCHAR],   -- Array of filter strings (optional)
    limit BIGINT            -- Maximum number of results (optional, default: 10, range: 1-100)
)
```

### Parameters

- **query**: Full-text search query string
- **filters**: Array of filter strings in format `"key=value"`, e.g., `["source.countries.name=Canada", "format=CSV"]`
- **limit**: Maximum number of results to return (1-100, default: 10)

### Return Schema

The function returns a table with the following columns:

| Column      | Type      | Description                    |
|-------------|-----------|--------------------------------|
| id          | VARCHAR   | Dataset identifier             |
| title       | VARCHAR   | Dataset title                  |
| description | VARCHAR   | Dataset description            |
| format      | VARCHAR   | Primary format                 |
| source      | VARCHAR   | Data source                   |
| url         | VARCHAR   | Dataset URL                    |
| created_at  | TIMESTAMP | Creation timestamp             |
| updated_at  | TIMESTAMP | Last update timestamp          |
| formats     | LIST[VARCHAR] | Available formats          |

## Filter Examples

Filters use the format `"field.path=value"`:

- `source.countries.name=Canada` - Datasets from Canada
- `source.langs.id=FR` - French language datasets
- `format=CSV` - CSV format datasets
- `source.organizations.name=NASA` - Datasets from NASA

Multiple filters are combined with AND logic.

## Examples

### Find Climate Datasets

```sql
SELECT title, format, source, url
FROM dateno_search('climate change', ARRAY[], 20)
WHERE format IN ('CSV', 'Parquet')
ORDER BY title;
```

### Search Canadian Datasets

```sql
SELECT * FROM dateno_search(
    'economic data',
    ARRAY['source.countries.name=Canada'],
    50
);
```

### Count Datasets by Format

```sql
SELECT format, COUNT(*) as count
FROM dateno_search('data', ARRAY[], 100)
GROUP BY format
ORDER BY count DESC;
```

## Error Handling

The extension provides clear error messages for common issues:

- **Missing query parameter**: Returns error if query is not provided
- **Invalid limit**: Returns error if limit is outside 1-100 range
- **Network errors**: Returns descriptive error messages for connection issues
- **API errors**: Returns API error messages with status codes

## Development

### Running Tests

```bash
# Unit tests
cargo test

# Integration tests (requires DATENO_API_KEY)
cargo test --features integration-tests
```

### Project Structure

```
.
├── src/
│   ├── lib.rs           # Extension entry points
│   ├── api.rs           # Dateno API client
│   ├── error.rs         # Error types
│   └── table_function.rs # Table function implementation
├── tests/               # Test suites
├── Cargo.toml          # Rust dependencies
└── README.md           # This file
```

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Links

- [Dateno API Documentation](https://docs.dateno.io/)
- [DuckDB Documentation](https://duckdb.org/docs/)
- [duckdb-rs Documentation](https://docs.rs/duckdb/)

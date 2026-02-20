# Compilation Fixes Needed

## Known API Compatibility Issues

### 1. VTab Registration Helper
- **Issue**: `vtab::register_table_function::<T>()` helper may not exist
- **Fix**: Use `TableFunction` builder pattern with manual callbacks, or check if there's a macro

### 2. DataChunkHandle API
- **Issue**: `append_data_chunk()` method may not exist
- **Fix**: Need to verify actual API - may need to use `DataChunk` methods directly or different output mechanism

### 3. DataChunk API
- **Issue**: `DataChunk::new()` and `append_row()` methods need verification
- **Fix**: Check duckdb-rs docs for correct DataChunk construction

### 4. Bind/Init Data Storage
- **Issue**: Using raw pointers with `set_bind_data()` and `set_init_data()` - need to ensure proper lifetime management
- **Fix**: Verify pointer handling is correct

## Recommended Approach

1. Simplify to use `TableFunction` builder pattern (already partially implemented in lib.rs)
2. Remove VTab trait implementation if helper doesn't exist, or find correct registration method
3. Verify DataChunk/DataChunkHandle API from duckdb-rs source/docs
4. Test with minimal example first

## Next Steps

1. Check duckdb-rs GitHub for examples
2. Create minimal working example
3. Gradually add features
4. Fix API calls based on actual compilation errors

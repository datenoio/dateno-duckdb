mod api;
mod error;
mod table_function;

pub use error::{DatenoError, Result};
pub use table_function::{DatenoSearchBindData, DatenoSearchFunction};

use duckdb::{Connection, vtab};
use std::ffi::CString;
use std::os::raw::c_void;

/// Extension entry point - called by DuckDB when extension is loaded
#[no_mangle]
pub unsafe extern "C" fn dateno_duckdb_ext_init(
    db: *mut c_void,
    _info: *mut c_void,
    _error: *mut *mut c_void,
) -> u32 {
    // Register the table function using duckdb-rs helper
    match vtab::register_table_function::<DatenoSearchFunction>(db, _info, _error) {
        Ok(_) => 0, // Success
        Err(_) => 1, // Error
    }
}

/// Extension version function
#[no_mangle]
pub unsafe extern "C" fn dateno_duckdb_ext_version() -> *const u8 {
    static VERSION: &str = "0.1.0\0";
    VERSION.as_ptr()
}

/// Helper function to register the table function with a DuckDB connection
/// This can be called from Rust code for testing
pub fn register_with_connection(conn: &Connection) -> Result<(), duckdb::Error> {
    use duckdb::vtab::{BindInfo, InitInfo, TableFunctionInfo, VTab};
    
    // Create table function builder
    let mut tf = duckdb::vtab::TableFunction::new();
    tf.set_name("dateno_search");
    
    // Add parameters
    if let Some(params) = DatenoSearchFunction::parameters() {
        for param in params {
            tf.add_parameter(&param);
        }
    }
    
    // Set bind callback
    tf.set_bind(Some(bind_callback));
    
    // Set init callback  
    tf.set_init(Some(init_callback));
    
    // Set function callback
    tf.set_function(Some(func_callback));
    
    // Register
    conn.register_table_function(tf)?;
    
    Ok(())
}

// FFI callbacks - these bridge to our VTab implementation
unsafe extern "C" fn bind_callback(bind_info: duckdb::vtab::duckdb_bind_info) {
    let bind = BindInfo::from(bind_info);
    
    match DatenoSearchFunction::bind(&bind) {
        Ok(bind_data) => {
            // Set return columns
            bind.add_result_column("id", duckdb::types::LogicalType::Varchar.into());
            bind.add_result_column("title", duckdb::types::LogicalType::Varchar.into());
            bind.add_result_column("description", duckdb::types::LogicalType::Varchar.into());
            bind.add_result_column("format", duckdb::types::LogicalType::Varchar.into());
            bind.add_result_column("source", duckdb::types::LogicalType::Varchar.into());
            bind.add_result_column("url", duckdb::types::LogicalType::Varchar.into());
            bind.add_result_column("created_at", duckdb::types::LogicalType::Timestamp.into());
            bind.add_result_column("updated_at", duckdb::types::LogicalType::Timestamp.into());
            bind.add_result_column("formats", duckdb::types::LogicalType::List(Box::new(duckdb::types::LogicalType::Varchar)).into());
            
            // Store bind data
            let boxed = Box::into_raw(Box::new(bind_data));
            bind.set_bind_data(boxed as *mut c_void, Some(free_bind_data));
        }
        Err(e) => {
            bind.set_error(&e.to_string());
        }
    }
}

unsafe extern "C" fn init_callback(init_info: duckdb::vtab::duckdb_init_info) {
    let init = InitInfo::from(init_info);
    
    match DatenoSearchFunction::init(&init) {
        Ok(init_data) => {
            let boxed = Box::into_raw(Box::new(init_data));
            init.set_init_data(boxed as *mut c_void, Some(free_init_data));
        }
        Err(e) => {
            init.set_error(&e.to_string());
        }
    }
}

unsafe extern "C" fn func_callback(
    func_info: duckdb::vtab::duckdb_function_info,
    output: duckdb::vtab::duckdb_data_chunk,
) {
    use duckdb::vtab::{DataChunkHandle, TableFunctionInfo};
    
    let func = TableFunctionInfo::<DatenoSearchFunction>::from(func_info);
    let mut output_handle = DataChunkHandle::from(output);
    
    if let Err(e) = DatenoSearchFunction::func(&func, &mut output_handle) {
        func.set_error(&e.to_string());
    }
}

unsafe extern "C" fn free_bind_data(ptr: *mut c_void) {
    let _ = Box::from_raw(ptr as *mut DatenoSearchBindData);
}

unsafe extern "C" fn free_init_data(ptr: *mut c_void) {
    let _ = Box::from_raw(ptr as *mut table_function::DatenoSearchInitData);
}

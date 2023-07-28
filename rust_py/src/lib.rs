use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;


// Formats 2 numbers as string
#[no_mangle]
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String>{
    Ok((a+b).to_string())
} 

#[pyfunction]
fn count_words(sentence:String) -> PyResult<HashMap<String, usize>> {
    let mut hm:HashMap<String, usize> = HashMap::new();
    for word in sentence.split_whitespace(){
        *hm.entry(word.to_string()).or_insert(0) +=1;
    }
    Ok(hm)
} 


/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rust_py_test(_py:Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(count_words, m)?)?;
    Ok(())
}
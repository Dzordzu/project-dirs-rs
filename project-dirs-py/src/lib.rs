use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn from_manifest(manifest: &str) -> PyResult<String> {
    let builder: project_dirs_builder::Builder = serde_json::from_str(&manifest)
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("Failed to parse manifest: {e}")))?;

    let result = serde_json::to_string(&builder.build())
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("Failed to serialize result: {e}")))?;

    Ok(result)
}

#[pyfunction]
fn xdg_data_dirs() -> PyResult<Vec<std::path::PathBuf>> {
    Ok(project_dirs::strategy::xdg::xdg_data_dirs())
}

#[pyfunction]
fn xdg_config_dirs() -> PyResult<Vec<std::path::PathBuf>> {
    Ok(project_dirs::strategy::xdg::xdg_config_dirs())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _project_dirs_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(from_manifest, m)?)?;
    m.add_function(wrap_pyfunction!(xdg_data_dirs, m)?)?;
    m.add_function(wrap_pyfunction!(xdg_config_dirs, m)?)?;
    Ok(())
}

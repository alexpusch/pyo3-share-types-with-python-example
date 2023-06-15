use pyo3::prelude::*;

#[pyclass]
pub struct Output {
    x: i32,
    y: i32,
}

#[pymethods]
impl Output {
    #[new]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[pymodule]
pub fn my_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Output>()?;
    Ok(())
}

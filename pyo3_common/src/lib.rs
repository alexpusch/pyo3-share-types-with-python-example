use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass]
pub struct Output {
    #[pyo3(get, set)]
    x: i32,
    #[pyo3(get, set)]
    y: i32,
}

#[pymethods]
impl Output {
    #[new]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn calc(&self) -> i32 {
        self.x + self.y
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct Input {
    #[pyo3(get, set)]
    x: i32,
    #[pyo3(get, set)]
    y: i32,
}

#[pymethods]
impl Input {
    #[new]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}

#[pymodule]
pub fn pyo3_common(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Output>()?;
    m.add_class::<Input>()?;
    Ok(())
}

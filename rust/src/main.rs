use pyo3::{prelude::*, types::PyDict};
use pyo3_common::{Input, Output};

fn main() {
    Python::with_gil(|py| {
        // recreate the pyo3_common module from the same pyo3_common function that we used to create the pyo3 module
        let child_module = PyModule::new(py, "pyo3_common")?;
        pyo3_common::pyo3_common(py, child_module)?;

        // add pyo3_common module to pythons modules list
        let sys = PyModule::import(py, "sys")?;
        let modules: &PyDict = sys.getattr("modules")?.downcast()?;
        modules.set_item("pyo3_common", child_module)?;

        let module = PyModule::import(py, "python_lib.lib")?;

        let result: Output = module
            .getattr("run")?
            .call1((Input::new(1, 3),))?
            .extract()?;

        dbg!(result);

        PyResult::Ok(())
    })
    .unwrap();
}

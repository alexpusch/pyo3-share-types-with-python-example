# pyo3 - sharing Rust lib types with Python example

<p align="center">
  <img src="https://github.com/alexpusch/pyo3-share-types-with-python-example/assets/1129358/3f8d7d99-4018-47d2-b47d-a680d4fd2eda" />
</p>

## [Bottom Line](#the-trick)

## The Use Case
We have a Rust codebase and a Python code base. In our use case, we want to call a pythonic library from Rust code using Rust's [pyo3](https://pyo3.rs/) crate. The Python library receives and outputs types that are also used in the Rust codebase.

In this use case, it would be ideal if we could share types between the Rust code and the Python library. However, this turns out to be trickier than you might think.

## Example Structure
This example comes with three folders:
- **python** - Our Python codebase, which includes the **/python_lib** library. 
  ```python
  from pyo3_common import Output, Input

  def run(input: Input):
      output = Output(input.x, input.y)

      return output
  ```
- **rust** - Our Rust codebase. This will use pyo3 to call python_lib.
  ```rust
    let module = PyModule::import(py, "python_lib.lib")?;

    let result: Output = module
        .getattr("run")?
        .call1((Input::new(1, 3),))?
        .extract()?;
  ```

- **pyo3_common** - This is our shared types written in Rust and used by both our Rust and Python codebases. It defines `Input` and `Output` types, i.e.,
  ```rust
  #[pyclass]
  pub struct Output {
      #[pyo3(get, set)]
      x: i32,
      #[pyo3(get, set)]
      y: i32,
  }
  ```

## Why This is Tricky
At first glance, this looks straightforward. `rust` can depend on the `pyo3_common` crate, while `python_lib` depends on the pyo3 generated wheel. Both codebases would have access to the common types, and all would be well.

Unfortunately, if we create such a setup, we'll come into a roadblock. If we invoke the python_lib `run` method that returns the `Output` type:
```rust
let result: Output = module.getattr("run")?.call0()?.extract()?;
```

we'll get the error
`TypeError("'Output' object cannot be converted to 'Output'")`

The reason for this incompatibility is that the `Output` type has been compiled twice - once as an rlib, which is used by the Rust codebase, and second, as a cdylib (.so file in Linux) that is used by the Python codebase.

Each compilation outputs a different concrete type, which prevents pyo3 from converting one Output to another.

## The Trick
The workaround is rather simple:
```rust
  // Recreate the pyo3_common module from the same pyo3_common function that we used to create the pyo3 module
  let child_module = PyModule::new(py, "pyo3_common")?;
  pyo3_common::pyo3_common(py, child_module)?;

  // Add pyo3_common module to Python's modules list
  let sys = PyModule::import(py, "sys")?;
  let modules: &PyDict = sys.getattr("modules")?.downcast()?;
  modules.set_item("pyo3_common", child_module)?;
```

This dynamically creates the pyo3_common Python module and injects it into the Python sys.modules dictionary on the fly. This ensures the `Output` type is identical across the board, and the code runs as expected.

## Running the Example
- **Running Rust code**: From the rust directory, run `PYTHONPATH=../python cargo run`
- **Running Python code independently**: From the python directory, 
  - Symlink the .so file to the python directory - `ln -s ../pyo3_common/target/debug/libpyo3_common.so pyo3_common.so`
  - Run `python main.py`

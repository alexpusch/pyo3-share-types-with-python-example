# pyo3 - sharing Rust lib types with Python example

run Rust 
  - from rust dir 
  - `PYTHONPATH=../python cargo run`

run Python 
  - from python dir 
  - `ln -s ../pyo3_common/target/debug/libpyo3_common.so pyo3_common.so`
  - `python main.py`
pub mod classes;
pub mod closure;
pub mod iter;
pub mod polars;
pub mod props;
pub mod py_in_rust;

use pyo3::prelude::*;

use crate::iter::get_vertices;
use crate::iter::ItemIterator;
use crate::iter::People;
use crate::iter::Person;
use crate::iter::PersonIterator;
use crate::iter::VertexIterator;
use crate::iter::Warehouse;

use crate::classes::point_serde;
use crate::classes::point_serde2;
use crate::classes::print_prop;
use crate::classes::print_str;
use crate::classes::General;
use crate::classes::Greeter;
use crate::classes::Int;
use crate::classes::Prop;
use crate::classes::Str;

use crate::py_in_rust::test_py_in_rust;

use crate::props::get_props;

use crate::closure::invoke_passed_func;
use crate::closure::invoke_passed_func3;

use crate::polars::read_lotr;
use crate::polars::Schema;
use crate::polars::Vertex;

#[pymodule]
fn pyo3_example(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<VertexIterator>()?;
    m.add_class::<ItemIterator>()?;
    m.add_class::<PersonIterator>()?;
    m.add_class::<Warehouse>()?;
    m.add_class::<Person>()?;
    m.add_class::<People>()?;
    m.add_class::<Prop>()?;
    m.add_class::<Str>()?;
    m.add_class::<Int>()?;
    m.add_class::<Greeter>()?;
    m.add_class::<General>()?;
    m.add_class::<Schema>()?;
    m.add_class::<Vertex>()?;
    m.add_function(wrap_pyfunction!(print_prop, m)?)?;
    m.add_function(wrap_pyfunction!(print_str, m)?)?;
    m.add_function(wrap_pyfunction!(get_vertices, m)?)?;
    m.add_function(wrap_pyfunction!(test_py_in_rust, m)?)?;
    m.add_function(wrap_pyfunction!(get_props, m)?)?;
    m.add_function(wrap_pyfunction!(invoke_passed_func, m)?)?;
    m.add_function(wrap_pyfunction!(invoke_passed_func3, m)?)?;
    m.add_function(wrap_pyfunction!(point_serde, m)?)?;
    m.add_function(wrap_pyfunction!(point_serde2, m)?)?;
    m.add_function(wrap_pyfunction!(read_lotr, m)?)?;

    Ok(())
}

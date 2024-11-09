use numpy::{ndarray::Array1, PyArray1};
use pyo3::{exceptions::PyException, pyclass, pymethods, PyErr, PyResult, Python};

use crate::game::actions::{Action, ActionList};

#[pyclass(name = "Action")]
pub struct ActionPy(Action);

#[pymethods]
impl ActionPy {
    #[getter]
    fn get_action(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
        // match self.0 {
        //     Action::StopAttack => Ok(String::from("StopAttack")),
        //     Action::Take => Ok(String::from("Take")),
        //     Action::Attack(card) => Ok(format!("Attack({:?})", card)),
        //     Action::Defend(card) => Ok(format!("Defend({:?})", card)),
        // }
    }

    fn to_index(&self) -> PyResult<u8> {
        Ok(self.0.into())
    }
}

#[pyclass(name = "ActionList")]
pub struct ActionListPy(pub ActionList);

#[pymethods]
impl ActionListPy {
    #[getter]
    pub fn get_actions(&self) -> PyResult<Vec<String>> {
        Ok(self.0.to_strings())
    }

    pub fn __len__(&self) -> PyResult<usize> {
        Ok(self.0 .0.len())
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("ActionList({:?})", self.0.to_strings()))
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("ActionList({:?})", self.0.to_strings()))
    }

    pub fn __getitem__(&self, idx: isize) -> PyResult<ActionPy> {
        match (idx as usize) < self.0 .0.len() {
            false => Err(PyErr::new::<PyException, _>("Index out of bounds")),
            _ => Ok(ActionPy(self.0 .0[idx as usize])),
        }
    }

    pub fn to_indices(&self) -> PyResult<Vec<u8>> {
        Ok(self.0.to_u8s())
    }

    pub fn to_bitmap(&self) -> PyResult<pyo3::Py<PyArray1<bool>>> {
        let arr = Array1::from_vec(self.0.to_bitmap());
        Ok(Python::with_gil(|py| {
            PyArray1::from_array(py, &arr).to_owned()
        }))
    }
}

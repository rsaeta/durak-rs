use numpy::ndarray::Array1;
use pyo3::{pyclass, pymethods, PyResult};

use crate::game::cards::{Card, Hand};

#[pyclass(name = "Card")]
#[derive(Clone)]
pub struct CardPy {
    pub card: Card,
}

#[pymethods]
impl CardPy {
    #[new]
    pub fn new(suit: u8, rank: u8) -> Self {
        Self {
            card: Card {
                suit: suit.into(),
                rank,
            },
        }
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.card))
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.card))
    }

    #[getter(suit)]
    pub fn suit(&self) -> u8 {
        self.card.suit.into()
    }

    #[getter(rank)]
    pub fn rank(&self) -> u8 {
        self.card.rank
    }
}

impl std::fmt::Debug for CardPy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.card)
    }
}

pub struct HandPy<'a>(pub &'a Hand);

impl Into<Vec<u8>> for HandPy<'_> {
    fn into(self) -> Vec<u8> {
        (self.0.clone()).into()
    }
}

impl Into<Array1<u8>> for HandPy<'_> {
    fn into(self) -> Array1<u8> {
        (self.0.clone()).into()
    }
}

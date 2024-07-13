use crate::mine::{compute_mic as r_compute_mic, Mine as RMine};
use numpy::PyReadonlyArray1;
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature = (x,y,alpha=0.6,c=15.0), text_signature = "(x,y,alpha:float=0.6,c:float=15.0)")]
/// Creates a new tensor filled with zeros.
/// &RETURNS&: Tensor
fn compute_mic_list(x: Vec<f64>, y: Vec<f64>, alpha: f64, c: f64) -> f64 {
    let mut x = x;
    let mut y = y;
    r_compute_mic(&mut x, &mut y, alpha, c)
}

#[pyfunction]
#[pyo3(signature = (x,y,alpha=0.6,c=15.0), text_signature = "(x,y,alpha:float=0.6,c:float=15.0)")]
/// Creates a new tensor filled with zeros.
/// &RETURNS&: Tensor
fn compute_mic_numpy(
    x: PyReadonlyArray1<f64>,
    y: PyReadonlyArray1<f64>,
    alpha: f64,
    c: f64,
) -> PyResult<f64> {
    let mut x = x.as_slice()?.to_owned();
    let mut y = y.as_slice()?.to_owned();
    Ok(r_compute_mic(&mut x, &mut y, alpha, c))
}

#[pyclass]
struct Mine {
    mine: RMine,
}

#[pymethods]
impl Mine {
    #[new]
    #[pyo3(signature = (alpha=0.6,c=15.0), text_signature = "(alpha:float=0.6,c:float=15.0)")]
    fn new(alpha: f64, c: f64) -> Self {
        let mine = RMine::new(alpha, c);
        Self { mine }
    }
    #[pyo3(signature = (x,y), text_signature = "(x,y)")]
    fn compute_score(
        &mut self,
        x: PyReadonlyArray1<f64>,
        y: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let x = x.as_slice()?.to_owned();
        let y = y.as_slice()?.to_owned();
        self.mine.compute_score(x, y);
        Ok(())
    }
    #[pyo3(signature = (x,y), text_signature = "(x,y)")]
    fn compute_score_list(&mut self, x: Vec<f64>, y: Vec<f64>) {
        self.mine.compute_score(x, y);
    }
    pub fn mic(&mut self) -> Option<f64> {
        self.mine.mic()
    }
    pub fn mas(&mut self) -> Option<f64> {
        self.mine.mas()
    }
    pub fn mev(&mut self) -> Option<f64> {
        self.mine.mev()
    }
    #[pyo3(signature = (eps = 0.0), text_signature = "(eps: float = 0.0)")]
    pub fn mcn(&mut self, eps: f64) -> Option<f64> {
        self.mine.mcn(eps)
    }
    #[pyo3(signature = (p = -1.0), text_signature = "(p: float = -1.0)")]
    pub fn gmic(&mut self, p: f64) -> Option<f64> {
        self.mine.gmic(p)
    }
    #[pyo3(signature = (norm = false), text_signature = "(norm: bool = False)")]
    pub fn tic(&mut self, norm: bool) -> Option<f64> {
        self.mine.tic(norm)
    }
}
#[pymodule]
pub fn mine(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Mine>()?;
    m.add_function(wrap_pyfunction!(compute_mic_list, m)?)?;
    m.add_function(wrap_pyfunction!(compute_mic_numpy, m)?)?;
    Ok(())
}

use crate::bindings::{mine_compute_score, mine_mic, mine_parameter, mine_problem, mine_score,self};

pub struct Mine {
    alpha: f64,
    c: f64,
    x: Vec<f64>,
    y: Vec<f64>,
    score: Option<mine_score>,
}

unsafe impl Send for Mine {}

impl Mine {
    pub fn new(alpha: f64, c: f64) -> Self {
        Self {
            alpha,
            c,
            x: Vec::new(),
            y: Vec::new(),
            score: None,
        }
    }
    pub fn compute_score(&mut self, x: Vec<f64>, y: Vec<f64>) {
        let n = x.len() as i32;
        self.x = x;
        self.y = y;
        let mut prob = mine_problem {
            n,
            x: self.x.as_mut_ptr(),
            y: self.y.as_mut_ptr(),
        };
        let est = bindings::EST_MIC_APPROX as i32;
        let mut param = mine_parameter {
            alpha: self.alpha,
            c: self.c,
            est,
        };
        let score = unsafe { mine_compute_score(&mut prob, &mut param) };
        let score = unsafe { *score };
        self.score = Some(score);
    }
    pub fn mic(&mut self) -> Option<f64> {
        self.score.as_mut().map(|score| unsafe { mine_mic(score) })
    }
    pub fn mas(&mut self) -> Option<f64> {
        self.score.as_mut().map(|score| unsafe { bindings::mine_mas(score) })
    }
    pub fn mev(&mut self) -> Option<f64> {
        self.score.as_mut().map(|score| unsafe { bindings::mine_mev(score) })
    }
    pub fn mcn(&mut self,eps: f64) -> Option<f64> {
        self.score.as_mut().map(|score| unsafe { bindings::mine_mcn(score, eps) })
    }
    pub fn gmic(&mut self,p:f64) -> Option<f64> {
        self.score.as_mut().map(|score| unsafe { bindings::mine_gmic(score, p) })
    }
    pub fn tic(&mut self,norm: bool)  -> Option<f64> {
        self.score.as_mut().map(|score| unsafe { bindings::mine_tic(score, norm as i32) })
    }
    
}
/// Calculates the MIC score for a given dataset.
///
/// This function takes in two vectors of data points `x` and `y`, as well
/// as two parameters, `alpha` and `c`. It returns the MIC score for the
/// given dataset.
pub fn compute_mic(x: &mut [f64], y: &mut [f64], alpha: f64, c: f64) -> f64 {
    let n = x.len() as i32;
    let mut prob = mine_problem {
        n,
        x: x.as_mut_ptr(),
        y: y.as_mut_ptr(),
    };
    let est = bindings::EST_MIC_APPROX as i32;
    let mut param = mine_parameter { alpha, c, est };
    let score = unsafe { mine_compute_score(&mut prob, &mut param) };
    unsafe { bindings::mine_mic(score) }
}

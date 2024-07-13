import numpy as np
from typing import List
class Mine:
    """
    pub fn mas(&mut self) -> Option<f64> {
        self.mine.mas()
    }
    pub fn mev(&mut self) -> Option<f64> {
        self.mine.mev()
    }
    pub fn mcn(&mut self, eps: f64) -> Option<f64> {
        self.mine.mcn(eps)
    }
    pub fn gmic(&mut self, p: f64) -> Option<f64> {
        self.mine.gmic(p)
    }
    pub fn tic(&mut self, norm: bool) -> Option<f64> {
        self.mine.tic(norm)
    }
    """
    def __init__(self, alpha: float = 0.6,c: float = 15.0):
        ...
    
    def compute_score(self,x:np.ndarray,y:np.ndarray):
        ...
    
    def mic(self) -> float:
        ...
    
    def mas(self) -> float:
        ...
    
    def mev(self) -> float:
        ...

    def mcn(self, eps: float = 0.0) -> float:
        ...
    
    def gmic(self, p: float = -1) -> float:
        ...
    
    def tic(self, norm: bool = False) -> float:
        ...

def compute_mic_list(x: List[float],y: List[float]) -> float:
    ...

def compute_mas_numpy(x: np.ndarray,y: np.ndarray) -> float:
    ...

use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};

use crate::loss_function::LinearLoss;
use crate::adversary::core::FullInformationAdversary;


const DEFAULT_SEED: u64 = 777;


/// An oblivious adversary that returns a loss vector
/// randomly generated from range `[lb..ub).`
/// By default, the seed for randomness is set as `DEFAULT_SEED: u64 = 777.`
#[derive(Clone)]
pub struct LinearAdversary {
    dist: Uniform<f64>,
    rng: StdRng,
    dim: usize,
    lb: f64,
    ub: f64,
}


impl LinearAdversary {
    /// Construct a new instance of `LinearLoss.`
    /// By default, 
    /// the seed for randomness is set as `DEFAULT_SEED: u64 = 777` and
    /// range `[lb..ub) = [0.0..1.0)`.
    #[inline(always)]
    pub fn new(dim: usize) -> Self {
        let rng = StdRng::seed_from_u64(DEFAULT_SEED);
        let (lb, ub) = (0.0, 1.0);
        let dist = Uniform::from(lb..ub);

        Self { dist, rng, dim, lb, ub }
    }


    /// Set the range `[lb..ub)` of loss vector.
    /// By default, `[lb..ub) = [0.0..1.0).`
    #[inline(always)]
    pub fn range(mut self, lb: f64, ub: f64) -> Self {
        self.lb = lb;
        self.ub = ub;
        self
    }


    /// Set the seed for the randomness.
    /// By default, `DEFAULT_SEED: u64 = 777` is adopted.
    #[inline(always)]
    pub fn seed(mut self, seed: u64) -> Self {
        self.rng = StdRng::seed_from_u64(seed);
        self.dist = Uniform::from(self.lb..self.ub);
        self
    }
}



impl FullInformationAdversary for LinearAdversary {
    type LossFunction = LinearLoss;
    fn reveal(&mut self) -> Self::LossFunction {
        let dim = self.dim;
        let coef = self.dist.sample_iter(&mut self.rng)
            .take(dim)
            .collect::<Vec<_>>();

        LinearLoss::new(coef)
    }
}

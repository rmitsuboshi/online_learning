/// Defines the behavior of loss functions.
pub trait Loss<T> {
    /// Evaluate the loss value at the given point `arg.`
    fn eval(&self, arg: T) -> f64;
}



/// The linear loss function.
pub struct LinearLoss {
    coef: Vec<f64>,
}


impl LinearLoss {
    /// Construct a new instance of `LinearLoss.`
    pub fn new(coef: Vec<f64>) -> Self {
        Self { coef }
    }
}


impl<T> Loss<T> for LinearLoss
    where T: AsRef<[f64]>
{
    fn eval(&self, arg: T) -> f64 {
        let arg = arg.as_ref();
        assert_eq!(arg.len(), self.coef.len());
        self.coef.iter()
            .zip(arg)
            .map(|(&c, &x)| c * x)
            .sum()
    }
}

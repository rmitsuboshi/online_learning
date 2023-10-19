/// Defines the behavior of loss functions.
pub trait Loss<T> {
    /// Evaluate the loss value at the given point `arg.`
    fn eval(&self, arg: T) -> f64;
}

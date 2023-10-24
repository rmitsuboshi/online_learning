use crate::loss_function::Loss;

/// A trait that defines the behavior of adversaries.
pub trait FullInformationAdversary {
    type LossFunction;
    /// Returns a loss function.
    /// Loss function is represented as a `struct`
    /// which implements `LossFunction` trait.
    ///
    /// If you want to construct an adversarial adversary,
    /// update `self` before returning `L.`
    fn reveal(&mut self) -> Self::LossFunction;
}



/// A trait that defines the bandit adversary.
/// This trait is automatically implemented
/// if you implement `FullInformationAdversary.`
pub trait BanditAdversary<L, T> : FullInformationAdversary<LossFunction = L>
    where L: Loss<T>,
{
    fn eval(&mut self, arg: T) -> f64 {
        let loss: L = self.reveal();
        loss.eval(arg)
    }
}

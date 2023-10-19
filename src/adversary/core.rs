use crate::loss_function::Loss;

/// A trait that defines the behavior of adversaries.
pub trait FullInformationAdversary {
    /// Returns a loss function.
    /// Loss function is represented as a `struct`
    /// which implements `LossFunction` trait.
    ///
    /// If you want to construct an adversarial adversary,
    /// update `self` before returning `L.`
    fn reveal<L, T>(&mut self) -> L
        where L: Loss<T>;
}



/// A trait that defines the bandit adversary.
/// This trait is automatically implemented
/// if you implement `FullInformationAdversary.`
pub trait BanditAdversary : FullInformationAdversary {
    fn eval<L, T>(&mut self, arg: T) -> f64
        where L: Loss<T>
    {
        let loss: L = self.reveal();
        loss.eval(arg)
    }
}

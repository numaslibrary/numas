use array::Array;
use std::f64::consts::E;


impl<T> Array<T> where T: Copy + Into<f64> {
    /// Applies logarithm with given base on elements from given array and creates new array
    #[inline]
    pub fn log(&self, base: f64) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::log(value.clone().into(), base));
    }

    /// Applies logarithm with base 2 on elements from given array and creates new array
    #[inline]
    pub fn log2(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::log2(value.clone().into()));
    }

    /// Applies logarithm with base 10 on elements from given array and creates new array
    #[inline]
    pub fn log10(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::log10(value.clone().into()));
    }

    /// Applies natural logarithm on elements from given array and creates new array
    #[inline]
    pub fn loge(&self) -> Array<f64> {
        return self.log(E);
    }
}

use array::Array;


impl<T> Array<T> where T: Copy + Into<f64> {
    /// Applies exponential of elements from given array and creates new array
    #[inline]
    pub fn exp(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::exp(value.clone().into()));
    }

    /// Applies exponential minus one with base 2 on elements from given array and creates new array
    #[inline]
    pub fn exp_m1(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::exp_m1(value.clone().into()));
    }

    /// Applies exponential of 2 with base 10 on elements from given array and creates new array
    #[inline]
    pub fn exp2(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::exp2(value.clone().into()));
    }
}

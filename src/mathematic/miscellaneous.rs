use array::Array;


impl<T> Array<T> where T: Clone + Into<f64> {
    /// Applies square root on elements from given array and creates new array
    #[inline]
    pub fn sqrt(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::sqrt(value.clone().into()));
    }
}

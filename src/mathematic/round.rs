use array::Array;


impl<T> Array<T> where T: Clone + Into<f64> {
    /// Applies rounding on elements from given array and creates new array
    #[inline]
    pub fn round(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::round(value.clone().into()));
    }

    /// Applies round ceil on elements from given array and creates new array
    #[inline]
    pub fn ceil(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::ceil(value.clone().into()));
    }

    /// Applies round floor on elements from given array and creates new array
    #[inline]
    pub fn floor(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::floor(value.clone().into()));
    }

    /// Applies truncating on elements from given array and creates new array
    #[inline]
    pub fn trunc(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::trunc(value.clone().into()));
    }
}
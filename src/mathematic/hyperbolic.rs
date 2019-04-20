use array::Array;

impl<T> Array<T> where T: Copy + Into<f64> {
    /// Applies hyperbolic sine on elements from given array and creates new array
    #[inline]
    pub fn sinh(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::sinh(value.clone().into()));
    }

    /// Applies hyperbolic cosine on elements from given array and creates new array
    #[inline]
    pub fn cosh(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::cosh(value.clone().into()));
    }

    /// Applies hyperbolic tangent on elements from given array and creates new array
    #[inline]
    pub fn tanh(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::tanh(value.clone().into()));
    }

    /// Applies inverse hyperbolic sine on elements from given array and creates new array
    #[inline]
    pub fn arcsinh(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::asinh(value.clone().into()));
    }

    /// Applies hyperbolic cosine on elements from given array and creates new array
    #[inline]
    pub fn arccosh(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::acosh(value.clone().into()));
    }

    /// Applies hyperbolic tangent on elements from given array and creates new array
    #[inline]
    pub fn arctanh(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::atanh(value.clone().into()));
    }
}

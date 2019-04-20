use array::Array;

impl<T> Array<T> where T: Copy + Into<f64> {
    /// Applies sine on elements from given array and creates new array
    #[inline]
    pub fn sin(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::sin(value.clone().into()));
    }

    /// Applies cosine on elements from given array and creates new array
    #[inline]
    pub fn cos(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::cos(value.clone().into()));
    }

    /// Applies inverse sine on elements from given array and creates new array
    #[inline]
    pub fn arcsin(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::asin(value.clone().into()));
    }

    /// Applies inverse cosine on elements from given array and creates new array
    #[inline]
    pub fn arccos(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::acos(value.clone().into()));
    }

    /// Applies inverse tangent on elements from given array and creates new array
    #[inline]
    pub fn arctan(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::atan(value.clone().into()));
    }

    /// Converts elements from given array to degrees and creates new array
    #[inline]
    pub fn degrees(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::to_degrees(value.clone().into()));
    }

    /// Converts elements from given array to radians and creates new array
    #[inline]
    pub fn radians(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::to_radians(value.clone().into()));
    }

    /// Converts elements from given array to degrees and creates new array
    #[inline]
    pub fn deg2rad(&self) -> Array<f64> {
        return self.radians();
    }

    /// Converts elements from given array to radians and creates new array
    #[inline]
    pub fn rad2deg(&self) -> Array<f64> {
        return self.degrees();
    }
}
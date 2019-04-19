use array::Array;


impl<T> Array<T> where T: Clone + Into<f64> {
    /// Applies rounding on elements from given array and creates new array
    ///
    /// # Examples
    ///
    /// ```
    ///  use numas::array::Array;
    ///
    ///  let array = Array::new(vec![1.4, 1.5, 2.1, 2.6], vec![4]);
    ///  let rounded = array.round();
    ///  let data = rounded.collect();
    ///
    ///  assert_eq!(data, vec![1.0, 2.0, 2.0, 3.0]);
    /// ```
    #[inline]
    pub fn round(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::round(value.clone().into()));
    }

    /// Applies round ceil on elements from given array and creates new array
    ///
    /// # Examples
    ///
    /// ```
    ///  use numas::array::Array;
    ///
    ///  let array = Array::new(vec![1.4, -1.5, 2.1, 2.6], vec![4]);
    ///  let rounded = array.ceil();
    ///  let data = rounded.collect();
    ///
    ///  assert_eq!(data, vec![2.0, -1.0, 3.0, 3.0]);
    /// ```
    #[inline]
    pub fn ceil(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::ceil(value.clone().into()));
    }

    /// Applies round floor on elements from given array and creates new array
    ///
    /// # Examples
    ///
    /// ```
    ///  use numas::array::Array;
    ///
    ///  let array = Array::new(vec![1.4, -1.5, 2.1, 2.6], vec![4]);
    ///  let rounded = array.floor();
    ///  let data = rounded.collect();
    ///
    ///  assert_eq!(data, vec![1.0, -2.0, 2.0, 2.0]);
    /// ```
    #[inline]
    pub fn floor(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::floor(value.clone().into()));
    }

    /// Applies truncating on elements from given array and creates new array
    ///
    /// # Examples
    ///
    /// ```
    ///  use numas::array::Array;
    ///
    ///  let array = Array::new(vec![1.4, -1.5, 2.1, 2.6], vec![4]);
    ///  let rounded = array.round();
    ///  let data = rounded.collect();
    ///
    ///  assert_eq!(data, vec![1.0, -2.0, 2.0, 3.0]);
    /// ```
    #[inline]
    pub fn trunc(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::trunc(value.clone().into()));
    }
}

use array::Array;


impl<T> Array<T> where T: Clone + Into<f64> {
    /// Applies square root on elements from given array and creates new array
    ///
    /// # Examples
    ///
    /// ```
    ///  use numas::array::Array;
    ///  use numas::shape::Shape;
    ///
    ///  let array = Array::new(vec![4, 9, 16, 25], vec![4]);
    ///  let sqrt_array = array.sqrt();
    ///  let data = sqrt_array.collect();
    ///
    ///  assert_eq!(data, vec![2.0, 3.0, 4.0, 5.0]);
    /// ```
    #[inline]
    pub fn sqrt(&self) -> Array<f64> {
        return super::apply(&self, |value: &T| f64::sqrt(value.clone().into()));
    }
}

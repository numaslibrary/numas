use array::Array;

use std::cmp::PartialOrd;


impl<T> Array<T> where T: Copy + PartialOrd {
    /// Returns array of 1s and 0s representing truth value of lesser than element wise
    ///
    /// # Arguments
    ///
    /// * `other` - Array to compare with
    ///
    /// # Examples
    ///
    /// ```
    ///  use numas::array::Array;;
    ///
    ///  let first = Array::new(vec![1, 2, 3, 3], vec![4]);
    ///  let second = Array::new(vec![1, 0, 3, 4], vec![4]);
    ///
    ///  let less = first.lt(&second);
    ///
    ///  assert_eq!(less.collect(), vec![0, 0, 0, 1]);
    /// ```
    pub fn lt(&self, other: &Array<T>) -> Array<u8> {
        super::check_shapes_compatibility(self.shape(), other.shape());

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<u8> = Vec::with_capacity(self.len());

        if other.shape.total_len() == 1 {
            let value = second_data[other.shape.get_bounds().start];

            for f in self.shape.get_bounds() {
                data.push(if first_data[f] < value { 1 } else { 0 });
            }
        } else {
            for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                data.push(if first_data[f] < second_data[s] { 1 } else { 0 });
            }
        }

        return Array::new(data, self.get_shape().clone());
    }

    /// Returns array of 1s and 0s representing truth value of lesser equal element wise
    ///
    /// # Arguments
    ///
    /// * `other` - Array to compare with
    ///
    /// # Examples
    ///
    /// ```
    ///  use numas::array::Array;;
    ///
    ///  let first = Array::new(vec![1, 2, 3, 3], vec![4]);
    ///  let second = Array::new(vec![1, 0, 3, 4], vec![4]);
    ///
    ///  let less_equal = first.le(&second);
    ///
    ///  assert_eq!(less_equal.collect(), vec![1, 0, 1, 1]);
    /// ```
    pub fn le(&self, other: &Array<T>) -> Array<u8> {
        super::check_shapes_compatibility(self.shape(), other.shape());

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<u8> = Vec::with_capacity(self.len());

        if other.shape.total_len() == 1 {
            let value = second_data[other.shape.get_bounds().start];

            for f in self.shape.get_bounds() {
                data.push(if first_data[f] <= value { 1 } else { 0 });
            }
        } else {
            for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                data.push(if first_data[f] <= second_data[s] { 1 } else { 0 });
            }
        }

        return Array::new(data, self.get_shape().clone());
    }

    /// Returns array of 1s and 0s representing truth value of greater than element wise
    ///
    /// # Arguments
    ///
    /// * `other` - Array to compare with with
    ///
    /// # Examples
    ///
    /// ```
    ///  use numas::array::Array;;
    ///
    ///  let first = Array::new(vec![1, 2, 3, 3], vec![4]);
    ///  let second = Array::new(vec![1, 0, 3, 4], vec![4]);
    ///
    ///  let greater = first.gt(&second);
    ///
    ///  assert_eq!(greater.collect(), vec![0, 1, 0, 0]);
    /// ```
    pub fn gt(&self, other: &Array<T>) -> Array<u8> {
        super::check_shapes_compatibility(self.shape(), other.shape());

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<u8> = Vec::with_capacity(self.len());

        if other.shape.total_len() == 1 {
            let value = second_data[other.shape.get_bounds().start];

            for f in self.shape.get_bounds() {
                data.push(if first_data[f] > value { 1 } else { 0 });
            }
        } else {
            for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                data.push(if first_data[f] > second_data[s] { 1 } else { 0 });
            }
        }

        return Array::new(data, self.get_shape().clone());
    }

    /// Returns array of 1s and 0s representing truth value of greater equal element wise
    ///
    /// # Arguments
    ///
    /// * `other` - Array to compare with
    ///
    /// # Examples
    ///
    /// ```
    ///  use numas::array::Array;;
    ///
    ///  let first = Array::new(vec![1, 2, 3, 3], vec![4]);
    ///  let second = Array::new(vec![1, 0, 3, 4], vec![4]);
    ///
    ///  let greater_equal = first.ge(&second);
    ///
    ///  assert_eq!(greater_equal.collect(), vec![1, 1, 1, 0]);
    /// ```
    pub fn ge(&self, other: &Array<T>) -> Array<u8> {
        super::check_shapes_compatibility(self.shape(), other.shape());

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<u8> = Vec::with_capacity(self.len());

        if other.shape.total_len() == 1 {
            let value = second_data[other.shape.get_bounds().start];

            for f in self.shape.get_bounds() {
                data.push(if first_data[f] >= value { 1 } else { 0 });
            }
        } else {
            for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                data.push(if first_data[f] >= second_data[s] { 1 } else { 0 });
            }
        }

        return Array::new(data, self.get_shape().clone());
    }
}

impl<T> Array<T> where T: Copy + PartialEq {
    /// Returns array of 1s and 0s representing truth value of equality element wise
    ///
    /// # Arguments
    ///
    /// * `other` - Array to comapre with
    ///
    /// # Examples
    ///
    /// ```
    ///  use numas::array::Array;;
    ///
    ///  let first = Array::new(vec![1, 2, 3, 4], vec![4]);
    ///  let second = Array::new(vec![1, 0, 3, 4], vec![4]);
    ///
    ///  let equality = first.eq(&second);
    ///
    ///  assert_eq!(equality.collect(), vec![1, 0, 1, 1]);
    /// ```
    pub fn eq(&self, other: &Array<T>) -> Array<u8> {
        super::check_shapes_compatibility(self.shape(), other.shape());

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<u8> = Vec::with_capacity(self.len());

        if other.shape.total_len() == 1 {
            let value = second_data[other.shape.get_bounds().start];

            for f in self.shape.get_bounds() {
                data.push(if first_data[f] == value { 1 } else { 0 });
            }
        } else {
            for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                data.push(if first_data[f] == second_data[s] { 1 } else { 0 });
            }
        }

        return Array::new(data, self.get_shape().clone());
    }

    /// Returns array of 1s and 0s representing truth value of not equality element wise
    ///
    /// # Arguments
    ///
    /// * `other` - Array to compare with
    ///
    /// # Examples
    ///
    /// ```
    ///  use numas::array::Array;;
    ///
    ///  let first = Array::new(vec![1, 2, 3, 4], vec![4]);
    ///  let second = Array::new(vec![1, 0, 3, 4], vec![4]);
    ///
    ///  let not_equality = first.neq(&second);
    ///
    ///  assert_eq!(not_equality.collect(), vec![0, 1, 0, 0]);
    /// ```
    pub fn neq(&self, other: &Array<T>) -> Array<u8> {
        super::check_shapes_compatibility(self.shape(), other.shape());

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<u8> = Vec::with_capacity(self.len());

        if other.shape.total_len() == 1 {
            let value = second_data[other.shape.get_bounds().start];

            for f in self.shape.get_bounds() {
                data.push(if first_data[f] != value { 1 } else { 0 });
            }
        } else {
            for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                data.push(if first_data[f] != second_data[s] { 1 } else { 0 });
            }
        }

        return Array::new(data, self.get_shape().clone());
    }
}

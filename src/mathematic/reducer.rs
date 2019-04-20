use std::ops::{AddAssign, MulAssign};
use array::Array;


impl<T> Array<T> where T: Copy + From<u8> {
    /// Returns sums of all elements in array or view
    ///
    /// # Examples
    ///
    /// ```
    ///  use numas::array::Array;;
    ///
    ///  let array = Array::new(vec![1, 2, 3, 4], vec![4]);
    ///
    ///  assert_eq!(array.sum(), 10);
    /// ```
    pub fn sum(&self) -> T where T: AddAssign {
        let data = self.data.borrow();
        let mut accumulator = T::from(0 as u8);

        for i in self.shape.get_bounds() {
            accumulator += data[i];
        }

        return accumulator;
    }

    /// Returns product of all elements in array or view
    ///
    /// # Examples
    ///
    /// ```
    ///  use numas::array::Array;;
    ///
    ///  let array = Array::new(vec![1, 2, 3, 4], vec![4]);
    ///
    ///  assert_eq!(array.prod(), 24);
    /// ```
    pub fn prod(&self) -> T where T: MulAssign {
        if self.len() == 0 {
            return T::from(0 as u8);
        }

        let data = self.data.borrow();
        let mut accumulator = T::from(1 as u8);

        for i in self.shape.get_bounds() {
            accumulator *= data[i];
        }

        return accumulator;
    }
}

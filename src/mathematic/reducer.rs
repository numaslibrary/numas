use std::ops::{AddAssign, MulAssign};
use array::Array;


impl<T> Array<T> where T: Clone + From<u8> {
    /// Returns sums of all elements in array or view
    pub fn sum(&self) -> T where T: AddAssign {
        let data = self.data.borrow();
        let mut accumulator = T::from(0 as u8);

        for i in self.shape.get_bounds() {
            accumulator += data[i].clone();
        }

        return accumulator;
    }

    /// Returns product of all elements in array or view
    pub fn prod(&self) -> T where T: MulAssign {
        if self.len() == 0 {
            return T::from(0 as u8);
        }

        let data = self.data.borrow();
        let mut accumulator = T::from(1 as u8);

        for i in self.shape.get_bounds() {
            accumulator *= data[i].clone();
        }

        return accumulator;
    }
}

use std::ops::{AddAssign, Sub, Div, Mul };
use array::Array;


impl<T> Array<T> where T: Clone {
    /// Sums all elements in array or view view
    pub fn sum(&self) -> T where T: AddAssign + From<u8> {
        let data = self.data.borrow();
        let mut accumulator = T::from(0 as u8);

        for i in self.shape.get_bounds() {
            accumulator += data[i].clone();
        }

        return accumulator;
    }
}

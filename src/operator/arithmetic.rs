use array::Array;

use std::ops::{
    Add,
    Sub,
    Div,
    Mul,
    Neg,
    AddAssign,
    SubAssign,
    DivAssign,
    MulAssign,
};

impl<T> Add for Array<T> where T: Clone + Add<Output=T> {
    type Output = Array<T>;

    fn add(self, other: Array<T>) -> Array<T> {
        if self.shape == other.shape {
            let first_data = self.data.borrow();
            let second_data = other.data.borrow();

            let mut data: Vec<T> = Vec::with_capacity(self.len());

            for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                data.push(first_data[f].clone() + second_data[s].clone());
            }

            return Array::new(data, self.get_shape().clone());
        }

        panic!("Shapes are not compatible");
    }
}

impl<T> AddAssign for Array<T> where T: Clone + Add<Output=T> {
    fn add_assign(&mut self, other: Array<T>) -> () {
        if self.shape == other.shape {
            if self.data.as_ptr() == other.data.as_ptr() {
                let mut first_data = self.data.borrow_mut();
                for f in self.shape.get_bounds() {
                    first_data[f] = first_data[f].clone() + first_data[f].clone();
                }
            } else {
                let mut first_data = self.data.borrow_mut();
                let second_data = other.data.borrow();

                for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                    first_data[f] = first_data[f].clone() + second_data[s].clone();
                }
            }
            return;
        }

        panic!("Shapes are not compatible");
    }
}

impl<T: Clone> Sub for Array<T> {
    type Output = Array<T>;

    fn sub(self, other: Array<T>) -> Array<T> {
        return self;
    }
}

impl<T: Clone> SubAssign for Array<T> {
    fn sub_assign(&mut self, other: Array<T>) -> () {}
}

impl<T: Clone> Mul for Array<T> {
    type Output = Array<T>;

    fn mul(self, other: Array<T>) -> Array<T> {
        return self;
    }
}

impl<T: Clone> MulAssign for Array<T> {
    fn mul_assign(&mut self, other: Array<T>) -> () {}
}

impl<T: Clone> Div for Array<T> {
    type Output = Array<T>;

    fn div(self, other: Array<T>) -> Array<T> {
        return self;
    }
}

impl<T: Clone> DivAssign for Array<T> {
    fn div_assign(&mut self, other: Array<T>) -> () {}
}

impl<T: Clone> Neg for Array<T> {
    type Output = Array<T>;

    fn neg(self) -> Array<T> {
        return self;
    }
}
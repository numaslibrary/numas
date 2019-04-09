use array::Array;
use shape::Shape;

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


#[inline]
fn check_shapes_compatibility(first: &Shape, second: &Shape) -> () {
    if first != second {
        panic!("Shapes are not compatible");
    }
}

impl<T> Add for Array<T> where T: Clone + Add<Output=T> {
    type Output = Array<T>;

    fn add(self, other: Array<T>) -> Array<T> {
        check_shapes_compatibility(&self.shape, &other.shape);

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<T> = Vec::with_capacity(self.len());

        for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
            data.push(first_data[f].clone() + second_data[s].clone());
        }

        return Array::new(data, self.get_shape().clone());
    }
}

impl<T> AddAssign for Array<T> where T: Clone + Add<Output=T> {
    fn add_assign(&mut self, other: Array<T>) -> () {
        check_shapes_compatibility(&self.shape, &other.shape);

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
    }
}

impl<T> Sub for Array<T> where T: Clone + Sub<Output=T> {
    type Output = Array<T>;

    fn sub(self, other: Array<T>) -> Array<T> {
        check_shapes_compatibility(&self.shape, &other.shape);

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<T> = Vec::with_capacity(self.len());

        for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
            data.push(first_data[f].clone() - second_data[s].clone());
        }

        return Array::new(data, self.get_shape().clone());
    }
}

impl<T> SubAssign for Array<T> where T: Clone + Sub<Output=T> {
    fn sub_assign(&mut self, other: Array<T>) -> () {
        check_shapes_compatibility(&self.shape, &other.shape);

        if self.data.as_ptr() == other.data.as_ptr() {
            let mut first_data = self.data.borrow_mut();
            for f in self.shape.get_bounds() {
                first_data[f] = first_data[f].clone() - first_data[f].clone();
            }
        } else {
            let mut first_data = self.data.borrow_mut();
            let second_data = other.data.borrow();

            for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                first_data[f] = first_data[f].clone() - second_data[s].clone();
            }
        }
    }
}

impl<T> Mul for Array<T> where T: Clone + Mul<Output=T> {
    type Output = Array<T>;

    fn mul(self, other: Array<T>) -> Array<T> {
        check_shapes_compatibility(&self.shape, &other.shape);

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<T> = Vec::with_capacity(self.len());

        for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
            data.push(first_data[f].clone() * second_data[s].clone());
        }

        return Array::new(data, self.get_shape().clone());
    }
}

impl<T> MulAssign for Array<T> where T: Clone + Mul<Output=T> {
    fn mul_assign(&mut self, other: Array<T>) -> () {
        check_shapes_compatibility(&self.shape, &other.shape);

        if self.data.as_ptr() == other.data.as_ptr() {
            let mut first_data = self.data.borrow_mut();
            for f in self.shape.get_bounds() {
                first_data[f] = first_data[f].clone() * first_data[f].clone();
            }
        } else {
            let mut first_data = self.data.borrow_mut();
            let second_data = other.data.borrow();

            for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                first_data[f] = first_data[f].clone() * second_data[s].clone();
            }
        }
    }
}

impl<T> Div for Array<T> where T: Clone + Div<Output=T> {
    type Output = Array<T>;

    fn div(self, other: Array<T>) -> Array<T> {
        check_shapes_compatibility(&self.shape, &other.shape);

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<T> = Vec::with_capacity(self.len());

        for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
            data.push(first_data[f].clone() / second_data[s].clone());
        }

        return Array::new(data, self.get_shape().clone());
    }
}

impl<T> DivAssign for Array<T> where T: Clone + Div<Output=T> {
    fn div_assign(&mut self, other: Array<T>) -> () {
        check_shapes_compatibility(&self.shape, &other.shape);

        if self.data.as_ptr() == other.data.as_ptr() {
            let mut first_data = self.data.borrow_mut();
            for f in self.shape.get_bounds() {
                first_data[f] = first_data[f].clone() / first_data[f].clone();
            }
        } else {
            let mut first_data = self.data.borrow_mut();
            let second_data = other.data.borrow();

            for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                first_data[f] = first_data[f].clone() / second_data[s].clone();
            }
        }
    }
}

impl<T: Clone> Neg for Array<T> {
    type Output = Array<T>;

    fn neg(self) -> Array<T> {
        return self;
    }
}
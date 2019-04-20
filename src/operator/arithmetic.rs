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


impl<T> Add for &Array<T> where T: Copy + Add<Output=T> {
    type Output = Array<T>;

    fn add(self, other: &Array<T>) -> Array<T> {
        super::check_shapes_compatibility(&self.shape, &other.shape);

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<T> = Vec::with_capacity(self.len());

        if other.shape.total_len() == 1 {
            let value = second_data[other.shape.get_bounds().start];

            for f in self.shape.get_bounds() {
                data.push(first_data[f] + value);
            }
        } else {
            for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                data.push(first_data[f] + second_data[s]);
            }
        }

        return Array::new(data, self.get_shape().clone());
    }
}

impl<T> AddAssign for &Array<T> where T: Copy + Add<Output=T> {
    fn add_assign(&mut self, other: &Array<T>) -> () {
        super::check_shapes_compatibility(&self.shape, &other.shape);

        if self.data.as_ptr() == other.data.as_ptr() {
            let mut first_data = self.data.borrow_mut();
            for f in self.shape.get_bounds() {
                first_data[f] = first_data[f] + first_data[f];
            }
        } else {
            let mut first_data = self.data.borrow_mut();
            let second_data = other.data.borrow();

            if other.shape.total_len() == 1 {
                let value = second_data[other.shape.get_bounds().start];

                for f in self.shape.get_bounds() {
                    first_data[f] = first_data[f] + value;
                }
            } else {
                for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                    first_data[f] = first_data[f] + second_data[s];
                }
            }
        }
    }
}

impl<T> Sub for &Array<T> where T: Copy + Sub<Output=T> {
    type Output = Array<T>;

    fn sub(self, other: &Array<T>) -> Array<T> {
        super::check_shapes_compatibility(&self.shape, &other.shape);

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<T> = Vec::with_capacity(self.len());

        if other.shape.total_len() == 1 {
            let value = second_data[other.shape.get_bounds().start];

            for f in self.shape.get_bounds() {
                data.push(first_data[f] - value);
            }
        } else {
            for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                data.push(first_data[f] - second_data[s]);
            }
        }

        return Array::new(data, self.get_shape().clone());
    }
}

impl<T> SubAssign for &Array<T> where T: Copy + Sub<Output=T> {
    fn sub_assign(&mut self, other: &Array<T>) -> () {
        super::check_shapes_compatibility(&self.shape, &other.shape);

        if self.data.as_ptr() == other.data.as_ptr() {
            let mut first_data = self.data.borrow_mut();
            for f in self.shape.get_bounds() {
                first_data[f] = first_data[f] - first_data[f];
            }
        } else {
            let mut first_data = self.data.borrow_mut();
            let second_data = other.data.borrow();

            if other.shape.total_len() == 1 {
                let value = second_data[other.shape.get_bounds().start];

                for f in self.shape.get_bounds() {
                    first_data[f] = first_data[f] - value;
                }
            } else {
                for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                    first_data[f] = first_data[f] - second_data[s];
                }
            }
        }
    }
}

impl<T> Mul for &Array<T> where T: Copy + Mul<Output=T> {
    type Output = Array<T>;

    fn mul(self, other: &Array<T>) -> Array<T> {
        super::check_shapes_compatibility(&self.shape, &other.shape);

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<T> = Vec::with_capacity(self.len());

        if other.shape.total_len() == 1 {
            let value = second_data[other.shape.get_bounds().start];

            for f in self.shape.get_bounds() {
                data.push(first_data[f] * value);
            }
        } else {
            for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                data.push(first_data[f] * second_data[s]);
            }
        }

        return Array::new(data, self.get_shape().clone());
    }
}

impl<T> MulAssign for &Array<T> where T: Copy + Mul<Output=T> {
    fn mul_assign(&mut self, other: &Array<T>) -> () {
        super::check_shapes_compatibility(&self.shape, &other.shape);

        if self.data.as_ptr() == other.data.as_ptr() {
            let mut first_data = self.data.borrow_mut();
            for f in self.shape.get_bounds() {
                first_data[f] = first_data[f] * first_data[f];
            }
        } else {
            let mut first_data = self.data.borrow_mut();
            let second_data = other.data.borrow();

            if other.shape.total_len() == 1 {
                let value = second_data[other.shape.get_bounds().start];

                for f in self.shape.get_bounds() {
                    first_data[f] = first_data[f] * value;
                }
            } else {
                for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                    first_data[f] = first_data[f] * second_data[s];
                }
            }
        }
    }
}

impl<T> Div for &Array<T> where T: Copy + Div<Output=T> {
    type Output = Array<T>;

    fn div(self, other: &Array<T>) -> Array<T> {
        super::check_shapes_compatibility(&self.shape, &other.shape);

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<T> = Vec::with_capacity(self.len());

        if other.shape.total_len() == 1 {
            let value = second_data[other.shape.get_bounds().start];

            for f in self.shape.get_bounds() {
                data.push(first_data[f] / value);
            }
        } else {
            for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                data.push(first_data[f] / second_data[s]);
            }
        }

        return Array::new(data, self.get_shape().clone());
    }
}

impl<T> DivAssign for &Array<T> where T: Copy + Div<Output=T> {
    fn div_assign(&mut self, other: &Array<T>) -> () {
        super::check_shapes_compatibility(&self.shape, &other.shape);

        if self.data.as_ptr() == other.data.as_ptr() {
            let mut first_data = self.data.borrow_mut();
            for f in self.shape.get_bounds() {
                first_data[f] = first_data[f] / first_data[f];
            }
        } else {
            let mut first_data = self.data.borrow_mut();
            let second_data = other.data.borrow();

            if other.shape.total_len() == 1 {
                let value = second_data[other.shape.get_bounds().start];

                for f in self.shape.get_bounds() {
                    first_data[f] = first_data[f] / value;
                }
            } else {
                for (f, s) in self.shape.get_bounds().zip(other.shape.get_bounds()) {
                    first_data[f] = first_data[f] / second_data[s];
                }
            }
        }
    }
}

impl<T> Neg for Array<T> where T: Copy + Neg<Output=T> {
    type Output = Array<T>;

    fn neg(self) -> Array<T> {
        {
            let mut data = self.data.borrow_mut();

            for i in self.shape.get_bounds() {
                data[i] = -data[i];
            }
        }
        return self;
    }
}
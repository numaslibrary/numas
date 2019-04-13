use array::Array;

use std::cmp::PartialOrd;


impl<T> Array<T> where T: Clone + PartialOrd {
    pub fn lt(&self, other: &Array<T>) -> Array<u8> {
        super::check_shapes_compatibility(self.shape(), other.shape());

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<u8> = Vec::with_capacity(self.len());

        if other.shape.total_len() == 1 {
            let value = second_data[other.shape.get_bounds().start].clone();

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

    pub fn le(&self, other: &Array<T>) -> Array<u8> {
        super::check_shapes_compatibility(self.shape(), other.shape());

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<u8> = Vec::with_capacity(self.len());

        if other.shape.total_len() == 1 {
            let value = second_data[other.shape.get_bounds().start].clone();

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

    pub fn gt(&self, other: &Array<T>) -> Array<u8> {
        super::check_shapes_compatibility(self.shape(), other.shape());

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<u8> = Vec::with_capacity(self.len());

        if other.shape.total_len() == 1 {
            let value = second_data[other.shape.get_bounds().start].clone();

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

    pub fn ge(&self, other: &Array<T>) -> Array<u8> {
        super::check_shapes_compatibility(self.shape(), other.shape());

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<u8> = Vec::with_capacity(self.len());

        if other.shape.total_len() == 1 {
            let value = second_data[other.shape.get_bounds().start].clone();

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

impl<T> Array<T> where T: Clone + PartialEq {
    pub fn eq(&self, other: &Array<T>) -> Array<u8> {
        super::check_shapes_compatibility(self.shape(), other.shape());

        let first_data = self.data.borrow();
        let second_data = other.data.borrow();

        let mut data: Vec<u8> = Vec::with_capacity(self.len());

        if other.shape.total_len() == 1 {
            let value = second_data[other.shape.get_bounds().start].clone();

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
}

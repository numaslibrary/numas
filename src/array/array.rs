use std::cmp;
use array::Shape;

pub struct Array<T: Sized> {
    pub data: Vec<T>,
    pub shape: Shape
}

impl <T: Sized> Array<T> {
    pub fn new(data: Vec<T>, shape: Vec<i32>) -> Array<T> {
        Array { data, shape: Shape::new(shape) }
    }

    pub fn set_shape(&mut self, shape: Vec<i32>) -> () {
        self.shape.set_shape(shape);
    }

    pub fn get_shape(&self) -> &Vec<i32> {
        self.shape.get_shape()
    }

    pub fn get_raw(&self, indices: Vec<usize>) -> &[T] {
        let (start, count) = self.shape.get_index(indices);

        return &self.data[start..(count + start)];
    }

    pub fn set(&mut self, indices: Vec<usize>, mut values: Vec<T>) -> () {
        let (start, count) = self.shape.get_index(indices);
        let range = 0..cmp::min(count, values.len());

        for (index, value) in values.drain(range).enumerate() {
            self.data[start + index] = value;
        }
    }

    pub fn len(self) -> usize {
        return self.data.len();
    }
}

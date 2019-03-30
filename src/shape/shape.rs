use std::ops::Range;

/// Array Shape Structure
#[derive(Clone)]
pub struct Shape {
    shape: Vec<i32>,
    strides: Vec<usize>,
    start: usize,
    end: usize,
}


impl Shape {
    /// Returns shape products for easier converting indices to linear index
    ///
    /// # Arguments
    ///
    /// * `shape` - Shape to calculate products for
    fn calculate_strides(shape: &Vec<i32>) -> Vec<usize> {
        let len = shape.len();
        let mut products: Vec<usize> = vec![1; len];
        let mut accumulator = 1;

        for i in (0..(len - 1)).rev() {
            accumulator *= shape[i + 1] as usize;
            products[i] = accumulator;
        }

        return products;
    }

    /// Returns new Shape
    ///
    /// # Arguments
    ///
    /// * `shape` - Initial shape matrix
    pub fn new(shape: Vec<i32>, start: usize, end: usize) -> Shape {
        let strides = Shape::calculate_strides(&shape);

        if Shape::len(&shape, &strides) != (end - start) as i32 {
            panic!(
                "Invalid shape given: shape size {}, data size {}",
                Shape::len(&shape, &strides),
                end - start
            );
        }

        return Shape { strides, shape, start, end };
    }

    /// Sets shape
    ///
    /// # Arguments
    ///
    /// * `shape` - New shape matrix
    pub fn set_shape(&mut self, shape: Vec<i32>) -> () {
        self.strides = Shape::calculate_strides(&shape);
        self.shape = shape;
    }

    /// Returns shape
    pub fn get_shape(&self) -> &Vec<i32> {
        return &self.shape;
    }

    /// Returns real index in linear array and number of elements
    ///
    /// # Arguments
    ///
    /// * `indices` - Indices
    pub fn get_index(&self, indices: Vec<usize>) -> (usize, usize) {
        let start = indices.iter()
            .zip(self.strides.iter())
            .fold(0, |index, (&i, &p)| index + i * p);

        let range = indices.len()..self.strides.len();

        let count = self.shape[range].iter()
            .fold(1, |index, &value| index * value) as usize;

        return (start + &self.start, count);
    }

    /// Converts vector of indices to one index in linear array
    ///
    /// # Arguments
    ///
    /// * `indices` - Indices
    pub fn to_index(&self, indices: &Vec<usize>) -> usize {
        return indices.iter()
            .zip(self.strides.iter())
            .fold(0, |acc, (&i, &s)| acc + i * s);
    }

    /// Returns bounds of shape
    pub fn get_bounds(&self) -> Range<usize> {
        return self.start..self.end;
    }

    /// Returns total number of elements from shape and strides
    ///
    /// # Arguments
    ///
    /// * `shape` - shape vector
    /// * `strides` - strides vector
    #[inline]
    fn len(shape: &Vec<i32>, strides: &Vec<usize>) -> i32 {
        return shape[0] * strides[0] as i32;
    }

    /// Returns total number of elements from shape
    ///
    /// # Arguments
    ///
    /// * `shape` - Array shape
    pub fn total_len(shape: &Shape) -> i32 {
        return Shape::len(&shape.shape, &shape.strides);
    }
}
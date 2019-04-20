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
    #[inline]
    pub fn get_shape(&self) -> &Vec<i32> {
        return &self.shape;
    }

    /// Converts vector of indices to Shape
    ///
    /// # Arguments
    ///
    /// * `indices` - Indices
    ///
    /// # Example
    pub fn indices_to_shape(&self, indices: Vec<usize>) -> Shape {
        let indices_len = indices.len();
        let shape_len = self.shape.len();
        let indices_len_half = indices_len / 2;

        let mut offset = self.start;

        for i in 0..indices_len_half {
            offset += indices[i * 2] * self.strides[i];
        }

        println!("{:?}", &indices);
        let shape = if indices[indices_len - 1] != 0 {
            let mut tmp: Vec<i32> = Vec::with_capacity(shape_len - indices_len_half + 1);
            tmp.push((indices[indices_len - 1] - indices[indices_len - 2]) as i32);
            tmp.extend(self.shape.iter().skip(indices_len_half));
            tmp
        } else if indices_len_half == shape_len {
            vec![1]
        } else {
            self.shape[indices_len_half..shape_len].to_vec()
        };

        let count = shape[0] as usize * self.strides[shape_len - shape.len()];
        let end = offset + count;


        return Shape::new(shape, offset as usize, end);
    }

    /// Returns bounds of shape
    #[inline]
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
    #[inline]
    pub fn total_len(&self) -> usize {
        return Shape::len(&self.shape, &self.strides) as usize;
    }
}
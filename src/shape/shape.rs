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

    /// Returns real index in linear array and number of elements
    ///
    /// # Arguments
    ///
    /// * `indices` - Indices
    pub fn get_index(&self, indices: Vec<Vec<usize>>) -> (usize, usize) {
        let indices_size = indices.len();

        if indices_size == 0 {
            return (0, self.total_len());
        }

        let (start, end): (i32, i32) = indices.iter()
            .zip(self.strides.iter())
            .fold(
                (0, -1),
                |(s, e), (i, &p)| (s + (i[0] * p) as i32, if i.len() == 2 && e == -1 { (i[1] * p) as i32 } else { -1 })
            );

        if end == -1 {
            return (start as usize, (self.shape[indices_size] as usize * self.strides[indices_size]));
        }

        return (start as usize, end as usize);
    }

    /// Converts vector of indices to Shape
    ///
    /// # Arguments
    ///
    /// * `indices` - Indices
    pub fn indices_to_shape(&self, indices: Vec<Vec<usize>>) -> Shape {
        let indices_len = indices.len();
        let shape_len = self.shape.len();

        let offset = indices
            .iter()
            .zip(self.strides.iter())
            .fold(0, |acc, (i, s)| acc + s * i[0]);

        let shape = if indices_len != 0 && indices[indices_len - 1].len() == 2 {
            let mut tmp: Vec<i32> = vec![(indices[indices_len - 1][1] - indices[indices_len - 1][0] + 1) as i32];
            tmp.extend(self.shape.iter().skip(indices_len));
            tmp
        } else if indices_len == shape_len {
            vec![1]
        } else {
            self.shape[indices_len..shape_len].to_vec()
        };

        let count = shape[0].clone() as usize * self.strides[self.strides.len() - shape.len()];
        let end = offset.clone() + count;

        return Shape::new(shape, offset.clone() as usize, end);
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
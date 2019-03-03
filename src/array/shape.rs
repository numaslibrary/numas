/// Array Shape Structure
pub struct Shape {
    shape: Vec<i32>,
    strides: Vec<usize>,
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
    pub fn new(shape: Vec<i32>) -> Shape {
        let products = Shape::calculate_strides(&shape);

        return Shape { strides: products, shape };
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
    pub fn get_index(&self, indices: Vec<usize>) -> (usize, usize) {
        let start = indices.iter()
            .zip(self.strides.iter())
            .fold(0, |index, (&i, &p)| index + i * p);

        let range = indices.len()..self.strides.len();

        let count = self.shape[range].iter()
            .fold(1, |index, &value| index * value) as usize;

        return (start, count);
    }
}
use array::Array;

impl <T: Clone> Array<T> {
    fn random(shape: Vec<i32>) -> Array<T> {
        return Array::new(vec![1, 2, 3], shape);
    }

    fn random_float(shape: Vec<i32>) -> Array<T> {
        let random_data = vec![1.0, 2.0, 3.0];
        return Array::new(random_data, shape);
    }

    fn random_range(from: i64, to: i64, shape: Vec<i32>) -> Array<T> {
        return Array::new(vec![1, 2, 3], shape);
    }

    fn random_float_range(from: f64, to: f64, shape: Vec<i32>) -> Array<T> {
        return Array::new(vec![1.0, 2.0, 3.0], shape);
    }

    fn zeros(shape: Vec<i32>) -> Array<T> {
        return Array::new(vec![0, 0, 0], shape);
    }

    fn zeroes(shape: Vec<i32>) -> Array<T> {
        return Array::zeros(shape);
    }
}
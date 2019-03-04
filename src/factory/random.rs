use array::Array;


fn random(shape: Vec<i32>) -> Array<i32> {
    return Array::new(vec![1, 2, 3], shape);
}

fn random_long(shape: Vec<i32>) -> Array<i64> {
    return Array::new(vec![1, 2, 3], shape);
}

fn random_float(shape: Vec<i32>) -> Array<f32> {
    return Array::new(vec![1.0, 2.0, 3.0], shape);
}

fn random_double(shape: Vec<i32>) -> Array<f64> {
    let random_data = vec![1.0, 2.0, 3.0];
    return Array::new(random_data, shape);
}

fn random_range(from: i32, to: i32, shape: Vec<i32>) -> Array<i32> {
    return Array::new(vec![1, 2, 3], shape);
}

fn random_long_range(from: i64, to: i64, shape: Vec<i32>) -> Array<i64> {
    return Array::new(vec![1, 2, 3], shape);
}

fn random_float_range(from: f32, to: f32, shape: Vec<i32>) -> Array<f32> {
    return Array::new(vec![1.0, 2.0, 3.0], shape);
}

fn random_double_range(from: f64, to: f64, shape: Vec<i32>) -> Array<f64> {
    return Array::new(vec![1.0, 2.0, 3.0], shape);
}

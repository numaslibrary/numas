use array::Array;

fn zeros(shape: Vec<i32>) -> Array<i32> {
    return Array::new(vec![0, 0, 0], shape);
}

fn zeroes(shape: Vec<i32>) -> Array<i32> {
    return zeros(shape);
}
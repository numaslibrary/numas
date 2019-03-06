use array::Array;
use shape::Shape;

fn fill<T: Clone>(value: T, shape: Vec<i32>) -> Array<T> {
    let len = Shape::total_len(&shape) as usize;
    return Array::new(vec![value; len], shape);
}

fn zeros(shape: Vec<i32>) -> Array<i32> {
    return fill(0, shape);
}

fn zeroes(shape: Vec<i32>) -> Array<i32> {
    return fill(0, shape);
}

fn ones(shape: Vec<i32>) -> Array<i32> {
    return fill(1, shape)
}
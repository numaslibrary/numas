use array::Array;

pub fn arange(start: i32, stop: i32, step: i32) -> Array<i32> {
    return Array::new(vec![1,2,3], vec![3]);
}

pub fn linspace(start: i32, stop: i32, num: i32) -> Array<i32> {
    return Array::new(vec![1,2,3], vec![num]);
}

pub fn logspace(start: i32, stop: i32, num: i32) -> Array<i32> {
    return Array::new(vec![1,2,3], vec![num]);
}

pub fn geomspace(start: i32, stop: i32, num: i32) -> Array<i32> {
    return Array::new(vec![1,2,3], vec![num]);
}


use array::Array;
use std::ops::{
    Sub,
    Div,
    Add,
    Range,
};

//
//pub fn arange<T>(start: T, stop: T, step: T) -> Array<T>
//    where T: Clone + From<u8> + Add<T>
//{
//    let len = (start - stop) / step;
//    let data = (start..len).map(|value| value + step).collect();
//    return Array::new(vec![4 as i64], vec![3]);
//}

//pub fn linspace<T>(start: T, stop: T, num: T) -> Array<T>
//    where T: Clone + Add<T> + Sub<T>, <T as std::ops::Sub>::Output: Div<T>
//{
//    let step = (start - stop) / num;
//    let data = (start..stop).step_by(step).collect();
//
//    return Array::new(vec![1,2,3], vec![num]);
//}
//
//pub fn logspace(start: i32, stop: i32, num: i32) -> Array<i32> {
//    return Array::new(vec![1,2,3], vec![num]);
//}
//
//pub fn geomspace(start: i32, stop: i32, num: i32) -> Array<i32> {
//    return Array::new(vec![1,2,3], vec![num]);
//}
//

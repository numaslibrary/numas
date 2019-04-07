use array::Array;
use std::ops::{Sub, Div, Add, Range, RangeFrom};


pub fn linspace(start: f64, stop: f64, num: usize) -> Array<f64> {
    let mut data: Vec<f64> = Vec::with_capacity(num);

    let step: f64 = (stop - start) / (num - 1) as f64;
    let mut s = start;

    for i in 0..num {
        data.push(s);
        s += step;
    }

    return Array::new(data, vec![num as i32]);
}

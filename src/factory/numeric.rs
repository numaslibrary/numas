use array::Array;

/// Returns evenly spaced elements from given interval
///
/// # Arguments
///
/// * `start` - start of interval
/// * `stop` - end of interval
/// * `num` - number of elements
pub fn linspace(start: f64, stop: f64, num: usize) -> Array<f64> {
    let mut data: Vec<f64> = Vec::with_capacity(num);

    let step: f64 = (stop - start) / (num - 1) as f64;
    let mut tmp = start;

    for _ in 0..num {
        data.push(tmp);
        tmp += step;
    }

    return Array::new(data, vec![num as i32]);
}

/// Returns evenly spaced interval within a given interval
///
/// # Arguments
///
/// * `start` - start of interval
/// * `stop` - end of interval
/// * `step` - step of interval
pub fn arange(start: f64, stop: f64, step: f64) -> Array<f64> {
    let num = f64::ceil((stop - start) / step) as i32;
    let mut data: Vec<f64> = Vec::with_capacity(num as usize);
    let mut tmp = start;

    for _ in 0..num {
        data.push(tmp.clone());
        tmp = tmp + step.clone();
    }

    return Array::new(data, vec![num]);
}

/// Returns evenly spaced elements on log space from given interval
///
/// # Arguments
///
/// * `start` - start of interval
/// * `stop` - end of interval
/// * `base` - base of log
/// * `num` - number of elements
pub fn logspace(start: f64, stop: f64, base: f64, num: usize) -> Array<f64> {
    let mut data: Vec<f64> = Vec::with_capacity(num);

    let step: f64 = (stop - start) / (num - 1) as f64;
    let mut tmp = start;

    for _ in 0..num {
        data.push(base.powf(tmp));
        tmp += step;
    }

    return Array::new(data, vec![num as i32]);
}



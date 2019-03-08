use array::Array;


fn sinh<T>(array: Array<T>) -> Array<f64> {
    let data = array.data.iter().map(|value| f64::sinh(value)).collect();

    return Array::new(data, array.get_shape().clone());
}

fn cosh<T>(array: Array<T>) -> Array<f64> {
    let data = array.data.iter().map(|value| f64::cosh(value)).collect();

    return Array::new(data, array.get_shape().clone());
}

fn tanh<T>(array: Array<T>) -> Array<f64> {
    let data = array.data.iter().map(|value| f64::tanh(value)).collect();

    return Array::new(data, array.get_shape().clone());
}

fn arcsinh<T>(array: Array<T>) -> Array<f64> {
    let data = array.data.iter().map(|value| f64::asinh(value)).collect();

    return Array::new(data, array.get_shape().clone());
}

fn arccosh<T>(array: Array<T>) -> Array<T> {
    let data = array.data.iter().map(|value| f64::acosh(value)).collect();

    return Array::new(data, array.get_shape().clone());
}

fn arctanh<T>(array: Array<T>) -> Array<T> {
    let data = array.data.iter().map(|value| f64::atanh(value)).collect();

    return Array::new(data, array.get_shape().clone());
}
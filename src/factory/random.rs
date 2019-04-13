extern crate rand;

use self::rand::{
    distributions::{
        Uniform,
        Distribution,
    },
    Rng,
};

use array::Array;


/// Creates new array of given shape filled with random values from given distribution
///
/// # Arguments
///
/// * `distr` - distribution of random values
/// * `shape` - shape of new array
pub fn random_from_distribution<T, D>(distr: D, shape: Vec<i32>) -> Array<T>
    where T: Clone, D: Distribution<T>
{
    let mut rng = rand::thread_rng();

    let len: i32 = shape.iter().product();
    let data: Vec<T> = rng.sample_iter(&distr).take(len as usize).collect();

    return Array::new(data, shape);
}

/// Creates new array of given shape filled with random values between given range
///
/// # Arguments
///
/// * `from` - start of range
/// * `to` - end of range
/// * `shape` - shape of new array
#[inline]
pub fn random_range<T>(from: T, to: T, shape: Vec<i32>) -> Array<T>
    where T: rand::distributions::uniform::SampleUniform + Clone
{
    return random_from_distribution(Uniform::new::<T, T>(from, to), shape);
}

/// Creates new array of given shape filled with random values from between 0 and 1 (floating point numbers only)
///
/// # Arguments
///
/// * `shape` - shape of new array
#[inline]
pub fn random<T>(shape: Vec<i32>) -> Array<T>
    where T: rand::distributions::uniform::SampleUniform + From<u8> + Clone
{
    return random_range::<T>(T::from(0), T::from(1), shape);
}


use array::Array;

use std::ops::{
    Add,
    Sub,
    Div,
    Mul,
    Neg,
    AddAssign,
    SubAssign,
    DivAssign,
    MulAssign,
};

impl <T: Clone> Add for Array<T> {
    type Output = Array<T>;

    fn add(self, other: Array<T>) -> Array<T> {
        return self;
    }
}

impl <T: Clone> AddAssign for Array<T> {
    fn add_assign(&mut self, other: Array<T>) -> () {

    }
}

impl <T: Clone> Sub for Array<T> {
    type Output = Array<T>;

    fn sub(self, other: Array<T>) -> Array<T> {
        return self;
    }
}

impl <T: Clone> SubAssign for Array<T> {
    fn sub_assign(&mut self, other: Array<T>) -> () {

    }
}

impl <T: Clone> Mul for Array<T> {
    type Output = Array<T>;

    fn mul(self, other: Array<T>) -> Array<T> {
        return self;
    }
}

impl <T: Clone> MulAssign for Array<T> {
    fn mul_assign(&mut self, other: Array<T>) -> () {

    }
}

impl <T: Clone> Div for Array<T> {
    type Output = Array<T>;

    fn div(self, other: Array<T>) -> Array<T> {
        return self;
    }
}

impl <T: Clone> DivAssign for Array<T> {
    fn div_assign(&mut self, other: Array<T>) -> () {

    }
}

impl <T: Clone> Neg for Array<T> {
    type Output = Array<T>;

    fn neg(self) -> Array<T> {
        return self;
    }
}
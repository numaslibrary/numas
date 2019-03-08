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
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;

impl <T> Add for Array<T> {
    type Output = Array<T>;

    fn add(self, other: Array<T>) -> Array<T> {
        return self;
    }
}

impl <T> AddAssign for Array<T> {
    fn add_assign(&mut self, other: Array<T>) -> () {

    }
}

impl <T> Sub for Array<T> {
    type Output = Array<T>;

    fn sub(self, other: Array<T>) -> Array<T> {
        return self;
    }
}

impl <T> SubAssign for Array<T> {
    fn sub_assign(&mut self, other: Array<T>) -> () {

    }
}

impl <T> Mul for Array<T> {
    type Output = Array<T>;

    fn add(self, other: Array<T>) -> Array<T> {
        return self;
    }
}

impl <T> MulAssign for Array<T> {
    fn add_assign(&mut self, other: Array<T>) -> () {

    }
}

impl <T> Div for Array<T> {
    type Output = Array<T>;

    fn div(self, other: Array<T>) -> Array<T> {
        return self;
    }
}

impl <T> DivAssign for Array<T> {
    fn div_assign(&mut self, other: Array<T>) -> () {

    }
}

impl <T> Neg for Array<T> {
    type Output = Array<T>;

    fn neg(self) -> Array<T> {
        return self;
    }
}
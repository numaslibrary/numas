#![feature(test)]
extern crate test;
extern crate numas;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use numas::factory::{
        fill,
    };

    use numas::array::Array;

    #[bench]
    fn sin(b: &mut Bencher) {
        let a = fill::full(5, vec![10, 100, 100]);
        b.iter(|| a.sin());
    }

    #[bench]
    fn sum(b: &mut Bencher) {
        let a = fill::full(5, vec![10, 100, 100]);
        b.iter(|| a.sum());
    }

    #[bench]
    fn sqrt(b: &mut Bencher) {
        let a = fill::full(5, vec![10, 100, 100]);
        b.iter(|| a.sqrt());
    }

    #[bench]
    fn round(b: &mut Bencher) {
        let a = fill::full(5, vec![10, 100, 100]);
        b.iter(|| a.round());
    }
}
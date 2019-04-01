#![feature(test)]
extern crate test;
extern crate numas;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use numas::factory::{
        random,
        fill,
    };


    #[bench]
    fn random_fill(b: &mut Bencher) {
        b.iter(|| random::random::<f64>(vec![100,10,10,10,10]));
    }

    #[bench]
    fn random_range(b: &mut Bencher) {
        b.iter(|| random::random_range::<i32>(10, 20, vec![100,10,10,10,10]));
    }

    #[bench]
    fn fill_ones(b: &mut Bencher) {
        b.iter(|| fill::ones::<f64>(vec![100,10,10,10,10]));
    }

    #[bench]
    fn fill_number(b: &mut Bencher) {
        b.iter(|| fill::fill::<i32>(5, vec![100,10,10,10,10]));
    }
}
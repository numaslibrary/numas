#![feature(test)]
extern crate test;
extern crate numas;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use numas::factory::random;


    #[bench]
    fn random_fill(b: &mut Bencher) {
        b.iter(|| random::random::<f64>(vec![100,10,10,10,10]));
    }

    #[bench]
    fn random_range(b: &mut Bencher) {
        b.iter(|| random::random_range::<f64>(10.0, 20.0, vec![100,10,10,10,10]));
    }
}
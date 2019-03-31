#![feature(test)]
extern crate test;
extern crate numas;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use numas::factory::random;


    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| random::random::<f64>(vec![100,10,10,10,10]));
    }
}
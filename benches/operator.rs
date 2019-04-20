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
    fn add(b: &mut Bencher) {
        let a = fill::full(5, vec![100, 10, 10, 10, 10]);
        let c = fill::full(5, vec![100, 10, 10, 10, 10]);
        b.iter(|| &a + &c);
    }

    #[bench]
    fn sub(b: &mut Bencher) {
        let a = fill::full(5, vec![100, 10, 10, 10, 10]);
        let c = fill::full(5, vec![100, 10, 10, 10, 10]);
        b.iter(|| &a - &c);
    }

    #[bench]
    fn mul(b: &mut Bencher) {
        let a = fill::full(5, vec![100, 10, 10, 10, 10]);
        let c = fill::full(5, vec![100, 10, 10, 10, 10]);
        b.iter(|| &a * &c);
    }

    #[bench]
    fn div(b: &mut Bencher) {
        let a = fill::full(10, vec![100, 10, 10, 10, 10]);
        let c = fill::full(5, vec![100, 10, 10, 10, 10]);
        b.iter(|| &a / &c);
    }

    #[bench]
    fn eq(b: &mut Bencher) {
        let a = fill::full(5, vec![100, 10, 10, 10, 10]);
        let c = fill::full(5, vec![100, 10, 10, 10, 10]);
        b.iter(|| a.eq(&c));
    }

    #[bench]
    fn neq(b: &mut Bencher) {
        let a = fill::full(5, vec![100, 10, 10, 10, 10]);
        let c = fill::full(5, vec![100, 10, 10, 10, 10]);
        b.iter(|| a.neq(&c));
    }

    #[bench]
    fn ge(b: &mut Bencher) {
        let a = fill::full(5, vec![100, 10, 10, 10, 10]);
        let c = fill::full(5, vec![100, 10, 10, 10, 10]);
        b.iter(|| a.ge(&c));
    }

    #[bench]
    fn gt(b: &mut Bencher) {
        let a = fill::full(5, vec![100, 10, 10, 10, 10]);
        let c = fill::full(5, vec![100, 10, 10, 10, 10]);
        b.iter(|| a.gt(&c));
    }

    #[bench]
    fn le(b: &mut Bencher) {
        let a = fill::full(5, vec![100, 10, 10, 10, 10]);
        let c = fill::full(5, vec![100, 10, 10, 10, 10]);
        b.iter(|| a.le(&c));
    }

    #[bench]
    fn lt(b: &mut Bencher) {
        let a = fill::full(5, vec![100, 10, 10, 10, 10]);
        let c = fill::full(5, vec![100, 10, 10, 10, 10]);
        b.iter(|| a.lt(&c));
    }
}
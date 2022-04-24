#![feature(test)]
extern crate test;

use bigdecimal::BigDecimal;
use num::bigint::BigInt;
use std::str::FromStr;
use test::Bencher;

#[bench]
fn loop_int(b: &mut Bencher) {
    b.iter(|| {
        let mut x: i64 = 0;
        for i in 10..10000 {
            x += i;
            x /= 3;
        }
        assert_eq!(x, 4999);
    })
}

#[bench]
fn loop_float(b: &mut Bencher) {
    b.iter(|| {
        let mut x: f64 = 0.0;
        for i in 10..10000 {
            x += i as f64;
            x /= 3 as f64;
        }
        assert_eq!(x, 4999.25);
    })
}

#[bench]
fn loop_int_heap(b: &mut Bencher) {
    b.iter(|| {
        let mut x: Box<i64> = Box::new(0);
        for i in 10..10000 {
            *x += i;
            *x /= 3;
        }
        assert_eq!(x, Box::new(4999));
    })
}

#[bench]
fn loop_float_heap(b: &mut Bencher) {
    b.iter(|| {
        let mut x: Box<f64> = Box::new(0.0);
        for i in 10..10000 {
            *x += i as f64;
            *x /= 3 as f64;
        }
        assert_eq!(x, Box::new(4999.25));
    })
}

#[bench]
fn loop_num(b: &mut Bencher) {
    b.iter(|| {
        let mut x = BigInt::from(0);
        for i in 10..10000 {
            x += i;
            x /= 3;
        }
        assert_eq!(x, BigInt::from(4999));
    })
}

#[bench]
fn loop_decimal(b: &mut Bencher) {
    b.iter(|| {
        let mut x = BigDecimal::from_str("0.0").unwrap();
        let div = BigDecimal::from_str("3.0").unwrap();
        for i in 10..10000 {
            x = x + BigDecimal::from(i);
            x = x / div.clone();
        }
        assert_eq!(x, BigDecimal::from_str("4999.25").unwrap());
    })
}

fn assert_delta (delta: f64, x: f64, y: f64) {
    assert! ((x - y) .abs () < delta);
}

fn sum_integers (a: f64, b: f64) -> f64 {
    if a > b {
        0.0
    } else {
        a + sum_integers (a + 1.0, b)
    }
}

#[test]
fn test_sum_integers () {
    assert_eq! (5050.0, sum_integers (0.0, 100.0));
}

fn cube (x: f64) -> f64 {
    x * x * x
}

fn sum_cubes (a: f64, b: f64) -> f64 {
    if a > b {
        0.0
    } else {
        cube (a) + sum_cubes (a + 1.0, b)
    }
}

#[test]
fn test_sum_cubes () {
    assert_eq! (
        sum_cubes (0.0, 3.0),
        cube (1.0) + cube (2.0) + cube (3.0));
    assert_eq! (
        sum_cubes (1.0, 10.0),
        3025.0);
}

fn pi_sum (a: f64, b: f64) -> f64 {
    if a > b {
        0.0
    } else {
        (1.0 / (a * (a + 2.0))) + pi_sum (a + 4.0, b)
    }
}

const PI: f64 = 3.1415926;

#[test]
fn test_pi_sum () {
    assert_delta (
        0.001,
        PI,
        8.0 * pi_sum (1.0, 10000.0));
}

fn inc (a: f64) -> f64 {
    a + 1.0
}

fn identity (x: f64) -> f64 {
    x
}

// we can not use the following
//   because currently equivalence between function types
//   is not handled properly  by rust's type system

// fn sum <F> (
//     term: &F, a: f64,
//     next: &F, b: f64,
// ) -> f64
// where F: Fn (f64) -> f64 {
//     if a > b {
//         0.0
//     } else {
//         term (a) +
//             sum (term, next (a),
//                  next, b)
//     }
// }

fn sum (
    term: &Fn (f64) -> f64, a: f64,
    next: &Fn (f64) -> f64, b: f64,
) -> f64 {
    if a > b {
        0.0
    } else {
        term (a) +
            sum (term, next (a),
                 next, b)
    }
}

mod with_sum {
    use super::{
        assert_delta,
        cube,
        PI,
        inc, identity,
        sum,
    };

    fn sum_cubes (a: f64, b: f64) -> f64 {
        sum (&cube, a, &inc, b)
    }

    #[test]
    fn test_sum_cubes () {
        assert_eq! (
            sum_cubes (0.0, 3.0),
            cube (1.0) + cube (2.0) + cube (3.0));
        assert_eq! (
            sum_cubes (1.0, 10.0),
            3025.0);
    }

    fn sum_integers (a: f64, b: f64) -> f64 {
        sum (&identity, a, &inc, b)
    }

    #[test]
    fn test_sum_integers () {
        assert_eq! (5050.0, sum_integers (0.0, 100.0));
    }

    fn pi_sum (a: f64, b: f64) -> f64 {
        let pi_term = |x| 1.0 / (x * (x + 2.0));
        let pi_next = |x| x + 4.0;
        sum (&pi_term, a, &pi_next, b)
    }

    #[test]
    fn test_pi_sum () {
        assert_delta (
            0.001,
            PI,
            8.0 * pi_sum (1.0, 10000.0));
    }

    fn integral (
        f: & Fn (f64) -> f64,
        a: f64, b: f64,
        dx: f64,
    ) -> f64 {
        let add_dx = |x| x + dx;
        sum (f, (a + (dx / 2.0)), &add_dx, b) * dx
    }

    #[test]
    fn test_integral () {
        assert_delta (
            0.001,
            integral (&cube, 0.0, 1.0, 0.001),
            1.0 / 4.0);
    }
}

fn odd_p (n: f64) -> bool {
    n % 2.0 == 1.0
}

fn even_p (n: f64) -> bool {
    n % 2.0 == 0.0
}

fn simpson_integral (
    f: & Fn (f64) -> f64,
    a: f64, b: f64,
    n: f64,
) -> f64 {
    let h = (b - a) / n;
    let simpson_coefficient = |k| -> f64 {
        if 0.0 == k || n == k {
            1.0
        } else if odd_p (k) {
            4.0
        } else {
            assert! (even_p (k));
            2.0
        }
    };
    let simpson_term = |k| -> f64 {
        simpson_coefficient (k) *
            f (a + (k * h))
    };
    sum (&simpson_term, 0.0, &inc, n) *
        (h / 3.0)
}

#[test]
fn test_simpson_term () {
    assert_delta (
        0.0001,
        simpson_integral (&cube, 0.0, 1.0, 100.0),
        1.0 / 4.0)
}

mod sum_iter {
    use super::{
        inc,
        cube,
    };

    // rust does not have letrec for now

    // fn sum (
    //     term: & Fn (f64) -> f64, a: f64,
    //     next: & Fn (f64) -> f64, b: f64,
    // ) -> f64 {
    //     letrec sum_iter = |a, result| {
    //         if a > b {
    //             result
    //         } else {
    //             sum_iter (next (a), term (a) + result)
    //         }
    //     };
    //     sum_iter (a, 0.0)
    // }

    fn sum_iter (
        term: & Fn (f64) -> f64, a: f64,
        next: & Fn (f64) -> f64, b: f64,
        result: f64,
    ) -> f64 {
        if a > b {
            result
        } else {
            sum_iter (
                term, next (a),
                next, b,
                term (a) + result)
        }
    }

    fn sum (
        term: & Fn (f64) -> f64, a: f64,
        next: & Fn (f64) -> f64, b: f64,
    ) -> f64 {
        sum_iter (term, a, next, b, 0.0)
    }

    fn sum_cubes (a: f64, b: f64) -> f64 {
        sum (&cube, a, &inc, b)
    }

    #[test]
    fn test_sum_cubes () {
        assert_eq! (
            sum_cubes (0.0, 3.0),
            cube (1.0) + cube (2.0) + cube (3.0));
        assert_eq! (
            sum_cubes (1.0, 10.0),
            3025.0);
    }
}

fn product (
    term: & Fn (f64) -> f64, a: f64,
    next: & Fn (f64) -> f64, b: f64,
) -> f64 {
    if a > b {
        1.0
    } else {
        term (a) *
            product (term, next (a),
                     next, b)
    }
}

fn wallis_product (a: f64, b: f64) -> f64 {
    let wallis_term = |x| {
        (((2.0 * x) + 0.0) * ((2.0 * x) + 2.0)) /
            (((2.0 * x) + 1.0) * ((2.0 * x) + 1.0))
    };
    product (&wallis_term, a, &inc, b)
}

#[test]
fn test_wallis_product () {
    assert_delta (
        0.001,
        wallis_product (1.0, 1000.0),
        PI / 4.0);
}

fn accumulate (
    combiner: &Fn (f64, f64) -> f64,
    null_value: f64,
    term: &Fn (f64) -> f64, a: f64,
    next: &Fn (f64) -> f64, b: f64,
) -> f64 {
    if a > b {
        null_value
    } else {
        combiner (term (a),
                  accumulate (
                      combiner, null_value,
                      term, next (a), next, b))
    }
}

fn add (x: f64, y: f64) -> f64 {
    x + y
}

fn mul (x: f64, y: f64) -> f64 {
    x * y
}

mod accumulate {
    use super::{
        assert_delta,
        inc,
        accumulate,
        add, mul,
        PI,
    };

    fn sum (
        term: &Fn (f64) -> f64, a: f64,
        next: &Fn (f64) -> f64, b: f64,
    ) -> f64 {
        accumulate (&add, 0.0, term, a, next, b)
    }

    fn product (
        term: &Fn (f64) -> f64, a: f64,
        next: &Fn (f64) -> f64, b: f64,
    ) -> f64 {
        accumulate (&mul, 1.0, term, a, next, b)
    }

    fn pi_sum (a: f64, b: f64) -> f64 {
        let pi_term = |x| 1.0 / (x * (x + 2.0));
        let pi_next = |x| x + 4.0;
        sum (&pi_term, a, &pi_next, b)
    }

    #[test]
    fn test_pi_sum () {
        assert_delta (
            0.001,
            PI,
            8.0 * pi_sum (1.0, 10000.0));
    }

    fn wallis_product (a: f64, b: f64) -> f64 {
        let wallis_term = |x| {
            (((2.0 * x) + 0.0) * ((2.0 * x) + 2.0)) /
                (((2.0 * x) + 1.0) * ((2.0 * x) + 1.0))
        };
        product (&wallis_term, a, &inc, b)
    }

    #[test]
    fn test_wallis_product () {
        assert_delta (
            0.001,
            wallis_product (1.0, 1000.0),
            PI / 4.0);
    }
}

fn filtered_accumulate (
    combiner: &Fn (f64, f64) -> f64,
    null_value: f64,
    term: &Fn (f64) -> f64, a: f64,
    next: &Fn (f64) -> f64, b: f64,
    filter: &Fn (f64) -> bool,
) -> f64 {
    if a > b {
        null_value
    } else if filter (a) {
        combiner (term (a),
                  filtered_accumulate (
                      combiner, null_value,
                      term, next (a), next, b,
                      filter))
    } else {
        filtered_accumulate (
            combiner, null_value,
            term, next (a), next, b,
            filter)
    }
}

use crate::ch_1_2::prime_p;

fn sum_prime_square (a: f64, b: f64) -> f64 {
    filtered_accumulate (
        &add, 0.0,
        &identity, a, &inc, b,
        &prime_p)
}

#[test]
fn test_sum_prime_square () {
    assert_eq! (sum_prime_square (0.0, 10.0),
                (1.0 + 2.0 + 3.0 + 5.0 + 7.0))
}

mod lambda {
    use super::{
        assert_delta,
        cube,
        PI,
        sum,
    };
    fn pi_sum (a: f64, b: f64) -> f64 {
        sum (&|x| 1.0 / (x * (x + 2.0)), a,
             &|x| x + 4.0, b)
    }

    #[test]
    fn test_pi_sum () {
        assert_delta (
            0.001,
            PI,
            8.0 * pi_sum (1.0, 10000.0));
    }

    fn integral (
        f: & Fn (f64) -> f64,
        a: f64, b: f64,
        dx: f64,
    ) -> f64 {
        sum (f, (a + (dx / 2.0)), &|x| x + dx, b) * dx
    }

    #[test]
    fn test_integral () {
        assert_delta (
            0.001,
            integral (&cube, 0.0, 1.0, 0.001),
            1.0 / 4.0);
    }
}

fn average (x: f64, y: f64) -> f64 {
    (x + y) / 2.0
}

fn positive_p (x: f64) -> bool {
    x > 0.0
}

fn negative_p (x: f64) -> bool {
    x < 0.0
}

fn search (
    f: &Fn (f64) -> f64,
    neg_point: f64,
    pos_point: f64,
) -> f64 {
    let midpoint = average (neg_point, pos_point);
    if close_enough_p (neg_point, pos_point) {
        midpoint
    } else {
        let test_value = f (midpoint);
        if positive_p (test_value) {
            search (f, neg_point, midpoint)
        } else if negative_p (test_value) {
            search (f, midpoint, pos_point)
        } else {
            midpoint
        }
    }
}

fn close_enough_p (x: f64,  y: f64) -> bool {
    (x - y) .abs () < 0.001
}

fn half_interval_method (
    f: &Fn (f64) -> f64,
    a: f64,
    b: f64,
) -> Result <f64, String> {
    let a_value = f (a);
    let b_value = f (b);
    if negative_p (a_value) && positive_p (b_value) {
        Ok (search (f, a, b))
    } else if negative_p (b_value) && positive_p (a_value) {
        Ok (search (f, b, a))
    } else {;
        Err (format! (
            "values are not of opposite sign, \
             a : {}, b : {}", a, b))
    }
}

#[test]
fn test_half_interval_method () {
    assert_delta (
        0.01,
        half_interval_method (
            &|x| x.sin (),
            2.0, 4.0) .unwrap (),
        3.14);
    assert_delta (
        0.01,
        half_interval_method (
            &|x| (x - 1.0) * (x - 3.0),
            0.0, 2.0) .unwrap (),
        1.0);
}

const TOLERANCE: f64 = 0.00001;

fn fixed_point (
    f: &Fn (f64) -> f64,
    first_guess: f64,
) -> f64 {
    fn close_enough_p (v1: f64, v2: f64) -> bool {
      (v1 - v2) .abs () < TOLERANCE
    }
    // rust does not have letrec for now
    fn try_guess (
        f: &Fn (f64) -> f64,
        guess: f64,
    ) -> f64 {
        let next = f (guess);
        if close_enough_p (guess, next) {
            next
        } else {
            try_guess (f, next)
        }
    }
    try_guess (f, first_guess)
}

#[test]
fn test_fixed_point () {
    println! ("{}", fixed_point (&|x| x.cos (), 1.0));
    println! ("{}", fixed_point (&|y| y.sin () + y.cos (), 1.0));
}

fn sqrt (x: f64) -> f64 {
    // fixed_point (&|y| x / y, x)
    fixed_point (&|y| average (y, x / y), x)
}

#[test]
fn test_sqrt () {
    println! ("{}", sqrt (4.0));
}

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
    term: &impl Fn (f64) -> f64, a: f64,
    next: &impl Fn (f64) -> f64, b: f64,
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
        inc, sum,
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

    fn identity (x: f64) -> f64 {
        x
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
        f: &impl Fn (f64) -> f64,
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
    f: &impl Fn (f64) -> f64,
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
    //     term: &impl Fn (f64) -> f64, a: f64,
    //     next: &impl Fn (f64) -> f64, b: f64,
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
        term: &impl Fn (f64) -> f64, a: f64,
        next: &impl Fn (f64) -> f64, b: f64,
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
        term: &impl Fn (f64) -> f64, a: f64,
        next: &impl Fn (f64) -> f64, b: f64,
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
    term: &impl Fn (f64) -> f64, a: f64,
    next: &impl Fn (f64) -> f64, b: f64,
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

//  (= (accumulate combiner null_value
//                 term a next b)
//     (if (gt a b)
//       null_value
//       (combiner (term a)
//                 (accumulate combiner null_value
//                             term (next a) next b))))

//  (= (sum term a next b)
//     (accumulate add 0 term a next b))

//  (= (product term a next b)
//     (accumulate mul 1 term a next b))

//  (= (pi_sum a b)
//     (= (pi_term x) (div 1 (mul x (add x 2))))
//     (= (pi_next x) (add x 4))
//     (sum pi_term a pi_next b))

//  (assert_delta 0.001
//    pi
//    (mul 8 (pi_sum 1 10000)))

//  (= (wallis_product a b)
//     (= (wallis_term x)
//        (div (mul (add (mul 2 x) 0) (add (mul 2 x) 2))
//             (mul (add (mul 2 x) 1) (add (mul 2 x) 1))))
//     (product wallis_term a inc b))

//  (assert_delta 0.001
//    (wallis_product 1 1000)
//    (div pi 4))

//  (= (filtered_accumulate
//      combiner null_value
//      term a next b
//      filter)
//     (if (gt a b)
//       null_value
//       (if (filter a)
//         (combiner (term a)
//                   (filtered_accumulate
//                    combiner null_value
//                    term (next a) next b
//                    filter))
//         (filtered_accumulate
//          combiner null_value
//          term (next a) next b
//          filter))))

//  (= section_1_2
//     (load "1_2_procedures_and_the_processes_they_generate.jo"))

//  (= prime_p section_1_2.prime_p)

//  (= (sum_prime_square a b)
//     (filtered_accumulate
//      add 0
//      identity a inc b
//      prime_p))

//  (assert (eq (sum_prime_square 0 10)
//              (add 1 (add 2 (add 3 (add 5 7))))))

//  (= (pi_sum a b)
//     (sum (lambda [x] (div 1 (mul x (add x 2))))
//          a
//          (lambda [x] (add x 4))
//          b))

//  (assert_delta 0.001
//    pi (mul 8 (pi_sum 1 10000)))

//  (= (integral f a b dx)
//     (mul (sum f (add a (div dx 2))
//               (lambda [x] (add x dx))
//               b)
//          dx))

//  (assert_delta 0.001
//    (integral cube 0 1 0.001)
//    (div 1 4))

//  (= (average x y) (div (add x y) 2))

//  (= (positive_p x) (gt x 0))
//  (= (negative_p x) (lt x 0))

//  (= (search f neg_point pos_point)
//     (= midpoint (average neg_point pos_point))
//     (if (close_enough_p neg_point pos_point)
//       midpoint
//       (let [(test_value (f midpoint))]
//         (cond
//           [(positive_p test_value)
//            (search f neg_point midpoint)]
//           [(negative_p test_value)
//            (search f midpoint pos_point)]
//           [else midpoint]))))

//  (= (close_enough_p x y)
//     (lt (abs (sub x y)) 0.001))

//  (= (half_interval_method f a b)
//     (let [(a_value (f a))
//           (b_value (f b))]
//       (cond [(and (negative_p a_value)
//                   (positive_p b_value))
//              (search f a b)]
//             [(and (negative_p b_value)
//                   (positive_p a_value))
//              (search f b a)]
//             [else
//              (println "- half_interval_method")
//              (println "  values are not of opposite sign")
//              (print "  a : ") (println a)
//              (print "  b : ") (println b)])))

//  (assert_delta 0.01
//    (half_interval_method num_sin 2 4)
//    3.14)

//  (assert_delta 0.01
//    (half_interval_method
//     (lambda [x] (mul (sub x 1) (sub x 3)))
//     0
//     2)
//    1)

//  (= tolerance 0.00001)

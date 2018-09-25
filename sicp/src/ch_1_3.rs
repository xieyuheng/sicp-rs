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

mod with_sum {
    use super::cube;

    // fn sum <F> (
    //     term: F, a: f64,
    //     next: F, b: f64,
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

    // error[E0308]: mismatched types
    //   --> sicp/src/ch_1_3.rs:90:23
    //    |
    // 90 |         sum (cube, a, inc, b)
    //    |                       ^^^ expected fn item, found a different fn item
    //    |
    //    = note: expected type `fn(f64) -> f64 {ch_1_3::cube}`
    //               found type `fn(f64) -> f64 {ch_1_3::with_sum::inc}`

    fn sum (
        term: impl Fn (f64) -> f64, a: f64,
        next: impl Fn (f64) -> f64, b: f64,
    ) -> f64 {
        if a > b {
            0.0
        } else {
            term (a) +
                sum (term, next (a),
                     next, b)
        }
    }

    fn inc (a: f64) -> f64 {
        a + 1.0
    }

    fn sum_cubes (a: f64, b: f64) -> f64 {
        sum (cube, a, inc, b)
    }

    #[test]
    fn test_sum_cubes () {
        assert_eq! (
            sum_cubes (0.0, 3.0),
            cube (1.0) + cube (2.0) + cube (3.0));
    }

    // (= (sum_cubes a b)
    //    (sum cube a inc b))

    // (assert (eq (sum_cubes 0 3)
    //             (add (cube 1)
    //                  (add (cube 2)
    //                       (cube 3)))))

    // (assert (eq (sum_cubes 1 10)
    //             3025))

    // (= (identity x) x)

    // (= (sum_integers a b)
    //    (sum identity a inc b))

    // (assert (eq 5050 (sum_integers 0 100)))

    // (= (pi_sum a b)
    //    (= (pi_term x) (div 1 (mul x (add x 2))))
    //    (= (pi_next x) (add x 4))
    //    (sum pi_term a pi_next b))


    // (assert_delta 0.001
    //   pi (mul 8 (pi_sum 1 10000)))

    // (= (integral f a b dx)
    //    (= (add_dx x) (add x dx))
    //    (mul (sum f (add a (div dx 2)) add_dx b)
    //         dx))

    // (assert_delta 0.001
    //   (integral cube 0 1 0.001)
    //   (div 1 4))
}

//  (= (simpson_integral f a b n)
//     (= h (div (sub b a) n))
//     (= (simpson_coefficient k)
//        (cond [(or (eq 0 k) (eq n k)) 1]
//              [(odd_p k) 4]
//              [(even_p k) 2]))
//     (= (simpson_term k)
//        (mul (simpson_coefficient k)
//             (f (add a (mul k h)))))
//     (mul (sum simpson_term 0 inc n)
//          (div h 3)))

//  (assert_delta 0.0001
//    (simpson_integral cube 0 1 100)
//    (div 1 4))

//  (note we do not have letrec
//    (= (sum term a next b)
//       (= (iter a result)
//          (if (gt a b)
//            result
//            (iter (next a) (add (term a) result))))
//       (iter a 0)))

//  (= (sum_iter term a next b result)
//     (if (gt a b)
//       result
//       (sum_iter term (next a) next b (add (term a) result))))

//  (= (sum term a next b)
//     (sum_iter term a next b 0))

//  (= (sum_cubes a b)
//     (sum cube a inc b))

//  (assert (eq (sum_cubes 0 3)
//              (add (cube 1)
//                   (add (cube 2)
//                        (cube 3)))))

//  (assert (eq (sum_cubes 1 10)
//              3025))

//  (= (product term a next b)
//     (if (gt a b)
//       1
//       (mul (term a)
//            (product term (next a) next b))))

//  (= (wallis_product a b)
//     (= (wallis_term x)
//        (div (mul (add (mul 2 x) 0) (add (mul 2 x) 2))
//             (mul (add (mul 2 x) 1) (add (mul 2 x) 1))))
//     (product wallis_term a inc b))

//  (assert_delta 0.001
//    (wallis_product 1 1000)
//    (div pi 4))

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

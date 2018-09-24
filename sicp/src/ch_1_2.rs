fn factorial (n: f64) -> f64 {
    if n <= 1.0 {
        1.0
    } else {
        n * factorial (n - 1.0)
    }
}

#[test]
fn test_factorial () {
    assert_eq! (factorial (0.0), 1.0);
    assert_eq! (factorial (1.0), 1.0);
    assert_eq! (factorial (2.0), 2.0);
    assert_eq! (factorial (3.0), 6.0);
    assert_eq! (factorial (4.0), 24.0);
    assert_eq! (factorial (5.0), 120.0);
    assert_eq! (factorial (6.0), 720.0);
}

mod fact_iter {
    fn factorial (n: f64) -> f64 {
        fact_iter (1.0, 1.0, n)
    }

    fn fact_iter (product: f64, counter: f64, max_count: f64) -> f64 {
        if counter > max_count {
            product
        } else {
            fact_iter (
                counter * product,
                counter + 1.0,
                max_count)
        }
    }

    #[test]
    fn test_factorial () {
        assert_eq! (factorial (0.0), 1.0);
        assert_eq! (factorial (1.0), 1.0);
        assert_eq! (factorial (2.0), 2.0);
        assert_eq! (factorial (3.0), 6.0);
        assert_eq! (factorial (4.0), 24.0);
        assert_eq! (factorial (5.0), 120.0);
        assert_eq! (factorial (6.0), 720.0);
    }
}

fn ackermann (x: f64, y: f64) -> f64 {

    if y == 0.0 {
        0.0
    } else if x == 0.0 {
        2.0 * y
    } else if y == 1.0 {
        2.0
    } else {
        ackermann (
            x - 1.0,
            ackermann (x, y - 1.0))
    }
}

#[test]
fn test_ackermann () {
    println! ("{}", ackermann (1.0, 10.0));
    println! ("{}", ackermann (2.0, 4.0));
    println! ("{}", ackermann (3.0, 3.0));
}

fn fib (n: f64) -> f64 {
    if n == 0.0 {
        0.0
    } else if n == 1.0 {
        1.0
    } else {
        fib (n - 1.0) + fib (n - 2.0)
    }
}

#[test]
fn test_fib () {
    assert_eq! (fib (10.0), 55.0);
}

mod fib_iter {
    fn fib (n: f64) -> f64 {
        fib_iter (1.0, 0.0, n)
    }

    fn fib_iter (a: f64, b: f64, count: f64) -> f64 {
        if count == 0.0 {
            b
        } else {
            fib_iter (a + b, a, count - 1.0)
        }
    }

    #[test]
    fn test_fib () {
        assert_eq! (fib (10.0), 55.0);
    }
}

fn expt (b: f64, n: f64) -> f64 {
    if n == 0.0 {
        1.0
    } else {
        b * expt (b, n - 1.0)
    }
}

#[test]
fn test_expt () {
    assert_eq! (expt (2.0, 10.0), 1024.0);
}

mod expt_iter {
    fn expt (b: f64, n: f64) -> f64 {
        expt_iter (b, n, 1.0)
    }

    fn expt_iter (b: f64, counter: f64, product: f64) -> f64 {
        if counter == 0.0 {
            product
        } else {
            expt_iter (b, counter - 1.0, b * product)
        }
    }

    #[test]
    fn test_expt () {
        assert_eq! (expt (2.0, 10.0), 1024.0);
    }
}


fn even_p (n: f64) -> bool {
    (n % 2.0) == 0.0
}

fn square (n: f64) -> f64 {
    n * n
}

// (= (fast_expt b n)
//    (cond [(eq n 0) 1]
//          [(even_p n) (square (fast_expt b (div n 2)))]
//          [else (mul b (fast_expt b (sub n 1)))]))

// (fast_expt 2 10)

// (= (fast_expt b n) (fast_expt_iter 1 b n))

// (= (fast_expt_iter a b n)
//    (cond [(eq n 0) a]
//          [(even_p n)
//           (fast_expt_iter a (square b) (div n 2))]
//          [else
//           (fast_expt_iter (mul a b) b (sub n 1))]))

// (fast_expt 2 10)

// (note Exercise 1.19

//   (note
//     (= (T p q)
//        a <- b q + a q + a p
//        b <- b p + a q)
//     (compose (T p q) (T p q)) = (T (p p + q q) (2 p q + q q)))

//   (fast_fib n) = (fast_fib_iter 1 0 0 1 n)

//   (= (fast_fib_iter a b p q n)
//      (cond [(eq n 0) b]
//            [(even_p n)
//             (fast_fib_iter
//              a b
//              (add (mul p p) (mul q q))
//              (add (mul 2 (mul p q)) (mul q q))
//              (div n 2))]
//            [else
//             (fast_fib_iter
//              (add (mul b q) (add (mul a q) (mul a p)))
//              (add (mul b p) (mul a q))
//              p q
//              (sub n 1))]))

//   (fast_fib 10))

// (= (gcd a b)
//    (if (eq b 0)
//      a
//      (gcd b (mod a b))))

// (gcd 206 40)

// (= (smallest_divisor n)
//    (find_divisor n 2))

// (= (find_divisor n test_divisor)
//    (cond [(gt (square test_divisor) n) n]
//          [(eq 0 (mod n test_divisor)) test_divisor]
//          [else (find_divisor n (add test_divisor 1))]))

// (assert (eq (smallest_divisor 123) 3))
// (assert (eq (smallest_divisor 121) 11))

// (= (prime_p n)
//    (eq n (smallest_divisor n)))

// (assert (not (prime_p 121)))
// (assert (prime_p 11))

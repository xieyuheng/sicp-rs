extern crate test;

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

fn fast_expt (b: f64, n: f64) -> f64 {
    if n == 0.0 {
        1.0
    } else if even_p (n) {
        square (fast_expt (b, n / 2.0))
    } else {
        b * fast_expt (b, n - 1.0)
    }
}

#[test]
fn test_fast_expt () {
    assert_eq! (fast_expt (2.0, 10.0), 1024.0);
}

mod fast_expt_iter {
    use super::{
        even_p,
        square,
    };

    fn fast_expt (b: f64, n: f64) -> f64 {
        fast_expt_iter (1.0, b, n)
    }

    fn fast_expt_iter (a: f64, b: f64, n: f64) -> f64 {
        if n == 0.0 {
            a
        } else if even_p (n) {
            fast_expt_iter (a, square (b), n / 2.0)
        } else {
            fast_expt_iter (a * b, b, n - 1.0)
        }
    }

    #[test]
    fn test_fast_expt () {
        assert_eq! (fast_expt (2.0, 10.0), 1024.0);
    }
}


// Exercise 1.19
//   (= (T p q)
//      a <- b q + a q + a p
//      b <- b p + a q)
//   (compose (T p q) (T p q)) = (T (p p + q q) (2 p q + q q))

fn fast_fib (n: f64) -> f64 {
    fast_fib_iter (1.0, 0.0, 0.0, 1.0, n)
}

fn fast_fib_iter (a: f64, b: f64, p: f64, q: f64, n: f64) -> f64 {
    if n == 0.0 {
        b
    } else if even_p (n) {
        fast_fib_iter (
            a, b,
            (p * p) + (q * q),
            (2.0 * (p * q)) + (q * q),
            n / 2.0)
    } else {
        fast_fib_iter (
            (b * q) + (a * q) + (a * p),
            (b * p) + (a * q),
            p, q,
            n - 1.0)
    }
}

#[test]
fn test_fast_fib () {
    assert_eq! (fast_fib (10.0), 55.0);
}

fn gcd (a: f64, b: f64) -> f64 {
    if b == 0.0 {
        a
    } else {
        gcd (b, a % b)
    }
}

#[test]
fn test_gcd () {
    assert_eq! (gcd (206.0, 40.0), 2.0);
    assert_eq! (gcd (13.0, 5.0), 1.0);
}

fn smallest_divisor (n: f64) -> f64 {
    find_divisor (n, 2.0)
}

fn find_divisor (n: f64, test_divisor: f64) -> f64 {
    if square (test_divisor) > n {
        n
    } else if 0.0 == n % test_divisor {
        test_divisor
    } else {
        find_divisor (n, test_divisor + 1.0)
    }
}

#[test]
fn test_smallest_divisor () {
    assert_eq! (smallest_divisor (123.0), 3.0);
    assert_eq! (smallest_divisor (121.0), 11.0);
}

fn prime_p (n: f64) -> bool {
    n == smallest_divisor (n)
}

#[test]
fn test_prime_p () {
    assert! (! prime_p (121.0));
    assert! (prime_p (11.0));
}

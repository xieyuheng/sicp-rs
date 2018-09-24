fn square (x: f64) -> f64 {
    x * x
}

fn average (x: f64, y: f64) -> f64 {
    (x + y) / 2.0
}

fn sqrt_iter (guess: f64, x: f64) -> f64 {
    if good_enough_p (guess, x) {
        guess
    } else {
        sqrt_iter (improve (guess, x), x)
    }
}

fn improve (guess: f64, x: f64) -> f64 {
    average (guess, x / guess)
}

fn good_enough_p (guess: f64, x: f64) -> bool {
    (square (guess) - x) .abs () < 0.001
}

fn sqrt (x: f64) -> f64 {
    sqrt_iter (1.0, x)
}

#[test]
fn test_sqrt_iter () {
    assert! ((sqrt (2.0) - 1.414).abs () < 0.001);
    assert! ((sqrt (9.0) - 3.0).abs () < 0.001);
    assert! ((sqrt (100.0) - 10.0).abs () < 0.001);
    assert! ((square (sqrt (1000.0)) - 1000.0).abs () < 0.001);
}

mod black_box {
    use super::{
        square,
        average,
    };

    fn sqrt_iter (guess: f64, x: f64) -> f64 {
        let improve = |guess| -> f64 {
            average (guess, x / guess)
        };
        let good_enough_p = |guess| -> bool {
            (square (guess) - x) .abs () < 0.001
        };
        if good_enough_p (guess) {
            guess
        } else {
            sqrt_iter (improve (guess), x)
        }
    }

    fn sqrt (x: f64) -> f64 {
        sqrt_iter (1.0, x)
    }

    #[test]
    fn test_sqrt_iter () {
        assert! ((sqrt (2.0) - 1.414).abs () < 0.001);
        assert! ((sqrt (9.0) - 3.0).abs () < 0.001);
        assert! ((sqrt (100.0) - 10.0).abs () < 0.001);
        assert! ((square (sqrt (1000.0)) - 1000.0).abs () < 0.001);
    }
}

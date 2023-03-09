use crate::frp::*;
use uuid::Uuid;

#[test]
fn frp() {
    use std::sync::Arc;
    use std::sync::Mutex;

    enum Button {
        Push,
    }

    enum Counter {
        Increment,
        Decrement,
        Reset,
    }

    let minus_button: Stream<Button> = Stream::new();
    let plus_button: Stream<Button> = Stream::new();
    let reset_button: Stream<Button> = Stream::new();

    let minus = minus_button.map(|_| Counter::Decrement);
    let plus = plus_button.map(|_| Counter::Increment);
    let reset = reset_button.map(|_| Counter::Reset);

    let label = minus
        .merge(&plus)
        .merge(&reset)
        .fold(0, |x, sig| match *sig {
            Counter::Increment => x + 1,
            Counter::Decrement => x - 1,
            Counter::Reset => 0,
        })
        .filter(|x| x % 2 == 0); // get only even counters

    let counter = Arc::new(Mutex::new(0));
    let counter_ = counter.clone();

    let handle_id = Uuid::new_v4().as_u128();
    let f = move |x: &i32| {
        let mut guard = counter_.lock().unwrap();
        *guard = x.clone();
    };
    label.observe_with_handle(f, handle_id);

    {
        let guard = counter.lock().unwrap();
        assert_eq!(*guard, 0);
    }

    {
        minus_button.send(&Button::Push);
        let guard = counter.lock().unwrap();
        assert_eq!(*guard, 0); // not -1 because -1 is odd
    }

    {
        minus_button.send(&Button::Push);
        let guard = counter.lock().unwrap();
        assert_eq!(*guard, -2);
    }

    {
        plus_button.send(&Button::Push);
        let guard = counter.lock().unwrap();
        assert_eq!(*guard, -2); // not -1, it’s odd
    }

    {
        reset_button.send(&Button::Push);
        let guard = counter.lock().unwrap();
        assert_eq!(*guard, 0);
    }
}

#[test]
fn mutual_recursion() {
    use std::sync::Arc;
    use std::sync::Mutex;

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum Button {
        ChangeLabel(String),
        Push,
    }

    fn f(sig: &Button) -> Option<Button> {
        match *sig {
            Button::Push => Some(Button::ChangeLabel("foo".to_owned())),
            _ => None,
        }
    }

    let (x, y) = Stream::entangled(f, f);

    let x_ref = Arc::new(Mutex::new(Button::Push));
    let x_ref_ = x_ref.clone();

    let y_ref = Arc::new(Mutex::new(Button::Push));
    let y_ref_ = y_ref.clone();

    x.observe(move |a| {
        let mut guard = x_ref_.lock().unwrap();
        *guard = a.clone()
    });
    y.observe(move |a| {
        let mut guard = y_ref_.lock().unwrap();
        *guard = a.clone()
    });

    {
        let x_ref_ = x_ref.lock().unwrap();
        assert_eq!(*x_ref_, Button::Push);
        let y_ref_ = y_ref.lock().unwrap();
        assert_eq!(*y_ref_, Button::Push);
    }

    x.send(&Button::Push);

    {
        let x_ref_ = x_ref.lock().unwrap();
        assert_eq!(*x_ref_, Button::Push);
        let y_ref_ = y_ref.lock().unwrap();
        assert_eq!(*y_ref_, Button::ChangeLabel("foo".to_owned()));
    }

    y.send(&Button::Push);

    {
        let x_ref_ = x_ref.lock().unwrap();
        assert_eq!(*x_ref_, Button::ChangeLabel("foo".to_owned()));
        let y_ref_ = y_ref.lock().unwrap();
        assert_eq!(*y_ref_, Button::Push);
    }
}

#[test]
fn zip() {
    use std::sync::Arc;
    use std::sync::Mutex;

    let x = Stream::new();
    let y = Stream::new();
    let z = x.zip(&y);

    let z_ref = Arc::new(Mutex::new(Either::Right(3)));
    let z_ref_ = z_ref.clone();

    z.observe(move |a| {
        let mut z_ref__ = z_ref_.lock().unwrap();
        *z_ref__ = a.clone()
    });

    {
        let z_ref_ = z_ref.lock().unwrap();
        assert_eq!(*z_ref_, Either::Right(3));
    }

    x.send(&false);

    {
        let z_ref_ = z_ref.lock().unwrap();
        assert_eq!(*z_ref_, Either::Left(false));
    }

    y.send(&42);

    {
        let z_ref_ = z_ref.lock().unwrap();
        assert_eq!(*z_ref_, Either::Right(42));
    }
}

#[test]
fn unzip() {
    use std::sync::Arc;
    use std::sync::Mutex;

    let tuple = Stream::new();
    let (x, y) = tuple.unzip();

    let x_ref = Arc::new(Mutex::new(0));
    let x_ref_ = x_ref.clone();

    let y_ref = Arc::new(Mutex::new(0));
    let y_ref_ = y_ref.clone();

    x.observe(move |a| {
        let mut x_ref__ = x_ref_.lock().unwrap();
        *x_ref__ = *a
    });
    y.observe(move |a| {
        let mut y_ref__ = y_ref_.lock().unwrap();
        *y_ref__ = *a
    });

    {
        let x_ref_ = x_ref.lock().unwrap();
        assert_eq!(*x_ref_, 0);
    }
    {
        let y_ref_ = y_ref.lock().unwrap();
        assert_eq!(*y_ref_, 0);
    }

    tuple.send(&Either::Left(34));
    tuple.send(&Either::Right(13));

    {
        let x_ref_ = x_ref.lock().unwrap();
        assert_eq!(*x_ref_, 34);
    }
    {
        let y_ref_ = y_ref.lock().unwrap();
        assert_eq!(*y_ref_, 13);
    }
}

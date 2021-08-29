use crate::bidule::*;

#[test]
fn frp() {
    use std::cell::RefCell;
    use std::rc::Rc;

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

    // FIXME: this part is ugly, we need to find a better API for “inspecting” streams
    let counter = Rc::new(RefCell::new(0));
    let counter_ = counter.clone();
    label.observe(move |x| *counter_.borrow_mut() = *x);

    assert_eq!(*counter.borrow(), 0);

    minus_button.send(&Button::Push);
    assert_eq!(*counter.borrow(), 0); // not -1 because -1 is odd

    minus_button.send(&Button::Push);
    assert_eq!(*counter.borrow(), -2);

    plus_button.send(&Button::Push);
    assert_eq!(*counter.borrow(), -2); // not -1, it’s odd

    reset_button.send(&Button::Push);
    assert_eq!(*counter.borrow(), 0);
}

#[test]
fn mutual_recursion() {
    use std::cell::RefCell;
    use std::rc::Rc;

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

    let x_ref = Rc::new(RefCell::new(Button::Push));
    let x_ref_ = x_ref.clone();

    let y_ref = Rc::new(RefCell::new(Button::Push));
    let y_ref_ = y_ref.clone();

    x.observe(move |a| *x_ref_.borrow_mut() = a.clone());
    y.observe(move |a| *y_ref_.borrow_mut() = a.clone());

    assert_eq!(*x_ref.borrow(), Button::Push);
    assert_eq!(*y_ref.borrow(), Button::Push);

    x.send(&Button::Push);
    assert_eq!(*x_ref.borrow(), Button::Push);
    assert_eq!(*y_ref.borrow(), Button::ChangeLabel("foo".to_owned()));

    y.send(&Button::Push);
    assert_eq!(*x_ref.borrow(), Button::ChangeLabel("foo".to_owned()));
    assert_eq!(*y_ref.borrow(), Button::Push);
}

#[test]
fn zip() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let x = Stream::new();
    let y = Stream::new();
    let z = x.zip(&y);

    let z_ref = Rc::new(RefCell::new(Either::Right(3)));
    let z_ref_ = z_ref.clone();

    z.observe(move |a| *z_ref_.borrow_mut() = a.clone());

    assert_eq!(*z_ref.borrow(), Either::Right(3));

    x.send(&false);
    assert_eq!(*z_ref.borrow(), Either::Left(false));

    y.send(&42);
    assert_eq!(*z_ref.borrow(), Either::Right(42));
}

#[test]
fn unzip() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let tuple = Stream::new();
    let (x, y) = tuple.unzip();

    let x_ref = Rc::new(RefCell::new(0));
    let x_ref_ = x_ref.clone();

    let y_ref = Rc::new(RefCell::new(0));
    let y_ref_ = y_ref.clone();

    x.observe(move |a| *x_ref_.borrow_mut() = *a);
    y.observe(move |a| *y_ref_.borrow_mut() = *a);

    assert_eq!(*x_ref.borrow(), 0);
    assert_eq!(*y_ref.borrow(), 0);

    tuple.send(&Either::Left(34));
    tuple.send(&Either::Right(13));
    assert_eq!(*x_ref.borrow(), 34);
    assert_eq!(*y_ref.borrow(), 13);
}

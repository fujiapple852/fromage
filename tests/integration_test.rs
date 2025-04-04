use fromage::{Fromage, Intoage, TryFromage, TryIntoage};
use non_local::{Bar, Foo};

struct X;

impl Fromage<Foo, X> for Bar {
    fn fromage(_value: Foo) -> Self {
        Bar
    }
}

impl Fromage<String, X> for usize {
    fn fromage(value: String) -> Self {
        value.len()
    }
}

impl TryFromage<Foo, X> for Bar {
    type Error = ();

    fn try_fromage(_value: Foo) -> Result<Self, Self::Error> {
        Ok(Bar)
    }
}

impl TryFromage<String, X> for usize {
    type Error = ();

    fn try_fromage(value: String) -> Result<Self, Self::Error> {
        Ok(value.len())
    }
}

#[test]
fn test_fromage() {
    assert_eq!(Bar, Bar::fromage(Foo));
    assert_eq!(5_usize, usize::fromage(String::from("hello")));
}

#[test]
fn test_intoage() {
    assert_eq!(Bar, Foo.intoage());
    assert_eq!(12_usize, String::from("hello, world").intoage());
}

#[test]
fn test_intoage_reflexive() {
    assert_eq!(Bar, Bar.intoage());
    assert_eq!(5_usize, 5_usize.intoage());
}

#[test]
fn test_try_fromage() {
    assert_eq!(Bar, Bar::try_fromage(Foo).unwrap());
    assert_eq!(0_usize, usize::try_fromage(String::new()).unwrap());
}

#[test]
fn test_try_intoage() {
    assert_eq!(Bar, Foo.try_intoage().unwrap());
    assert_eq!(5_usize, String::from("hello").try_intoage().unwrap());
}

#[test]
fn test_try_intoage_reflexive() {
    assert_eq!(Bar, Bar.try_intoage().unwrap());
    assert_eq!(5_usize, 5_usize.try_intoage().unwrap());
}

#[test]
fn test_impl_intoage() {
    fn intoage_foo(value: impl Intoage<Bar, X>) -> Bar {
        value.intoage()
    }
    assert_eq!(Bar, intoage_foo(Foo));
}

#[test]
fn test_impl_try_intoage() {
    fn try_intoage_foo(value: impl TryIntoage<Bar, X, Error = ()>) -> Result<Bar, ()> {
        value.try_intoage()
    }
    assert_eq!(Bar, try_intoage_foo(Foo).unwrap());
}

#[test]
fn test_collect_intoage() {
    let vec: Vec<Bar> = vec![Foo, Foo].into_iter().map(Intoage::intoage).collect();
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], Bar);
    assert_eq!(vec[1], Bar);
}

#[test]
fn test_try_collect_intoage() {
    let vec: Vec<Bar> = vec![Foo, Foo]
        .into_iter()
        .map(TryIntoage::try_intoage)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], Bar);
    assert_eq!(vec[1], Bar);
}

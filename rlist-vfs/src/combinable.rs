pub trait Combinable<T> {
    fn combine(from: Vec<T>) -> Self;
}
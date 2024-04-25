pub trait Combinable
where
    Self: Sized,
{
    fn combine(from: Vec<Self>) -> Self;
}

#[macro_export]
/// Combine items to one item, items must be the same type and implement `Combinable`
macro_rules! combine {
    ($($item:expr),*) => {{
        let items = vec![$($item),*];
        $crate::combinable::Combinable::combine(items)
    }};
}

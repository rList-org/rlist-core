pub trait Combinable
    where Self: Sized
{
    fn combine(from: Vec<Self>) -> Self;
}
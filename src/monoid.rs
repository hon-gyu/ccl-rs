/// Monoid constituent without proof of associativity
pub trait Monoid {
    fn empty() -> Self;
    fn merge(self, other: Self) -> Self;

    fn aggregate(items: Vec<Self>) -> Self
    where
        Self: Sized,
    {
        items
            .into_iter()
            .fold(Self::empty(), |acc, item| acc.merge(item))
    }
}


pub trait ResetIter
{
    type Item<'a> where Self: 'a;

    fn has_next<'a>(&'a self) -> bool;

    fn next<'a>(&'a mut self) -> Self::Item<'a>;

    fn reset<'a>(&'a mut self);
}
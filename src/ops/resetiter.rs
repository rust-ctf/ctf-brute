
pub trait ResetIter
{
    type Item<'a> where Self: 'a;

    fn has_next<'a>(&'a self) -> bool;

    fn get_next<'a>(&'a mut self) -> Self::Item<'a>;

    fn reset<'a>(&'a mut self);
}

// impl<T> Iterator for dyn ResetIter<Item = T>
// {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         if !self.has_next() {
//             return None;
//         }
//         Some(self.get_next())
//     }
// }
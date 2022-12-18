use std::ops::{BitAndAssign, BitOrAssign, Shl};

use num_traits::{PrimInt, AsPrimitive};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BitSet<T: PrimInt + BitOrAssign>(T);

impl<T> BitSet<T>
where
    T: PrimInt + BitOrAssign + BitAndAssign + Shl<Output = T> + 'static,
{
    pub fn new() -> Self {
        Self(T::zero())
    }

    pub fn new_full_up_to<U: AsPrimitive<T>>(max: U) -> Self {
        let bound = T::one() << max.as_() + T::one();
        Self(!bound)
    }

    pub fn add<U: AsPrimitive<T>>(&mut self, c: U) 
    {
        self.0 |= T::one() << c.as_();
    }

    pub fn with_added<U: AsPrimitive<T>>(&self, c: U) -> Self {
        let mut new = *self;
        new.add(c);
        new
    } 
    

    pub fn contains<U: AsPrimitive<T>>(&self, c: U) -> bool {
        self.0 & (T::one() << c.as_()) != T::zero()
    }

    pub fn remove<U: AsPrimitive<T>>(&mut self, c: U) {
        self.0 &= !(T::one() << c.as_());
    }

    pub fn with_removed<U: AsPrimitive<T>>(&self, c: U) -> Self {
        let mut new = *self;
        new.remove(c);
        new
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
}

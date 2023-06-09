pub use lanime_bindfields_macros::*;

pub trait Lens<'a>: Sized {
    type Input;
    type Output;

    fn get_mut(&self, input: &'a mut Self::Input) -> &'a mut Self::Output;

    #[inline]
    fn then<L>(self, other: L) -> LensCombination<Self, L>
    where
        L: Lens<'a, Input = Self::Output>,
    {
        LensCombination(self, other)
    }
}

#[derive(Clone, Copy)]
pub struct BindableField<Target, Type>(pub fn(&mut Target) -> &mut Type);

impl<'a, Target: 'static, Type: 'static> Lens<'a> for BindableField<Target, Type> {
    type Input = Target;
    type Output = Type;

    #[inline]
    fn get_mut(&self, input: &'a mut Target) -> &'a mut Type {
        (self.0)(input)
    }
}

pub struct LensCombination<A, B>(A, B);

impl<'a, A, B> Lens<'a> for LensCombination<A, B>
where
    A: Lens<'a> + 'static,
    B: Lens<'a, Input = A::Output> + 'static,
{
    type Input = A::Input;
    type Output = B::Output;

    #[inline]
    fn get_mut(&self, input: &'a mut A::Input) -> &'a mut B::Output {
        self.1.get_mut(self.0.get_mut(input))
    }
}

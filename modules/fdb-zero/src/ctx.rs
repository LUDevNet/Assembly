use std::ops::Deref;

use yoke::Yokeable;

pub struct WithCtx<'a, T> {
    ctx: &'a [u8],
    inner: T,
}

impl<'a, T> Deref for WithCtx<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> WithCtx<'a, T> {
    pub fn new(ctx: &'a [u8], inner: T) -> Self { Self { ctx, inner } }

    pub fn ctx(v: &Self) -> &'a [u8] {
        v.ctx
    }
}

unsafe impl<'a, T: 'static + for<'b> Yokeable<'b>> yoke::Yokeable<'a> for WithCtx<'static, T> {
    type Output = WithCtx<'a, <T as Yokeable<'a>>::Output>;

    fn transform(&'a self) -> &'a Self::Output {
        unsafe { std::mem::transmute(self) }
    }

    fn transform_owned(self) -> Self::Output {
        debug_assert!(std::mem::size_of::<Self::Output>() == std::mem::size_of::<Self>());
        unsafe {
            let ptr: *const Self::Output = (&self as *const Self).cast();
            std::mem::forget(self);
            std::ptr::read(ptr)
        }
    }

    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(std::mem::size_of::<WithCtx<'a, T>>() == std::mem::size_of::<Self>());
        let ptr: *const Self = (&from as *const Self::Output).cast();
        std::mem::forget(from);
        std::ptr::read(ptr)
    }

    fn transform_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        unsafe { f(std::mem::transmute::<&mut Self, &mut Self::Output>(self)) }
    }
}
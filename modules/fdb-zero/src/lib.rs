use std::ops::Deref;

use yoke::Yokeable;

pub mod unloaded;
pub mod loaded;
pub mod ctx;

#[macro_export]
macro_rules! yoke_impl {
    ($name:ident) => {
        unsafe impl<'a> yoke::Yokeable<'a> for $name<'static> {
            type Output = $name<'a>;
        
            fn transform(&'a self) -> &'a Self::Output {
                self
            }
        
            fn transform_owned(self) -> Self::Output {
                self
            }
        
            unsafe fn make(from: Self::Output) -> Self {
                debug_assert!(std::mem::size_of::<$name<'a>>() == std::mem::size_of::<Self>());
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
    };
}
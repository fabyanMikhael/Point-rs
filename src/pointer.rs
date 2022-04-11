use std::ops::{Deref, DerefMut};


pub struct Pointer<T>{
    value: usize,
    _marker: std::marker::PhantomData<T>
}

impl<T> Pointer<T>{
    pub fn new(value: &mut T) -> Pointer<T>{
        Pointer{
            value: value as *mut T as usize,
            _marker: std::marker::PhantomData
        }
    }
}

impl<T> Deref for Pointer<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe{&*(self.value as *const T)}
    }
}
impl<T> DerefMut for Pointer<T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe{&mut *(self.value as *mut T)}
    }
}
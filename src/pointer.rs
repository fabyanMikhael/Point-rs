use std::ops::{Deref, DerefMut};


pub fn Pointer<T>(value: &T) -> Pointer<T>{
    Pointer::new(value)
}

pub fn MutPointer<T>(value: &mut T) -> MutPointer<T>{
    MutPointer::new(value)
}



#[derive(Clone, Copy)]
pub struct Pointer<T>{
    value: usize,
    _marker: std::marker::PhantomData<T>
}

#[derive(Clone, Copy)]
pub struct MutPointer<T>{
    value: usize,
    _marker: std::marker::PhantomData<T>
}

impl<T> Deref for Pointer<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe{&*(self.value as *const T)}
    }
}

impl<T> Deref for MutPointer<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe{&*(self.value as *const T)}
    }
}

impl<T> DerefMut for MutPointer<T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe{&mut *(self.value as *mut T)}
    }
}

impl<T> Pointer<T>{
    pub fn new(value: &T) -> Self{
        Pointer{
            value: value as *const T as usize,
            _marker: std::marker::PhantomData
        }
    }
}

impl<T> MutPointer<T>{
    pub fn new(value: &mut T) -> Self{
        MutPointer{
            value: value as *mut T as usize,
            _marker: std::marker::PhantomData
        }
    }
    fn pointer(&self) -> Pointer<T>{
        Pointer{value: self.value, _marker: std::marker::PhantomData}
    }
}
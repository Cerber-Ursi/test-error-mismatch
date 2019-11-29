pub fn create<T: Trait>() -> T {
    T::get()
}

pub trait Trait: Sized + Clone + Copy {
    fn get() -> Self;
}

pub fn collect<T: Trait>() -> T {
    create()
}


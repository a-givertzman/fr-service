pub trait DebugTypeOf<T> {
    fn printTypeOf(&self) {
        println!("{}", std::any::type_name::<T>())
    }
}

impl<T> DebugTypeOf<T> for T {

}

pub trait TypeOf<T> {
    fn typeOf<'a>(&self) -> &str {
        std::any::type_name::<T>()
    }
}

impl<T> TypeOf<T> for T {

}
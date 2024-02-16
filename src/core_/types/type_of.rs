pub trait DebugTypeOf<T> {
    fn print_type_of(&self) {
        println!("{}", std::any::type_name::<T>())
    }
}

impl<T> DebugTypeOf<T> for T {

}

pub trait TypeOf<T> {
    fn type_of<'a>(&self) -> &str {
        std::any::type_name::<T>()
    }
}

impl<T> TypeOf<T> for T {

}
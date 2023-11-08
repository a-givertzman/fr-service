pub trait DebugTypeOf<T> {
    fn typeOf(&self) {
        println!("{}", std::any::type_name::<T>())
    }    
}

impl<T> DebugTypeOf<T> for T {

}
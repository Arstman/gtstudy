use std::slice::Chunks;


pub trait IteratorExt: Iterator {
    fn window_count<T>(&self, size: usize) -> Chunks<'_,T>
    {
        self.collect::<Vec<_>>().into_boxed_slice().chunks(size).into()
    }
        

}


fn main() {
    println!("Hello, world!");
}

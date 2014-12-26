
mod intrusive {
    trait Intrusive<T> {
        fn hook(&mut self) -> *const T;
    }
}

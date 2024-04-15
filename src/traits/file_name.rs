pub trait SingletonFileName {
    fn filename() -> String;
}
pub trait InstanceFileName {
    fn filename(&self) -> String;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Machine<T>
where
    T: Copy + Clone,
{
    pub state: T,
}

impl<T> Machine<T>
where
    T: Copy + Clone,
{
    pub fn get(&self) -> T {
        self.state
    }
}

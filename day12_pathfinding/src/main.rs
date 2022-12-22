use std::error::Error;

struct Vec2D<T> {
    data: Vec<T>,
    count: usize,
}

impl<T> Vec2D<T> {
    fn new<B: Sized>(count: usize) -> Vec2D<B> {
        Vec2D {
            data: Vec::<B>::new(),
            count,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(x + y * self.count)
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data.get_mut(x + y * self.count)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("inp_off.txt")?;

    Ok(())
}

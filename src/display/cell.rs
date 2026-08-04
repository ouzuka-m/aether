#[derive(Debug)]
pub struct Cell {
    start_x: usize,
    start_y: usize,
    width: usize,
}

impl Cell {
    pub fn new(start_x: usize, start_y: usize, width: usize) -> Self {
        Self {
            start_x,
            start_y,
            width,
        }
    }

    pub fn start_x(&self) -> usize {
        self.start_x
    }

    pub fn start_y(&self) -> usize {
        self.start_y
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

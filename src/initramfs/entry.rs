#[derive(Debug)]
pub struct Entry<'a> {
    name: &'a str,
    data: &'a [u8],
}

impl<'a> Entry<'a> {
    pub fn new(name: &'a str, data: &'a [u8]) -> Self {
        Entry { name, data }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn data(&self) -> &'a [u8] {
        self.data
    }
}

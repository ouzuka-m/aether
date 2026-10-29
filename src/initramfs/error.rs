#[derive(Debug)]
pub enum FsError {
    NotFound,
    NotExecutable,
    NoUsableMemory,
    MapperError,
    InvalidSegmentData,
}

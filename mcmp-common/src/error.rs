
#[repr(usize)]
pub enum ErrorId {
    NotAuthed = 1,
    UserNotFound = 2,
    WrongPasswd = 3
}

impl Into<usize> for ErrorId {
    fn into(self) -> usize {
        self as usize
    }
}

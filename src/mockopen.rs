/// This module is used for mocking the open crate, so that tests do not open
/// a browser window, and so that the content of the file that is opened can be
/// read by the tests instead.
///
/// The content of the file to be opened is passed to the test via the global
/// variable FILE_CONTENT.
pub mod open {
    use std::cell::RefCell;
    thread_local! {
        pub static FILE_CONTENT: RefCell<String> = RefCell::new(String::new());
    }
    use std::fs;
    use std::path::Path;
    use std::{ffi::OsStr, io};

    pub fn that(path: impl AsRef<OsStr>) -> io::Result<()> {
        let path = Path::new(path.as_ref());
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => format!("Failed to read file {:?}. Error: {}", path, e),
        };

        FILE_CONTENT.with(|global_data| {
            *global_data.borrow_mut() = content;
        });
        io::Result::Ok(())
    }
}

use std::io;

error_chain! {
    foreign_links {
        Io(io::Error) #[doc = "IO"];
    }
}

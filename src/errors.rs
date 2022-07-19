use std;

use crate::io;

error_chain! {
    links {
        Io(io::Error, io::ErrorKind) #[doc = "IO"];
    }

    foreign_links {
        StdIo(std::io::Error) #[doc = "IO"];
    }
}

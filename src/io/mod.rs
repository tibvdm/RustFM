use std::io::{
    BufReader,
    BufWriter,
    Read,
    Write
};

use serde::{
    Deserialize,
    Serialize
};

use crate::{
    alphabet::Alphabet,
    index::{
        bidirectional_fm_index::BidirectionalFMIndex,
        fm_index::FMIndex
    }
};

pub trait Binary {
    fn to_bin<W>(&self, writer: BufWriter<W>) -> Result<()>
    where
        W: Write,
        Self: Serialize + Sized
    {
        Ok(bincode::serialize_into(writer, self)?)
    }

    fn from_bin<R>(reader: BufReader<R>) -> Result<Self>
    where
        R: Read,
        for<'de> Self: Deserialize<'de>
    {
        Ok(bincode::deserialize_from(reader)?)
    }
}

impl<A: Alphabet> Binary for FMIndex<A> {}

impl<A: Alphabet> Binary for BidirectionalFMIndex<A> {}

error_chain! {
    foreign_links {
        Bincode(bincode::Error);
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        fs::File,
        io::{
            BufReader,
            BufWriter
        },
        path::Path
    };

    use serde::{
        Deserialize,
        Serialize
    };

    use crate::io::Binary;

    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        a: usize,
        b: (usize, usize)
    }

    impl Binary for TestStruct {}

    #[test]
    fn test_to_bin() {
        let test_struct = TestStruct {
            a: 10, b: (5, 25)
        };

        assert_eq!(test_struct.a, 10);
        assert_eq!(test_struct.b, (5, 25));

        let f = File::create("./test_to_bin").expect("Unable to create file");
        let f = BufWriter::new(f);

        test_struct.to_bin(f);

        assert_eq!(Path::new("./test_to_bin").exists(), true);

        fs::remove_file("./test_to_bin");
    }

    #[test]
    fn test_from_bin() {
        let test_struct = TestStruct {
            a: 10, b: (5, 25)
        };

        assert_eq!(test_struct.a, 10);
        assert_eq!(test_struct.b, (5, 25));

        let f = File::create("./test_from_bin").expect("Unable to create file");
        let f = BufWriter::new(f);

        test_struct.to_bin(f);

        assert_eq!(Path::new("./test_from_bin").exists(), true);

        let f = File::open("./test_from_bin").expect("Unable to create file");
        let f = BufReader::new(f);

        match TestStruct::from_bin(f) {
            Ok(test_struct_new) => {
                assert_eq!(test_struct_new.a, 10);
                assert_eq!(test_struct_new.b, (5, 25));
            }
            Err(_) => assert_eq!(true, false)
        }

        fs::remove_file("./test_from_bin");
    }
}

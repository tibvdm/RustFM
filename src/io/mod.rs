use std::io::{ Read, BufReader, Write, BufWriter };
use serde::{ Serialize, Deserialize };

use crate::suffix_array::SparseSuffixArray;
use crate::fm_index::FMIndex;
use crate::alphabet::Alphabet;

pub trait Binary {
    fn to_bin<W>(&self, writer: BufWriter<W>) -> Result<()>
    where W: Write, Self: Serialize + Sized {
        Ok(bincode::serialize_into(writer, self)?)
    }

    fn from_bin<R>(reader: BufReader<R>) -> Result<Self> 
    where R: Read, for<'de> Self: Deserialize<'de> {
        Ok(bincode::deserialize_from(reader)?)
    }
}

impl<A: Alphabet> Binary for FMIndex<A> {}

error_chain! {
    foreign_links {
        Bincode(bincode::Error);
    }
}

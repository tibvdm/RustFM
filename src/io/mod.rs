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

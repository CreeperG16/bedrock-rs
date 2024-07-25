use std::io::Cursor;
use std::sync::Arc;


use bedrockrs_core::LE;

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for bool {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match self {
            true => match LE::<u8>::new(1).write(buf) {
                Ok(_) => Ok(()),
                Err(e) => Err(ProtoCodecError::IOError(Arc::new(e))),
            },
            false => match LE::<u8>::new(0).write(buf) {
                Ok(_) => Ok(()),
                Err(e) => Err(ProtoCodecError::IOError(Arc::new(e))),
            },
        }
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        // a Bool is represented as a byte
        return match LE::<u8>::read(stream) {
            Ok(v) => {
                match v.into_inner() {
                    // 0 is counted as false
                    0 => Ok(false),
                    // Anything above 0 is true
                    _ => Ok(true),
                }
            }
            Err(e) => Err(ProtoCodecError::IOError(Arc::new(e))),
        };
    }
}

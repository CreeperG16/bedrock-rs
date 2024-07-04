use std::sync::Arc;
use bedrock_core::LE;
use bedrock_core::read::ByteStreamRead;
use bedrock_core::write::ByteStreamWrite;
use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;

#[derive(Debug, Clone)]
pub enum EditorWorldType {
    NotEditor,
    Project,
    TestLevel,
}

impl ProtoCodec for EditorWorldType {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError> {
        let int = match self {
            EditorWorldType::NotEditor => { 0x00 }
            EditorWorldType::Project => { 0x01 }
            EditorWorldType::TestLevel => { 0x02 }
        };

        match LE::<u8>::new(int).write(stream) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(ProtoCodecError::IOError(Arc::new(e))) }
        }
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError> {
        todo!()
    }
}

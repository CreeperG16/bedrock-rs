use std::io::Cursor;
use bedrockrs_core::int::LE;
use bedrockrs_core::Vec3;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_shared::actor_runtime_id::ActorRuntimeID;
use crate::types::interact_action::InteractAction;

#[derive(Debug, Clone)]
pub struct InteractPacket {
    action: InteractAction,
    target_runtime_id: ActorRuntimeID
}

impl ProtoCodec for InteractPacket {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let (action, pos) = match self.action {
            InteractAction::Invalid => {
                (0, None)
            }
            InteractAction::StopRiding(pos) => {
                (3, Some(pos))
            }
            InteractAction::InteractUpdate(pos) => {
                (4, Some(pos))
            }
            InteractAction::NpcOpen => {
                (5, None)
            }
            InteractAction::OpenInventory => {
                (6, None)
            }
        };

        u8::proto_serialize(&action, stream)?;

        if let Some(pos) = pos {
            pos.to_le().proto_serialize(stream)?;
        }

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let action = u8::proto_deserialize(stream)?;

        let target_runtime_id = ActorRuntimeID::proto_deserialize(stream)?;

        let action = match action {
            0 => {
                InteractAction::Invalid
            }
            3 => {
                let pos = Vec3::<LE<f32>>::proto_deserialize(stream)?;
                let pos = Vec3::<f32>::from_le(pos);

                InteractAction::StopRiding(pos)
            }
            4 => {
                let pos = Vec3::<LE<f32>>::proto_deserialize(stream)?;
                let pos = Vec3::<f32>::from_le(pos);

                InteractAction::InteractUpdate(pos)
            }
            5 => {
                InteractAction::NpcOpen
            }
            6 => {
                InteractAction::OpenInventory
            }
            other => return Err(ProtoCodecError::InvalidEnumID(format!("{other:?}"), String::from("InteractAction"))),
        };

        println!("YIPPIE: {:?}", &stream.get_ref()[(stream.position() as usize)..]);

        Ok(Self {
            action,
            target_runtime_id
        })
    }
}

use crate::connection::ConnectionShard;
use crate::error::LoginError;
use crate::gamepacket::GamePacket;
use crate::login::provider::{LoginProviderServer, LoginProviderStatus};
use crate::packets::play_status::PlayStatusPacket;
use crate::types::play_status::PlayStatusType;

pub async fn play_status_login(conn: &mut ConnectionShard, provider: &impl LoginProviderServer) -> Result<(), LoginError> {
    //////////////////////////////////////
    // Play Status Packet (Login)
    //////////////////////////////////////

    let mut play_status = PlayStatusPacket {
        status: PlayStatusType::LoginSuccess,
    };

    match provider.on_play_status_pk(&mut play_status) {
        LoginProviderStatus::ContinueLogin => {}
        LoginProviderStatus::AbortLogin { reason } => {
            return Err(LoginError::Abort { reason });
        }
    };

    match conn.send(GamePacket::PlayStatus(play_status)).await {
        Ok(_) => {}
        Err(e) => return Err(LoginError::ConnError(e)),
    }

    match conn.flush().await {
        Ok(_) => {}
        Err(e) => return Err(LoginError::ConnError(e)),
    };

    Ok(())
}
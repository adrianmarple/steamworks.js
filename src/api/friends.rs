use crate::api::localplayer::PlayerSteamId;
use napi_derive::napi;

#[napi(object)]
pub struct Friend {
    pub id: PlayerSteamId,
    pub name: String,
}

#[napi]
pub mod friends {
    use super::Friend;
    use crate::api::localplayer::PlayerSteamId;
    use steamworks::FriendFlags;

    #[napi]
    pub async fn get_friends() -> Vec<Friend> {
        let client = crate::client::get_client();

        client
            .friends()
            .get_friends(FriendFlags::all())
            .into_iter()
            .map(|friend| Friend {
                id: PlayerSteamId::from_steamid(friend.id()),
                name: friend.name(),
            })
            .collect()
    }
}

const { init, SteamCallback } = require('../index.js')

const client = init(480)
const callback1 = client.callback.register(SteamCallback.LobbyDataUpdate, (data) => {
    console.log('LobbyDataUpdate', data)
});


(async () => {
    console.log("All friends")
    console.log(await client.friends.getFriends())
})();
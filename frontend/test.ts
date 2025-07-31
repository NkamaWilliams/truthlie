import * as dotenv from "dotenv"

dotenv.config()
console.log("Testing testing...")

const API_URL = process.env.API_URL;
console.log(API_URL);

(async () => {
    let name = "DarkNigga";
    let resp = await fetch(`${API_URL}/players/`, {
        method: "POST",
        body: JSON.stringify({name}),
        headers: {
            "Content-type": "application/json"
        }
    });
    let player = await resp.json()
    console.log('Response:', player)

    let req_body = {
        host_id: player.id,
        with_staking: true,
        stake_amount: 200
    }

    resp = await fetch(`${API_URL}/games/`, {
        method: "POST",
        headers: {
            "Content-type": "application/json"
        },
        body: JSON.stringify(req_body)
    })

    console.log("Response:", await resp.json())
})();
import dotenv from "dotenv";
import fetch from "cross-fetch";
import { describe, it } from "mocha";
import assert from "assert";
dotenv.config();

const API_URL = process.env.API_URL!;
if (!API_URL) {
  throw new Error("API_URL is not defined in environment variables");
}

interface Player {
  id: string;
  name: string;
  [key: string]: any;
}

interface Game {
  id: string;
  host_id: string;
  players: string[];
  [key: string]: any;
}

interface JoinResponse {
  message: string;
  body: Game;
}

interface ErrorResponse {
  error: string;
  code: number;
}

describe("Truth or Lie API", function () {
  let player1: Player;
  let player2: Player;
  let player3: Player;
  let game: Game;

  it("should create first player", async function () {
    const resp = await fetch(`${API_URL}/players/`, {
      method: "POST",
      body: JSON.stringify({ name: "DarkNigga" }),
      headers: { "Content-Type": "application/json" },
    });
    assert.strictEqual(resp.status, 201, "Expected 201 Created for first player");
    player1 = await resp.json();
    assert.ok(player1.id, "Player1 should have id");
    assert.strictEqual(player1.name, "DarkNigga");
  });

  it("should create second player", async function () {
    const resp = await fetch(`${API_URL}/players/`, {
      method: "POST",
      body: JSON.stringify({ name: "LightNigga" }),
      headers: { "Content-Type": "application/json" },
    });
    assert.strictEqual(resp.status, 201, "Expected 201 Created for second player");
    player2 = await resp.json();
    assert.ok(player2.id, "Player2 should have id");
    assert.strictEqual(player2.name, "LightNigga");
  });

  it("should create third player", async function () {
    const resp = await fetch(`${API_URL}/players/`, {
      method: "POST",
      body: JSON.stringify({ name: "GreyNigga" }),
      headers: { "Content-Type": "application/json" },
    });
    assert.strictEqual(resp.status, 201, "Expected 201 Created for third player");
    player3 = await resp.json();
    assert.ok(player3.id, "Player2 should have id");
    assert.strictEqual(player3.name, "GreyNigga");
  });

  it("should create a game with player1 as host", async function () {
    const reqBody = {
      host_id: player1.id,
      with_staking: true,
      stake_amount: 200,
      max_players: 3
    };

    const resp = await fetch(`${API_URL}/games/`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(reqBody),
    });
    assert.strictEqual(resp.status, 201, "Expected 201 Created for game");
    game = await resp.json();
    assert.ok(game.id, "Game should have id");
    assert.strictEqual(game.host_id, player1.id);
    assert.strictEqual(game.with_staking, true);
    assert.strictEqual(game.stake_amount, 200);
  });

  it("should let player2 join the game", async function () {
    const resp = await fetch(`${API_URL}/games/${game.id}/join`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ player_id: player2.id }),
    });
    assert.strictEqual(resp.status, 200, "Expected 200 OK for join");
    const joinResp: JoinResponse = await resp.json();
    assert.strictEqual(joinResp.message, "Joined game successfully");
    assert.ok(joinResp.body.players.includes(player2.id), "Player2 should be in game");
  });

  it("should not let player2 join the game again", async function () {
    const resp = await fetch(`${API_URL}/games/${game.id}/join`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({player_id: player2.id})
    });
    assert.strictEqual(resp.status, 400, "Expected 400 error for join");
    const joinResp: ErrorResponse = await resp.json();
    assert.strictEqual(joinResp.error, "Validation failed: Player already in game");
  });

  it("player2 should not be able start the game", async function() {
    const resp = await fetch(`${API_URL}/games/${game.id}/start`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({player_id: player2.id})
    });
    assert.strictEqual(resp.status, 401, "Expected 401 error for start");
    const joinResp: ErrorResponse = await resp.json();
    assert.strictEqual(joinResp.error, "Unauthorized: Only host can start game!");
  });

  it("player1 should be able start the game", async function() {
    const resp = await fetch(`${API_URL}/games/${game.id}/start`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({player_id: player1.id})
    });
    assert.strictEqual(resp.status, 200, "Expected 200 error for start");
    const joinResp: JoinResponse = await resp.json();
    assert.strictEqual(joinResp.message, "Started game successfully");
  });

  it("should not let player3 join the game", async function () {
    const resp = await fetch(`${API_URL}/games/${game.id}/join`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({player_id: player3.id})
    });
    assert.strictEqual(resp.status, 400, "Expected 400 Error for join");
    const joinResp: ErrorResponse = await resp.json();
    assert.ok(joinResp.error, "Validation failed: Game has already started!");
  })

  it("player1 should be able to start game session via ws", function (done) {
    const ws = new WebSocket(`ws://127.0.0.1:8080/ws/${game.id}/${player1.id}`);
    const ws2 = new WebSocket(`ws://127.0.0.1:8080/ws/${game.id}/${player2.id}`);

    ws.onopen = () => {
      console.log("WS1 connected, sending StartGame...");
      const message = {
        type: "StartGame",
        data: { game_id: game.id, player_id: player1.id },
      };
      ws.send(JSON.stringify(message));
    };

    ws2.onmessage = (msg) => {
      console.log("WS2 received:", msg.data);
      const payload = JSON.parse(msg.data);

      assert.strictEqual(payload.type, "StartedGame");
      assert.strictEqual(payload.data.game_id, game.id);

      ws.close();
      ws2.close();
      done();
    };

    ws.onerror = (err) => done(err);
    ws2.onerror = (err) => done(err);
  });

  it("player2 should not be able to start game session via ws", function (done) {
    const ws = new WebSocket(`ws://127.0.0.1:8080/ws/${game.id}/${player2.id}`);
    const ws2 = new WebSocket(`ws://127.0.0.1:8080/ws/${game.id}/${player1.id}`);

    ws.onopen = () => {
      console.log("WS1 connected, sending StartGame...");
      const message = {
        type: "StartGame",
        data: { game_id: game.id, player_id: player2.id },
      };
      ws.send(JSON.stringify(message));
    };

    ws2.onmessage = (msg) => {
      console.log("WS2 received:", msg.data);
      const payload = JSON.parse(msg.data);

      assert.strictEqual(payload.type, "Error");
      assert.strictEqual(payload.data.message, "Only the host can start a game");

      ws.close();
      ws2.close();
      done();
    };

    ws.onerror = (err) => done(err);
    ws2.onerror = (err) => done(err);
  });
});



//
// const game_id = "6711aab1-4c2b-45b6-a6f4-ba0123fc114f"

// const player_id = "fbc28665-a2cd-4170-80c8-2bdd6698eacb"

// let ws = new WebSocket(`ws://127.0.0.1:8080/ws/${game_id}/${player_id}`);

// ws.onopen = () => console.log("Connection Opened!");
// () => console.log("Connection Opened!")
// ws.onmessage = (msg) => console.log("Received:", msg.data);
// ws.onclose = () => console.log("Disconnected");
// ws.onerror = (err) => console.error(err);

// const message = {
//     "type": "StartGame",
//     "data": {
//         "game_id": `${game_id}`,
//         "player_id": "fbc28665-a2cd-4170-80c8-2bdd6698eacb"
//     }
// };
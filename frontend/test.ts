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

  it("should let player3 join the game", async function () {
    const resp = await fetch(`${API_URL}/games/${game.id}/join`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({player_id: player3.id})
    });
    assert.strictEqual(resp.status, 200, "Expected 200 Created for join");
    const joinResp: JoinResponse = await resp.json();
    assert.ok(joinResp.body.players.includes(player3.id), "Player3 should be in game");
  })
});
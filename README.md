# 🎭 Truth or Lie

A multiplayer social deduction game where players test their deception skills by submitting truths and lies. Others must guess which statement is true - and optionally, stake tokens on the outcome. Think: *Two Truths and a Lie*, powered by Solana.

Built for fun, and technical mastery.

---

## 🧩 Overview

Truth or Lie is a game where:
- Each player submits one **true** and one **false** statement.
- Other players must guess which is true.
- Players earn ranks, badges, and optional winnings.
- Sessions can be public or invite-only.
- Optional: players can **stake tokens**,  winner takes all.

---

## 📦 Tech Stack

### ✅ Backend
- **Rust** (2024 Edition)
- **Actix Web** for async HTTP & routing
- **DashMap** for in-memory multiplayer state
- **Serde** + **UUID** + **Chrono**
- Optional Web3 support via wallet address tracking

### ⚙️ Planned Frontend (TBD)
- **React + TypeScript**
- **Tailwind CSS**
- **Wallet integration (e.g. Phantom / Solana)** if staking is used
- **WebSocket or polling** for real-time gameplay
- Mobile-first design


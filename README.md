# TCG Engine (Rust + WebAssembly)

a headless, web-based Trading Card Game built with Rust and Webassembly.
Designed for building card game systems in mordern web applications.

---

## Overview

This project provide a minimal but extensible TCG engine that runs directly in the browser using WebAssembly.
The core game logic is implemented in Rust for safety and determinism, while the frontend is handled separately (e.g. React)

---

## Tech Stack

Rust - Core game enigne
WebAssembly - Browser integration
Typescript / React - Frontend

---

Example Usage

```
await init();

const engine = new Engine();

// get current state
const state = engine.get_state();

// play a card
const next = engine.play_first_card();

// end turn
const afterTurn = engine.end_turn();
```

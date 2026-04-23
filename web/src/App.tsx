import { useEffect, useMemo, useState } from "react";
import init, { Engine } from "tcg_core";

type Card = {
  id: string;
  name: string;
  cost: number;
};

type PlayerState = {
  id: string;
  hand: Card[];
  field: Card[];
};

type GameState = {
  current_player: number;
  players: PlayerState[];
  turn: number;
  log: string[];
};

function App() {
  const [ready, setReady] = useState(false);
  const [engine, setEngine] = useState<Engine | null>(null);
  const [state, setState] = useState<GameState | null>(null);

  useEffect(() => {
    const setup = async () => {
      await init();
      const e = new Engine();
      const initialState = e.get_state() as GameState;

      setEngine(e);
      setState(initialState);
      setReady(true);
    };
    setup();
  }, []);

  const currentPlayer = useMemo(() => {
    if (!state) return null;
    return state.players[state.current_player];
  }, [state]);

  const onPlayCard = () => {
    if (!engine) return null;

    try {
      const nextState = engine.play_first_card() as GameState;
      setState(nextState);
    } catch (err) {
      alert(String(err));
    }
  };

  const onEndTurn = () => {
    if (!engine) return null;
    const nextState = engine.end_turn() as GameState;
    setState(nextState);
  };

  if (!ready || !state || !currentPlayer) {
    return <div style={{ padding: 24 }}>Loading...</div>;
  }

  return (
    <div style={{ padding: 24, fontFamily: "sans-serif" }}>
      <h1>TCG WASM Demo</h1>
      <p>Turn: {state.turn}</p>
      <p>Current Player: {currentPlayer.id}</p>

      <div style={{ display: "flex", gap: 24, marginTop: 24 }}>
        {state.players.map((player) => (
          <div
            key={player.id}
            style={{
              border: "1px solid #ccc",
              borderRadius: 12,
              padding: 16,
              width: 280,
            }}
          >
            <h2>{player.id}</h2>

            <h3>Hand</h3>
            <ul>
              {player.hand.map((card) => (
                <li key={card.id}>
                  {card.name} (cost {card.cost})
                </li>
              ))}
            </ul>

            <h3>Field</h3>
            <ul>
              {player.field.map((card) => (
                <li key={card.id}>{card.name}</li>
              ))}
            </ul>
          </div>
        ))}
      </div>

      <div style={{ display: "flex", gap: 12, marginTop: 24 }}>
        <button onClick={onPlayCard}>Play First Card</button>
        <button onClick={onEndTurn}>End Turn</button>
      </div>

      <div style={{ marginTop: 24 }}>
        <h3>Log</h3>
        <ul>
          {state.log.map((line, idx) => (
            <li key={idx}>{line}</li>
          ))}
        </ul>
      </div>
    </div>
  );
}

export default App;

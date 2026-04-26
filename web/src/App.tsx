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

type GameEvent =
  | { type: "GameStarted" }
  | {
      type: "CardPlayed";
      player_id: string;
      card_id: string;
      card_name: string;
    }
  | {
      type: "TurnEnded";
      player_id: string;
      next_player_id: string;
      turn: number;
    }
  | {
      type: "CommandRejected";
      reason: string;
    };

type GameState = {
  current_player: number;
  players: PlayerState[];
  turn: number;
  events: GameEvent[];
};

type Command =
  | {
      type: "PlayCard";
      player_id: string;
      card_id: string;
    }
  | {
      type: "EndTurn";
      player_id: string;
    };

function App() {
  const [ready, setReady] = useState(false);
  const [engine, setEngine] = useState<Engine | null>(null);
  const [state, setState] = useState<GameState | null>(null);

  useEffect(() => {
    const setup = async () => {
      await init();

      const e = new Engine();
      const initialState = e.get_state() as unknown as GameState;

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

  const refreshState = () => {
    if (!engine) return;
    const nextState = engine.get_state() as unknown as GameState;
    setState(nextState);
  };

  const dispatchCommand = (command: Command) => {
    if (!engine) return;

    try {
      engine.dispatch(command);
      refreshState();
    } catch (err) {
      alert(String(err));
    }
  };

  const onPlayCard = (cardId: string) => {
    if (!currentPlayer) return;

    dispatchCommand({
      type: "PlayCard",
      player_id: currentPlayer.id,
      card_id: cardId,
    });
  };

  const onEndTurn = () => {
    if (!currentPlayer) return;

    dispatchCommand({
      type: "EndTurn",
      player_id: currentPlayer.id,
    });
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
                  {card.name} (cost {card.cost}){" "}
                  {player.id === currentPlayer.id && (
                    <button onClick={() => onPlayCard(card.id)}>Play</button>
                  )}
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
        <button onClick={onEndTurn}>End Turn</button>
      </div>

      <div style={{ marginTop: 24 }}>
        <h3>Events</h3>
        <ul>
          {state.events.map((event, idx) => (
            <li key={idx}>
              <code>{JSON.stringify(event)}</code>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}

export default App;

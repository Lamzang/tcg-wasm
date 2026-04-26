import { useEffect, useMemo, useState } from "react";
import init, { Engine } from "tcg_core";

type CardType = "Unit" | "Spell";

type EffectDefinition =
  | { Draw: { amount: number } }
  | { Damage: { amount: number } }
  | { Heal: { amount: number } };

type CardDefinition = {
  id: string;
  name: string;
  cost: number;
  card_type: CardType;
  effects: EffectDefinition[];
};

type Zone = "Deck" | "Hand" | "Field" | "Graveyard";

type CardInstance = {
  id: string;
  definition_id: string;
  owner_id: string;
  controller_id: string;
  zone: Zone;
  attack: number | null;
  health: number | null;
  max_health: number | null;
  exhausted: boolean;
};

type PlayerState = {
  id: string;
  hp: number;
  mana: number;
  max_mana: number;
  deck: string[];
  hand: string[];
  field: string[];
  graveyard: string[];
};

type GameEvent =
  | { type: "GameStarted" }
  | {
      type: "CardPlayed";
      player_id: string;
      card_instance_id: string;
      card_definition_id: string;
      card_name: string;
      cost: number;
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
  card_definitions: Map<string, CardDefinition>;
  card_instances: Map<string, CardInstance>;
  events: GameEvent[];
};

type Command =
  | {
      type: "PlayCard";
      player_id: string;
      card_instance_id: string;
    }
  | {
      type: "EndTurn";
      player_id: string;
    }
  | {
      type: "AttackUnit";
      player_id: string;
      attacker_id: string;
      target_id: string;
    }
  | {
      type: "AttackPlayer";
      player_id: string;
      attacker_id: string;
      target_player_id: string;
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

  useEffect(() => {
    if (state) {
      console.log("Game State:", state);
      console.log("Player 1 Hand:", state.players[0]?.hand);
      console.log("Player 2 Hand:", state.players[1]?.hand);
      console.log("Card Instances:", state.card_instances);
    }
  }, [state]);

  const currentPlayer = useMemo(() => {
    if (!state) return null;
    return state.players[state.current_player];
  }, [state]);

  const getCardDefinition = (cardInstanceId: string) => {
    if (!state) return null;

    const instance = state.card_instances.get(cardInstanceId);
    if (!instance) return null;

    return state.card_definitions.get(instance.definition_id) ?? null;
  };

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

  const onPlayCard = (cardInstanceId: string) => {
    if (!currentPlayer) return;

    dispatchCommand({
      type: "PlayCard",
      player_id: currentPlayer.id,
      card_instance_id: cardInstanceId,
    });
  };

  const onEndTurn = () => {
    if (!currentPlayer) return;

    dispatchCommand({
      type: "EndTurn",
      player_id: currentPlayer.id,
    });
  };

  const onAttackPlayer = (attackerId: string, targetPlayerId: string) => {
    if (!currentPlayer) return;

    dispatchCommand({
      type: "AttackPlayer",
      player_id: currentPlayer.id,
      attacker_id: attackerId,
      target_player_id: targetPlayerId,
    });
  };

  const onAttackUnit = (attackerId: string, targetId: string) => {
    if (!currentPlayer) return;

    dispatchCommand({
      type: "AttackUnit",
      player_id: currentPlayer.id,
      attacker_id: attackerId,
      target_id: targetId,
    });
  };

  if (!ready || !state || !currentPlayer) {
    return <div style={{ padding: 24 }}>Loading...</div>;
  }

  return (
    <div style={{ padding: 24, fontFamily: "sans-serif" }}>
      <h1>TCG WASM Demo</h1>

      <p>Turn: {state.turn}</p>
      <p>
        Current Player: <strong>{currentPlayer.id}</strong>
      </p>
      <p>
        Mana: {currentPlayer.mana}/{currentPlayer.max_mana}
      </p>

      <div style={{ display: "flex", gap: 24, marginTop: 24 }}>
        {state.players.map((player) => (
          <div
            key={player.id}
            style={{
              border: "1px solid #ccc",
              borderRadius: 12,
              padding: 16,
              width: 320,
            }}
          >
            <h2>{player.id}</h2>
            <p>HP: {player.hp}</p>
            <p>
              Mana: {player.mana}/{player.max_mana}
            </p>

            <h3>Hand</h3>
            <ul>
              {player.hand.map((cardInstanceId) => {
                const card = getCardDefinition(cardInstanceId);
                if (!card) return null;

                const canClick = player.id === currentPlayer.id;

                return (
                  <li key={cardInstanceId} style={{ marginBottom: 8 }}>
                    <strong>{card.name}</strong> [{card.card_type}] cost{" "}
                    {card.cost}
                    <br />
                    <small>instance: {cardInstanceId}</small>
                    <br />
                    {canClick && (
                      <button onClick={() => onPlayCard(cardInstanceId)}>
                        Play
                      </button>
                    )}
                  </li>
                );
              })}
            </ul>

            <h3>Field</h3>
            <ul>
              {player.field.map((cardInstanceId) => {
                const card = getCardDefinition(cardInstanceId);
                const instance = state.card_instances.get(cardInstanceId);
                if (!card || !instance) return null;

                const isCurrentPlayersUnit = player.id === currentPlayer.id;

                return (
                  <li key={cardInstanceId} style={{ marginBottom: 8 }}>
                    <strong>{card.name}</strong>
                    <br />
                    ATK: {instance.attack} / HP: {instance.health}/
                    {instance.max_health}
                    <br />
                    Exhausted: {String(instance.exhausted)}
                    <br />
                    {isCurrentPlayersUnit && !instance.exhausted && (
                      <>
                        {state.players
                          .filter((p) => p.id !== currentPlayer.id)
                          .map((enemy) => (
                            <button
                              key={enemy.id}
                              onClick={() =>
                                onAttackPlayer(cardInstanceId, enemy.id)
                              }
                            >
                              Attack {enemy.id}
                            </button>
                          ))}

                        {state.players
                          .filter((p) => p.id !== currentPlayer.id)
                          .flatMap((enemy) =>
                            enemy.field.map((targetId) => {
                              const targetCard = getCardDefinition(targetId);
                              if (!targetCard) return null;

                              return (
                                <button
                                  key={targetId}
                                  onClick={() =>
                                    onAttackUnit(cardInstanceId, targetId)
                                  }
                                >
                                  Attack {targetCard.name}
                                </button>
                              );
                            }),
                          )}
                      </>
                    )}
                  </li>
                );
              })}
            </ul>

            <h3>Graveyard</h3>
            <ul>
              {player.graveyard.map((cardInstanceId) => {
                const card = getCardDefinition(cardInstanceId);
                if (!card) return null;

                return (
                  <li key={cardInstanceId}>
                    {card.name} <small>({cardInstanceId})</small>
                  </li>
                );
              })}
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

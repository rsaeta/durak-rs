const API_BASE = 'http://localhost:3000/api';

let currentGameId = null;
let currentGameState = null;

// Initialize
document.addEventListener('DOMContentLoaded', () => {
  document.getElementById('newGameBtn').addEventListener('click', createNewGame);
});

async function createNewGame() {
  try {
    const response = await fetch(`${API_BASE}/games`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      throw new Error('Failed to create game');
    }

    const data = await response.json();
    currentGameId = data.game_id;
    document.getElementById('gameIdDisplay').textContent = `Game ID: ${currentGameId}`;
    document.getElementById('gameArea').style.display = 'block';

    await updateGameState();
  } catch (error) {
    console.error('Error creating game:', error);
    alert('Failed to create game: ' + error.message);
  }
}

async function updateGameState() {
  if (!currentGameId) return;

  try {
    const response = await fetch(`${API_BASE}/games/${currentGameId}`);

    if (!response.ok) {
      throw new Error('Failed to fetch game state');
    }

    const data = await response.json();
    currentGameState = data;

    renderGameState(data);
  } catch (error) {
    console.error('Error updating game state:', error);
  }
}

function renderGameState(state) {
  // Update opponent info
  document.getElementById('opponentCards').textContent = state.game_state.cards_in_opponent;
  document.getElementById('deckSize').textContent = state.game_state.num_cards_in_deck;

  // Update trump card
  const trumpCardEl = document.getElementById('trumpCard');
  if (state.game_state.visible_card) {
    trumpCardEl.innerHTML = renderCard(state.game_state.visible_card);
    trumpCardEl.className = `card ${state.game_state.visible_card.suit.toLowerCase()}`;
  }

  // Update attack table
  const attackTable = document.getElementById('attackTable');
  attackTable.innerHTML = '';
  state.game_state.attack_table.forEach(card => {
    const cardEl = document.createElement('div');
    cardEl.className = `card ${card.suit.toLowerCase()}`;
    cardEl.innerHTML = renderCard(card);
    attackTable.appendChild(cardEl);
  });

  // Update defense table
  const defenseTable = document.getElementById('defenseTable');
  defenseTable.innerHTML = '';
  state.game_state.defense_table.forEach(card => {
    const cardEl = document.createElement('div');
    cardEl.className = `card ${card.suit.toLowerCase()}`;
    cardEl.innerHTML = renderCard(card);
    defenseTable.appendChild(cardEl);
  });

  // Update player hand
  const playerHand = document.getElementById('playerHand');
  playerHand.innerHTML = '';
  const isPlayerTurn = state.game_state.acting_player === 'Player1';

  // Get trump suit
  const trumpSuit = state.game_state.visible_card ? state.game_state.visible_card.suit : null;

  // Sort cards: by suit, with trump suit last (rightmost), and by rank within each suit
  const sortedHand = [...state.game_state.hand].sort((a, b) => {
    // Suit order: Spades, Hearts, Diamonds, Clubs (trump suit goes last)
    const suitOrder = ['Spades', 'Hearts', 'Diamonds', 'Clubs'];

    // Get suit indices, with trump suit always last
    const getSuitIndex = (suit) => {
      if (suit === trumpSuit) {
        return 999; // Trump suit always last
      }
      return suitOrder.indexOf(suit);
    };

    const suitIndexA = getSuitIndex(a.suit);
    const suitIndexB = getSuitIndex(b.suit);

    // First sort by suit
    if (suitIndexA !== suitIndexB) {
      return suitIndexA - suitIndexB;
    }

    // Within same suit, sort by rank (ascending)
    return a.rank - b.rank;
  });

  sortedHand.forEach((card, index) => {
    const cardEl = document.createElement('div');
    cardEl.className = `card ${card.suit.toLowerCase()}`;

    // Add visual indicator for trump suit cards
    if (card.suit === trumpSuit) {
      cardEl.classList.add('trump-card');
    }

    cardEl.innerHTML = renderCard(card);

    // Find if this card is a valid action
    const attackAction = state.legal_actions.find(a =>
      a.action_type === 'Attack' &&
      a.card &&
      a.card.rank === card.rank &&
      a.card.suit === card.suit
    );
    const defendAction = state.legal_actions.find(a =>
      a.action_type === 'Defend' &&
      a.card &&
      a.card.rank === card.rank &&
      a.card.suit === card.suit
    );

    // Make card clickable if it's a valid action and player's turn
    if (isPlayerTurn && (attackAction || defendAction)) {
      cardEl.classList.add('clickable');
      cardEl.style.cursor = 'pointer';
      cardEl.title = attackAction ? `Click to attack with this card` : `Click to defend with this card`;
      cardEl.addEventListener('click', () => {
        // Use the action's card to ensure format consistency
        const actionCard = attackAction ? attackAction.card : defendAction.card;
        const actionType = attackAction ? 'Attack' : 'Defend';
        makeMove({ action_type: actionType, card: actionCard });
      });
    } else {
      cardEl.style.cursor = 'default';
      cardEl.style.opacity = isPlayerTurn ? '0.6' : '1';
      if (isPlayerTurn) {
        cardEl.title = 'This card cannot be played right now';
      }
    }

    playerHand.appendChild(cardEl);
  });

  // Update actions
  renderActions(state.legal_actions);

  // Update action history from server
  renderHistory(state.action_history || []);

  // Update game status
  const statusEl = document.getElementById('gameStatus');
  if (state.is_over) {
    statusEl.textContent = state.winner ? `Game Over! Winner: ${state.winner}` : 'Game Over - Draw';
    statusEl.className = 'game-status winner';
  } else {
    const isPlayerTurn = state.game_state.acting_player === 'Player1';
    if (isPlayerTurn) {
      statusEl.textContent = 'Your turn - Make a move!';
      statusEl.className = 'game-status waiting';
    } else {
      statusEl.textContent = 'AI is thinking...';
      statusEl.className = 'game-status waiting';
    }
  }
}

function renderCard(card) {
  const suitSymbols = {
    'Spades': '♠',
    'Hearts': '♥',
    'Diamonds': '♦',
    'Clubs': '♣'
  };

  const rankDisplay = card.rank === 11 ? 'J' :
    card.rank === 12 ? 'Q' :
      card.rank === 13 ? 'K' :
        card.rank === 14 ? 'A' :
          card.rank.toString();

  return `
        <div class="card-rank">${rankDisplay}</div>
        <div class="card-suit">${suitSymbols[card.suit]}</div>
    `;
}

// Removed selectCard function - cards now directly perform actions

function renderActions(actions) {
  const actionsEl = document.getElementById('actions');
  actionsEl.innerHTML = '';

  // Only show non-card actions (StopAttack, Take)
  // Card actions are handled by clicking the cards directly
  const nonCardActions = actions.filter(action =>
    action.action_type === 'StopAttack' || action.action_type === 'Take'
  );

  if (nonCardActions.length === 0) {
    actionsEl.innerHTML = '<p style="color: #6c757d; font-style: italic;">Click a card to attack or defend</p>';
    return;
  }

  nonCardActions.forEach(action => {
    const btn = document.createElement('button');
    btn.className = 'action-btn';

    if (action.action_type === 'StopAttack') {
      btn.textContent = 'Stop Attack';
      btn.addEventListener('click', () => makeMove({ action_type: 'StopAttack', card: null }));
    } else if (action.action_type === 'Take') {
      btn.textContent = 'Take Cards';
      btn.addEventListener('click', () => makeMove({ action_type: 'Take', card: null }));
    }

    actionsEl.appendChild(btn);
  });
}

function formatCard(card) {
  if (!card) return '';
  const rankDisplay = card.rank === 11 ? 'J' :
    card.rank === 12 ? 'Q' :
      card.rank === 13 ? 'K' :
        card.rank === 14 ? 'A' :
          card.rank.toString();
  return `${rankDisplay}${card.suit[0]}`;
}

async function makeMove(action) {
  if (!currentGameId) return;

  try {
    const response = await fetch(`${API_BASE}/games/${currentGameId}/move`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(action),
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(error || 'Failed to make move');
    }

    const data = await response.json();
    currentGameState = data;

    renderGameState(data);
  } catch (error) {
    console.error('Error making move:', error);
    alert('Failed to make move: ' + error.message);
  }
}

// Poll for game state updates more frequently when it's AI's turn
setInterval(() => {
  if (currentGameId && currentGameState && !currentGameState.is_over) {
    // Poll more frequently if it's AI's turn
    const isPlayerTurn = currentGameState.game_state.acting_player === 'Player1';
    if (!isPlayerTurn) {
      // AI's turn - poll more frequently
      updateGameState();
    }
  }
}, 500); // Poll every 500ms for faster AI response

function formatAction(action) {
  if (!action) return '';

  switch (action.action_type) {
    case 'Attack':
      return `Attacked with ${formatCard(action.card)}`;
    case 'Defend':
      return `Defended with ${formatCard(action.card)}`;
    case 'Take':
      return 'Took cards';
    case 'StopAttack':
      return 'Stopped attack';
    default:
      return 'Made a move';
  }
}

function renderHistory(history) {
  const historyEl = document.getElementById('actionHistory');
  historyEl.innerHTML = '';

  if (!history || history.length === 0) {
    return;
  }

  // Render in reverse order (newest first)
  const reversed = [...history].reverse();

  reversed.forEach(entry => {
    const playerName = entry.player === 'Player1' ? 'You' : 'AI';
    const playerClass = entry.player === 'Player1' ? 'player1' : 'player2';
    const actionText = formatAction(entry.action);
    const timestamp = new Date(entry.timestamp * 1000).toLocaleTimeString();

    const itemEl = document.createElement('div');
    itemEl.className = `history-item ${playerClass}`;
    itemEl.innerHTML = `
      <div class="player-name">${playerName}</div>
      <div class="action-text">${actionText}</div>
      <div class="timestamp">${timestamp}</div>
    `;
    historyEl.appendChild(itemEl);
  });
}

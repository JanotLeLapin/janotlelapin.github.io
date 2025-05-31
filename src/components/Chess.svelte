<script lang="ts">
  import { onMount } from 'svelte';

  import * as poulet from '../lib/poulet'

  let board: poulet.Square[] = $state(Array(64).fill(null));

  let game: number | null = $state(null);
  let brains: number[] | null = $state(null);
  let turn: poulet.Color = $state('white');

  onMount(() => {
    const script = document.createElement('script');
    script.src = '/wasm/poulet.js';
    console.log('created script');
    script.onload = () => poulet.onInitialized(async () => {
      game = poulet.initGame();
      board = poulet.getBoard(game);
      brains = [poulet.initBrain(), poulet.initBrain()];
    });
    document.head.appendChild(script);
  })

  const next = () => {
    if (!brains || !game) {
      return;
    }

    const move = poulet.nextMove(game, turn === 'white' ? brains[0] : brains[1], turn, 0.6);
    if (!move) {
      console.log('checkmate!');
      return;
    }

    turn = turn === 'white' ? 'black' : 'white';
    poulet.doMove(game, move.src.x, move.src.y, move.dst.x, move.dst.y);
    board = poulet.getBoard(game);
  }
</script>

<div>
  <h1>Chess!</h1>
  <div class="board">
    {#each board as piece, i}
      <div class="square {((i + Math.floor(i / 8)) % 2 === 0) ? 'light' : 'dark'}">
        {#if piece}
          <img src="/chess/{piece.color}-{piece.piece}.svg" alt="{piece.color} {piece.piece}" />
        {/if}
      </div>
    {/each}
  </div>
  <button onclick={next}>Next</button>
</div>

<style>
.board {
  display: grid;
  grid-template-columns: repeat(8, 1fr);
  grid-template-rows: repeat(8, 1fr);
  width: 100vmin;
  height: 100vmin;
  max-width: 600px;
  max-height: 600px;
  margin: auto;
  border: 2px solid #333;
}

.square {
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 2rem;
  user-select: none;
}

.light {
  background: white;
}

.dark {
  background: blue;
}
</style>

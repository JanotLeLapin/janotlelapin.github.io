<script lang="ts">
  import { onMount } from 'svelte';

  import * as poulet from '../lib/poulet'

  let board: poulet.Square[] = $state(Array(64).fill(null));

  onMount(() => {
    const script = document.createElement('script');
    script.src = '/wasm/poulet.js';
    console.log('created script');
    script.onload = () => poulet.onInitialized(async () => {
      const gamePtr = poulet.initGame();
      board = poulet.getBoard(gamePtr);
      const brainPtr = poulet.initBrain();
      setInterval(() => {
        const whiteMove = poulet.nextMove(gamePtr, brainPtr, "white");
        if (whiteMove) {
          poulet.doMove(gamePtr, whiteMove.src.x, whiteMove.src.y, whiteMove.dst.x, whiteMove.dst.y);
        }

        const blackMove = poulet.nextMove(gamePtr, brainPtr, "black");
        if (blackMove) {
          poulet.doMove(gamePtr, blackMove.src.x, blackMove.src.y, blackMove.dst.x, blackMove.dst.y);
        }

        board = poulet.getBoard(gamePtr);
      }, 5000);
      // poulet.freeGame(gamePtr);
      // poulet.freeBrain(brainPtr);
    });
    document.head.appendChild(script);
  })
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

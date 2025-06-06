<script lang="ts">
  import { onMount } from 'svelte';

  import * as poulet from '../wasm'

  let board: poulet.PouletPiece[] = $state(Array(64).fill(null));
  let latestMove: poulet.PouletMove | null = $state(null);

  let game: poulet.PouletGame | null = $state(null);
  let brains: poulet.PouletNetwork[] | null = $state(null);

  onMount(async () => {
    const network = await fetch('/chess/brain.model').then(res => res.arrayBuffer())
    await poulet.default();
    game = poulet.PouletGame.new();
    board = game.get_board();
    brains = [poulet.PouletNetwork.load(new Uint8Array(network)) as poulet.PouletNetwork, poulet.PouletNetwork.init() as poulet.PouletNetwork];
  })

  const next = () => {
    if (!brains || !game) {
      return;
    }

    const move = (game.get_turn() == poulet.PouletColor.from_string("white") ? brains[0] : brains[1]).next_move(game, 1.0);
    if (!move) {
      console.log('end of game!');
      return;
    }

    game.do_move(move.get_src_x(), move.get_src_y(), move.get_dst_x(), move.get_dst_y());
    latestMove = move;
    board = game.get_board();
  }
</script>

<div>
  <h1>Chess!</h1>
  <div class="board">
    {#each board as piece, i}
      <div class="square {((i + Math.floor(i / 8)) % 2 === 0) ? 'light' : 'dark'} {latestMove && i == latestMove.get_dst_x() + latestMove.get_dst_y() * 8 ? 'dst' : ''} {latestMove && i == latestMove.get_src_x() + latestMove.get_src_y() * 8 ? 'src' : ''}">
        {#if piece?.is_some()}
          <img src="/chess/{piece.get_color()?.to_string()}-{piece.get_piece_type()?.to_string()}.svg" alt="{piece.get_color()?.to_string()} and {piece.get_piece_type()?.to_string()}" />
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
  transition: background ease-in-out 50ms;
}

.square.light {
  background: oklch(94.8% 0.028 342.258);
}

.square.dark {
  background: oklch(40.8% 0.153 2.432);
}

.square.src {
  background: oklch(81% 0.117 11.638);
}

.square.dst {
  background: oklch(58.6% 0.253 17.585);
}
</style>

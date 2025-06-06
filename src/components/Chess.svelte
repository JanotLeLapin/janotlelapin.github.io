<script lang="ts">
  import { onMount } from 'svelte';

  import * as poulet from '../wasm'

  type Game = {
    game: poulet.PouletGame;
    controller: poulet.PouletNetwork | poulet.PouletNetwork[];
  };

  type Position = {
    x: number,
    y: number,
  };

  type Move = {
    src: Position,
    dst: Position,
  };

  let board: poulet.PouletPiece[] = $state(Array(64).fill(null));
  let latestMove: Move | null = $state(null);

  let game: Game | null = $state(null);

  let click: Position | null = $state(null);

  onMount(async () => {
    const params = await fetch('/chess/brain.model').then(res => res.arrayBuffer())
    await poulet.default();

    const smartNetwork = poulet.PouletNetwork.load(new Uint8Array(params));
    if (!smartNetwork) {
      console.error('could not load smart neural network');
      return;
    }

    game = {
      game: poulet.PouletGame.new(),
      controller: smartNetwork,
    };
    board = game.game.get_board();
  })

  const next = () => {
    if (!game) {
      return;
    }

    let move: poulet.PouletMove;

    if (Array.isArray(game.controller)) {
      const tmp = (game.game.get_turn().to_string() == 'white' ? game.controller[0] : game.controller[1]).next_move(game.game, game.game.get_turn().to_string() == 'white' ? 0.1 : 1.0);
      if (!tmp) {
        console.log('end of game!');
        return;
      }
      move = tmp;
    } else {
      if (game.game.get_turn().to_string() !== 'white') {
        return;
      }

      const tmp = game.controller.next_move(game.game, 1.0);
      if (!tmp) {
        console.log('end of game!');
        return;
      }
      move = tmp;
    }

    game.game.do_move(move.get_src_x(), move.get_src_y(), move.get_dst_x(), move.get_dst_y());
    latestMove = {
      src: { x: move.get_src_x(), y: move.get_src_y() },
      dst: { x: move.get_dst_x(), y: move.get_dst_y() },
    };
    board = game.game.get_board();
  }

  const clickSquare = (x: number, y: number) => {
    if (!game || Array.isArray(game.controller)) {
      return;
    }

    if (click == null) {
      click = { x, y }
    } else {
      if (!game.game.is_move_safe(click.x, click.y, x, y)) {
        click = null;
        return;
      }

      game.game.do_move(click.x, click.y, x, y);
      latestMove = {
        src: { x: click.x, y: click.y },
        dst: { x, y },
      }
      board = game.game.get_board();
      click = null;
    }
  }
</script>

<div>
  <h1>Chess!</h1>
  <div class="board">
    {#each board as piece, i}
      <button onclick={() => clickSquare(i % 8, Math.floor(i / 8))} class="square {((i + Math.floor(i / 8)) % 2 === 0) ? 'light' : 'dark'} {latestMove && i == latestMove.dst.x + latestMove.dst.y * 8 ? 'dst' : ''} {latestMove && i == latestMove.src.x + latestMove.src.y * 8 ? 'src' : ''} {click && i == click.x + click.y * 8 ? 'click' : ''}">
        {#if piece?.is_some()}
          <img src="/chess/{piece.get_color()?.to_string()}-{piece.get_piece_type()?.to_string()}.svg" alt="{piece.get_color()?.to_string()} and {piece.get_piece_type()?.to_string()}" />
        {/if}
      </button>
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
  border-radius: 1rem;
}

.square {
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font: inherit;
  font-size: 2rem;
  color: inherit;
  user-select: none;
  border: none;
  outline: inherit;
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

.square.click {
  background: oklch(76.5% 0.177 163.223);
}
</style>

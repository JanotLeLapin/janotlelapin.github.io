export type Move = 'illegal' | 'legal' | 'take' | 'takeEnPassant' | 'castle' | 'unsafe';
export type Color = 'black' | 'white';
export type Piece = 'pawn' | 'bishop' | 'knight' | 'rook' | 'queen' | 'king';

export type Square = {
  color: Color,
  piece: Piece,
} | null;

export type Position = {
  x: number,
  y: number,
};

const INDEXED_MOVES: Move[] = ['illegal', 'legal', 'take', 'takeEnPassant', 'castle', 'unsafe'];
const INDEXED_COLORS: Color[] = ['black', 'white'];
const INDEXED_PIECES: Piece[] = ['pawn', 'bishop', 'knight', 'rook', 'queen', 'king'];

const getModule = (): any => (window as any).Module;

export const onInitialized = (callback: () => Promise<any>) => {
  getModule().onRuntimeInitialized = callback;
}

export const initGame = (): number => {
  const gameSize = getModule().ccall('w_chess_game_t_size', 'number');
  const gamePtr: number = getModule()._malloc(gameSize);

  getModule().ccall('chess_init', null, ['number'], [gamePtr]);

  return gamePtr;
}

export const getBoard = (gamePtr: number): Square[] => {
  const res = Array(64).fill(null);

  const mem = new Uint8Array(getModule().HEAPU8.buffer, gamePtr, 64);
  for (let i = 0; i < 64; i++) {
    if (!mem[i]) {
      continue;
    }

    res[i] = {
      color: INDEXED_COLORS[mem[i] >> 3],
      piece: INDEXED_PIECES[(mem[i] & 0x07) - 1],
    }
  }

  return res;
}

export const isMoveSafe = (gamePtr: number, ax: number, ay: number, bx: number, by: number): Move => {
  const move = getModule().ccall(
    'chess_safe_move',
    'number',
    ['number', 'number', 'number', 'number', 'number'],
    [gamePtr, ax, ay, bx, by],
  );

  return INDEXED_MOVES[move];
}

export const doMove = (gamePtr: number, ax: number, ay: number, bx: number, by: number) => {
  getModule().ccall(
    'chess_do_move',
    null,
    ['number', 'number', 'number', 'number', 'number'],
    [gamePtr, ax, ay, bx, by],
  );
}

export const initBrain = (): number => {
  const brainSize = getModule().ccall('w_ai_brain_t_size', 'number');
  const brainPtr: number = getModule()._malloc(brainSize);

  getModule().ccall('poulet_brain_init', null, ['number'], [brainPtr]);

  return brainPtr;
}

export const nextMove = (gamePtr: number, brainPtr: number, color: Color, temperature: number = 0.6): { src: Position, dst: Position } | null => {
  const res = getModule()._malloc(4);

  const hasMove = getModule().ccall(
    'poulet_next_move',
    'number',
    ['number', 'number', 'number', 'number', 'number'],
    [res, gamePtr, brainPtr, INDEXED_COLORS.indexOf(color), temperature],
  );

  if (-1 == hasMove) {
    getModule()._free(res);
    return null;
  }

  const mem = new Uint8Array(getModule().HEAPU8.buffer, res, 4);
  const src: Position = {
    x: mem[1],
    y: mem[0],
  };
  const dst: Position = {
    x: mem[3],
    y: mem[2],
  };

  getModule()._free(res);

  return { src, dst };
}

export const freeGame = (gamePtr: number) => {
  getModule()._free(gamePtr);
}

export const freeBrain = (brainPtr: number) => {
  getModule().ccall('ai_brain_free', null, ['number'], [brainPtr]);
  getModule()._free(brainPtr);
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct PouletMove {
    src_x: u8,
    src_y: u8,
    dst_x: u8,
    dst_y: u8,
}

#[wasm_bindgen]
impl PouletMove {
    #[wasm_bindgen]
    pub fn get_src_x(&self) -> u8 {
        self.src_x
    }
    #[wasm_bindgen]
    pub fn get_src_y(&self) -> u8 {
        self.src_y
    }
    #[wasm_bindgen]
    pub fn get_dst_x(&self) -> u8 {
        self.dst_x
    }
    #[wasm_bindgen]
    pub fn get_dst_y(&self) -> u8 {
        self.dst_y
    }
}

#[wasm_bindgen]
pub struct PouletColor(poulet::chess::Color);

#[wasm_bindgen]
impl PouletColor {
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        match self.0 {
            poulet::chess::Color::White => "white",
            poulet::chess::Color::Black => "black",
        }
        .to_string()
    }

    #[wasm_bindgen]
    pub fn from_string(color: &str) -> Option<Self> {
        match color {
            "white" => Some(PouletColor(poulet::chess::Color::White)),
            "black" => Some(PouletColor(poulet::chess::Color::Black)),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub struct PouletPieceType(poulet::chess::PieceType);

#[wasm_bindgen]
impl PouletPieceType {
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        match self.0 {
            poulet::chess::PieceType::Pawn => "pawn",
            poulet::chess::PieceType::Bishop => "bishop",
            poulet::chess::PieceType::Knight => "knight",
            poulet::chess::PieceType::Rook => "rook",
            poulet::chess::PieceType::Queen => "queen",
            poulet::chess::PieceType::King => "king",
        }
        .to_string()
    }
}

#[wasm_bindgen]
pub struct PouletPiece(Option<poulet::chess::Piece>);

#[wasm_bindgen]
impl PouletPiece {
    #[wasm_bindgen]
    pub fn get_color(&self) -> Option<PouletColor> {
        Some(PouletColor(self.0?.color))
    }

    #[wasm_bindgen]
    pub fn get_piece_type(&self) -> Option<PouletPieceType> {
        Some(PouletPieceType(self.0?.piece_type))
    }

    #[wasm_bindgen]
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }
}

#[wasm_bindgen]
pub struct PouletGame(poulet::chess::Game);

#[wasm_bindgen]
impl PouletGame {
    #[wasm_bindgen]
    pub fn new() -> Self {
        PouletGame(poulet::chess::Game::default())
    }

    #[wasm_bindgen]
    pub fn get_board(&self) -> Vec<PouletPiece> {
        self.0
            .board
            .0
            .iter()
            .map(|square| PouletPiece(square.clone()))
            .collect()
    }

    #[wasm_bindgen]
    pub fn get_turn(&self) -> PouletColor {
        PouletColor(self.0.turn)
    }

    #[wasm_bindgen]
    pub fn is_move_safe(&mut self, src_x: u8, src_y: u8, dst_x: u8, dst_y: u8) -> bool {
        self.0.safe_move(src_x, src_y, dst_x, dst_y)
    }

    #[wasm_bindgen]
    pub fn do_move(&mut self, src_x: u8, src_y: u8, dst_x: u8, dst_y: u8) {
        self.0.do_move(src_x, src_y, dst_x, dst_y)
    }
}

#[wasm_bindgen]
struct PouletNetwork(poulet::ai::Network);

#[wasm_bindgen]
impl PouletNetwork {
    #[wasm_bindgen]
    pub fn init() -> Option<Self> {
        let network = match poulet::new_chess_network() {
            Ok(v) => v,
            Err(_) => return None,
        };
        Some(PouletNetwork(network))
    }

    #[wasm_bindgen]
    pub fn next_move(&mut self, game: &mut PouletGame) -> Option<PouletMove> {
        let res = match poulet::next_move(&mut self.0, &mut game.0) {
            Ok(v) => v,
            Err(_) => return None,
        }?;

        Some(PouletMove {
            src_x: res.src.0,
            src_y: res.src.1,
            dst_x: res.dst.0,
            dst_y: res.dst.1,
        })
    }
}

#[wasm_bindgen]
pub fn hello(name: &str) {
    log(&format!("hello, {}!", name));
}

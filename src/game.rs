use super::board::{Coordinate, GamePiece, Move, PieceColor};

pub struct GameEngine {
    board: [[Option<GamePiece>; 8]; 8],
    current_turn: PieceColor,
    move_count: u32,
}

pub struct MoveResult {
    pub mv: Move,
    pub crowned: bool,
}

impl GameEngine {
    pub fn new() -> GameEngine {
        let mut engine = GameEngine {
            board: [[None; 8]; 8],
            current_turn: PieceColor::Black,
            move_count: 0,
        };
        engine.initialize_pieces();
        engine
    }

    pub fn initialize_pieces(&mut self) {
        [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7]
            .iter()
            .zip([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2].iter())
            .map(|(a, b)| (*a as usize, *b as usize))
            .for_each(|(x, y)| self.board[x][y] = Some(GamePiece::new(PieceColor::White)));

        [0, 2, 4, 6, 1, 3, 5, 7, 0, 2, 4, 6]
            .iter()
            .zip([5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7].iter())
            .map(|(a, b)| (*a as usize, *b as usize))
            .for_each(|(x, y)| self.board[x][y] = Some(GamePiece::new(PieceColor::Black)))
    }

    pub fn move_piece(&mut self, mv: &Move) -> Result<MoveResult, ()> {
        let legal_moves = self.legal_moves();
        if !legal_moves.contains(mv) {
            return Err(());
        }
        let Coordinate(fx, fy) = mv.from;
        let Coordinate(tx, ty) = mv.to;
        let piece = self.board[fx][fy].unwrap();
        let midpiece_coordinate = self.midpiece_coordinate(fx, fy, tx, ty);
        if let Some(Coordinate(x, y)) = midpiece_coordinate {
            self.board[x][y] = None; // remove the jumped piece
        }
        // Move piece from source to dest
        self.board[tx][ty] = Some(piece);
        self.board[fx][fy] = None;
        let crowned = if self.should_crown(piece, mv.to) {
            self.crown_piece(mv.to);
            true
        } else {
            false
        };
        self.advance_turn();
        Ok(MoveResult {
            mv: mv.clone(),
            crowned: crowned,
        })
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for col in 0..8 {
            for row in 0..8 {
                if let Some(piece) = self.board[col][row] {
                    if piece.color == self.current_turn {
                        let loc = Coordinate(col, row);
                        let mut vmoves = self.valid_moves_from(loc);
                        moves.append(&mut vmoves);
                    }
                }
            }
        }
        moves
    }

    fn valid_moves_from(&self, loc: Coordinate) -> Vec<Move> {
        let Coordinate(x, y) = loc;
        if let Some(p) = self.board[x][y] {
            let mut jumps = loc
                .jump_targets_from()
                .filter(|t| self.valid_jump(&p, loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone(),
                }).collect::<Vec<Move>>();

            let mut moves = loc
                .move_targets_from()
                .filter(|t| self.valid_move(&p, loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone(),
                }).collect::<Vec<Move>>();
            jumps.append(&mut moves);
            jumps
        } else {
            Vec::new()
        }
    }

    fn midpiece_coordinate(&self, fx: usize, fy: usize, tx: usize, ty: usize) -> Option<Coordinate> {
        if (fx as isize - tx as isize).abs() == 2 && (fy as isize - ty as isize).abs() == 2 {
            Some(Coordinate((fx + tx) / 2, (fy + ty) / 2))
        } else {
            None
        }
    }

    fn should_crown(&self, piece: GamePiece, to: Coordinate) -> bool {
        match piece.color {
            PieceColor::White => to.1 == 7,
            PieceColor::Black => to.1 == 0,
        }
    }

    fn crown_piece(&mut self, coord: Coordinate) {
        if let Some(piece) = &mut self.board[coord.0][coord.1] {
            *piece = GamePiece::crowned(piece.clone());
        }
    }

    fn advance_turn(&mut self) {
        self.current_turn = match self.current_turn {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };
        self.move_count += 1;
    }

    fn valid_move(&self, piece: &GamePiece, from: Coordinate, to: &Coordinate) -> bool {
        let Coordinate(tx, ty) = *to;
        if tx < 8 && ty < 8 {
            self.board[tx][ty].is_none()
        } else {
            false
        }
    }

    fn valid_jump(&self, piece: &GamePiece, from: Coordinate, to: &Coordinate) -> bool {
        let Coordinate(fx, fy) = from;
        let Coordinate(tx, ty) = *to;
        if (fx as isize - tx as isize).abs() == 2 && (fy as isize - ty as isize).abs() == 2 {
            if let Some(Coordinate(mx, my)) = self.midpiece_coordinate(fx, fy, tx, ty) {
                if let Some(mid_piece) = self.board[mx][my] {
                    return mid_piece.color != piece.color;
                }
            }
        }
        false
    }
    pub fn current_turn(&self) -> PieceColor {
        self.current_turn
    }

    pub fn get_piece(&self, coord: Coordinate) -> Result<Option<GamePiece>, Box<dyn std::error::Error>> {
        // Your implementation here
        Ok(None)
    }
}

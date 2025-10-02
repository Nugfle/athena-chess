fn check() {
    let (p, c) = *match self.board.get_piece_on_square(from) {
        Some(v) => v,
        None => return Err(IllegalMoveError::EmptySquare { square: from }),
    };

    if p != mv.get_piece() {
        return Err(IllegalMoveError::DifferentPiece {
            expected: mv.get_piece(),
            found: p,
        });
    }

    // make sure the color of the piece in the move matches the player whos turn it is
    if c != self.turn {
        return Err(IllegalMoveError::NotYourPiece { color: c, square: from });
    }

    // filter out all moves that would take own piece
    if self.board.get_piece_on_square(to).is_some_and(|(_, col)| *col == c) {
        return Err(IllegalMoveError::TakesOwnPiece {
            mv,
            piece: self.board.get_piece_on_square(to).unwrap().0,
        });
    }

    // check whether the move is valid for the type of piece
    match mv.get_piece() {
        Piece::Pawn => {
            self.pawn_move(&mut mv)?;
        }

        Piece::King { has_moved } => {
            if from.get_delta_rank(to).abs() > 1 {
                return Err(IllegalMoveError::MoveInvalid { mv });
            }
            // we can at most move 2 squares
            if from.get_delta_file(to).abs() > 2 {
                return Err(IllegalMoveError::MoveInvalid { mv });
            }
            // handles castling
            if from.get_delta_file(to).abs() == 2 {
                // if we castle we can only move on the same rank and the king must not have
                // moved
                if has_moved || from.get_delta_rank(to) != 0 {
                    return Err(IllegalMoveError::MoveInvalid { mv });
                } else if from.get_delta_file(to) == -3 {
                    // long castle
                    self.long_castle(from, mv)?;
                } else if from.get_delta_file(to) == 2 {
                    // short castle
                    self.short_castle(from, mv)?;
                } else {
                    return Err(IllegalMoveError::MoveInvalid { mv });
                }
            }
        }

        Piece::Knight => {
            if !ATTACK_TABLES.get_attack_pattern_knight(from).contains(to) {
                return Err(IllegalMoveError::MoveInvalid { mv });
            }
        }

        Piece::Bishop => {
            if !ATTACK_TABLES.get_attack_pattern_bishop(from, self.board.occupancy).contains(to) {
                return Err(IllegalMoveError::MoveInvalid { mv });
            }
        }

        Piece::Rook { .. } => {
            if !ATTACK_TABLES.get_attack_pattern_rook(from, self.board.occupancy).contains(to) {
                return Err(IllegalMoveError::MoveInvalid { mv });
            }
        }

        Piece::Queen => {
            if !ATTACK_TABLES.get_attack_pattern_queen(from, self.board.occupancy).contains(to) {
                return Err(IllegalMoveError::MoveInvalid { mv });
            }
        }
    }
}

fn short_castle(&mut self, from: Square, mv: Move) -> Result<(), IllegalMoveError> {
    let rook_sq = Square::from_rank_file(from.get_rank(), File::H);
    if let Some((pc, col)) = self.board.get_piece_on_square_mut(rook_sq) {
        if *col != self.turn {
            return Err(IllegalMoveError::NotYourPiece {
                color: *col,
                square: rook_sq,
            });
        }
        match pc {
            Piece::Rook { has_moved } if !*has_moved => {
                let f = Square::from_rank_file(from.get_rank(), File::F);
                let g = Square::from_rank_file(from.get_rank(), File::G);
                let e = Square::from_rank_file(from.get_rank(), File::E);

                if self.board.square_is_controlled_by(e, !self.turn) {
                    return Err(IllegalMoveError::IsInCheck);
                }

                if self.board.is_occupied(f) || self.board.is_occupied(g) {
                    return Err(IllegalMoveError::Blocked { mv, square: mv.get_to() });
                }

                // we have a clear line to an unmoved rook
                // now we need to check whether the fields the king is moving through are under
                // attack by an enemy piece
                if self.board.square_is_controlled_by(f, !self.turn) || self.board.square_is_controlled_by(g, !self.turn) {
                    return Err(IllegalMoveError::MoveInvalid { mv });
                }

                let (mut rook, col) = self.board.remove_piece_from_square(rook_sq).unwrap();
                rook.make_moved();

                self.board.place_piece_on_square(rook, col, f);
                Ok(())
            }
            Piece::Rook { has_moved } if *has_moved => Err(IllegalMoveError::MoveInvalid { mv }),
            _ => Err(IllegalMoveError::DifferentPiece {
                expected: Piece::Rook { has_moved: false },
                found: *pc,
            }),
        }
    } else {
        Err(IllegalMoveError::EmptySquare { square: rook_sq })
    }
}

fn long_castle(&mut self, from: Square, mv: Move) -> Result<(), IllegalMoveError> {
    let rook_sq = Square::from_rank_file(from.get_rank(), File::A);
    if let Some((pc, col)) = self.board.get_piece_on_square(rook_sq) {
        if *col != self.turn {
            return Err(IllegalMoveError::NotYourPiece {
                color: *col,
                square: rook_sq,
            });
        }
        match pc {
            Piece::Rook { has_moved } if !has_moved => {
                let b = Square::from_rank_file(from.get_rank(), File::B);
                let c = Square::from_rank_file(from.get_rank(), File::C);
                let d = Square::from_rank_file(from.get_rank(), File::D);
                let e = Square::from_rank_file(from.get_rank(), File::E);

                if self.board.is_occupied(b) || self.board.is_occupied(c) || self.board.is_occupied(d) {
                    return Err(IllegalMoveError::Blocked { mv, square: mv.get_to() });
                }

                if self.board.square_is_controlled_by(e, !self.turn) {
                    return Err(IllegalMoveError::IsInCheck);
                }

                // we have a clear line to an unmoved rook
                // now we need to check whether the fields the king is moving accross are not
                // under attack
                if self.board.square_is_controlled_by(c, !self.turn) || self.board.square_is_controlled_by(d, !self.turn) {
                    return Err(IllegalMoveError::MoveInvalid { mv });
                }

                let (rook, col) = self.board.remove_piece_from_square(rook_sq).unwrap();
                self.board.place_piece_on_square(rook, col, d);
                Ok(())
            }
            Piece::Rook { has_moved } if *has_moved => Err(IllegalMoveError::MoveInvalid { mv }),
            _ => Err(IllegalMoveError::DifferentPiece {
                expected: Piece::Rook { has_moved: false },
                found: *pc,
            }),
        }
    } else {
        Err(IllegalMoveError::EmptySquare { square: rook_sq })
    }
}

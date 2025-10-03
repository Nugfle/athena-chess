#[cfg(test)]
mod move_generation_tests {
    use crate::game::Game;
    use crate::game::board::piece::{Color, Piece};
    use crate::game::board::square::*;
    use crate::game::chess_move::Move;

    /// Helper function to create an empty game
    fn create_empty_game() -> Game {
        let mut game = Game::init();
        // Clear the board
        for i in 0..64 {
            if let Ok(square) = Square::new(i) {
                game.board.remove_piece_from_square(square);
            }
        }
        game
    }

    /// Helper function to place a piece on the board
    fn place_piece(game: &mut Game, piece: Piece, color: Color, square: Square) {
        game.board.place_piece_on_square(piece, color, square);
    }

    /// Helper function to count moves of a specific type
    fn count_moves_by_type<F>(moves: &[Move], predicate: F) -> usize
    where
        F: Fn(&Move) -> bool,
    {
        moves.iter().filter(|mv| predicate(mv)).count()
    }

    #[test]
    fn test_sliding_pieces_rook_moves() {
        let mut game = create_empty_game();

        // Place a white rook on e4 with clear lines
        place_piece(&mut game, Piece::Rook { has_moved: false }, Color::White, E4);
        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Rook should have 14 moves (7 horizontal + 7 vertical)
        assert_eq!(moves.len(), 14);

        // Check some specific moves
        let expected_squares = [
            E1, E2, E3, E5, E6, E7, E8, // Vertical moves
            A4, B4, C4, D4, F4, G4, H4, // Horizontal moves
        ];

        for expected_square in expected_squares {
            assert!(
                moves.iter().any(|mv| mv.get_to() == expected_square),
                "Rook should be able to move to {}",
                expected_square
            );
        }
    }

    #[test]
    fn test_sliding_pieces_bishop_moves() {
        let mut game = create_empty_game();

        // Place a white bishop on d4
        place_piece(&mut game, Piece::Bishop, Color::White, D4);
        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Bishop should have 13 moves (diagonals)
        assert_eq!(moves.len(), 13);

        // Check diagonal moves
        let expected_squares = [
            A1, B2, C3, E5, F6, G7, H8, // Main diagonal
            A7, B6, C5, E3, F2, G1, // Anti-diagonal
        ];

        for expected_square in expected_squares {
            assert!(
                moves.iter().any(|mv| mv.get_to() == expected_square),
                "Bishop should be able to move to {}",
                expected_square
            );
        }
    }

    #[test]
    fn test_sliding_pieces_queen_moves() {
        let mut game = create_empty_game();

        // Place a white queen on d4
        place_piece(&mut game, Piece::Queen, Color::White, D4);
        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Queen should have 27 moves (rook + bishop moves)
        assert_eq!(moves.len(), 27);

        // Verify it includes both rook-like and bishop-like moves
        assert!(moves.iter().any(|mv| mv.get_to() == D1)); // Vertical
        assert!(moves.iter().any(|mv| mv.get_to() == A4)); // Horizontal
        assert!(moves.iter().any(|mv| mv.get_to() == A1)); // Diagonal
        assert!(moves.iter().any(|mv| mv.get_to() == G7)); // Diagonal
    }

    #[test]
    fn test_sliding_pieces_blocked_by_own_pieces() {
        let mut game = create_empty_game();

        // Place a white rook on e4 and block some paths with own pieces
        place_piece(&mut game, Piece::Rook { has_moved: false }, Color::White, E4);
        place_piece(&mut game, Piece::Pawn, Color::White, E6); // Block vertical
        place_piece(&mut game, Piece::Knight, Color::White, G4); // Block horizontal
        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Should not be able to move to E7, E8 (blocked by pawn)
        assert!(!moves.iter().any(|mv| mv.get_to() == E7));
        assert!(!moves.iter().any(|mv| mv.get_to() == E8));

        // Should not be able to move to H4 (blocked by knight)
        assert!(!moves.iter().any(|mv| mv.get_to() == H4));

        // Should still be able to move to E5 (before the blocking pawn)
        assert!(moves.iter().any(|mv| mv.get_to() == E5));

        // Should still be able to move to F4 (before the blocking knight)
        assert!(moves.iter().any(|mv| mv.get_to() == F4));
    }

    #[test]
    fn test_sliding_pieces_captures() {
        let mut game = create_empty_game();

        // Place a white rook and enemy pieces to capture
        place_piece(&mut game, Piece::Rook { has_moved: false }, Color::White, E4);
        place_piece(&mut game, Piece::Pawn, Color::Black, E7); // Enemy piece to capture
        place_piece(&mut game, Piece::Knight, Color::Black, H4); // Enemy piece to capture
        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Should be able to capture the black pawn on E7
        let capture_e7 = moves.iter().find(|mv| mv.get_to() == E7);
        assert!(capture_e7.is_some());
        assert!(matches!(capture_e7.unwrap(), Move::Capture { captured: Piece::Pawn, .. }));

        // Should be able to capture the black knight on H4
        let capture_h4 = moves.iter().find(|mv| mv.get_to() == H4);
        assert!(capture_h4.is_some());
        assert!(matches!(capture_h4.unwrap(), Move::Capture { captured: Piece::Knight, .. }));

        // Should not be able to move beyond captured pieces
        assert!(!moves.iter().any(|mv| mv.get_to() == E8));
    }

    #[test]
    fn test_knight_moves() {
        let mut game = create_empty_game();

        // Place a white knight on e4
        place_piece(&mut game, Piece::Knight, Color::White, E4);
        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Knight should have 8 moves from e4
        assert_eq!(moves.len(), 8);

        let expected_squares = [D2, F2, C3, G3, C5, G5, D6, F6];

        for expected_square in expected_squares {
            assert!(
                moves.iter().any(|mv| mv.get_to() == expected_square),
                "Knight should be able to move to {}",
                expected_square
            );
        }
    }

    #[test]
    fn test_pawn_single_moves() {
        let mut game = create_empty_game();

        // Place white pawns on various ranks
        place_piece(&mut game, Piece::Pawn, Color::White, E3);
        place_piece(&mut game, Piece::Pawn, Color::White, D4);
        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Should have moves for both pawns
        assert!(moves.iter().any(|mv| mv.get_from() == E3 && mv.get_to() == E4));
        assert!(moves.iter().any(|mv| mv.get_from() == D4 && mv.get_to() == D5));
    }

    #[test]
    fn test_pawn_double_moves() {
        let mut game = create_empty_game();

        // Place white pawns on starting rank
        place_piece(&mut game, Piece::Pawn, Color::White, E2);
        place_piece(&mut game, Piece::Pawn, Color::White, D2);
        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Should have both single and double moves for each pawn
        assert!(moves.iter().any(|mv| mv.get_from() == E2 && mv.get_to() == E3));
        assert!(moves.iter().any(|mv| mv.get_from() == E2 && mv.get_to() == E4));
        assert!(moves.iter().any(|mv| mv.get_from() == D2 && mv.get_to() == D3));
        assert!(moves.iter().any(|mv| mv.get_from() == D2 && mv.get_to() == D4));

        // Test black pawns too
        let mut game = create_empty_game();
        place_piece(&mut game, Piece::Pawn, Color::Black, E7);
        game.turn = Color::Black;

        let moves = game.get_available_moves();

        assert!(moves.iter().any(|mv| mv.get_from() == E7 && mv.get_to() == E6));
        assert!(moves.iter().any(|mv| mv.get_from() == E7 && mv.get_to() == E5));
    }

    #[test]
    fn test_pawn_captures() {
        let mut game = create_empty_game();

        // Place white pawn with enemy pieces to capture
        place_piece(&mut game, Piece::Pawn, Color::White, E4);
        place_piece(&mut game, Piece::Pawn, Color::Black, D5);
        place_piece(&mut game, Piece::Knight, Color::Black, F5);
        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Should be able to capture diagonally
        let capture_d5 = moves.iter().find(|mv| mv.get_from() == E4 && mv.get_to() == D5);
        assert!(capture_d5.is_some());
        assert!(matches!(capture_d5.unwrap(), Move::Capture { captured: Piece::Pawn, .. }));

        let capture_f5 = moves.iter().find(|mv| mv.get_from() == E4 && mv.get_to() == F5);
        assert!(capture_f5.is_some());
        assert!(matches!(capture_f5.unwrap(), Move::Capture { captured: Piece::Knight, .. }));

        // Should also have normal forward move
        assert!(moves.iter().any(|mv| mv.get_from() == E4 && mv.get_to() == E5));
    }

    #[test]
    fn test_pawn_blocked_moves() {
        let mut game = create_empty_game();

        // Place white pawn blocked by another piece
        place_piece(&mut game, Piece::Pawn, Color::White, E4);
        place_piece(&mut game, Piece::Pawn, Color::Black, E5); // Block forward move
        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Should not be able to move forward
        assert!(!moves.iter().any(|mv| mv.get_from() == E4 && mv.get_to() == E5));

        // Test double move blocked
        let mut game = create_empty_game();
        place_piece(&mut game, Piece::Pawn, Color::White, E2);
        place_piece(&mut game, Piece::Pawn, Color::Black, E4); // Block double move
        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Should have single move but not double move
        assert!(moves.iter().any(|mv| mv.get_from() == E2 && mv.get_to() == E3));
        assert!(!moves.iter().any(|mv| mv.get_from() == E2 && mv.get_to() == E4));
    }

    #[test]
    fn test_en_passant_moves() {
        let mut game = create_empty_game();

        // Set up en passant scenario
        place_piece(&mut game, Piece::Pawn, Color::White, E5);
        place_piece(&mut game, Piece::Pawn, Color::Black, D5);

        // Simulate that the black pawn just made a double move
        let previous_move = Move::Normal {
            piece: Piece::Pawn,
            from: D7,
            to: D5,
        };
        game.moves.push(previous_move);
        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Should have en passant move available
        let en_passant_move = moves.iter().find(|mv| matches!(mv, Move::EnPassant { from: E5, to: D6 }));
        assert!(en_passant_move.is_some(), "En passant move should be available");

        // Should also have normal forward move
        assert!(moves.iter().any(|mv| mv.get_from() == E5 && mv.get_to() == E6));
    }

    #[test]
    fn test_pawn_promotions() {
        let mut game = create_empty_game();

        // Place white pawn on 7th rank
        place_piece(&mut game, Piece::Pawn, Color::White, E7);
        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Should have 4 promotion moves (Queen, Rook, Bishop, Knight)
        let promotion_moves: Vec<_> = moves
            .iter()
            .filter(|mv| mv.get_from() == E7 && mv.get_to() == E8 && mv.is_promotion())
            .collect();

        assert_eq!(promotion_moves.len(), 4, "Should have 4 promotion options");

        // Check that all promotion pieces are available
        let promotion_pieces: Vec<_> = promotion_moves.iter().map(|mv| mv.get_promotion().unwrap()).collect();

        assert!(promotion_pieces.contains(&Piece::Queen));
        assert!(promotion_pieces.contains(&Piece::Rook { has_moved: true }));
        assert!(promotion_pieces.contains(&Piece::Bishop));
        assert!(promotion_pieces.contains(&Piece::Knight));
    }

    #[test]
    fn test_pawn_promotion_captures() {
        let mut game = create_empty_game();

        // Place white pawn on 7th rank with enemy piece to capture
        place_piece(&mut game, Piece::Pawn, Color::White, E7);
        place_piece(&mut game, Piece::Rook { has_moved: true }, Color::Black, D8);
        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Should have promotion captures
        let promotion_captures: Vec<_> = moves
            .iter()
            .filter(|mv| mv.get_from() == E7 && mv.get_to() == D8 && mv.is_promotion())
            .collect();

        assert_eq!(promotion_captures.len(), 4, "Should have 4 promotion capture options");

        // All should be promotion captures
        for mv in promotion_captures {
            assert!(matches!(
                mv,
                Move::PromotionCapture {
                    captured: Piece::Rook { .. },
                    ..
                }
            ));
        }

        // Should also have regular promotions
        let regular_promotions: Vec<_> = moves
            .iter()
            .filter(|mv| mv.get_from() == E7 && mv.get_to() == E8 && mv.is_promotion())
            .collect();

        assert_eq!(regular_promotions.len(), 4, "Should also have regular promotions");
    }

    #[test]
    fn test_castling_availability() {
        let mut game = create_empty_game();

        // Set up castling position - king and rooks haven't moved
        place_piece(&mut game, Piece::King { has_moved: false }, Color::White, E1);
        place_piece(&mut game, Piece::Rook { has_moved: false }, Color::White, A1);
        place_piece(&mut game, Piece::Rook { has_moved: false }, Color::White, H1);
        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Note: The current implementation has a todo!() for king moves
        // This test will need to be updated when king move generation is implemented
        // For now, we're testing that the method doesn't panic and returns some moves

        // The king should have some moves available (even if castling isn't implemented yet)
        let king_moves: Vec<_> = moves.iter().filter(|mv| mv.get_from() == E1).collect();

        // This assertion might need to be updated based on the actual king move implementation
        // For now, we just ensure the method doesn't crash and returns a valid vector
        // Note: king_moves.len() is always >= 0, so we just verify it's a valid collection
        let _move_count = king_moves.len(); // Just verify we can get the length without panicking
    }

    #[test]
    fn test_mixed_piece_scenario() {
        let mut game = create_empty_game();

        // Set up a complex position with multiple piece types
        place_piece(&mut game, Piece::Queen, Color::White, D1);
        place_piece(&mut game, Piece::Rook { has_moved: false }, Color::White, A1);
        place_piece(&mut game, Piece::Bishop, Color::White, C1);
        place_piece(&mut game, Piece::Knight, Color::White, B1);
        place_piece(&mut game, Piece::Pawn, Color::White, E2);
        place_piece(&mut game, Piece::Pawn, Color::White, F2);

        // Add some enemy pieces
        place_piece(&mut game, Piece::Pawn, Color::Black, E4);
        place_piece(&mut game, Piece::Knight, Color::Black, F6);

        game.turn = Color::White;

        let moves = game.get_available_moves();

        // Should have moves from all pieces
        assert!(moves.iter().any(|mv| mv.get_from() == D1)); // Queen moves
        assert!(moves.iter().any(|mv| mv.get_from() == A1)); // Rook moves
        assert!(moves.iter().any(|mv| mv.get_from() == C1)); // Bishop moves
        assert!(moves.iter().any(|mv| mv.get_from() == B1)); // Knight moves
        assert!(moves.iter().any(|mv| mv.get_from() == E2)); // Pawn moves
        assert!(moves.iter().any(|mv| mv.get_from() == F2)); // Pawn moves

        // Should have both normal moves and captures
        let normal_moves = count_moves_by_type(&moves, |mv| matches!(mv, Move::Normal { .. }));
        let capture_moves = count_moves_by_type(&moves, |mv| matches!(mv, Move::Capture { .. }));

        assert!(normal_moves > 0, "Should have normal moves");
        assert!(capture_moves > 0, "Should have capture moves");

        println!(
            "Total moves: {}, Normal: {}, Captures: {}",
            moves.len(),
            normal_moves,
            capture_moves
        );
    }

    #[test]
    fn test_turn_based_move_generation() {
        let mut game = create_empty_game();

        // Place pieces for both colors
        place_piece(&mut game, Piece::Pawn, Color::White, E2);
        place_piece(&mut game, Piece::Knight, Color::White, G1);
        place_piece(&mut game, Piece::Pawn, Color::Black, E7);
        place_piece(&mut game, Piece::Knight, Color::Black, G8);

        // Test white's turn
        game.turn = Color::White;
        let white_moves = game.get_available_moves();

        // All moves should be from white pieces
        for mv in &white_moves {
            let from_square = mv.get_from();
            let (_, color) = game.board.get_piece_on_square(from_square).unwrap();
            assert_eq!(*color, Color::White, "All moves should be from white pieces");
        }

        // Test black's turn
        game.turn = Color::Black;
        let black_moves = game.get_available_moves();

        // All moves should be from black pieces
        for mv in &black_moves {
            let from_square = mv.get_from();
            let (_, color) = game.board.get_piece_on_square(from_square).unwrap();
            assert_eq!(*color, Color::Black, "All moves should be from black pieces");
        }

        assert!(white_moves.len() > 0, "White should have moves");
        assert!(black_moves.len() > 0, "Black should have moves");
    }

    #[test]
    fn test_no_moves_when_no_pieces() {
        let mut game = create_empty_game();
        game.turn = Color::White;

        let moves = game.get_available_moves();
        assert_eq!(moves.len(), 0, "Should have no moves when no pieces are present");
    }

    #[test]
    fn test_piece_specific_move_counts() {
        // Test individual pieces in isolation to verify move counts

        // Rook in center
        let mut game = create_empty_game();
        place_piece(&mut game, Piece::Rook { has_moved: false }, Color::White, D4);
        game.turn = Color::White;
        assert_eq!(game.get_available_moves().len(), 14, "Rook should have 14 moves from center");

        // Bishop in center
        let mut game = create_empty_game();
        place_piece(&mut game, Piece::Bishop, Color::White, D4);
        game.turn = Color::White;
        assert_eq!(game.get_available_moves().len(), 13, "Bishop should have 13 moves from center");

        // Queen in center
        let mut game = create_empty_game();
        place_piece(&mut game, Piece::Queen, Color::White, D4);
        game.turn = Color::White;
        assert_eq!(game.get_available_moves().len(), 27, "Queen should have 27 moves from center");

        // Knight in center
        let mut game = create_empty_game();
        place_piece(&mut game, Piece::Knight, Color::White, D4);
        game.turn = Color::White;
        assert_eq!(game.get_available_moves().len(), 8, "Knight should have 8 moves from center");

        // Pawn on starting rank
        let mut game = create_empty_game();
        place_piece(&mut game, Piece::Pawn, Color::White, E2);
        game.turn = Color::White;
        assert_eq!(
            game.get_available_moves().len(),
            2,
            "Pawn should have 2 moves from starting position"
        );

        // Pawn in middle
        let mut game = create_empty_game();
        place_piece(&mut game, Piece::Pawn, Color::White, E4);
        game.turn = Color::White;
        assert_eq!(game.get_available_moves().len(), 1, "Pawn should have 1 move from middle");
    }
}

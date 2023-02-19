use std::convert::TryFrom;

#[derive(Copy, Clone, Default)]
struct Board {
    isWhiteTurn: bool,
    bitboards: Bitboards
}

#[derive(Copy, Clone, Default)]
struct Bitboards {
    white_pieces: u64,
    black_pieces: u64,
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    kings: u64,
    empty: u64
}


fn build_board(fen: &str) -> Option<Board> {
    let sections: Vec<&str> = fen.split(' ').collect();
    match sections.len() {
        6 => {
            let mut board = Some(
                Default::default()
            );
            
            [piece_placement, side_to_move, castling_ability, en_passant_target_square, halfmove_clock, fullmove_counter]
                .iter()
                .fold(0, |acc, f| {
                    f(&mut board, sections[acc]);
                    acc + 1
                });

            board
        },
        _ => None
    }
}

fn piece_placement(board: &mut Option<Board>, piece_placement: &str) {
    if board.is_some() {
        let ranks: Vec<&str> = piece_placement.split('/').collect();
        match ranks.len() {
            8 => {
                for i in 7..0 {
                    piece_placement_rank(board, i, ranks[usize::try_from(i).unwrap()]);
                }
            },
            _ => *board = None
        }
        determine_empty(board);
    }
}

fn piece_placement_rank(board: &mut Option<Board>, rank: u8, rank_fen: &str) {
    if board.is_some() {
        let mut file = 0;
        for c in rank_fen.chars() {
            if file < 8 && board.is_some() {
                match c {
                    '1' => file += 1,
                    '2' => file += 2,
                    '3' => file += 3,
                    '4' => file += 4,
                    '5' => file += 5,
                    '6' => file += 6,
                    '7' => file += 7,
                    '8' => file += 8,
                    _ => {
                        let mut bitboards = &mut board.unwrap().bitboards;
                        match c {
                            'P' => place_piece(&mut bitboards.white_pieces, &mut bitboards.pawns, rank, file),
                            'N' => place_piece(&mut bitboards.white_pieces, &mut bitboards.knights, rank, file),
                            'B' => place_piece(&mut bitboards.white_pieces, &mut bitboards.bishops, rank, file),
                            'R' => place_piece(&mut bitboards.white_pieces, &mut bitboards.rooks, rank, file),
                            'Q' => place_piece(&mut bitboards.white_pieces, &mut bitboards.queens, rank, file),
                            'K' => place_piece(&mut bitboards.white_pieces, &mut bitboards.kings, rank, file),
                            'p' => place_piece(&mut bitboards.black_pieces, &mut bitboards.pawns, rank, file),
                            'n' => place_piece(&mut bitboards.black_pieces, &mut bitboards.knights, rank, file),
                            'b' => place_piece(&mut bitboards.black_pieces, &mut bitboards.bishops, rank, file),
                            'r' => place_piece(&mut bitboards.black_pieces, &mut bitboards.rooks, rank, file),
                            'q' => place_piece(&mut bitboards.black_pieces, &mut bitboards.queens, rank, file),
                            'k' => place_piece(&mut bitboards.black_pieces, &mut bitboards.kings, rank, file),
                            _ => *board = None
                        }
                        file += 1;
                    }
                }
            }
        }
        if file != 8 {
            *board = None
        }
    }
}

fn place_piece(colour: &mut u64, piece: &mut u64, rank: u8, file: u8) {
    let i = 1 << (8*rank + file);
    *colour = *colour | i;
    *piece = *piece | i;
}

fn determine_empty(board: &mut Option<Board>) {
    if board.is_some() {
        let mut bitboards = &mut board.unwrap().bitboards;
        bitboards.empty = ![bitboards.white_pieces, bitboards.black_pieces]
            .iter()
            .fold(0, |acc, b| acc | b);
    }
}

fn side_to_move(board: &mut Option<Board>, side_to_move: &str) {

}

fn castling_ability(board: &mut Option<Board>, castling_ability: &str) {

}

fn en_passant_target_square(board: &mut Option<Board>, en_passant_target_square: &str) {

}

fn halfmove_clock(board: &mut Option<Board>, halfmove_clock: &str) {

}

fn fullmove_counter(board: &mut Option<Board>, fullmove_counter: &str) {

}

fn generate_moves(board: &Board) -> Vec<Board> {
    generate_pseudo_legal_moves(board)
        .iter()
        .filter_map(|b| match is_legal(b) {
            true => Some(*b),
            false => None
        })
        .collect()
}

fn generate_pseudo_legal_moves(board: &Board) -> Vec<Board> {
    [push_pawns, move_knights, move_bishops, move_rooks, move_queens, move_kings]
        .iter()
        .map(|f| f(board))
        .flatten()
        .collect()
}

fn push_pawns(board: &Board) -> Vec<Board> {
    match board.isWhiteTurn {
        true => {
            let white_pawns = board.bitboards.white_pieces & board.bitboards.pawns;
            let single_pushes = (white_pawns << 8) & bitboards.empty; // promotion?
            
        },
        false => {
            Vec::new()
        }
    }
}

fn move_knights(board: &Board) -> Vec<Board> {
    Vec::new()
}

fn move_bishops(board: &Board) -> Vec<Board> {
    Vec::new()
}

fn move_rooks(board: &Board) -> Vec<Board> {
    Vec::new()
}

fn move_queens(board: &Board) -> Vec<Board> {
    Vec::new()
}

fn move_kings(board: &Board) -> Vec<Board> {
    Vec::new()
}

fn is_legal(board: &Board) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use crate::board::{Board, build_board, generate_moves};

    #[test]
    fn perft() {
        let expected = [1, 20];
        let mut nodes = Vec::new();
        for i in expected {
            nodes = next_nodes(nodes);
            assert_eq!(nodes.len(), i);
        }
    }

    fn next_nodes(nodes: Vec<Board>) -> Vec<Board> {
        match nodes.len() {
            0 => {
                let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
                let board = build_board(fen).unwrap();
                vec![board]
            },
            _ => {
                nodes
                    .iter()
                    .map(|b| generate_moves(b))
                    .flatten()
                    .collect()
            }
        }
    }
}

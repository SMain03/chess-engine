struct Board {

}

fn build_board(fen: String) -> Option<Board> {
    None
}

fn generate_moves(board: Board) -> Vec<Board> {
    generate_pseudo_legal_moves(board)
        .into_iter()
        .filter_map(|b| match is_legal(&b) {
            true => Some(b),
            false => None
        })
        .collect()
}

fn generate_pseudo_legal_moves(board: Board) -> Vec<Board> {
    Vec::new()
}

fn is_legal(board: &Board) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use crate::board::{Board, build_board};

    #[test]
    fn perft() {
        let expected = [1, 20];
        let mut nodes = Vec::new();
        for i in &expected {
            nodes = next_nodes(nodes);
            assert_eq!(nodes.len(), *i);
        }
    }

    fn next_nodes(nodes: Vec<Board>) -> Vec<Board> {
        match nodes.len() {
            0 => {
                let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
                let board = build_board(fen).unwrap();
                vec![board]
            },
            _ => Vec::new()
        }
    }
}

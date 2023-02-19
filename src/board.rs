struct Board {

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
}

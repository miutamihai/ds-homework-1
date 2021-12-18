pub enum Status {
    Dead = 0,
    Alive = 1,
}

pub type Board = Vec<Vec<Status>>;

type IndexPair = (usize, usize);

fn generate_neighbour_indexes(index_pair: IndexPair) -> Vec<IndexPair> {
    let mut indexes: Vec<IndexPair> = vec![];
    indexes.push((index_pair.0 - 1, index_pair.1 - 1));
    indexes.push((index_pair.0 - 1, index_pair.1));
    indexes.push((index_pair.0 - 1, index_pair.1 + 1));
    indexes.push((index_pair.0, index_pair.1 - 1));
    indexes.push((index_pair.0, index_pair.1 + 1));
    indexes.push((index_pair.0 + 1, index_pair.1 - 1));
    indexes.push((index_pair.0 + 1, index_pair.1));
    indexes.push((index_pair.0 + 1, index_pair.1 + 1));

    indexes
}

fn count_alive_neighbours(&board: Board, index_pair: IndexPair) -> usize {
    generate_neighbour_indexes(index_pair)
        .iter()
        .filter(|current_pair| board[current_pair.0][current_pair.1] == Status::Alive)
        .count()
}

fn is_lonely(&board: Board, &cell: &mut Status, index_pair: IndexPair) -> bool {
    let alive_neighbours_count = count_alive_neighbours(board, index_pair);

    cell == Status::Alive && (alive_neighbours_count == 0 || alive_neighbours_count == 1)
}

fn is_suffocated(&board: Board, &cell: &mut Status, index_pair: IndexPair) -> bool {
    let alive_neighbours_count = count_alive_neighbours(board, index_pair);

    cell == Status::Alive && (alive_neighbours_count >= 4)
}

fn is_resurrected(&board: Board, &cell: &mut Status, index_pair: IndexPair) -> bool {
    let alive_neighbours_count = count_alive_neighbours(board, index_pair);

    cell == Status::Dead && (alive_neighbours_count == 3)
}

pub fn make_next_generation(&mut board: Board) -> Board {
    for (i, row) in board.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            let dies = is_lonely(board, cell, (i, j)) || is_suffocated(board, cell, (i, j));

            if dies {
                board[i][j] = Status::Dead;
            } else if is_resurrected(board, cell, (i, j)) {
                board[i][j] = Status::Alive;
            }
        }
    }

    board
}

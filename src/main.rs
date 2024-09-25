use caspervk_chess::*;

fn main(){
	let mut game = Game::new();
	let possible_movements = game.get_position_possible_movements(board_pos_to_index("a2".to_string()));
	let board_state = game.do_move(board_pos_to_index("a2".to_string()), board_pos_to_index("a4".to_string()));
	for x in possible_movements {
		println!("{}", x);
	}
	let npossible_movements = game.get_position_possible_movements(board_pos_to_index("a4".to_string()));
	for x in npossible_movements {
		println!("{} -> {}", board_pos_to_index("a4".to_string()), x);
	}

	for x in game.board_pieces {
		println!("piece: {:?}", x);
	}
}
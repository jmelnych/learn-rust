fn main() {
    let dice_roll = 9;

match dice_roll {
    3 => get_hat(),
    7 => loose_hat(),
    other => move_player(other), // alternatively _
}

fn get_hat(){}

fn loose_hat(){}
fn move_player(num_spaces: u8) {}
}


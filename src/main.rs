use rand::Rng;
use std::io;

// (x,y)
// (1,1)(1,2)(1,3)
// (2,1)(2,2)(2,3)
// (3,1)(3,2)(3,3)

#[derive(Debug)]
struct Block{
    x: u8,
    y: u8
}

#[derive(Debug)]
struct Player{
    x: u8,
    y: u8
}

// enum BlockingPositions {
//     VerticalBlockingPositions((i32, i32), (i32,i32)),
//     HorizontalBLockingPositions((i32, i32), (i32,i32)),
// }

trait CheckBlockingPosition {
    fn is_blocking_positions(&self, other: &Block, player: &Player) -> bool;
}

trait Move {
    fn right(&mut self);
    fn left(&mut self);
    fn up(&mut self);
    fn down(&mut self);
}

impl CheckBlockingPosition for Block {
    fn is_blocking_positions(&self, other: &Block, player: &Player) -> bool {
        if self.x == other.x || self.y == other.y || self.x == player.x || self.y == player.y {
            return true;
        }

        println!("{:?}, {:?}",other, player);
        return false;
    }
}

impl Move for Player {
    fn right(&mut self) {
        self.x += 1;
    }
    fn left(&mut self) {
        self.x -= 1;
    }
    fn up(&mut self) {
        self.y -= 1;
    }
    fn down(&mut self) {
        self.y += 1;
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut player = Player{
        x: rng.gen_range(0..5),
        y: rng.gen_range(0..5),
    };

    let mut block1 = Block{
        x: 1,
        y: 1,
    };

    let mut block2 = Block{
        x: 1,
        y: 1,
    };

    loop {
        block1.x = rng.gen_range(0..5);
        block1.y = rng.gen_range(0..5);

        if block1.x != player.x && block1.y != player.y {
            break;
        }
    }

    loop {
        block2.x = rng.gen_range(0..5);
        block2.y = rng.gen_range(0..5);

        if !block2.is_blocking_positions(&block1, &player) {
            break;
        }
    }

    player.down();
    let mut direction_input = String::new();
    match io::stdin().read_line(&mut direction_input) {
        Ok(_) => {
            direction_input = direction_input.trim().to_string();
            println!("{:?}, {:?}, {:?}, {:?}", block1, block2, player, direction_input);
        }
        Err(err) => println!("Error: {}", err)
    }

}

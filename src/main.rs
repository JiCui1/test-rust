use rand::Rng;

// (x,y)
// (1,1)(2,1)(3,1)
// (2,1)(2,2)(3,2)
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

trait Move {
    fn right(&mut self);
    fn left(&mut self);
    fn up(&mut self);
    fn down(&mut self);
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

    let mut block = Block{
        x: 1,
        y: 1,
    };

    player.down();

    println!("{}, {} ", block.x, player.y);
}

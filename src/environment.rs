pub enum Tile {
    Nothing, 
    Wall,
    Breakable {broken: bool},
    Pusher,
}
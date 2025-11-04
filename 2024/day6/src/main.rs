use std::fs;



enum Direction {
    Up,
    Right,
    Down,
    Left,
}
struct Map {
    map: Vec<Vec<char>>,
}
impl Map {
    fn input_to_map(input: &str) -> Self{
        let map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        Map { map }
    }
    fn print_map(&self) {
        for row in &self.map {
            for &ch in row {
                print!("{}", ch);
            }
            println!();
        }
    }

    fn find_guard(&self) -> Option<(i32, i32)>  {
        for (y, row) in self.map.iter().enumerate() {
            for (x, char) in row.iter().enumerate() {
                match char {
                    '^' | '>' | 'v' | '<' => return Some((x as i32, y as i32)),
                    _ => {},
                };
            }
        }
        None
    }
   
    fn move_guard(&self, direction: &Direction) {
        
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        };
        
        let (mut x, mut y) = self.find_guard().unwrap();
        let mut next_move = Direction::Up;
        // if current char == '^' {
            // if next x,y = '#' {
        //          turn
        // } else {
        //     keep going
        // }
        loop {
            if next_move.char() == '#' {
                if self.map[x][y] == '^' {
                    next_move = Direction::Right
                }
                 
            }
            
            // checking within bounds
            if x < 0 || y < 0 || y as usize >= self.map.len() || x as usize >= self.map[y as usize].len() {
                break;
            }
            x += dx;
            y += dy;
            println!("x:{:?}, y{:?}",x ,y);
        }                        
    }
}
fn main() {
    
    let input = fs::read_to_string("example.txt").expect("can't read");
    // print!("{}", input);

    // make it into a 2d vec
    let map = Map::input_to_map(&input);
    map.print_map();
    // let guard = find_guard(&map);
    // println!("{:?}", guard);

    map.move_guard(&Direction::Up);

}

use std::fs;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }

}
#[derive(Clone)]
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
    // fn print_map(&self) {
    //     for row in &self.map {
    //         for &ch in row {
    //             print!("{}", ch);
    //         }
    //         println!();
    //     }
    // }

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
    
    fn move_guard(&mut self) {
        
        let (mut x, mut y) = self.find_guard().unwrap();

        let mut direction = match self.map[y as usize][x as usize] {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("Guard not found!"),
        };
        
        loop {            
            // Compute next coordinates
            let (dx, dy) = direction.to_delta();
            let next_x = x + dx;
            let next_y = y + dy;

            // Check bounds
            if next_x < 0 || next_y < 0 
                || next_y as usize >= self.map.len() 
                || next_x as usize >= self.map[0].len() {
                break;
            }

            let next_tile = self.map[next_y as usize][next_x as usize];

            // If wall ahead, turn right
            if next_tile == '#' {
                direction = direction.turn_right();
                continue;
            }

            // Otherwise move forward
            self.map[y as usize][x as usize] = 'X'; // change old guard position
            x = next_x;
            y = next_y;
            self.map[y as usize][x as usize] = direction.char();
            // 9️⃣ (Optional) print for debugging
            // self.print_map();
            // println!();
        }
        let mut count = 1;
        for row in &self.map {
            for &ch in row {
                if ch == 'X' {
                    count += 1;
                }   
            }
        }
        println!("unique visits: {}", count);
                                
    }
    fn simulate_guard(&self) -> bool {
        use std::collections::HashSet;
        let map = self.map.clone();
        let (mut x, mut y) = self.find_guard().unwrap();
        let mut direction = match self.map[y as usize][x as usize] {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("Guard not found!"),
        };
        let mut visited = HashSet::new();

        loop {
            let state = (x, y, direction);
            if !visited.insert(state) {
                // already seen => loop detected
                return true;
            }

            let (dx, dy) = direction.to_delta();
            let next_x = x + dx;
            let next_y = y + dy;

            if next_x < 0 || next_y < 0
                || next_y as usize >= map.len()
                || next_x as usize >= map[0].len() {
                return false; // escaped map
            }

            let next_tile = map[next_y as usize][next_x as usize];
            if next_tile == '#' {
                direction = direction.turn_right();
            } else {
                x = next_x;
                y = next_y;
            }
        }
    }
    fn count_loops(&self) -> usize {
        let mut count = 0;

        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                if self.map[y][x] == '.' {
                    let mut temp = self.clone();
                    temp.map[y][x] = '#';
                    if temp.simulate_guard() {
                        count += 1;
                    }
                }
            }
        }

        count
    }


}
fn main() {
    
    let input = fs::read_to_string("puzzle.txt").expect("can't read");
    // print!("{}", input);

    // make it into a 2d vec
    let mut map = Map::input_to_map(&input);
    // map.print_map();
    // let guard = find_guard(&map);
    // println!("{:?}", guard);
    let loops = map.count_loops();
    println!("loops: {}", loops);
    map.move_guard();

}

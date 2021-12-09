#[derive(Debug, Copy, Clone)]
struct Position {
    height: i32,
    x: usize,
    y: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Position {}

#[derive(Debug)]
struct Heightmap {
    heights: Vec<Vec<Position>>,
}

impl Heightmap {
    pub fn new() -> Heightmap {
        Heightmap {
            heights: Vec::new(),
        }
    }

    pub fn add_row(&mut self, chars: &str) {
        let y = self.heights.len();
        let mut x = 0;
        let mut row = Vec::new();
        for pos in chars.chars().filter_map(|c| c.to_digit(10)) {
            row.push(Position {
                height: pos as i32,
                x,
                y,
            });
            x += 1;
        }
        self.heights.push(row);
    }

    pub fn rows(&self) -> &Vec<Vec<Position>> {
        &self.heights
    }

    fn at(&self, x: usize, y: usize) -> Option<&Position> {
        if let Some(row) = self.heights.iter().nth(y) {
            if let Some(pos) = row.iter().nth(x) {
                return Some(pos);
            }
        }
        None
    }

    pub fn adjacent(&self, pos: &Position) -> Vec<&Position> {
        let mut neighbours = Vec::new();

        if pos.x > 0 {
            if let Some(p) = self.at(pos.x - 1, pos.y) {
                neighbours.push(p);
            }
        }
        if let Some(p) = self.at(pos.x + 1, pos.y) {
            neighbours.push(p);
        }
        if pos.y > 0 {
            if let Some(p) = self.at(pos.x, pos.y - 1) {
                neighbours.push(p);
            }
        }
        if let Some(p) = self.at(pos.x, pos.y + 1) {
            neighbours.push(p);
        }

        neighbours
    }

    pub fn find_basin(&self, pos: &Position, found: &mut Vec<Position>) {
        for neighbour in self.adjacent(pos) {
            if neighbour.height == 9 || found.iter().any(|p| p == neighbour) {
                continue;
            }

            found.push(neighbour.clone());
            self.find_basin(neighbour, found);
        }
    }
}

pub fn go(lines: Vec<String>) {
    let mut heightmap = Heightmap::new();
    for line in lines {
        heightmap.add_row(&line);
    }

    let mut low_points = Vec::new();
    let mut basins = Vec::new();

    for row in heightmap.rows() {
        for pos in row {
            let adjacent = heightmap.adjacent(pos);
            if adjacent.iter().all(|&a| pos.height < a.height) {
                low_points.push(pos);
                let mut basin = vec![pos.clone()];
                heightmap.find_basin(pos, &mut basin);
                basins.push(basin.iter().len() as i32);
            }
        }
    }

    let risk_sum = low_points.iter().map(|&p| p.height + 1).sum::<i32>();
    println!("Risk sum {}", risk_sum);
    
    basins.sort();
    basins.reverse();
    let three_largest_basins = basins.into_iter().take(3);
    let basin_sizes_product = three_largest_basins.into_iter().reduce(|a, b| a * b).unwrap();
    println!("Basin sized product {}", basin_sizes_product);
}

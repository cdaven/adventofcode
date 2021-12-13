#[derive(Debug)]
struct Paper {
    coords: Vec<Vec<bool>>,
}

impl Paper {
    pub fn new(xsize: usize, ysize: usize) -> Paper {
        Paper {
            coords: vec![vec![false; xsize]; ysize],
        }
    }

    pub fn from(coords: Vec<Vec<bool>>) -> Paper {
        Paper {
            coords
        }
    }

    pub fn print(&self) {
        for y in &self.coords {
            for x in y {
                if *x {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    pub fn set(&mut self, x: usize, y: usize) {
        // TODO: Extend coords as needed
        self.coords[y][x] = true;
    }

    pub fn count(&self) -> i32 {
        let mut count = 0;
        for y in &self.coords {
            for x in y {
                if *x {
                    count += 1;
                }
            }
        }
        count
    }

    fn overlay(main: &mut Vec<Vec<bool>>, folded: &Vec<Vec<bool>>) {
        // When folded part is smaller than main part
        let y_offset = main.len() - folded.len();
        let x_offset = main[0].len() - folded[0].len();

        for y in 0..main.len() {
            if y_offset > y {
                continue;
            }

            for x in 0..main[0].len() {
                if x_offset > x {
                    continue;
                }

                if folded[y - y_offset][x - x_offset] {
                    main[y][x] = true;
                }
            }
        }
    }

    pub fn fold_up(&self, at: usize) -> Paper {
        let mut top: Vec<Vec<bool>> = self.coords.iter().take(at).cloned().collect();
        let bottom: Vec<Vec<bool>> = self.coords.iter().skip(at + 1).rev().cloned().collect();

        Paper::overlay(&mut top, &bottom);
        Paper::from(top)
    }

    pub fn fold_left(&self, at: usize) -> Paper {
        let mut left: Vec<Vec<bool>> = self.coords.iter().map(|line| line.iter().take(at).cloned().collect::<Vec<bool>>()).collect();
        let right: Vec<Vec<bool>> = self.coords.iter().map(|line| line.iter().skip(at + 1).rev().cloned().collect::<Vec<bool>>()).collect();

        Paper::overlay(&mut left, &right);
        Paper::from(left)
    }
}

pub fn go(lines: Vec<String>) {
    let mut paper = Paper::new(1311, 893);
    for line in &lines {
        if line.is_empty() {
            break;
        }

        let coords: Vec<usize> = line
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        paper.set(coords[0], coords[1]);
    }

    // let folded = paper.fold_up(7);
    // paper.count();

    // fold along x=655
    let fold1 = paper.fold_left(655);
    // fold along y=447
    let fold2 = fold1.fold_up(447);
    // fold along x=327
    let fold3 = fold2.fold_left(327);
    // fold along y=223
    let fold4 = fold3.fold_up(223);
    // fold along x=163
    let fold5 = fold4.fold_left(163);
    // fold along y=111
    let fold6 = fold5.fold_up(111);
    // fold along x=81
    let fold7 = fold6.fold_left(81);
    // fold along y=55
    let fold8 = fold7.fold_up(55);
    // fold along x=40
    let fold9 = fold8.fold_left(40);
    // fold along y=27
    let fold10 = fold9.fold_up(27);
    // fold along y=13
    let fold11 = fold10.fold_up(13);
    // fold along y=6
    let fold12 = fold11.fold_up(6);
   

    fold12.print();
}

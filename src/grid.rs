use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Grid<Cell> {
    pub elems: Vec<Vec<Cell>>,
}

#[allow(dead_code)]
impl<Cell> Grid<Cell> {
    pub fn new(elems: Vec<Vec<Cell>>) -> Grid<Cell> {
        Grid { elems }
    }

    pub fn new_with<F: FnMut(usize, usize) -> Cell>(x: usize, y: usize, mut f: F) -> Grid<Cell> {
        Grid {
            elems: (0..y).map(|y| (0..x).map(|x| f(x, y)).collect()).collect(),
        }
    }

    pub fn col_size(&self) -> usize {
        self.elems.len()
    }

    pub fn row_size(&self) -> usize {
        self.elems.get(0).map(|row| row.len()).unwrap_or(0)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.elems
            .get(y as usize)
            .and_then(|col| col.get(x as usize))
    }

    pub fn set(&mut self, x: usize, y: usize, mut new: Cell) -> Option<Cell> {
        self.elems.get_mut(y as usize).and_then(|col| {
            let prev = col.get_mut(x as usize)?;
            std::mem::swap(prev, &mut new);
            Some(new)
        })
    }

    fn legal(&self, x: i32, y: i32) -> bool {
        y >= 0 && y < self.col_size() as i32 && x >= 0 && x < self.row_size() as i32
    }

    pub fn nbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let x = x as i32;
        let y = y as i32;
        [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .iter()
            .copied()
            .filter(|(x, y)| self.legal(*x, *y))
            .map(|(x, y)| (x as usize, y as usize))
            .collect()
    }

    pub fn diag_nbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let x = x as i32;
        let y = y as i32;
        [
            (x - 1, y),
            (x + 1, y),
            (x, y - 1),
            (x, y + 1),
            (x - 1, y - 1),
            (x - 1, y + 1),
            (x + 1, y - 1),
            (x + 1, y + 1),
        ]
        .iter()
        .copied()
        .filter(|(x, y)| self.legal(*x, *y))
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
    }

    pub fn ray<'a>(
        &'a self,
        mut x: i32,
        mut y: i32,
        dx: i32,
        dy: i32,
        cont: fn(&'a Cell) -> bool,
    ) -> Option<&'a Cell> {
        x += dx;
        y += dy;

        if dx == 0 && dy == 0 {
            return None;
        }

        while self.legal(x, y) && cont(&self.elems[y as usize][x as usize]) {
            x += dx;
            y += dy;
        }

        if self.legal(x, y) {
            self.get(x as usize, y as usize)
        } else {
            None
        }
    }

    pub fn map<T, F: FnMut(&Cell) -> T>(&self, mut f: F) -> Grid<T> {
        Grid::new_with(self.row_size(), self.col_size(), |x, y| {
            f(self.get(x as usize, y as usize).unwrap())
        })
    }
}

impl<Cell: fmt::Display> fmt::Display for Grid<Cell> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rows = 10;
        for row in &self.elems {
            let mut cols = 10;
            for cell in row {
                cols -= 1;
                write!(f, "{}", cell)?;
                if cols == 0 {
                    write!(f, "|")?;
                    cols = 10;
                }
            }
            rows -= 1;
            writeln!(f)?;
            if rows == 0 {
                writeln!(f, "-")?;
                rows = 10;
            }
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::*;

    #[test]
    fn ray() {
        let grid = Grid::new(vec![
            vec![0, 1, 1, 1],
            vec![1, 1, 0, 1],
            vec![0, 1, 1, 1],
            vec![1, 1, 1, 1],
        ]);

        assert_eq!(None, grid.ray(0, 0, 0, 0, |_| unreachable!()));
        assert_eq!(None, grid.ray(0, 0, 1, 0, |n| *n == 1));
        assert_eq!(Some(&0), grid.ray(0, 1, 1, 0, |n| *n == 1));
        assert_eq!(None, grid.ray(0, 1, 1, 1, |n| *n == 1));
        assert_eq!(Some(&0), grid.ray(1, 0, 1, 1, |n| *n == 1));
        assert_eq!(Some(&0), grid.ray(3, 3, -1, -1, |n| *n == 1));
        assert_eq!(Some(&0), grid.ray(0, 3, 1, -1, |n| *n == 1));
        assert_eq!(Some(&0), grid.ray(-1, 3, 1, -1, |n| *n == 1));
        assert_eq!(Some(&0), grid.ray(-1, 4, 1, -1, |n| *n == 1));
        assert_eq!(Some(&0), grid.ray(-1, 1, 1, 0, |n| *n == 1));
    }
}

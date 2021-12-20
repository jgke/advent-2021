use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Grid<Cell> {
    pub elems: Vec<Vec<Cell>>,
}

#[allow(dead_code)]
impl<Cell: std::fmt::Debug> Grid<Cell> {
    pub fn new(elems: Vec<Vec<Cell>>) -> Grid<Cell> {
        Grid { elems }
    }

    pub fn col_size(&self) -> usize {
        self.elems.len()
    }

    pub fn row_size(&self) -> usize {
        self.elems[0].len()
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&Cell> {
        if x < 0 || y < 0 {
            None
        } else {
            self.elems
                .get(y as usize)
                .and_then(|col| col.get(x as usize))
        }
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

        while x >= 0
            && y >= 0
            && x < self.row_size() as i32
            && y < self.col_size() as i32
            && cont(&self.elems[y as usize][x as usize])
        {
            x += dx;
            y += dy;
        }

        self.get(x, y)
    }
}

impl<Cell: fmt::Display> fmt::Display for Grid<Cell> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.elems {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
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

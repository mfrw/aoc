#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoord {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl std::fmt::Debug for GridCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl From<GridCoord> for (usize, usize) {
    fn from(value: GridCoord) -> Self {
        (value.x, value.y)
    }
}

pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<Option<T>>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let mut v = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            v.push(None);
        }
        Grid {
            width,
            height,
            data: v,
        }
    }

    pub fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    #[allow(dead_code)]
    pub fn cell_mut(&mut self, coord: GridCoord) -> Option<&mut T> {
        if !self.in_bounds(coord) {
            return None;
        }
        self.data[coord.y * self.width + coord.x].as_mut()
    }

    pub fn cell(&self, coord: GridCoord) -> Option<&T> {
        if !self.in_bounds(coord) {
            return None;
        }
        self.data[coord.y * self.width + coord.x].as_ref()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn neighbors(
        &self,
        current: GridCoord,
        with_diagonals: bool,
    ) -> impl Iterator<Item = (GridCoord, &T)> + '_ {
        let mut dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        if with_diagonals {
            dirs.extend(vec![(-1, -1), (-1, 1), (1, -1), (1, 1)]);
        }
        let c = (current.x as i64, current.y as i64);
        dirs.into_iter()
            .map(move |(x, y)| (c.0 + x, c.1 + y))
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .map(|c| (c.0 as usize, c.1 as usize).into())
            .filter(|&c| self.in_bounds(c))
            .map(|c| (c, self.cell(c).unwrap()))
    }

    pub fn replace(&mut self, c: GridCoord, elm: T) -> Option<T> {
        self.data[c.y * self.width + c.x].replace(elm)
    }
}

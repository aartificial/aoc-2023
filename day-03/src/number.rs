use itertools::Itertools;

const START_RANGE: i32 = -1;
const END_RANGE: i32 = 1;

#[derive(Debug, Copy, Clone)]
pub struct Number {
    pub value: u32,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl Number {
    fn positions(&self) -> Vec<(usize, usize)> {
        (self.start..=self.end)
            .map(|pos| (self.line, pos))
            .collect()
    }

    pub(crate) fn is_adjacent(&self, i: usize, j: usize) -> bool {
        let pos = self.positions();
        let points = (START_RANGE..=END_RANGE)
            .cartesian_product(START_RANGE..=END_RANGE)
            .collect::<Vec<_>>();
        points
            .iter()
            .any(|&(dx, dy)| self.is_nearby_position_included(dx, dy, i, j, &pos))
    }

    fn is_nearby_position_included(
        &self,
        dx: i32,
        dy: i32,
        i: usize,
        j: usize,
        pos: &[(usize, usize)],
    ) -> bool {
        let x = dx + i as i32;
        let y = dy + j as i32;
        x >= 0 && y >= 0 && !(dx == 0 && dy == 0) && pos.contains(&(x as usize, y as usize))
    }
}

use crate::custom_error::AocError;
use crate::part1::parse;
use crate::{Direction, Point};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct UltraCrucible {
    loss: u32,
    pos: Point,
    dir: Direction,
    steps: u8,
}

impl Ord for UltraCrucible {
    fn cmp(&self, other: &Self) -> Ordering {
        other.loss.cmp(&self.loss)
    }
}

impl PartialOrd for UltraCrucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl UltraCrucible {
    fn new(loss: u32, pos: Point, dir: Direction, steps: u8) -> Self {
        Self {
            loss,
            pos,
            dir,
            steps,
        }
    }

    fn successors(&self, graph: &[Vec<u32>]) -> Vec<Self> {
        let mut successors = Vec::new();
        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if self.steps < 4 && dir != self.dir {
                continue;
            }

            if self.steps == 10 && self.dir == dir {
                continue;
            }

            if self.dir.opposite() == dir {
                continue;
            }
            if let Some(pos) = self.pos.next(&dir, graph.len(), graph[0].len()) {
                let loss = self.loss + graph[pos.x][pos.y];
                let steps = if self.dir == dir { self.steps + 1 } else { 1 };
                successors.push(UltraCrucible {
                    loss,
                    pos,
                    dir,
                    steps,
                });
            }
        }
        successors
    }
}

fn dijkstra(input: &str) -> Option<u32> {
    let graph = parse(input);
    let goal = Point::new(graph.len() - 1, graph[0].len() - 1);

    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();

    let right = UltraCrucible::new(graph[0][1], Point::new(0, 1), Direction::Right, 1);
    let down = UltraCrucible::new(graph[1][0], Point::new(1, 0), Direction::Down, 1);

    queue.push(right);
    queue.push(down);

    while let Some(crucible) = queue.pop() {
        if crucible.pos == goal && crucible.steps >= 4 {
            return Some(crucible.loss);
        }

        for crucible in crucible.successors(&graph) {
            if seen.insert((crucible.pos, crucible.dir, crucible.steps)) {
                queue.push(crucible);
            }
        }
    }
    None
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    if let Some(value) = dijkstra(input) {
        return Ok(value);
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> miette::Result<()> {
        let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
        assert_eq!(94, process(input)?);
        Ok(())
    }

    #[test]
    fn test_2() -> miette::Result<()> {
        let input = r#"111111111111
999999999991
999999999991
999999999991
999999999991"#;
        assert_eq!(71, process(input)?);
        Ok(())
    }
}

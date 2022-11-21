use super::BlockIter;
use super::ColIter;
use super::RowIter;

pub struct Neighbours {
    pos: (usize, usize),
    col: ColIter,
    row: RowIter,
    block: BlockIter,
}
impl Neighbours {
    pub fn of(x: usize, y: usize) -> Neighbours {
        Neighbours {
            pos: (x, y),
            col: ColIter::at(x),
            row: RowIter::at(y),
            block: BlockIter::at(x, y),
        }
    }
}
impl Iterator for Neighbours {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        let ret = self
            .col
            .next()
            .or_else(|| self.row.next())
            .or_else(|| self.block.next());

        if ret == Some(self.pos) {
            self.next()
        } else {
            ret
        }
    }
}

#[test]
fn test_neighbours() {
    let mut iter = Neighbours::of(7, 3);
    for y in 0..3 {
        assert_eq!(iter.next(), Some((7, y)));
    }
    for y in 4..9 {
        assert_eq!(iter.next(), Some((7, y)));
    }
    for x in 0..7 {
        assert_eq!(iter.next(), Some((x, 3)));
    }
    for x in 8..9 {
        assert_eq!(iter.next(), Some((x, 3)));
    }
    assert_eq!(iter.next(), Some((6, 3)));
    assert_eq!(iter.next(), Some((8, 3)));
    for y in 4..6 {
        for x in 6..9 {
            assert_eq!(iter.next(), Some((x, y)));
        }
    }
    assert_eq!(iter.next(), None);
}

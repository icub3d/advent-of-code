use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day03.txt");

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn next(self, c: char) -> Self {
        let mut next = self;

        match c {
            '>' => next.x += 1,
            '<' => next.x -= 1,
            '^' => next.y += 1,
            'v' => next.y -= 1,
            _ => (),
        }

        next
    }
}

// Move santa around based on the input directions and count the unique points visited.
fn p1(input: &str) -> usize {
    // We can use scan() and unique() to do the same thing as tracking cur and using a HashSet.
    input
        .chars()
        .scan(Point::new(0, 0), |cur, c| {
            *cur = cur.next(c);
            Some(*cur)
        })
        .chain(std::iter::once(Point::new(0, 0)))
        .unique()
        .count()
}

// Same as p1 but with two points (santa and robo-santa) moving alternately.
fn p2(input: &str) -> usize {
    // We can do the same thing as p1, but track two points in the scan() using an array.
    input
        .chars()
        .enumerate()
        .scan([Point::new(0, 0), Point::new(0, 0)], |cur, (i, c)| {
            let index = i % 2;
            cur[index] = cur[index].next(c);
            Some(cur[index])
        })
        .chain(std::iter::once(Point::new(0, 0)))
        .unique()
        .count()
}

fn main() {
    let now = std::time::Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = std::time::Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}

use std::borrow::Borrow;
use std::u32::MAX;

pub fn solve(lines: Vec<String>) -> (i32, i32) {
    let sol1 = solution1(lines.borrow());
    let sol2 = solution2(lines.borrow());
    (sol1, sol2)
}


#[derive(Debug, PartialEq, Clone)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn get_translation_by(&self, vec: &Vector) -> Point {
        match vec.d {
            Direction::Left => Point{x: self.x - vec.v as i32, y: self.y},
            Direction::Right => Point{x: self.x + vec.v as i32, y: self.y},
            Direction::Down => Point{x: self.x, y: self.y - vec.v as i32},
            Direction::Up => Point{x: self.x, y: self.y + vec.v as i32},
        }
    }

    fn distance_from_origin(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

#[derive(Debug, PartialEq)]
struct Line {
    s: Point,
    e: Point,
}

impl Line {
    fn len(&self) -> u32 {
        (self.e.x - self.s.x + self.e.y - self.s.y) as u32
    }
}

struct Vector {
    d: Direction,
    v: u32,
}

impl Vector {
    fn build_line_from(self, p: Point) -> Line {
        Line::new( p.get_translation_by(&self), p)
    }
}

#[derive(Debug, PartialEq)]
enum LineOrientation {
    Vertical,
    Horizontal,
    None,
}


impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        let start;
        let end;
        if p1.x < p2.x || p1.y < p2.y {
            start = p1;
            end = p2;
        } else {
            start = p2;
            end = p1;
        }
        Line{s: start, e: end}
    }

    fn get_orientation(&self) -> LineOrientation {
        if self.s.x == self.e.x {
            LineOrientation::Vertical
        } else if self.s.y == self.e.y {
            LineOrientation::Horizontal
        } else {
            LineOrientation::None
        }
    }

    fn has_same_orientation(&self, other: &Self) -> bool {
        self.get_orientation() == other.get_orientation()
    }

    fn contains(&self, p: &Point) -> bool {
        return (self.get_orientation() == LineOrientation::Horizontal &&
            self.s.y == p.y && self.s.x <= p.x && p.x <= self.e.x) ||
            (self.get_orientation() == LineOrientation::Vertical &&
            self.s.x == p.x && self.s.y <= p.y && p.y <= self.e.y)
    }

    fn get_intersection(&self, other: &Self) -> Option<Point> {
        if self.has_same_orientation(other) {
            None
        } else {
            let mut intersection = Point { x: 0, y: 0 };
            if self.get_orientation() == LineOrientation::Horizontal {
                intersection.y = self.s.y;
                intersection.x = other.s.x;
            } else {
                intersection.x = self.s.x;
                intersection.y = other.s.y;
            }
            if self.contains(&intersection) && other.contains(&intersection) {
                Some(intersection)
            } else {
                None
            }
        }
    }

    fn get_second_point(&self, first_point: &Point) -> Point {
        if self.s == *first_point {
            self.e.clone()
        } else {
            self.s.clone()
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(c: u8) -> Direction {
        match c {
            b'U' => Self::Up,
            b'D' => Self::Down,
            b'L' => Self::Left,
            b'R' => Self::Right,
            c => panic!(format!("Character '{}' doesn't correspond to direction.", c))
        }
    }
}

fn get_directions(lines_str: &Vec<String>) -> Vec<Vec<Line>> {
    let mut cur_point = Point{x: 0, y: 0};
    lines_str
        .iter()
        .map(|line_str| {
            let line = line_str
                .split(",")
                .map(|word| {
                    let dir = Direction::new(word.chars().nth(0).unwrap() as u8);
                    let val = word[1..].parse::<u32>().unwrap();
                    let vec = Vector{d: dir, v: val};
                    let cur_point_copy = cur_point.clone();
                    cur_point = cur_point.get_translation_by(&vec);
                    vec.build_line_from(cur_point_copy)
                }).collect();
            cur_point = Point{x: 0, y: 0};
            line
        })
        .collect()
}

fn solution1(lines: &Vec<String>) -> i32 {
    let directions = get_directions(lines);
    let mut smallest_distance = MAX;

    for line1 in directions[0].iter() {
        for line2 in directions[1].iter() {
            if let Some(intersection) = line1.get_intersection(line2) {
                let intersection_distance = intersection.distance_from_origin();
                if intersection_distance != 0 && smallest_distance > intersection_distance {
                    smallest_distance = intersection_distance
                }
            }
        }
    }

    smallest_distance as i32
}

fn count_steps_to_first_intersection(lines1: &Vec<Line>, lines2: &Vec<Line>) -> u32 {
    let mut wire1_length = 0;
    let mut cur_point1 = Point{x: 0, y: 0};
    let mut final_length = 0;
'outer:
    for line1 in lines1.iter() {
        let mut wire2_length = 0;
        let mut cur_point2 = Point{x: 0, y: 0};
        for line2 in lines2.iter() {
            if let Some(intersection) = line1.get_intersection(line2) {
                if intersection.distance_from_origin() != 0 {
                    final_length = wire1_length + wire2_length +
                        Line::new(cur_point1, intersection.clone()).len() +
                        Line::new(cur_point2, intersection).len();
                    break 'outer;
                }
            }
            wire2_length += line2.len();
            cur_point2 = line2.get_second_point(&cur_point2);

        }
        wire1_length += line1.len();
        cur_point1 = line1.get_second_point(&cur_point1);
    }
    final_length
}

fn solution2(lines: &Vec<String>) -> i32 {
    let directions = get_directions(lines);
    count_steps_to_first_intersection(&directions[0], &directions[1]).min(
        count_steps_to_first_intersection(&directions[1], &directions[0])
    ) as i32
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let start = Point{x: 0, y: 0};
        let end = Point{x: 5, y: 0};

        let line1 = Line::new(end.clone(), start.clone());
        assert_eq!(start, line1.s);
        assert_eq!(end, line1.e);
    }

    #[test]
    fn test_vertical_orientation() {
        let line1 = Line::new(Point{x: 0, y: 0}, Point{x: 0, y: 1});
        assert_eq!(LineOrientation::Vertical, line1.get_orientation());
    }

    #[test]
    fn test_horizontal_orientation() {
        let line1 = Line::new(Point{x: 0, y: 0}, Point{x: 1, y: 0});
        assert_eq!(LineOrientation::Horizontal, line1.get_orientation());
    }

    #[test]
    fn test_has_same_orientation() {
        let line1 = Line::new(Point{x: 0, y: 0}, Point{x: 1, y: 0});
        let line2 = Line::new(Point{x: 4, y: 4}, Point{x: 1, y: 4});
        assert!(line1.has_same_orientation(&line2));
    }

    #[test]
    fn test_has_not_same_orientation() {
        let line1 = Line::new(Point{x: 1, y: 9}, Point{x: 1, y: 0});
        let line2 = Line::new(Point{x: 4, y: 4}, Point{x: 1, y: 4});
        assert!(!line1.has_same_orientation(&line2));
    }

    #[test]
    fn test_contains() {
        let line1 = Line::new(Point{x: 1, y: 9}, Point{x: 1, y: 0});
        let point = Point{x: 1, y: 4};
        assert!(line1.contains(&point));
    }

    #[test]
    fn test_not_contains() {
        let line1 = Line::new(Point{x: 1, y: 9}, Point{x: 1, y: 0});
        let point = Point{x: 1, y: 10};
        assert!(!line1.contains(&point));
    }

    #[test]
    fn test_intersection() {
        let line1 = Line::new(Point{x: 1, y: 9}, Point{x: 1, y: 0});
        let line2 = Line::new(Point{x: -3, y: 7}, Point{x: 5, y: 7});
        assert_eq!(Some(Point{x: 1, y: 7}), line1.get_intersection(&line2));
    }

    #[test]
    fn test_no_intersection() {
        let line1 = Line::new(Point{x: 1, y: 5}, Point{x: 1, y: 5});
        let line2 = Line::new(Point{x: -3, y: 7}, Point{x: 5, y: 7});
        assert_ne!(Some(Point{x: 1, y: 7}), line1.get_intersection(&line2));


        let line1 = Line::new(Point{x: 1, y: 9}, Point{x: 1, y: 0});
        let line2 = Line::new(Point{x: -3, y: 7}, Point{x: 0, y: 7});
        assert_ne!(Some(Point{x: 1, y: 7}), line1.get_intersection(&line2));
    }

    #[test]
    fn test_get_translation_by() {
        let p1 = Point{x: 0, y: 0};
        let v = Vector{d: Direction::Up, v: 5};
        assert_eq!(Point{x: 0, y: 5}, p1.get_translation_by(&v));
    }

    #[test]
    fn test_build_line_from() {
        let p = Point{x: 0, y: 0};
        let v = Vector{d: Direction::Up, v: 5};
        assert_eq!(Line{s: Point{x: 0, y: 0}, e: Point{x: 0, y: 5}}, v.build_line_from(p));
    }

    #[test]
    fn task1_example1() {
        let lines = [String::from("R8,U5,L5,D3"), String::from("U7,R6,D4,L4")];
        assert_eq!(6, solution1(&lines.to_vec()))
    }

    #[test]
    fn task1_example2() {
        let lines = [String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"), String::from("U62,R66,U55,R34,D71,R55,D58,R83")];
        assert_eq!(159, solution1(&lines.to_vec()))
    }

    #[test]
    fn task1_example3() {
        let lines = [String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"), String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")];
        assert_eq!(135, solution1(&lines.to_vec()))
    }

    #[test]
    fn task2_example1() {
        let lines = [String::from("R8,U5,L5,D3"), String::from("U7,R6,D4,L4")];
        assert_eq!(30, solution2(&lines.to_vec()))
    }

    #[test]
    fn task2_example2() {
        let lines = [String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"), String::from("U62,R66,U55,R34,D71,R55,D58,R83")];
        assert_eq!(610, solution2(&lines.to_vec()))
    }

    #[test]
    fn task2_example3() {
        let lines = [String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"), String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")];
        assert_eq!(410, solution2(&lines.to_vec()))
    }
}
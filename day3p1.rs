#[derive(Copy, Clone)]

struct Vector {
    right: u32,
    down: u32,
}

//  We're gonna do some vector maths
// So pattern repeats rightwards, so we treat each line as it's own string
// down moves down a vector
// if we go larger than the stright right we start the positioning again

fn main() {
    let vector = Vector { right: 3, down: 1 };
    let vector2 = vector;
    let pattern = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    fn toboggan_path(vector: Vector, pattern: &str) -> u32 {
        let lines: Vec<String> = pattern.split('\n').map(|s| s.to_string()).collect();
        let mut y = 0;
        let mut x = 0;
        let mut collisions = 0;
        while y < lines.len() {
            if x > (lines[y].len() - 1) {
                x = x - lines[y].len();
            }
            if lines[y].chars().nth(x).unwrap() == '#' {
                collisions = collisions + 1;
            }
            y = y + vector.down as usize;
            x = x + vector.right as usize;
        }
        return collisions;
    };
    let pattern2 = "......#..##..#...#...#.###.....
#..#............#..........#...
..........#....#..........#....
....#..#.#..........#..#.....#.
#.......#...#......#........###
#####........#.#....##..##..#..
......#.#..#..#..##.#..#.##....
.#..#.#..............##....##..
..##......#....#........#...###
...#....#.#....#.#..#......#..#
..................#.....#.....#
#.#...#...#....#............#.#
.#...#.....#...##........#.....
...#....#........#..#....#..###
#...##.....##.#.#...........#.#
.###........#.#.#.........#....
...#.............###.....#.#..#
.####.#..#....#.....#.........#
.#.#........#.#.....#.....#....
.#.......#................##.##
...#.#..#...###.....#....#..##.
...#....##..#............##...#
#...#............######...#.##.
.........#........#.#...#..##..
.....###..#.#.....##.#.#......#
..#.#...#.#..#.#.##..#.....#.#.
..#......#.#....#...#..........
..#...#.....#.#...##.....#.....
.##...........####........##...
....#............#.#...........
.....####.........#.##....###..
#..#..#.#..............#.#.....
...#.#........#.........#......
......#.#.#...#.....#....#.....
........#.#...#####..#..#......
.....#.#....#....#...........##
.#...#.........#.......##......
.#.##..##......#...............
...#.....#.......#.#.#.........
.........#..#...#...#.#.##....#
.#......##....#..#.........#...
....#.....#........#.........##
......#...........##...........
.....#..............###.#....#.
........#..#...#..#..#..#..#.#.
.#.....#.##.#..#..#.#.....#....
...#....#...#.#.....##.#...#..#
#..#......#..#.###...........#.
.##...##.#........#.#......#.#.
...#.#..#.#.......#..###...##..
#.......#.#....#..........#....
.#.....#..#.#.#..#..#........#.
.#...#......#.#...#.##.....#.##
...######..#.#....#.........##.
#.#.......................#....
..#..##...#...#.#..##.......#..
.##..#.......##......##.#..#...
#.#....##.......#..#...........
..#...#............#..#........
........#.#.........#...#..#..#
.#...###...............##...#..
...........#.....#....#....###.
#..#....##..#................##
...#.#..#..##......#....##....#
...#.##...#....#..#....#.......
#...##..##.#.........#...#....#
.##........###.#..........#....
..#..#..#...#.##..#.#......#...
.......##..#....###.##.....#..#
#....#...#.#.....#..###....##..
.#.......#.........#....#.#..#.
.........#.......#.#.......#...
..........#...##..#...#....#.##
..#........#.......#...........
#....#.....##......#....#.#...#
......#.....#....#.....#..#....
.#....##...#...##..............
..#....#......#...#....#...#...
#....###...##..#.#....##......#
..#.......#.........#..#......#
...#...#.##.......#....##..#...
..#.#...#.##..#..#..#...#.#...#
.#.........###....#....#.....#.
.#.##.#..##..#...........#....#
....##..#..##.#.......#....#..#
....#..#.........##..#......#.#
..........#.#.#....##.#......##
.##...#....###...#..........#..
#..#.....#..#.#.#.#..#......#.#
......#....#......##.#......#.#
...#.....#.......#....#.......#
.#.#................#..........
......#..#..#...............##.
##......#...#.####....#.#.#....
...#..##............#....#.....
..#..#.#...#..................#
.##.#.#..##.###.....#..#.......
..#...#.#...#......#..#........
.###..........##...###..##..#..
#.#...#........#.......##......
..##...#........#....##...##...
.......#.##.....#.#.##..#..##..
........#............#....##...
...#.#.#..#.........#.#.......#
..#..##.##...#.##...#....#...#.
.....##.#...##............##...
.#...#.###....#.......#...#...#
.......#######.#....#.....#.#..
......#.......#............##..
.....#...........#......#.....#
........#....#.##.#............
.#........#.......##.#.#....#..
#.....#..####.#................
.....#.......................##
.#.....#..##.#..##........#.#.#
#...##....#..##................
......##.###..........#.....#..
.#........#...#..............##
..#..........###.........#.....
....#.....##....#..#..#.#.#....
....#.......#.##...#.####.#....
#........#............#.##.....
..#......##.....#..#...#.......
..#......###...#.##......#..#..
#..#..#............#..#.###....
...##.........#..##...#..#.#...
..#.###..#.##.#........#..#....
......#..###.#........#........
.#....#.#..#.....#..#..#.......
#.....##.##...#...###.#.#..#.#.
.#....#..#.........#..#....###.
......##.####...#....#........#
##..#........#..#..##...#......
#.........#.........#...#..#.#.
..........#...................#
###....#....#....#......###...#
#....##........#..###.#..#.....
.#......#.....#.#.........#..#.
...#.......##.....#.........###
..............#........#.....##
....#.#..#.....###.#....##.....
.........#..##.#....#.#........
...#....#.......#.#.#..#.#....#
...........#...#..........#.#..
#.................##........###
####..#.#..#...#.....###.......
..#.#......##.#.......#........
.......##........#..#.....#..#.
...#..#......#..#.#.......###..
#....#...##..#.#.#.#.........#.
....#....#....#.#..#..........#
...###........#.#.###......##..
................#.....#.#...##.
..#..#.###...........#...###.#.
.........................#..#.#
#...#..#..##.###.....##.##.#...
...#..................#.#....#.
......#..##.#.......#.......#..
.##....#.#................#....
.#...#..#.#.#....##....#.......
.##......#.....#..........#....
..#...........#..##.........#..
....#.#...........#..........##
....#.#.#...........#.#........
......#.....#..#....##....##...
............##...##......#.#.##
#.#.....#..#....#..#...#.#...#.
.#...###..#..#.......#.......#.
.....#..#.##.....#....#...#....
##.....#..##.......##..#.#.#..#
....#.#......##....#.....#..###
.#...#.#......#.##...#..##.....
.#...#...#......##..#..#...#.#.
.#.........#....##...###...##..
###.....#......####.....#.#....
.....#..##.##................#.
.#.................#...#..##.#.
....#....#..#.......#.....#....
.##....#..#..#.....###.#..#..#.
#.#.......#.....##...#.....#...
#.#........#.#.###...#....#....
.#.....#.....##.#...#..#.......
..###.#............#...##.###..
.....#.....#..#..##............
.#.#..#.#..##..#....#...##.....
.#...........#..#.......#...#.#
#.#.#.#.....##....#............
...#.................#.#......#
.....##.............#...#.#....
.##......#.#....#..........#.#.
.#.##.......##...#...#.....#.#.
#...#.#........#......##....#.#
#....##....#....#...#..#..#.#.#
......#..........#...#.....#..#
#..#....#....#..##.#..#.#...#..
......#..#.#....#.....#.#..#..#
...#.#...###........#.#......##
..#............................
...#.#..##...##...#...#......##
...#.####......#.........#....#
.#...#.#...##....#......#.#....
.#.....##..##.#................
.#...............#.............
......#.....#...#..##..##......
...#..##.......#.......#..#.#.#
......##.....#..#.....#...#.#.#
........##........#.#........##
.#....#.....###..#.......#...#.
#...#....#.........#.......#...
...##..#........#####.#........
###..#....#.#..#...#.####......
..#..........#.#.............#.
#......#.#....#.#.#....#.##....
.#.#.#.............#....#...#..
......#.....#.#...#..###.#..#..
.....#..#............#...#...##
..#......###..#........#.#.....
#..##......#.#.#.#...........#.
#..#...##.##.....#....#..#.....
...##.#..........#.#....#...#..
.#.#.#.#..#.#...#......#.......
....#......###.#...............
.........#...#....#...#.#....#.
##.#.........#...##............
........#..........#.#...#.....
..#........#....#.......#......
#..#...............#..#...##.#.
#........#.....##.#..#....#...#
..##....#....#.#...........##..
....#.#.........#..#.....#..#..
.......##....#.#.#....###.#....
......#....#.#...#..#.........#
.....##..#....#.#......#.#.#...
#.##..##.#.......#..#...##.#.##
........#.#..#...##.#.#..#.....
#..#......#......#...#.#..#....
.....#......#.#....##....##....
....#.##...##..#..........##.#.
.#....#.......#.........#......
.#.......#.#...#...............
....#.##.......#.##..#.##..#...
#..#.......#.....#..#..........
..#.##.......#....#.#..##..#...
.#.....#...##.#.#..#...#.......
.......#.........#......#.#....
#.##.....##.......#....#.......
##.#.#.........##..#.....#....#
....#.#.#.#....#..#..##.......#
#...#...........#.#............
...#...#.#..#..##..............
......#.......#.........#..#.#.
#.....##.#....#...#..#.........
#...#..###.##..###...##.....#..
#....#.#.#...#.#..........#....
................#.#....#.....##
#.##..............####.....#.##
................#.....#........
#...#..#......#.....#......#...
.........##...........#...#...#
#.#....#...##.....#.....#..#..#
.....#...##..##.............#..
....###.#.......#.........#...#
..#.......#......#..#...#.#....
#.#....#......#.##....#.##.#...
.#.#...#.......#.#...#.##..#...
..........#......#.....#.......
........#...#.....#...##...#.#.
.....##....#.##..#........#.##.
..........##.....#..#........#.
.#....#..#.......#.##..........
.#..#..#...#...#........#.##...
.#...#.##.......#...#........#.
.....#....#.............#..#...
...#....##...#...#.....##......
#.#####.........##...#.....#...
......#.......#....#.....#..#..
..#..............#.#..#..#.....
....#.................#...#....
###.#..##.#....#...#.#......#.#
..##......#.#........#.#...##..
.....#...#...#..#.#..#..##..#..
.##...#......#...#...##.#...#..
.......###.#...........##.##...
.#.##..#.#.###.......#..##...#.
..#....#.......#..##......#....
.#....#.#..#..#.#.#....#...#...
..........##....#....#.#.......
.....#.......#.#..###.#.###....
.#.#....#.##..#.#..#.....#.#.#.
....#.....#.#.#............#...
.###....#...##......##..###..#.
...#.#..#.....#...#....##..#...
.#.#....#..........#...##.....#
#.....##...#........#.#..##..#.
.......#....#.#..........#...#.
.........#..#.#.###.........##.
..................#.#....#....#
....#....#.#..#.......###.##.##
....#...#.................#....
...#..#####.......#.#..##.##...
##.#....#...............#..#...
....#..........#...........#.#.
..##.#.##.#..#.#....#..........
.....#....#....##.#....#....#.#
.......#..##.....###...#....#.#
.#.......#..#.#.#...........#..
.#...........##.#.##....#.#....
....#.#....#.#.#......##.......
.........##......#.#.....###...
........#.#...#.##.....#.##.##.
##.#..##.#.........#....#......
.#.#.#....#..........#.#....#..
....###.........#.#.#..........
#..#....##.....#...............
#.##....#.#...#.....#......#.#.
............#.##........#......
.....#.#.....##..##............
.##..........#.......#......#..
...##..##......#.....#..#....##
.##.##...#.................##..
#....#.#........#..#....#..##.#
....##..##......#....###.#.#..#
.....#....#..#..#...##...#...#.";
    assert_eq!(toboggan_path(vector, pattern), 7);
    println!("{}", toboggan_path(vector2, pattern2).to_string())
}

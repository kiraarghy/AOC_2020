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
    fn toboggan_path(vector: Vector, pattern: &str) -> u64 {
        let lines: Vec<String> = pattern.split('\n').map(|s| s.to_string()).collect();
        let mut y = 0;
        let mut x = 0;
        let mut collisions: u64 = 0;
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
    // got overflow with u32 needed to use u64!!
    assert_eq!(toboggan_path(vector, pattern), 7);
    let vector2: Vec<Vector> = vec![
        Vector { right: 1, down: 1 },
        Vector { right: 3, down: 1 },
        Vector { right: 5, down: 1 },
        Vector { right: 7, down: 1 },
        Vector { right: 1, down: 2 },
    ];
    let res: Vec<u64> = vector2
        .iter()
        .map(|x| toboggan_path(*x, pattern2))
        .collect::<Vec<u64>>();
    println!("{}", res.into_iter().product::<u64>().to_string())
}

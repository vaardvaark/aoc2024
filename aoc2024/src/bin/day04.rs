fn part1(input: &str) -> impl std::fmt::Display {
    let grid = gridify_ascii(input.lines());
    let mut count = 0;
    for (col, row) in grid.iter_pos() {
        let mut search = vec![
            [(col, row), (col + 1, row), (col + 2, row), (col + 3, row)],
            [(col, row), (col, row + 1), (col, row + 2), (col, row + 3)],
            [
                (col, row),
                (col + 1, row + 1),
                (col + 2, row + 2),
                (col + 3, row + 3),
            ],
        ];
        if row >= 3 {
            search.push([
                (col, row),
                (col + 1, row - 1),
                (col + 2, row - 2),
                (col + 3, row - 3),
            ]);
        }

        for search in search {
            let Some(found) = map(&grid, &search) else {
                continue;
            };
            if ["XMAS", "SAMX"].contains(&found.as_str()) {
                count += 1;
            }
        }
    }
    count
}

fn part2(input: &str) -> impl std::fmt::Display {
    let grid = gridify_ascii(input.lines());
    let mut count = 0;
    for (col, row) in grid.iter_pos() {
        let pattern = [
            (col, row),         // top-left
            (col + 2, row),     // top-right
            (col, row + 2),     // bottom-left
            (col + 2, row + 2), // bottom-right
            (col + 1, row + 1), // middle
        ];
        let Some(found) = map(&grid, &pattern) else {
            continue;
        };
        if ["MMSSA", "MSMSA", "SSMMA", "SMSMA"].contains(&found.as_str()) {
            count += 1;
        }
    }

    count
}
fn map(grid: &Grid<u8>, positions: &[(usize, usize)]) -> Option<String> {
    let bytes: Vec<_> = positions
        .into_iter()
        .filter_map(|&(col, row)| grid.get((col, row)).copied())
        .collect();

    if bytes.len() == positions.len() {
        Some(String::from_utf8(bytes).unwrap())
    } else {
        None
    }
}

aoc::aoc!(day04, "18", "9");

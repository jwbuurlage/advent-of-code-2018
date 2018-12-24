use std::collections::{BinaryHeap, HashSet};
use std::io::{self, Read};

#[derive(Debug, Clone)]
struct Unit {
    pos: (usize, usize), // (i, j) matrix style
    hp: i32,
    team: char,
    active: bool,
}

fn bfs(start: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let dx: Vec<i32> = vec![-1, 0, 1, 0];
    let dy: Vec<i32> = vec![0, 1, 0, -1];
    let large = (grid.len() * grid[0].len() + 10000) as i32;

    let mut visited: Vec<Vec<bool>> = grid
        .iter()
        .map(|line| line.iter().map(|_| false).collect())
        .collect();
    let mut distances: Vec<Vec<i32>> = grid
        .iter()
        .map(|line| line.iter().map(|_| large).collect())
        .collect();
    let mut q = BinaryHeap::new();
    q.push((large, start));
    visited[start.0][start.1] = true;

    while let Some((d, (i, j))) = q.pop() {
        distances[i][j] = large - d;
        for k in 0..4 {
            let a = ((i as i32) + dx[k]) as usize;
            let b = ((j as i32) + dy[k]) as usize;
            if grid[a][b] != '.' {
                continue;
            }
            if visited[a][b] {
                continue;
            }
            q.push((d - 1, (a, b)));
            visited[a][b] = true;
        }
    }

    return distances;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    for elf_attack in 3.. {
        let input = input.clone();

        let mut grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        let mut units: Vec<Unit> = vec![];

        // find all units in the grid
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 'E' || grid[i][j] == 'G' {
                    units.push(Unit {
                        pos: (i, j),
                        hp: 200,
                        team: grid[i][j],
                        active: true,
                    });
                }
            }
        }

        for line in &grid {
            let line_as_str: String = line.iter().collect();
            println!("{}", line_as_str);
        }

        println!("{:?} {:?}", units.len(), units);

        let mut elf_died: bool = false;
        let mut ends: bool = false;
        let mut rounds = 0;

        loop {
            units.sort_by(|lhs, rhs| lhs.pos.cmp(&rhs.pos));

            for i in 0..units.len() {
                if !units[i].active {
                    continue;
                }

                let units2 = units.clone();
                let mut targets: HashSet<(usize, usize)> = HashSet::new();

                let other = match units[i].team {
                    'G' => 'E',
                    'E' => 'G',
                    _ => '?',
                };

                // check if terminates
                let mut target_count = 0;
                for unit in &units2 {
                    if unit.team != other || !unit.active {
                        continue;
                    }
                    target_count += 1;
                }
                if target_count == 0 {
                    ends = true;
                    break;
                }

                if !(grid[units[i].pos.0 - 1][units[i].pos.1] == other
                    || grid[units[i].pos.0 + 1][units[i].pos.1] == other
                    || grid[units[i].pos.0][units[i].pos.1 - 1] == other
                    || grid[units[i].pos.0][units[i].pos.1 + 1] == other)
                {
                    // we move

                    for unit in &units2 {
                        if unit.team != other || !unit.active {
                            continue;
                        }

                        targets.insert((unit.pos.0 - 1, unit.pos.1));
                        targets.insert((unit.pos.0 + 1, unit.pos.1));
                        targets.insert((unit.pos.0, unit.pos.1 - 1));
                        targets.insert((unit.pos.0, unit.pos.1 + 1));
                    }

                    for target in targets.clone() {
                        if grid[target.0][target.1] != '.' {
                            targets.remove(&target);
                        }
                    }

                    // sort by distance
                    // choose best target
                    // find minimum target
                    let ds = bfs(units[i].pos, &grid);
                    let mut min = ds[0][0];
                    let mut best = (0, 0);
                    let mut sorted_targets: Vec<(usize, usize)> =
                        targets.clone().iter().map(|x| *x).collect();
                    sorted_targets.sort();
                    for target in &sorted_targets {
                        let i = target.0;
                        let j = target.1;
                        if (ds[i][j] < min) {
                            best = (i, j);
                            min = ds[i][j];
                        }
                    }

                    if min < ds[0][0] {
                        // step to best also in reading order
                        // just try all..
                        for start in &[
                            (units[i].pos.0 - 1, units[i].pos.1),
                            (units[i].pos.0, units[i].pos.1 - 1),
                            (units[i].pos.0, units[i].pos.1 + 1),
                            (units[i].pos.0 + 1, units[i].pos.1),
                        ] {
                            if grid[start.0][start.1] == '.' {
                                let xs = bfs(*start, &grid);
                                if xs[best.0][best.1] == ds[best.0][best.1] - 1 {
                                    grid[units[i].pos.0][units[i].pos.1] = '.';
                                    units[i].pos = *start;
                                    grid[units[i].pos.0][units[i].pos.1] = units[i].team;
                                    break;
                                }
                            }
                        }
                    }
                }

                // 2. attack
                let mut min = 201;
                let mut best = (0, 0);
                for unit in &units2 {
                    if unit.team != other || !unit.active {
                        continue;
                    }
                    if vec![
                        (units[i].pos.0 - 1, units[i].pos.1),
                        (units[i].pos.0, units[i].pos.1 - 1),
                        (units[i].pos.0, units[i].pos.1 + 1),
                        (units[i].pos.0 + 1, units[i].pos.1),
                    ]
                    .contains(&unit.pos)
                    {
                        if (unit.hp < min)
                            || (unit.hp == min && unit.pos.0 < best.0)
                            || (unit.hp == min && unit.pos.0 == best.0 && unit.pos.1 < best.1)
                        {
                            best = unit.pos;
                            min = unit.hp;
                        }
                    }
                }

                for j in 0..units.len() {
                    if !units[j].active {
                        continue;
                    }
                    if units[j].pos == best {
                        if units[i].team == 'E' {
                            units[j].hp -= elf_attack;
                        } else {
                            units[j].hp -= 3;
                        }
                        if units[j].hp < 1 {
                            if units[j].team == 'E' {
                                elf_died = true;
                            }
                            grid[units[j].pos.0][units[j].pos.1] = '.';
                            units[j].active = false;
                        }
                    }
                }
            }

            if (!ends) {
                rounds += 1;
                println!("{:?} {:?}", units.len(), units);
                println!("Round: {}", rounds);
                for line in &grid {
                    let line_as_str: String = line.iter().collect();
                    println!("{}", line_as_str);
                }
            }

            if ends {
                break;
            }
        }

        let mut active_team = '.';
        let mut total_hp = 0;
        for unit in &units {
            if unit.active {
                total_hp += unit.hp;
                active_team = unit.team;
            }
        }

        println!("{} {} {} {}", active_team, elf_attack, rounds, total_hp);
        println!("{}", rounds * total_hp);

        if !elf_died {
            break;
        }
    }
}

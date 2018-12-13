use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Cart {
    pos: (usize, usize),
    mov: usize,
    dir: char,
    id: i32,
    active: bool,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut carts: Vec<Cart> = vec![];

    let mut cid = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '>' || grid[i][j] == '<' || grid[i][j] == '^' || grid[i][j] == 'v' {
                carts.push(Cart {
                    pos: (i, j),
                    mov: 0,
                    dir: grid[i][j],
                    id: cid,
                    active: true
                });
                cid += 1;
                grid[i][j] = match grid[i][j] {
                    '>' => '-',
                    '<' => '-',
                    '^' => '|',
                    'v' => '|',
                    _ => 'N',
                };
            }
        }
    }

    loop {
        // tick
        // sort carts by coordinates
        carts.sort_by(|a, b| a.pos.cmp(&b.pos));

        for i in 0..carts.len() {
            {
                if !carts[i].active {
                    continue;
                }

                let mut cart = &mut carts[i];

                match grid[cart.pos.0][cart.pos.1] {
                    '-' => match cart.dir {
                        '>' => {
                            cart.pos.1 += 1;
                        }
                        '<' => {
                            cart.pos.1 -= 1;
                        }
                        _ => {
                            assert!(false);
                        }
                    },
                    '|' => match cart.dir {
                        '^' => {
                            cart.pos.0 -= 1;
                        }
                        'v' => {
                            cart.pos.0 += 1;
                        }
                        _ => {
                            assert!(false);
                        }
                    },
                    '/' => match cart.dir {
                        '<' => {
                            cart.pos.0 += 1;
                            cart.dir = 'v';
                        }
                        '^' => {
                            cart.pos.1 += 1;
                            cart.dir = '>';
                        }
                        '>' => {
                            cart.pos.0 -= 1;
                            cart.dir = '^';
                        }
                        'v' => {
                            cart.pos.1 -= 1;
                            cart.dir = '<';
                        }
                        _ => {
                            assert!(false);
                        }
                    },
                    '\\' => match cart.dir {
                        '<' => {
                            cart.pos.0 -= 1;
                            cart.dir = '^';
                        }
                        '^' => {
                            cart.pos.1 -= 1;
                            cart.dir = '<';
                        }
                        '>' => {
                            cart.pos.0 += 1;
                            cart.dir = 'v';
                        }
                        'v' => {
                            cart.pos.1 += 1;
                            cart.dir = '>';
                        }
                        _ => {
                            assert!(false);
                        }
                    },
                    '+' => {
                        match cart.dir {
                            '<' => match cart.mov {
                                0 => {
                                    cart.pos.0 += 1;
                                    cart.dir = 'v';
                                }
                                1 => {
                                    cart.pos.1 -= 1;
                                    cart.dir = '<';
                                }
                                2 => {
                                    cart.pos.0 -= 1;
                                    cart.dir = '^';
                                }
                                _ => {
                                    assert!(false);
                                }
                            },
                            '>' => match cart.mov {
                                0 => {
                                    cart.pos.0 -= 1;
                                    cart.dir = '^';
                                }
                                1 => {
                                    cart.pos.1 += 1;
                                    cart.dir = '>';
                                }
                                2 => {
                                    cart.pos.0 += 1;
                                    cart.dir = 'v';
                                }
                                _ => {
                                    assert!(false);
                                }
                            },
                            '^' => match cart.mov {
                                0 => {
                                    cart.pos.1 -= 1;
                                    cart.dir = '<';
                                }
                                1 => {
                                    cart.pos.0 -= 1;
                                    cart.dir = '^';
                                }
                                2 => {
                                    cart.pos.1 += 1;
                                    cart.dir = '>';
                                }
                                _ => {
                                    assert!(false);
                                }
                            },
                            'v' => match cart.mov {
                                0 => {
                                    cart.pos.1 += 1;
                                    cart.dir = '>';
                                }
                                1 => {
                                    cart.pos.0 += 1;
                                    cart.dir = 'v';
                                }
                                2 => {
                                    cart.pos.1 -= 1;
                                    cart.dir = '<';
                                }
                                _ => {
                                    assert!(false);
                                }
                            },
                            _ => {
                                assert!(false);
                            }
                        };
                        cart.mov = (cart.mov + 1) % 3
                    }
                    _ => {
                        assert!(false);
                    }
                }
            }

            for (j, cart) in carts.clone().iter().enumerate() {
                if !cart.active { continue; }
                if carts[i].id == cart.id {
                    continue;
                }

                // collision?
                // TODO output crash and return
                if carts[i].pos == cart.pos {
                    carts[i].active = false;
                    carts[j].active = false;
                    break;
                }
            }
        }


        println!("{}",  carts.iter().filter(|cart| cart.active).count());

        if carts.iter().filter(|cart| cart.active).count() == 1 {
            for cart in &carts {
                if cart.active {
                    println!("{:?}", cart.pos);
                    return;
                }
            }
        }

        // let mut output = grid.clone();
        // for cart in &mut carts {
        //     output[cart.pos.0][cart.pos.1] = cart.dir;
        // }
        // for line in output {
        //     let line_str: String = line.iter().collect();
        //     println!("{}", line_str);
        // }
    }
}

use std::{fs};
use std::thread::sleep;
use std::time::Duration;
#[derive(Clone)]
struct DataBlock{
    position: (usize, usize),
    value: i32
}

fn main() {

    let input = fs::read_to_string("fibonacci.cv").expect("FS err");
    let map = input.split("\n").map(|row| row.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut data_blocks: Vec<DataBlock> = vec![
        DataBlock{
            position: (0, 0),
            value: 1
        }
    ];

    let onesec = Duration::from_secs(1);
    loop {
        let mut new_blocks: Vec<DataBlock> = vec![];

        let mut i: usize = 0;
        let mut decrement: bool = false;
        while i < data_blocks.len(){
            let block = data_blocks[i].clone();
            let operator = map[block.position.1][block.position.0];

            match operator {
                '>' => {
                    data_blocks[i].position.0 += 1;
                    //new_blocks.push(data_blocks[i].clone());
                },
                '<' => {
                    data_blocks[i].position.0 -= 1;
                    //new_blocks.push(data_blocks[i].clone());
                },
                'v' => {
                    data_blocks[i].position.1 += 1;
                    //new_blocks.push(data_blocks[i].clone());
                },
                '^' => {
                    data_blocks[i].position.1 -= 1;
                    //new_blocks.push(data_blocks[i].clone());
                },
                'P' => {
                    println!("{}", data_blocks[i].value);
                    data_blocks.remove(i);
                    decrement = true;
                },
                '"' => {
                    let neighbs = get_neighbours(&map, block.position);
                    data_blocks[i].position = neighbs[0];
                    data_blocks.insert(0, DataBlock{
                        position: (neighbs[1].0, neighbs[1].1),
                        value: block.value
                    });
                    i += 1;
                },
                '+' => {
                    for n in 0..data_blocks.len(){
                        if n == i{continue;}
                        if data_blocks[i].position == data_blocks[n].position{
                            let neighbs = get_neighbours(&map, block.position);
                            data_blocks[i].position = neighbs[0];
                            data_blocks[i].value += data_blocks[n].value;
                            data_blocks.remove(n);
                            if n < i{decrement = true;};
                            break;
                        }
                    }
                    /*for n in neighbours.iter(){
                        if map[(block.position.1 as i32 + n.1) as usize][(block.position.0 as i32 + n.0) as usize] == n.2{
                            new_blocks.push(DataBlock{
                                position: ((block.position.0 as i32 + n.0) as usize, (block.position.1 as i32 + n.1) as usize),
                                value: block.value
                            });
                        }
                    }*/
                },
                _ => ()
            }

            if !decrement{
                i+=1;
            }
            decrement = false;
        }

        //data_blocks = new_blocks;

        // RENDER
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        let mut vals: Vec<(char, i32)> = vec![];

        let mut vis = map.clone();
        for (i, block) in data_blocks.iter().enumerate(){
            vis[block.position.1][block.position.0] = (65 + i) as u8 as char;
            vals.push(((65 + i) as u8 as char, block.value));
        }

        //vis.iter().map(|row| println!("{}", row.iter().collect::<String>()));
        for row in vis.iter(){
            println!("{}", row.iter().collect::<String>());
        }

        println!();

        for val in vals{
            println!("{}: {}" , val.0, val.1);
        }

        if data_blocks.len() == 0{
            break
        }

        sleep(onesec);
    }
    println!("Done!");
}

fn get_neighbours(map: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<(usize, usize)>{

    let neighbour_offsets: Vec<(i32, i32, char)> = vec![
        (0, -1, '^'),
        (1, 0, '>'),
        (0, 1, 'v'),
        (-1, 0, '<')
    ];

    let mut neighbours: Vec<(usize, usize)> = vec![];

    for n in neighbour_offsets.iter(){
        if map[(pos.1 as i32 + n.1) as usize][(pos.0 as i32 + n.0) as usize] == n.2{
            neighbours.push(((pos.0 as i32 + n.0) as usize, (pos.1 as i32 + n.1) as usize));
        }
    }

    neighbours
}

enum ConveyorOperation{
    Move
}
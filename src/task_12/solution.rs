use crate::common::Timer;

pub fn run(timer: &mut Timer) {
    let input = include_str!("./test.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| {
                    char as u32
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let START: u32 = char::from('S') as u32;
    let END: u32 = char::from('E') as u32;

    let mut row_pos = 0;
    let mut col_pos = 0;

    'outer: for (row_index, line) in input.iter().enumerate() {
        for (column_index, cell) in line.iter().enumerate() {
            if *cell == START {
                row_pos = row_index;
                col_pos = column_index;
                break 'outer;
            }
        }
    }

    println!("{} {}", row_pos, col_pos);


    let last_height = START;

    for i in 0..10 {
        
    }

    // while last_height != END {

    // }

    let result = 1;

    // assert_eq!(result, 55944);
    // timer.log("12.1", result);
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        
    }
}

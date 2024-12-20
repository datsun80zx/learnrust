fn main() {

    for row in 0..7 {
        let mid: i32 = 3;
        let row: i32 = row as i32;
        let num_hashes = 4 - (row - mid).abs();

        for _ in 0..num_hashes {
            print!("#");
        }
        println!();
    }


    let total_rows: i32 = 25;
    let middle: i32 = total_rows/2;
    

    for row in 0..total_rows {
        let num_spaces: i32 = (row - middle).abs();
        let num_hashes: i32 = 2 * (middle - (row - middle).abs()) + 1;
        let row: i32 = row as i32;

        for _ in 0..num_spaces {
            print!(" ");
        } 

        for _ in 0..num_hashes {
            print!("#");
        }
        println!();
    }

}
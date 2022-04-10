mod matrix;
mod read;

use crate::matrix::Matrix;

fn main() {
    println!("Please enter the the path of the first file");
    let file_1 =
        read::read_file_lines(read::read_from_console()).expect("Error en lectura de archivo");

    println!("Please enter the the path of the second file");
    let file_2 =
        read::read_file_lines(read::read_from_console()).expect("Error en lectura de archivo");

    let mut matrix = Matrix::new(file_1.len() + 1, file_2.len() + 1, 0);

    matrix.lcs(&file_1, &file_2);

    matrix.print_diff(&file_1, &file_2, file_1.len(), file_2.len());
}

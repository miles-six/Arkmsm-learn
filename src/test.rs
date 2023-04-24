// TODO: remove this when you're done with your implementation.


fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = matrix[j][i];
            println!("{}",result[i][j])
        }
    }
    result
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            println!("{}",matrix[i][j])
        }
    }
}


#[test]
fn test_transpose() {
        let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    // println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}




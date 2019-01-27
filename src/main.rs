mod array;

use array::Array;

fn main() {
    let narray = Array::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        vec![2, 2, 2],
        String::from("smthing")
    );

    println!("{:?}", narray.get_raw(vec![1]));
}


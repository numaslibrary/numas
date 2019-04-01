pub mod array;
pub mod shape;
pub mod mathematic;
pub mod factory;
pub mod operator;


use self::array::Array;
use std::cell::RefCell;
use std::rc::Rc;
use std::cell::Cell;


fn main() {
//    let arr = factory::fill::zeroes(vec![2,2]);
//
//    println!("{:?}", arr);
//
//
//    let arrr = factory::random::random_range::<f64>(5.0, 20.2, vec![2,3]);
////    let sarr = mathematic::hyperbolic::sinh(&arrr);
//

    let mut sarr = Array::new_bounded(vec![0,1,2,3,4,5,6,7,8,9], vec![3,3], 1, 10);
//    println!("{:?}", arrr);

//    sarr = sarr.get();
    println!("{:?}", sarr.get(vec![vec![2]]));

//    let mut narray = Array::new(
//        vec![1, 2, 3, 4, 5, 6, 7, 8],
//        vec![2, 2, 2]
//    );
//
//    let sarray = narray.clone();
//    let mut varray = narray.view();
//
//    varray.set_t(5, 78797);
//
//    println!("{:?}", narray);
//    println!("{:?}", narray.get_raw(vec![1, 1, 0]));

//    println!("{:?}", sarray);
//    println!("{:?}", varray);


//    let mut test = vec![1, 2, 3];
//    let mut test_cell = Rc::new(test);
//
//    let mut wwwr = &test_cell;
//    let mut ttsts = &test_cell;
//
//    let mut wwr = &test_cell;
//
//    wwr[0] = 5;
//
//    let mut tsts = ttsts;
////    let mut br: Vec<i32> = tst.as_mut();
////
////    let mut brr: Vec<i32> = tsts.as_mut();
//
////    br[0] = 5;
//
////    tst[0] = 4;
//    println!("{:?}", wwr);
//    println!("{:?}", tsts);

}


/*
    data: [1, 2, 3, 4, 5, 6, 7, 8]
    shape: [2, 2, 2]


    [
        [
            [ 1 2 ]
            [ 3 4 ]
        ]
        [
            [ 5 6 ]
            [ 7 8 ]
        ]
    ]

    1

*/

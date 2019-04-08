/// Returns indices for slicing, each dimension separated with ';' and range with '=>'
///
/// # Examples
///
/// ```
/// let example = s![2,4; 5]; // Returns vector of [[2, 4], [5]]
///
/// assert(s![1; 5 => 2; 3], vec![vec![1], vec![5,2], vec![3]]);
/// assert(s![1; 5; 2; 3], vec![vec![1], vec![5], vec![2], vec![3]]);
/// ```
#[macro_export]
macro_rules! s {
    ($($( $y:expr )=>*);*) => {
        vec![ $(vec![$( $y ),*]),* ]
    }
}

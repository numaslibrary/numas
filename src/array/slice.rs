/// Returns indices for slicing, each dimension separated with ';' and range with '=>'
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate numas;
///
/// assert_eq!(s![1; 5 => 2; 3], vec![1, 0, 5, 2, 3, 0]);
/// assert_eq!(s![1; 5; 2; 3], vec![1, 0, 5, 0, 2, 0, 3, 0]);
/// ```
#[macro_export]
macro_rules! s {
    ($($( $y:expr )=>*);*) => (
        {
            let mut vec: Vec<usize> = Vec::new();

            $(
                tuple![vec; $( $y ),*];
            )*

            vec
        }
    )
}

/// Adds to vector always two values, if only one value provided use zero instead
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate numas;
///
/// let mut vec: Vec<i32> = Vec::new();
/// tuple![vec; 1, 5];
/// assert_eq!(vec, vec![1, 5]);
///
/// tuple![vec; 5];
/// assert_eq!(vec, vec![1, 5, 5, 0])
/// ```
#[macro_export]
macro_rules! tuple {
    ($vec:expr; $y:expr, $x:expr) => {
        $vec.push($y);
        $vec.push($x);
    };

    ($vec:expr; $y:expr) => {
        $vec.push($y);
        $vec.push(0);
    };
}

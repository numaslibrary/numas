/// Returns tuple of start, step and end index
///
/// # Examples
///
/// ```
/// let example = s![2,4,5]; // Returns tuple of (2, 4, 5)
///
/// assert(s![1, 5], (1, 1, 5));
/// assert(s![, 5], (0, 1, 5));
/// assert(s![2, 4, 5], (2, 4, 5));
/// ```
#[macro_export]
macro_rules! s {
    ($start:expr, $step:expr, $end:expr) => (($start, $step, $end));
    ($start:expr, $step:expr,) => (($start, $step, -1));
    ($start:expr, $end:expr) => (($start, 1, $end));
    (,$step:expr, $end:expr) => ((0, $step, $end));
    ($start:expr) => (($start, 1, $start + 1));
    ($start:expr,) => (($start, 1, -1));
    (,$end:expr) => ((0, 1, $end));
}
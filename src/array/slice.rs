macro_rules! s {
    ($start:expr, $step:expr, $end:expr) => ($start, $step, $end);
    ($start:expr, $end:expr) => ($start, 1, $end);
    (,$end:expr) => (0, 1, $end);
    ($start:expr,) => ($start, 1, -1);
}
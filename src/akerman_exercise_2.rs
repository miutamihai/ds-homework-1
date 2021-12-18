pub fn akerman(m: i32, n:i32) -> i32 {
    if m == 0 {
       n + 1
    } else if n == 0 {
       akerman(m - 1, 1)
    } else {
        akerman(m - 1, akerman(m, n - 1))
    }
}

pub fn recursive(n: i32) -> i32 {
   if n == 0 {
       1
   } else {
       n * recursive(n - 1)
   }
}

pub fn iterative(n: i32) -> i32 {
    let mut result = 1;

    for i in 1..=n {
        result = result * i;
    }

    result
}

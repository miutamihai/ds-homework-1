pub fn recursive(n: i32, k: i32) -> i32 {
    if n < 0 {
        return 0;
    }
    if n < k - 2 {
        0
    } else if n == k - 2 {
        1
    } else {
        recursive(n - 1, k) + recursive(n - 2, k)
    }
}

pub fn iterative(n: i32, k: i32) -> i32 {
    if n < 0 {
        return 0;
    }

    if n < k - 2 {
        return 0;
    } else if n == k - 2 {
        return 1;
    }

    let mut first_number: i32 = 0;
    let mut second_number: i32 = 1;
    let mut current_number: i32 = 1;

    let mut i: i32 = 1;

    while i < n {
        first_number = second_number;

        second_number = current_number;

        current_number = first_number + second_number;

        i = i + 1;
    }
    return current_number;
}

pub fn nth_fibonacci_number(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let mut n1 = 0;
        let mut n2 = 1;
        let mut i = 2;
        while i < n {
            let x = n1 + n2;
            n1 = n2;
            n2 = x;
            i += 1;
        }

        return n1 + n2;
    }
}

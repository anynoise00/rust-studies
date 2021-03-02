fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Another way we could implement largest is for the function to return a reference
// to a T value in the slice. If we change the return type to &T instead of T,
// thereby changing the body of the function to return a reference, we wouldn’t need
// the Clone or Copy trait bounds and we could avoid heap allocations.
// Try implementing these alternate solutions on your own!
fn largest_ref<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest_index = 0;

    for i in 0..list.len() {
        if list[i] > list[largest_index] {
            largest_index = i;
        }
    }

    &list[largest_index]
}

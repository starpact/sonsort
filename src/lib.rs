#![allow(dead_code)]

fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        let mut swapped = false;
        for j in 1..arr.len() - i {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn selection_sort(arr: &mut [i32]) {
    for i in 0..arr.len() - 1 {
        let mut min_index = i;
        for j in i..arr.len() {
            if arr[min_index] > arr[j] {
                min_index = j;
            }
        }
        if i != min_index {
            arr.swap(i, min_index);
        }
    }
}

fn insertion_sort(arr: &mut [i32]) {
    for unsorted in 1..arr.len() {
        let mut i = unsorted;
        while i > 0 && arr[i] < arr[i - 1] {
            arr.swap(i - 1, i);
            i -= 1;
        }
    }
}

fn insertion_sort1(arr: &mut [i32]) {
    for unsorted in 1..arr.len() {
        let index = arr[..unsorted]
            .binary_search(&arr[unsorted])
            .unwrap_or_else(|x| x);
        arr[index..=unsorted].rotate_right(1);
    }
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = arr[0];
    let (mut left, mut right) = (1, arr.len() - 1);
    while left < right {
        if arr[left] <= pivot {
            left += 1;
        } else if arr[right] >= pivot {
            right -= 1;
        } else {
            arr.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
    if arr[left] > pivot {
        left -= 1;
    }
    arr.swap(0, left);

    quick_sort(&mut arr[..left]);
    quick_sort(&mut arr[left + 1..]);
}

fn quick_sort1(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = arr[arr.len() - 1];
    let mut index = 0;
    for i in 0..arr.len() - 1 {
        if arr[i] < pivot {
            while index <= i {
                if arr[index] > pivot {
                    arr.swap(i, index);
                    index += 1;
                    break;
                }
                index += 1;
            }
        }
    }
    arr.swap(index, arr.len() - 1);

    quick_sort1(&mut arr[..index]);
    quick_sort1(&mut arr[index + 1..]);
}

fn merge_sort(arr: &mut [i32]) {
    fn f(arr: &mut [i32], helper: &mut [i32]) {
        if arr.len() <= 1 {
            return;
        }
        let mid = arr.len() / 2;
        f(&mut arr[..mid], &mut helper[..mid]);
        f(&mut arr[mid..], &mut helper[mid..]);
        let (mut i, mut j, mut index) = (0, mid, 0);
        loop {
            if i == mid {
                break;
            }
            if j == arr.len() {
                arr.copy_within(i..mid, index);
                break;
            }
            if arr[i] < arr[j] {
                helper[index] = arr[i];
                i += 1;
            } else {
                helper[index] = arr[j];
                j += 1;
            }
            index += 1;
        }
        arr[..index].copy_from_slice(&helper[..index]);
    }

    f(arr, &mut vec![0; arr.len()]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_bubble_sort() {
        do_sort(bubble_sort);
    }

    #[test]
    fn test_selection_sort() {
        do_sort(selection_sort);
    }

    #[test]
    fn test_insertion_sort() {
        do_sort(insertion_sort);
        do_sort(insertion_sort1);
    }

    #[test]
    fn test_quick_sort() {
        do_sort(quick_sort);
        do_sort(quick_sort1);
    }

    #[test]
    fn test_merge_sort() {
        do_sort(merge_sort);
    }

    fn do_sort<F>(f: F)
    where
        F: Fn(&mut [i32]),
    {
        let n = 100;
        let mut arr = Vec::with_capacity(n);
        for _ in 0..n {
            arr.push(rand::thread_rng().gen_range(0..n as i32));
        }
        let mut arr_correct = arr.clone();
        arr_correct.sort_unstable();
        f(&mut arr);
        assert_eq!(arr, arr_correct);
    }
}

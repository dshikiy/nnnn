pub fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }

    for i in 0..len {
        let mut swapped = false;
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

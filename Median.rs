fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (arr[mid_left] as f64 + arr[mid_right] as f64) / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
    let arr1 = vec![1, 2, 3, 4, 5];
    let arr2 = vec![1, 2, 3, 4, 5, 6];
    let arr3 = vec![1, 2, 3, 4, 5, 6, 7];
    
    println!("Median of {:?}: {}", arr1, median(&arr1));
    println!("Median of {:?}: {}", arr2, median(&arr2));
    println!("Median of {:?}: {}", arr3, median(&arr3));
}

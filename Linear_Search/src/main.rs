fn linear_search(item: i32, arr: &[i32]) -> i32 {
    let mut idx_pos = -1;
    for (idx, data) in arr.iter().enumerate() {
        if item == *data {
            idx_pos = idx as i32;
            return idx_pos;
        }
    }
    idx_pos
}
fn main() {
    let item = -17;
    let arr: [i32; 6] = [77, 25, -17, 1, 48, 7];
    let index = linear_search(item, &arr);
    if index >= 0 {
        println!("Element found at position: {}", index);
    }
    else {
        println!("Element not found");
    }
}


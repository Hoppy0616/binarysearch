use std::io;
use std::convert::TryInto;
fn main() {
    let a = [5,8,12,15,17,23,38,41,62,91];
    let mut number = String::new();
    let (mut pos,mut left,mut right) = (-1isize,0isize,9isize);
    let mut middle;
    print!("input number: ");
    io::stdin().read_line(&mut number).ok();
    let mut x = number.trim().parse().ok().unwrap();
    while (pos == -1 && left <= right){
        middle = (left + right) as usize / 2;
        if (a[middle] == x){
            pos = (middle as usize).try_into().unwrap();
        }
        else if (a[middle] > x){
            right = (middle - 1 as usize).try_into().unwrap();
        }
        else {
            left = (middle + 1 as usize).try_into().unwrap();
        }
    }
    print!("pos = {}",pos);
}

const N: usize = 10;

fn main() {
    println!("{}", find_duplicate([1, 2, 8, 3, 4, 5, 6, 7, 8, 9]))
}

fn find_duplicate(nums: [usize; N]) -> usize {
    let mut tortoise = nums[0];
    let mut hare = nums[0];
    loop {
        tortoise = nums[tortoise];
        hare = nums[nums[hare]];
        if tortoise == hare {
            break;
        }
    }

    let mut ptr1 = nums[0];
    let mut ptr2 = tortoise;
    while ptr1 != ptr2 {
        ptr1 = nums[ptr1];
        ptr2 = nums[ptr2];
    }
    ptr1
}
fn main() {
    let random_nums = [1u32, 2, u32::MAX];
    println!("sum of following array {:?}", &random_nums);
    println!("result: {:?}", sum(&random_nums))
}


fn sum(nums: &[u32]) -> Option<u32>{
    let mut result = 0u32;
    for i in nums.iter(){
        match result.checked_add(*i){
            Some(x) => result = x,
            None => return None
        }
    }
    return Some(result);
}
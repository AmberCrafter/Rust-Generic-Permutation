#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn permute_one<T>(nums: &Vec<T>) -> Vec<T> 
    where 
        T: PartialEq + PartialOrd + Copy + std::fmt::Debug,
    {
        /*
        example: 
                        nums = [2 ,1, 6, 4, 2]
        step 1: find the decrease point from the end of list
                                   ^
                                   dp
           
        step 2: swap the dp value with the larger one which figure out from the end of list
                                            ^
                                            swap target

        step 3: reverse the part of the list after dp
                [2, 2, 6, 4, 1]  -> [2, 1, 4, 6, 2]
                       ^
                       reverse  start point
        */


        let nums_len = nums.len();
        
        let mut nums = (*nums).clone();
        if nums_len==0 {return nums;}
        // println!("Input: {:?}", nums);

        // step 1: find the decrease point
        let mut dp = 0;
        let mut flag = true;
        for i in 1..nums_len{
            // println!("Step1: i:{:?}\tx:{:?}\ty:{:?}", i, nums[nums_len-i], nums[nums_len-i-1]);
            if nums[nums_len-i]>=nums[nums_len-i-1] {
                dp = nums_len-i-1;
                flag = false;
                break;
            }
        };

        // println!("Step1: {:?}\t {:?}", dp, nums);
        if dp==0 && flag{
            nums.reverse(); return nums
        };

        // step 2: swap the value which larger than dp
        for i in 1..nums_len-dp {
            if nums[nums_len-i]>nums[dp] {nums.swap(nums_len-i, dp); break;}
        }
        // println!("Step2: {:?}", nums);

        println!("dp: {}", dp);
        // step 3: reverse the last parts of the list after dp
        for i in 1..=(nums_len-dp)/2 {
            nums.swap(nums_len-i, dp+i);
            println!("Step3: {:?}", nums);
        }
        nums
    }


    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::new();
        results.push(nums);

        let mut next = Solution::permute_one(results.last().unwrap());
        while next!=results[0] {
            results.push(next);
            next = Solution::permute_one(results.last().unwrap());
        }
        results
    }
}

fn main() {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permute_one() {
        let nums = vec![3,2,1];
        let result = Solution::permute_one(&nums);
        println!("result: {:?}", result);
        let result = Solution::permute_one(&result);
        println!("result: {:?}", result);
        let result = Solution::permute_one(&result);
        println!("result: {:?}", result);
        let result = Solution::permute_one(&result);
        println!("result: {:?}", result);
    }
}
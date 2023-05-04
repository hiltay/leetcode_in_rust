struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1 = version1.split(".").collect::<Vec<&str>>();
        let v2 = version2.split(".").collect::<Vec<&str>>();
        let mut v1_value = 0;
        let mut v2_value = 0;
        let max_length = if v1.len() > v2.len() {
            v1.len()
        } else {
            v2.len()
        };
        for i in 0..max_length {
            if i >= v1.len() {
                v1_value = 0;
                v2_value = v2[i].parse::<i32>().unwrap();
            } else if i >= v2.len() {
                v2_value = 0;
                v1_value = v1[i].parse::<i32>().unwrap();
            } else {
                v2_value = v2[i].parse::<i32>().unwrap();
                v1_value = v1[i].parse::<i32>().unwrap();
            }

            if v1_value > v2_value {
                return 1;
            } else if v1_value < v2_value {
                return -1;
            }
        }
        0
    }
}

fn main() {
    let version1 = "1.0".to_string();
    let version2 = "1.0.0".to_string();
    let result = Solution::compare_version(version1, version2);
    println!("{:?}", result);
}

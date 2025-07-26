struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("(", ")");
        map.insert("[", "]");
        map.insert("{", "}");

        let mut map2 = HashMap::new();
        map2.insert("hello".to_string(), "world");

        let mut v = Vec::new();
        for c in s.chars() {
            let key = c.to_string();
            if map.contains_key(key.as_str()) {
                // eprintln!("insert {}", &c);
                v.push(c);
            } else {
                if let Some(lv) = v.pop(){
                    // eprintln!("compare {} and {}", &lv, &c);
                    if map.get(lv.to_string().as_str()).unwrap() == &c.to_string() {
                        continue
                    } else {
                        return false
                    }
                } else {
                    return false 
                }
            }
        }
        if v.len() > 0 {false} else {true}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid(){
        let s1 = String::from("()");
        let s2 = String::from("()[]{}");
        let s3 = String::from("(]");
        let s4 = String::from("([])");
        let s5 = String::from("([)]");

        assert_eq!(Solution::is_valid(s1), true);
        assert_eq!(Solution::is_valid(s2), true);
        assert_eq!(Solution::is_valid(s3), false);
        assert_eq!(Solution::is_valid(s4), true);
        assert_eq!(Solution::is_valid(s5), false);
    }
}
use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    let mut hashmap = HashMap::new();
    hashmap.insert('}', '{');
    hashmap.insert(')', '(');
    hashmap.insert(']', '[');
    
    for c in string.chars() {
        match "{([".find(c) {
            Some(_) => {
                stack.push(c);
            } 
            None => {
                if let None = "})]".find(c) {
                    continue;
                }
                if stack.len() == 0 {
                    return false;
                } else if stack.last() == hashmap.get(&c) {
                    stack.pop();
                } else {
                    return false;
                }
            }
        }
    }
    stack.len() == 0
}

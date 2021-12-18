// To be called `subset(set, [], [], 0);`

pub fn subsets(set: &Vec<i32>, result: &mut Vec<Vec<i32>>, subset: &Vec<i32>, index: usize) -> () {
    if index > set.len() {
        result.push(subset.clone());
    }
    subsets(set, result, subset, index + 1);
    subsets(set, result, subset + set[index], index + 1);
}

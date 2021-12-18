mod subsets_exercise_4;

use subsets_exercise_4::subset;

fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);
    let pset = subset(&v);
    println!("{:?}", pset);
}

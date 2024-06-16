fn main() {
    let x = 1;
    let mut x = x + 4;
    println!("here x is {x}");
    x = 9;
    println!("here x is {x}");

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [0; 5];

    let a = [1, 2, 3, 4];
    for number in a {
      println!("{number}");
    }

    for number in (1..=4).rev() {
      println!("{number}");
    }
}

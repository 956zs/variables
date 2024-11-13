fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("x 內部範圍值: {x}");
    }
    println! {"x 的值: {x}"};
}

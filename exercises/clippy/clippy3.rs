// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.




fn main() {
   // let my_option: Option<()> = None;

    // 我们不再在 is_none() 后调用 unwrap()，避免 panic
    // if my_option.is_none() {
    //     my_option.unwrap(); // 这会 panic!
    // }

    // ✅ 修复数组：添加缺失的逗号
    let my_arr = &[-1, -2, -3, -4, -5, -6];

    println!("My array! Here it is: {:?}", my_arr);

    // ✅ 修复 Vec：使用 clear() 来清空
    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.clear(); // ✅ 正确方式清空 Vec

    println!("This Vec is empty, see? {:?}", my_vec);

    // ✅ 正确交换两个变量
    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);

    println!("value a: {}; value b: {}", value_a, value_b);
}
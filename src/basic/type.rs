/*
@Author : NewtSun
@Time : 2023/11/6 15:39
@Description : 基本类型
*/

// 数值类型:
// ① 有符号整数 (i8, i16, i32, i64, isize)
// ② 无符号整数 (u8, u16, u32, u64, usize)
// ③ 浮点数 (f32, f64)、以及有理数、复数
// 字符串：字符串字面量和字符串切片 &str
// 布尔类型： true和false
// 字符类型: 表示单个 Unicode 字符，存储为 4 个字节
// 单元类型: 即 () ，其唯一的值也是 ()

use num::complex::Complex;

pub fn guess() {
    let guess = "42".parse().expect("Not a number!"); // 这个无法解析
    let guess = "42".parse::<i32>().expect("Not a number!");
}

// 1.数值类型

// 关于整形溢出，要显式处理可能的溢出，可以使用标准库针对原始数字类型提供的这些方法：
// 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
// 如果使用 checked_* 方法时发生溢出，则返回 None 值
// 使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
// 使用 saturating_* 方法使值达到最小值或最大
pub fn over_flow() {
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);  // 19
}

pub fn float_assert() {
    // 断言0.1 + 0.2与0.3相等
    // 这段代码有问题：
    // 因为二进制精度问题，导致了 0.1 + 0.2 并不严格等于 0.3，它们可能在小数点 N 位后存在误差
    assert_eq!(0.1 + 0.2, 0.3);
    // 可以考虑用这种方式 (0.1_f64 + 0.2 - 0.3).abs() < 0.00001
}

// NaN 指的是：
// 对于数学上未定义的结果，例如对负数取平方根 -42.1.sqrt()
// 会产生一个特殊的结果：Rust 的浮点数类型使用 NaN (not a number)来处理这些情况
pub fn nan_assert() {
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }
}

// range 序列
pub fn generate_range() {
    for i in 1..=5 {
        println!("{}", i)
    }

    for i in 'a'..='z' {
        println!("{}", i);
    }
}

// 无理数
pub fn complex_num() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}

// 2.字符类型

// 1个字符占用4个字节
pub fn char_size() {
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));
}

// 3.单元类型
// 单元类型就是 () ，对，你没看错，就是 () ，唯一的值也是 ()
// main 函数就返回这个单元类型 () 你不能说 main 函数无返回值
// 没有返回值的函数在 Rust 中是有单独的定义的：发散函数( diverge function )，顾名思义，无法收敛的函数
// 可以用 () 作为 map 的值 表示我们不关注具体的值 只关注 key
// 这种用法和 Go 语言的 struct{} 类似，可以作为一个值用来占位，但是完全不占用任何内存
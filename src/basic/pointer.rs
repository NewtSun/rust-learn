/*
@Author : NewtSun
@Time : 2023/11/7 13:39 
@Description : 指针
*/

use std::ops::Deref;

// 智能指针
pub fn smart_pointer() {
    // 最直接的智能指针是 box
    // 其类型是 Box<T>，box 允许你将一个值放在堆上而不是栈上，留在栈上的则是指向堆数据的指针
    // 使用 box 在堆上储存一个 i32：

    let b = Box::new(5);
    println!("b = {}", b);

    // 这里定义了变量 b，其值是一个指向被分配在堆上的值 5 的 Box
    // 这个程序会打印出 b = 5
    // 在这个例子中，我们可以像数据是储存在栈上的那样访问 box 中的数据
    // 正如任何拥有数据所有权的值那样，当像 b 这样的 box 在 main 的末尾离开作用域时，它将被释放
    // 这个释放过程作用于 box 本身（位于栈上）和它所指向的数据（位于堆上）
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(T)
    }
}

impl<T> Deref for MyBox<T> {
    type target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
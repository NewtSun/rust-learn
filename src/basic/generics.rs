/*
@Author : NewtSun
@Time : 2023/11/6 15:39
@Description : 泛型
*/

// 不是所有 T 类型都能进行相加操作
// 因此我们需要用 std::ops::Add<Output = T> 特征（Trait） 对 T 进行限制
fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

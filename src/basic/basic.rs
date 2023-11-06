/*
@Author : NewtSun
@Time : 2023/11/6 15:39
@Description : 基本类型
*/

pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

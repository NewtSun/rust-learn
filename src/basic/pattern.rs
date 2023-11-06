/*
@Author : NewtSun
@Time : 2023/11/6 15:39
@Description : 模式匹配
*/

struct Point {
    x: i32,
    y: i32,
}

pub fn pattern_let() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(0, b);
}
//cargo new addr --lib 自动生成测试模块和函数
//使用cargo test来运行测试
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)] //config,当配置参数为test(即运行cargo test)时才被执行
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
 //集成测试
 use adder; //导入库

 mod common;//导入公共的函数

 #[test]
 fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
 }
# rust 入门
由于此前并未写日志，且中间有较长时间忙于其它事，这里先简单总结一下前面利用零碎时间做的工作

主要参考：
1. [The Rust Programming Language](https://doc.rust-lang.org/stable/book/title-page.html)(主)
2. [rust官方文档](https://doc.rust-lang.org/std/index.html)
3. [rust圣经](https://course.rs/about-book.html)

## rust 安装
首先尝试了在windows系统下安装rust，跟随官网上的[教程](https://www.rust-lang.org/tools/install)，并进行了测试。安装rust时会自动下载cargo等等。
```
rustc --version       //查看版本信息
rustup update         //更新
rustup self uninstall //卸载
```

使用训练营给出的配置好的盘符新建了VM虚拟机,并在虚拟机中完成了'Hello,World!','Hello,Cargo!'以及简单猜数游戏的编程实践。

可以直接使用``rustc file.rs``进行编译并在同目录下生成可执行文件。更方便是使用cargo来进行管理，相关命令如下：
```
cargo --version

cargo new programName //创建名为programName的项目
/**
  同时生成以下内容：
  src         //源文件目录
    main.rs
  .gitignore //相当于自动初始化了git仓库
  Cargo.lock //用于确保构建是可重现的，无需手动修改 
  Cargo.toml //配置文件，可添加相关依赖

 */

cargo build //编译并产生可执行文件，在./target/debug/目录下
cargo run  //编译并运行项目
cargo check //只编译不产生可执行文件，比build更快
cargo build --release //优化编译项目，在./target/release/目录下

```
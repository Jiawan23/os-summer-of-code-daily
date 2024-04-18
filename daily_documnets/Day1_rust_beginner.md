# rust 入门 (4.18)
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

使用训练营给出的配置好的盘符新建了VM虚拟机,并在虚拟机中完成了'Hello,World!','Hello,Cargo!'以及简单猜数游戏的编程实践。相关程序在practice目录下。
后来转到了windows下直接实践

## rust 编译与运行

可以直接使用``rustc file.rs``进行编译并在同目录下生成可执行文件。更方便是使用cargo来进行管理，相关命令如下：
```
cargo --version

cargo new programName //创建名为programName的项目
/**
  同时生成以下内容：
  src         //源文件目录
    main.rs
  .gitignore // git时忽略掉的文件内容(如target/)
  Cargo.lock //用于确保构建是可重现的，无需手动修改 
  Cargo.toml //配置文件，可添加相关依赖(如外部crate)
  .git        //相当于自动初始化了git仓库
 */

cargo build //编译并产生可执行文件，在./target/debug/目录下
cargo run  //编译并运行项目
cargo check //只编译不产生可执行文件，比build更快
cargo build --release //优化编译项目，在./target/release/目录下
cargo update //升级crate
```
由于cargo在创建项目时自动添加了.git，如果要将项目作为其它项目的一部分时可以考虑删除掉该项目中的.git文件


## 答题方式
我采用了在线环境提交练习，在生成Github Classroom的相关仓库后，直接使用codespace来完成练习并提交(直接根据rustlings终端的提示修改对应的文件，最后push即可)
也可在windows下的VS Code中进行练习，但注意克隆到本地后需要安装rustlings,使用``cargo install --force --path . ``安装rustlings,安装完成后再执行``rustings watch``依次查看练习情况


## 猜数游戏小结
* 初步了解rust的输入与输出，以下是一个示例：
``` rust
use std::io
fn main(){
  let mut guess = String::new(); //mut表示可变

  io::stdin() //标准输入
    .read_line(&mut guess) //返回一个Result类型(是一种枚举类型)，用来编码错误处理的信息，成员为OK和Err
    .expect("Failed to read line"); 
    //若Result实例值为Err，expect会导致程序崩溃，并显示当做参数传递expect的信息
    //不调用expect能编译但会有警告

  println!("You guessed: {guess}"); //标准输出，!表示宏
}  
```
* 初步了解crate及其使用，在Cargo.toml文件的``[dependencies]``下添加crate，如``rand = "0.8.5"``,注意表示的实际是至少是0.8.5但小于0.9.0的版本。\(这样的版本指定确保了我们可以获取
能使本章代码编译的最新的补丁\(patch\)版本。任何大于等于 0.9.0  的版本不能保证和接下
来的示例采用了相同的 API\) 需要升级crate时可以使用指令``cargo update``,但注意仍然是前面提到的版本范围，若需要升级到0.9.0及以后，则需要修改Cargo.toml文件
* 表示范围，1..=100表示1到100\(含100\),去掉等号则不含100
* 更多细节见实践文件注释

## 常见编程概念
### 变量与可变性
``` rust
fn main(){
  let x = 5;
  let mut y = 15;  // mut声明为可变
  // x = 7; //编译报错，不可变
  y = 10;

  let x = x + 1;//隐藏了前面的x,可以重复使用let来多次隐藏
  {
      let x = x * 2;
      println!("The value of x in the inner scope is: {x}"); //输出12
  }

  println!("The value of x is: {x}");//输出6

  let x = " "//正确，隐藏可以改变类型
  // let y = " " //错误，mut变量不可改变类型

  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //常量，不可变,不能使用mut
}
```

### 数据类型
rust是**静态语言**，编译时就必须直到所有变量的类型

#### 标量类型
整形、浮点型、布尔型和字符类型。注意rust中字符类型为**四个字节**，代表一个**Unicode**标量值，可以表示很多内容，甚至表情。

| Length  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

浮点型是f32和f64，默认为f64,都是有符号的

布尔型示例：``let f:bool = fasle`` 以及``let t = true``

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

#### 复合类型
##### 元组(tuple)
定长，数据类型可不同
```rust
fn main() {
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1); //元组

    let one = tup.2;//访问元组，第一个索引值为0

    let (x, y, z) = tup;  //使用模式匹配来解构

    println!("The value of y is: {y}");
}
```
不带任何值的元组叫 **单元\(unit\)** 元组，写作(),如果表达式不返回任何其他值，则会隐式返回单元值

##### 数组(array)
定长，数据类型必须相同，存放在栈上，索引无效/越界时将导致运行时错误。
```rust
fn main() {
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [1, 2, 3, 4, 5];
    let a = [3; 5]; //包含5个元素，初值为3

    let first = a[0]; //访问数组
}
```

### 函数
```rust
fn main() {
    println!("Hello, world!");

    another_function();
    print_labeled_measurement(5, 'h');
}

fn another_function() { //无参
    println!("Another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {  //有参，类型必须声明
    println!("The measurement is: {value}{unit_label}");
}

```
函数不关心位置，只要在同一个模块中即可。

函数调用是一个表达式。宏调用是一个表达式。用大括号创建的一个新的块作用域也是一个表达式。表达式才有返回值
```rust
fn main() {
  // let x = (let y = 6); //错误，let y=6语句不返回值， rust中不能写x=y=6

  let y = {
    let x = 3;
        x + 1 //作为表达式，最后一个不带分号，其值即为表达式的值
 };
}

 fn five() -> i32 {
    5 //不带分号，为返回值，若此处加分号，则表示返回值为 (),与i32不匹配，会报错
 }
```

### 注释
同C/C++

### 控制流
```rust
fn main() {
    let number = 3;

    // if 分支
    if number < 5 { //注意条件必须是bool值，与C/C++不同，换成number会报错
        println!("condition was true");
    } else if num == 5{
        println!("num == 5");
    } else {
        println!("condition was false");
    }


    //在let中使用if,没有分号，表示返回值，但是返回值的类型必须相同
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; //类型不同，会报错


    /* 三种循环loop,while,for */

    //loop
    loop { //一直不停的执行
        println!("again!");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //从break返回值，注意此时有分号
        }
    };


    let mut count = 0;
    'counting_up: loop { //'counting_up指定循环标签，只有左边有一个单引号
        println!("count = {count}");
        let mut remaining = 10;

        loop {    //可以嵌套
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; //退出该层循环
            }
            if count == 2 {
                break 'counting_up;//根据标签退出指定循环，注意这里只有左边有一个单引号
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    //while 略

    //for
    let a = [10, 20, 30, 40, 50];
    for element in a {    //用in来遍历
      println!("the value is: {element}");
    }

    // for number in (1..4).rev() //.rev()用于反转range
}
```

## 认识所有权

### 什么是所有权
所有权规则：
1. Rust 中的每一个值都有一个**所有者**（owner）。
2. 值在任一时刻有且只有一个所有者。
3. 当所有者（变量）离开作用域，这个值将被丢弃。

部分语言有**垃圾回收**(GC)机制，但rust采用Rust 采取<u>内存在拥有它的变量离开作用域后就被自动释放</u>

变量与数据交互的方式（一）**移动**:
```rust
fn main()
{
    let s1 = String::from("hello");
    let s2 = s1; //s1被移动到了s2, 此后s1不再能被使用
}
```
隐含了一个设计选择：Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何**自动**的复制都可以被认为是对运行时性能影响较小的

变量与数据交互的方式（二）**克隆**:
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); //确实需要深拷贝时，采用clone,此后s1仍然可用

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x; //只在栈上的数据，拷贝，此后x仍然可用
}
```
如果一个类型实现了 Copy  trait(用在类似整型这样的存储在栈上的类型)，那么一个旧的变量在将其赋值给其他变量后仍然可用

函数在传递参数和返回值时可以转移所有权，可以使用元组来返回多个值
``` rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过drop被清理掉，除非数据被移动为另一个变量所有。

不用所有权就使用值时可以采取**引用**

### 引用与借用
& 符号就是引用，它们允许你使用值但不获取其所有权。
创建一个引用的行为称为 借用（borrowing）
```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; //这里会报错
    /*如果你有一个对该变量的可变引用，你就不能
      再同时创建对该变量的引用，可避免数据竞争*/
    
    //注意也不能在拥有不可变引用的同时拥有可变引用
    /**
     * let r1 = &s; // 没问题
     * let r2 = &s; // 没问题
     * let r3 = &mut s; // 大问题
     */
    

    {
        let r2 = &mut s; //可行
    } // r2 在这里离开了作用域，所以我们完全可以创建一个新的引用

    change(&mut s);
}

fn calculate_length(s: &String) -> usize {//不可变
    s.len()
}

 fn change(some_string: &mut String) {  //可变引用
    some_string.push_str(", world");
 }
```

悬垂引用不被允许，下面是一个示例
```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle 返回一个字符串的引用
let s = String::from("hello"); // s 是一个新字符串
&s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
// 危险！
```

### Slice类型
slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一种引用，所以它没有所有权。使用slice引用可以让编译器更早的发现索引是否有问题。
```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5]; //类型为&str
    let world = &s[6..11];
    let all = &s[..];

    let s = "Hello, world!";//字符串字面值就是 slice

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // 数组的slice，工作方式同字符串，此处类型为&[&i32]
}
```

## 结构体
```rust
struct User { //定义
 active: bool,  //名称+类型，称为字段
 username: String,
 email: String,
 sign_in_count: u64,  // 最后一个有逗号
}

 struct Point(i32, i32, i32); //元组结构体
  struct AlwaysEqual; //类单元结构体

fn main() {
    let user1 = User {  //构造
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    //user1.email = String::from("anotheremail@example.com");//错误，user1不可变，需要在let后加上mut才正确

    let user2 = User {
        active: user1.active, //使用结构体更新语法从其他实例创建实例
        username: user1.username, //注意此处为移动，总体上user1不能再被使用了
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,

        // 只列出不同的值，其余的用..user1来创建
        // email: String::from("another@example.com"),
        // ..user1
    };

    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, 
        email: email,
        sign_in_count: 1,
    }

    //使用字段初始化简写语法(参数名与字段名相同时)
    // User {
    //     active: true,
    //     username,
    //     email,
    //     sign_in_count: 1,
    // }
}
```

结构体的打印
```rust
#[derive(Debug)] //显式选择外部属性功能，此处为打印调试信息的功能
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),//此处返回表达式的值
        height: 50,
    };

    println!("rect1 is {:?}", rect1); //:?表示debug的输出格式，:#?是更好的debug输出格式。不接受所有权，打印到stdout中
    dbg!(&rect1); //另一种方式 dbg!宏，会接受一个表达式的所有权，打印后再返回所有权，会打印到stderr中
}

```

方法语法
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {    //impl来实现方法（带self)
    fn area(&self) -> u32 { //关联函数
        self.width * self.height
    }

    fn square(size: u32) -> Self {  //不是方法的关联函数，通常用作返回一个结构体新实例的构造函数
        Self {  //关键字Self指代impl后出现的类型，注意这里是首字母大写
            width: size,
            height: size,
        }
    }
}

impl Rectangle{ //可以有多个
    fn can_hold(&self, other: &Rectangle) -> bool {
      self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```


## 枚举和模式匹配
### 枚举的定义
```rust
//定义1
enum IpAddrKind {
    V4,
    V6,
}
//使用1
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
struct IpAddr {
    kind: IpAddrKind,//可以作为结构体的类型
    address: String,  //注意结构体和枚举类型指定类型的方式的差异
}
let home = IpAddr { //构造含枚举类型的结构体
    kind: IpAddrKind::V4, //指定(构造）枚举的成员
    address: String::from("127.0.0.1"),
};

//定义2
enum IpAddr {
    V4(String), //用括号来指明类型
    V6(String),
}
//使用2
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1")); //用括号进行构建

//定义3
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
//使用3
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

//定义4
struct Ipv4Addr {
    // --snip--
}
struct Ipv6Addr {
    // --snip--
}
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

//定义5
enum Message {
    Quit, //没有关联任何数据
    Move { x: i32, y: i32 }, //类似结构体包含命名字段
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
枚举的方法的定义同结构体

#### Option枚举和其相对于空值的优势
<u>Rust 并没有空值</u>，不过它确实拥有一个可以编码存在或不存在概念的枚举。这个枚举是Option\<T\> ，而且它定义于标准库中，无需显式引入作用域，其成员也是。
```rust
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let some_number = Some(5);
    let some_char = Some('e'); //可以自动推断类型

    let absent_number: Option<i32> = None; //此时需要指定类型
}
```
Option\<T\>和T（这里T可以是任何类型）是不同的类型，编译器不允许像
一个肯定有效的值那样使用 Option\<T\>。

在对Option\<T\>进行运算之前必须将其转换为 T 。通常这能帮助我们捕获到空值最常见的问题之一：假设某值不为空但实际上为空的情况。

只要一个值不是 Option\<T\>类型，你就**可以**安全的认定它的值不为空。这是 Rust 的一个经过深思熟虑的设计决策，来限制空值的泛滥以增加 Rust 代码的安全性。


### match 控制流结构

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            //要使用枚举变量绑定的值时
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

//匹配Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),//此处可以完成绑定的值的递增
        }
    }

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
```

match的分支必须覆盖所有的可能性，可以使用通配模式和_占位符来对剩下的可能性进行覆盖
```rust
other => /*表达式*/,   //通配模式
other => fun(other),  //可以再使用匹配到的值
_ => /*表达式*/,      //占位符，不需要再使用匹配到的值
_=> (),              //用空元组表示无事发生
```

### if let 简洁控制流
```rust
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```
if let是match的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。

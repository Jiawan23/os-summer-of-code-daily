# rust 学习2
## 使用包、Crate 和模块管理不断增长的项目

### 包(package)和Crate

carte是编译时最小代码单位，包含二进制项(可编译为可执行程序，必须含main)和库(无main,能提供函数等，不可编译为可执行程序)两种形式。

包是一系列提供功能的一个或多个crate,一个包含一个Cargo.toml文件(阐述如何构建这些crate)。包中包含至多一个库crate(src/lib.rs),任意个二进制crate(可放在src/bin目录下，每个文件都会编译成一个独立的二进制crate),但是至少包含一个crate。

## 定义模块来控制作用域与私有性

### Modules Cheat Sheet
* **编译从crate根节点开始** (通常库crate为src/lib.rs,二进制crate为main.rs)
* **声明模块** 在根文件中声明，例如`mod garden`,编译器会在如下路径寻找模块代码：
  * 内联(若紧跟的是大括号而不是分号)
  * 在文件src/garden.rs
  * 在文件src/garden/mod.rs
* **声明子模块** 在除根节点外其它文件中定义子模块。例如在src/garden.rs中定义了mod vegetables，编译器会在如下目录中寻找子模块代码：
  * 内联(后面紧跟大括号时)
  * 在文件src/garden/vegetables.rs
  * 在文件src/garden/vegetables/mod.rs
* **模块中的代码路径**一旦一个模块是crate的一部分，可以在隐私规则允许前提下，在同一crate的任意地方通过代码路劲引用该模块的代码。
* **公有vs私有**，一个模块代码默认对其父模块私有。可以用`pub mod`声明为公有
* **use关键字**,创建一个成员的快捷方式，减少路径的重复。

### 引用模块项目的路径
```rust
mod front_of_house { //兄弟模块可以相互访问，不用pub,且相对路径有效
    pub mod hosting { //pub 暴露路径声明为公有，但是其子模块还是默认私有
        pub fn add_to_waitlist() {} //让子模块也公有
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径 (从crate根开始)
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); //super开始的相对路径，表示从父模块开始构建相对路径
    }

    fn cook_order() {}

    pub struct Breakfast { //虽然声明为公有结构体，但是其成员还是默认私有
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer { //枚举设为公有后所有成员都公有
        Soup,
        Salad,
    }
}
```

### 使用use关键字将路径引入作用域
```rust
use rand::Rng//使用外部包，需要在Cargo.toml中添加rand = "0.8.5"

use std::{cmp::Ordering, io};//使用嵌套路径消除大量的use行

use std::collections::*;//使用glob运算符*将所有公有定义引入作用域，常用于测试和prelude模式

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use crate::front_of_house::hosting; 放在模块外 customer内是不会使用的
pub use crate::front_of_house::hosting;//使用pub use重导出名称，此时外部代码也可以使用

mod customer {
    // use crate::front_of_house::hosting; //应在模块内引入
    // use super::front_of_house::hosting; //或者
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

use std::io::Result as IoResult; //使用as提供新的名称
```
注意将函数引入作用域时应该引入其父模块而不是直接引入函数，而引入结构体、枚举和其它项时习惯指定完整路径（但相同名称的项应该是引入其父模块）

注意 std  标准库对于你的包来说也是外部 crate。因为标准库随 Rust 语言一同分发，无需修
改 Cargo.toml 来引入 std ，不过需要通过 use  将标准库中定义的项引入项目包的作用域中来
引用它们，比如我们使用的 HashMap

模块的拆分可参考模块代码的寻找路径，注意第三种寻找路径是老风格，仍然支持，但是不能混用第二种和第三种风格。

## Common Collections
### Vector
只能存储同类型的值，但可以使用枚举类型以存放不同类型的数据
```rust
fn main()
{
  //创建vector
  let v:Vec<i32> = Vec::new();//不可变，要指定类型
  let v = vec![1,2,3]; //不可变，自动推出类型

  //更新vector
  let mut v = Vec::new();
  v.push(5);//mut，能根据push的值确定类型

  //读取vector
  let v=vec![1,2,3,4,5];
  let third:&i32 = &v[2]; // 索引越界会panic
  let third:Option<&i32> = v.get(2);//索引越界返回None

  let mut v = vec![1, 2, 3, 4, 5];
  let first = &v[0];
  v.push(6);
  println!("The first element is: {first}"); // 错误
  //不能在相同作用域内同时存在可变和不可变引用，去掉push或println任意一个后才能编译


  //遍历vector
  let v = vec![100, 32, 57];
  for i in &v {//不转移所有权
      println!("{i}");
  }
  let mut v = vec![100, 32, 57];
  for i in &mut v {
      *i += 50; //用*解引用后修改值
      //注意在for内不能插入或删除项
  }
} //<-注意丢弃vector时也会丢弃其所有元素
```

### String
Rust 的核心语言中只有一种字符串类型：字符串 slice `str` ，它通常以被借用的形式(`&str`)出现。

String类型 是可增长、可变、可拥有、**UTF-8**编码的字符串类型

```rust
fn main()
{
    //新建字符串
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let hello = String::from("Здравствуйте");//可以包含任何正确编码的数据

    //更新字符串
    let mut s = String::from("foo");
    s.push_str("bar");//向后面添加，注意不会获得参数的所有权
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{s1}+{s2}") //不会获取所有权
    let s3 = s1 +  "+" + &s2; // 注意 s1 被移动了，不能继续使用
    let s4 = "cars 111".replace("cars","balloons");//进行部分替换

    //索引字符串
    // Rust 的字符串不支持索引!!!,可查看教材说明不支持的原因

    //字符串slice
    let hello = "Здравствуйте";
    let s = &hello[0..4]; //将会是Зд,因为这些字母都是两个字节长的
    //let s = &hello[0..1];//会导致panic，同vector无效索引

    //遍历字符串，需要明确表示是字符还是字节
    for c in "Зд".chars() {//遍历字符
      println!("{c}");
    }
    for b in "Зд".bytes() {//遍历字节
      println!("{b}");
    }


}
```

```rust
//区分string_slice和string
fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string_slice(String::from("hi").as_str());
    string("rust is fun!".to_owned()); //创建一个新的可变字符串实例
    string("nice weather".into());//同to_owened()
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
```

### Hash map
HashMap\<K, V\>  类型储存了一个键类型K对应一个值类型V的映射。

HashMap默认使用一种叫做 SipHash 的哈希函数，它可以抵御涉及哈希表(hash table) 的拒绝服务(Denial of Service, DoS)攻击。然而这并不是可用的最快的算法，不过为了更高的安全性值得付出一些性能的代价。如果性能监测显示此哈希函数非常慢，以致于你无法接受，你可以指定一个不同的 hasher 来切换为其它函数。
```rust
fn main()
{
   use std::collections::HashMap;
  //新建hash map
  let mut scores = HashMap::new();

  //修改hash map
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);//注意insert会导致String等类型所有权的转移
  scores.insert(String::from("Blue"), 25);//会导致之前的值被覆盖
  scores.entry(String::from("Red")).or_insert(50);//只在没有键值对时插入键值对

  //访问hash map
  let team_name = String::from("Blue");
  let score = scores.get(&team_name).copied().unwrap_or(0);
  /*get返回的是个result对象，copied()用于不转移
  所有权,unwarp_or用于获取值，若为OK正常得值，
  若为Err则值为参数值0*/

  //遍历hasp map
  for (key, value) in &scores {
      println!("{key}: {value}");
  }

  //根据旧值更新一个值
  let text = "hello world wonderful world";
  let mut map = HashMap::new();
  for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
  }
}
```

## 错误处理
Rust没有异常。相反，它有`Result<T, E>`类型，用于处理可恢复的错误，还有`panic!`宏，在程序遇到不可恢复的错误时停止执行。

### 用panic!处理不可恢复的错误
造成panic的方法：
* 执行会造成panic的操作（比如访问数组结尾的内容）
* 显示调用`panic!`宏通常会打印一个错误信息，展开并清理栈数据。eg:`panic!("crash and burn");`

示例、代码原型和测试都非常适合panic。

```toml
# 想要在release中panic时终止
[profile.release]
panic = 'abort'
```
```bash
# 获取backtrace,注意会修改环境变量(永久)，不需要的时候应该修改回0
> $env:RUST_BACKTRACE=1; cargo run # windows系统
$ RUST_BACKTRACE=1 cargo run #Linux或macOS
```
### 使用Result处理可恢复的错误
```rust
//定义
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");//会返回一个Result变量

    let greeting_file = match greeting_file_result {//用match进行处理
        Ok(file) => file,//取出Ok中的值
        Err(error) => match error.kind() {//处理不同的错误
            ErrorKind::NotFound => match File::create("hello.txt") {//没找到就创建文件并判断是否创建成功
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => { //其它类型的错误
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    //更简洁的做法
    let greeting_file = File::open("hello.txt").unwrap();//失败时会panic并打印错误信息，成功时返回Ok中的值
    let greeting_file = File::open("hello.txt")
      .expect("hello.txt should be included in this project")//失败时输出expect的参数
}
```

```rust
// 传播错误
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {//返回值为Result用于传播错误
    let mut username_file = File::open("hello.txt")?; 
    /*使用?来传播错误，值为Ok继续执行，得到Ok中的值，值为Err该值被返回，
    且收到的错误类型会被转换为当前函数返回类型指定的错误类型*/
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)//最后要有一个正确返回的Ok值
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;//使用链式方法调用进一步缩短代码
    Ok(username)
 }
```
```rust
// 更简短的写法
use std::fs;
use std::io;
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
    //会打开文件、新建一个 String 、读取文件的内容，并将内容放入 String ，接着返回它
}
```
Ps: map_err可用于传播自定义的错误类型，示例可参考练习errors6.rs


Ps: ?运算符只能作用于返回值与？作用的值相兼容的函数，在main中使用的示例如下
```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {//接受所有类型的错误
    let greeting_file = File::open("hello.txt")?;

    Ok(()) //Ok中为()
}
```

## 泛型、trait和生命周期
### 泛型数据类型
```rust
fn largest<T:std::cmp::PartialOrd>(list: &[T]) -> &T {
// <T:泛型要实现的类型(限制条件,可选)>
    let mut largest = &list[0];

    for item in list {
        if item > largest { 
          //涉及比较，泛型必须能够进行比较，可以通过添加std::cmp::PartialOrd来约束
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

//结构体重泛型的定义，此时x,y可以不同类型，若全为T,则应该同类型
struct Point<T, U> { 
 x: T,
 y: U,
}
//let integer_and_float = Point { x: 5, y: 4.0 };

//枚举定义中的泛型定义,例如Result类型

//方法中的泛型定义
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> { //必须在impl后面声明T
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {//泛型指定限制,为某些类型单独提供方法
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


//结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {//泛型参数不同
        Point {
          x: self.x,
          y: other.y,
        }
    }
}
```

Rust 通过在编译时进行泛型代码的**单态化**(monomorphization)来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。使用泛型没有运行时开销

### Trait定义共同行为
```rust
pub trait Summary { //trait的定义
    fn summarize(&self) -> String;

    // 提供默认行为时
    // fn summarize(&self) -> String {
    //   String::from("(Read more...)")
    // }

    // 默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现，此时可以无需用impl显式指出该默认实现
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {} //前面有提供默认行为时。{}表示采用默认行为
impl Summary for NewsArticle { //为某些数据类型实现trait
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


//trait作为参数
pub fn notify(item: &impl Summary) { 
//&impl 指出接受的参数类型必须实现的trait
//多个trait时采用+连接，如&impl Summary + Display
  println!("Breaking news! {}", item.summarize());
}
//使用trait bound(可以强制多个参数为同类型)
pub fn notify<T: Summary + Display>(item1: &T,item2: &T) {
}
//使用where整理trait bound
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

// 返回实现了某些trait的类型(只适用于返回单一类型，不能同时返回多个类型)
fn returns_summarizable() -> impl Summary {}


//使用trait bound有条件的实现方法或trait
impl<T: Display + PartialOrd> Pair<T> {} //有条件实现方法
impl<T: Display> ToString for T {} //有条件实现trait
```
注意trait必须和类型一起引入作用域以便使用额外的trait方法，如`use aggregator::{Summary,Tweet};`。其他依赖`aggregator  crate`的crate也可以将`Summary`引入作用域以便为其自己的类型实现该 trait。需要注意的限制是，只有在trait或类型至少有一个属于当前crate时，我们才能对类型实现该trait。
(**相干性** **孤儿规则**)
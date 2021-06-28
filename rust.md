---
title: Learning Rust
date: 2021-06-16 00:39:24
tags:
    - Rust
    - Programing language
    - Learn-note
categories: Language
toc: true
---

阅读  [rust online book](https://doc.rust-lang.org/book/) 时记录的笔记，辅以备忘。

<!-- more -->

Rust具有安全高效等语言特性，提供了3个工具：

- cargo: 依赖管理和构建工具
- rustfmt: 代码风格
- Rust Language Server

# 一、开始学习rust

## hello, world

``` rust
// main.rs
fn main() {
    println!("hello, world!");
}
```

对于这个起点程序，和C语言类似，main 函数是 rust 程序的入口，函数体使用 '{}' 包围，将左大括号和函数声明放在一行是 rust 推荐的编程习惯,此外，rust 语言风格使用 4 个空格来缩进，而非 TAB，且使用分号作为每个语句表达的结束。可以使用 rustfmt 工具格式化代码为官方推荐的格式

``` shell
> rustfmt main.rs
```

和c语言有所区别的是，用于输出的语句 "println!" 非函数，而是 Rust macro，它和函数的直观上的区别在于是否有 "!"，若 "func_name" 则为一个普通的函数

对于简单的 rust 程序，可以使用 rustc 进行编译，并得到可运行的二进制文件

``` shell
> rustc main.rs
```

然而对于复杂的工程，使用 cargo 来管理项目则是更好的选择。

## 使用 Cargo

Cargo 是 Rust 语言系统中的依赖管理和构建工具。利用cargo创建新的项目，并创建所需的文件项，同时在非 git 仓库中将同时初始化 git 并添加 gitignore 文件。

``` shell
> cargo new hello_cargo
> tree hello_cargo
  |- Cargo.toml
  |- src
    |- main.rs

```

在 Cargo.toml 文件中记录了项目相关信息和依赖项目，文件为TOML (Tom’s Obvious, Minimal Language) 格式。Cargo 设计希望将源码放到 src目录中，顶层目录放置 README、LICENSE 等。

构建和运行使用 Cargo 创建的项目很简单，在第一次构建完成后，在顶层目录将创建一个 Cargo.lock 文件用以记录依赖，无需手工管理。

``` shell
> cd hello_cargo
# write binary file into target/
> cargo build 
# exec the program
> cargo run
# or exec the binary file direcly
> ./target/debug/hello_cargo
```

此外 Cargo 提供了快速检测代码但不生成二进制文件的命令，其速度快于 build，所以经常 check 一下刚刚写的代码是个很好的习惯。 

``` shell
> cargo check
```

当程序发版使用时，使用带有 --release 参数的 build 命令生成 release 版本，编译器优化将使程序具备更高效的运行效率，但是编译时间会更长。

``` shell
> cargo build --release
```

# 二、猜数程序实践

实践永远是学习新东西最快的方法。下面使用熟知的猜数游戏学习一些新的语言规则。

首先，使用在上一章 hello world 程序的基础上，引入了一些新的知识点：

``` rust
// main.rs
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("Your guessd is: {}", guess);
}
```

默认情况下 rust 只引入了少量的类型，为了获取用户的输入输出，需要使用 use 引入 std::io 到作用域中，std 表示 io 是标准库的一部分。

rust 使用 let 关键字创建变量和常量，默认情况下，rust 提供的变量是不可变的，若需要可变的变量需要显式使用 mut 关键字指出。

``` rust
let foo = bar;     // immutable
let mut foo = bar; // mutable
```

let mut guess = String::new() 语句中，guess 变量绑定到 String::new() 的返回结果，String 是标准库提供的可变的、utf-8 格式的字符串类型，"::" 表示 new 是 String 的一个关联函数，其无需实例化即可调用，类似其他语言的静态函数。new() 方法将创建一个新的 String 空实例。

为了和用户交互，使用了 std::io，io::stdin() 将返回 std::io::Stdin，即标准 I/O 的一个句柄，read_line 函数将从终端获取用户输入，并**追加**到 guess 字符串变量后，因此，guess 必须是一个可变对象。"&" 表示使用了对象的引用，使用引用以避免对变量的重复拷贝。默认情况下，引用和变量相同，均为不可变，因此需要使用 "&mut guess" 而非 "&guess"。

'.expect("...")' 对函数返回结果的潜在风险进行处理。read_line 函数读取用户输入，并返回一个 io::Result 类型的数据。Result 类型广泛存在于 rust 的多个模块中，其实质是一个枚举类型，其值包括 Err、Ok，若得到的返回值为 Err，则将导致程序 crash 并使用expect提供的信息，若得到 Ok，则返回其携带的数值。若为使用 expect 函数，在编译过程中，rust 将给出警告。

rust 使用 "{}" 作为程序格式化输出的占位符：

``` rust
let x = 5;
let y = 6;
println!("x={}, y={}", x, y);
```

其次，为了完成猜数游戏，需要学习如何获得随机数。rust 的标准库中并不提供随机数的支持，不过其拥有丰富的 crates 作为语言的扩展支持，修改 Cargo.toml 引入 rand 模块的依赖

```
[dependencies]
rand = "0.8.3"
```

cargo 在执行 build 时将自动构建对应的依赖关系，包括 rand 模块本身的依赖内容。其版本号符合 SemVer 标准，表明项目依赖的 rand 模块需要在 0.8.3 到 0.9.0 之间，高于或等于 0.9.0 则无法保证 api 的一致性。cargo build 将只对程序修改内容进行编译，引入的 crates 只会编译一次。Cargo.lock 指明了依赖项目的版本，从而保证任何时间、任何人都可以成功编译这份项目代码。在 crates 有可升级的版本时，请在项目根目录下执行 cargo update。若需要大版本的更新，请修改 Cargo.toml 文件。

下面使用 rand 生成 1 到 100 的随机数：

``` rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}
```

Rng trait 中定义了很多关于随机数生成方法的接口，为了使用这些方法，首先使用 use 引入。rand::thread_rng 提供了随机数生成器：在当前线程中并使用系统种子运行。利用 gen_range 生成 1 到 100 之间的随机数，范围左闭右开，当然，也可使用 "1..=100" 作为左闭右闭的参数。

当使用一个新的 crate 时，可以在项目目录中使用命令 cargo doc --open 查看当前项目中所有模块的文档。

接下来，需要对用户输入和随机数字进行比较：

``` rust
use std::cmp::Ordering;

fn main() {
    let guess = 2;
    let secret_number = 3;
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!")
            // more
        },
    }
}

```

为了进行结果的比较，需要引入 Ordering，类似于 Result，其亦为枚举类型，不过其内包含 Greater、Less、Equal 三个元素。使用变量的内联方法 cmp 对两个数值结果进行比较，其将返回一个 Ordering 类型的结果，使用 match 对该结果进行分支比较，依次比较 3 种 Ordering 的可能值，当匹配成功则执行 => 后的语句，可以使用 "{}" 执行多条语句。

不过 cmp 函数需要比较相同的类型，如整数和 string 执行 cmp，将无法通过编译。rust 内置了一些基本的类型，比如数字的 i32，u32，i64，u64 等，分别表示有符号和无符号的 32 位和 64 位整形数据，在定义变量时可明确指出：

``` rust
{
    let secret_number = 3;
    let mut guess = String::new();
    let guess: u32 = guess.trim().parse().expect("Please input number!");
    // get input from terminal here
    match guess.cmp(secret_number) {
        // arms here
    }
}
```

在 rust 中，可以重复定义一个变量，这在将一个数据类型转换为其他数据类型的情况下很有用，无需定义两个不同类型的相同变量。trim 函数将去除字符串前后的空白字符，parse 函数则解析字符串并转换为数字，定义变量时通过 ": u32" 指明 guess 为无符号32位整型数据，因此，rust 在执行 cmp 时，即可隐式推断 secret_number 为一个 u32 类型数据。

rust 可以使用 loop 进行循环，并使用 break 和 continue 实现循环的跳转。

``` rust
fn main() {
    loop {
        // loop body
        // break;
        // continue;
    }
}
```

最后，可以为标准 io 提供更健壮的错误处理方式:

``` rust
fn main() {
    loop {
        // ...
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(\_) => continue,
        };
        // ...
    }
}
```

Ok(num) 匹配附带一个参数的 Ok 枚举值，Err(\_) 匹配附带任意参数的错误结果。完整程序如下：

``` rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse()
        //                        .expect("Please input a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessd is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            },
        }
    }
}
```

# 三、语言基础

在这一部分，主要学习 rust 语言的基础知识，如变量、数据类型、函数、注释以及控制流。

学习之前，请首先[了解 rust 保留的关键字](https://doc.rust-lang.org/book/appendix-01-keywords.html)，在后续程序的编写过程中以避免使用这些关键字来定义自己的名称。

## 变量及其可变性

正如第二章提到的，默认情况下，rust 定义的变量都是不可变的，这与其他语言有所区别，也同样因此使 rust 更具安全性和并发性。当然，也可以根据需要，令定义的变量可变。

如之前所提到的，rust 使用 let 来定义一个变量，如果该变量是不可变的，一旦这个变量绑定到某个值后，其值将不能被改变，当尝试编译如下的程序时，将会失败，并给出 " cannot assign twice to immutable variable" 的警告。

``` rust
fn main() {
    let x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);
}
```

rust 保证了声明为不可变的变量一旦绑定了数值后将永远不再改变，对于这种变量，无需考虑其在何时、何处以及怎样发生改变。

如果需要可变的变量，需要使用 mut 关键字显式声明，只需将 mut 放在变量名前即可。此时我们修改上述程序即可正常编译并运行，因为我们操纵的是一个可变的变量。使用变量的可变性是对错误和效率等问题的权衡和折中，不可变行提供了更高的安全性，而可变变量则可能避免了新变量的反复创建和拷贝等。

``` rust
fn main() {
    // let x = 5;
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);
}
```

在其他语言中，有常量（constant）的概念，类似于 rust 的不可变变量，但是 rust 的常量和变量存在一些区别：

- 不允许将 mut 和常量一起使用，因为常量是永远的恒值，而非默认为恒值；
- 当使用 const 而不是 let 来声明一个常量时，必须指明数据类型；
- const 可以声明在任意作用域中，包括全局作用，而 let 无法声明在全局作用域中；
- 常量的值只能是常量表达式，不能是任意一个运行时获取的值。

下面声明了两个常量，rust 建议使用大写作为常量的名称，否则将在编译器给出警告。

``` rust
const MAX: u32 = 123;
fn main() {
    let x = 5;
    const MIN: u32 = 123;
    println!("x is {}, MAX is {}, MIN is {}", x, MAX, MIN);
}
```

除变量不可变的特性外，变量与其他语言仍有一个明显的特点：rust 支持对已定义的变量进行覆盖（常量不具有这样的特性，rust 将其称为 Shadowing），即在已定义的变量后，可以重新定义一个同名的变量来覆盖，如下面给出的程序，最后 x 的值为 7。

``` rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x + 1;
    println!("x is {}", x);
}
```

Shadowing 和 mut 是不同的，若没有 let 关键字，这种 "x = x + 1" 对变量的操作是不允许的，通过Shadowing 可以对已有变量进行一些转换并得到新的不可变变量。

除了上述特性外，Shadowing 也可以实现不同类型的转换，这和 mut 是不同的：

``` rust
let spaces = "  ";        // String
let spaces = space.len(); // integer
```

如果使用 mut 来定义 spaces，则无法通过编译，即我们不能改变 mut 变量名的类型。

## 基本数据类型

rust 是静态类型语言，在编译期必须明确各个变量的数据类型。数据类型可以在代码中明确指定，除此外，也可以通过上下文推断。如猜数游戏中，定义 guess 必须指定其类型为 "u32"，否则，parse 函数通过编译。

``` rust
let guess: u32 = "42".parse().expect("Not a number!");
```

下面介绍 rust 数据类型的两个子集：标量类型和复合类型。

### 标量类型

rust 中的标量类型，即在其他语言中常见的如整型、浮点类型、布尔类型和字符类型。

#### 整型

在 rust 表示整形的方法为 "u/i位长度"，u 表示无符号数，i表示有符号整数，包括：

| 长度 | 有符号 | 无符号 |
| :-----: | :----: | :----: |
| 8-bits | i8 | u8 |
| 16-bits | i16 | u16 |
| 32-bits | i32 | u32 |
| 64-bits | i64 | u64 |
| 128-bits | i128 | u128 |
| arch | isize | usize |

可以使用 "i/usize" 使用操作系统支持的整形长度，在对集合进行索引时常常使用到这种类型。此外，rust 支持多种进制的字面值表示：

| 字面值类型 | 表示 |
| :-----: | :----: |
| 10进制 | 123_456_789 |
| 16进制 | 0xABCD |
| 8进制 | 0o77 |
| 2进制 | 0b1111_0000 |
| 字符(u8) | b'A'


#### 浮点类型

rust 使用 f32 和 f64 分别表示 32 位浮点数和 64 位浮点数，rust 默认使用 64 位浮点数。

``` rust
fn main() {
    let a = 1.0;        // f64
    let x: f32 = 1.2;   // f32
    let y: f64 = 2.2;   // f64
}
```

#### 数值操作

同样，rust 为数值类型提供了加减乘除的操作符，其计算结果绑定到一个变量上：

``` rust
fn main() {
    let sum = 1 + 2;
    let dif = 2 - 1;
    let mul = 1 * 2;
    let div = 2 / 1;
    let m   = 2 % 1;
}
```

#### 布尔类型

rust 使用 true、false 作为布尔值 bool 的字面值：

``` rust
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```

#### 字符类型

rust 的 char 类型为 4 字节的长度的 unicode 支持的常量值，能够表示包括中文、日文等多种字符。使用单引号表示：

``` rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

### 复合类型

rust 的复合类型可以将多个数值集合到一个数据类型中来表示，主要有两种：元组（tuples）和数组（arrays）。

#### 元组

元组是一种将多种不同类型数据集合到一起的常用的方法，其一旦创建，长度固定不可修改，元组使用圆括号表示：

``` rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

使用元组，有方便的方法对其中的每个元素解包：

``` rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

除此之外，可以通过元组索引来访问其中任意元素，使用 "tuple.index" 实现，其索引范围从 0 开始：

``` rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

#### 数组

数组中的每一个元素的数据类型必须相同，其长度是固定的，使用方括号表示：

``` rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

当所需数据集合为相同类型且希望将数据分配在堆上而不是栈上或始终需要固定数量的数据时，使用数据可能是一个选择，不过，其不如 vector （标准库提供，后续介绍）灵活，后者可动态扩容。若无法明确使用数组或 vector，请使用 vector。

如，程序需要固定的一些信息，使用 array：

``` rust
fn main() {
    let months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];
}
```

声明数组时，也可指明元素类型和长度：

``` rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

可以创建一个有相同元素的数组：

``` rust
let a = [3; 5];
```

a 的值为 [3, 3, 3, 3, 3]。

数组的元素值可以使用索引访问：

``` rust
let a = [1, 2, 3, 4, 5];
let first = a[0];
let end   = a[4];
```

在程序中，如果潜在索引越界，程序是可以编译成功的，但当运行时遇到越界问题，将导致程序运行失败。在其他语言中，当遇到越界问题时，程序会继续运行，而 rust 将阻止这种情况的发生，通过立即退出来阻止对非法内存的访问，这里利用所学的语言特性提供了例子

``` rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("input index:");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Faile to read line");

    let index : usize = index.trim().parse().expect("Index not a number"); 

    let element = a[index];

    println!("value is {}, index is {}", element, index);
}
```

## 函数

### 函数定义和传参

rust 使用 fn 关键字来定义函数，如所见的 main 函数，我们同样可以定义其他函数，包括无参数函数、有参数函数等，函数参数必须指明数据类型，当然各个参数可以有各自的类型。

``` rust
fn main() {
    test();
    test_args(x: i32, y: i32);
}

fn test() {
    println!("This is test fn");
}

fn test_args(x: i32, y: i32) {
    println!("The arg is x:{}, y:{}", x, y);
}
```

### statements 和 expressions

函数体由 statements 和 expressions 组成，expressions 是 statement 的一部分。rust 是基于表达式的语言 （ expression-based）。statement 执行一些动作但是不返回值，expression 总是能够推断出结果。如 let 语句为一个 statement：

``` rust
fn main() {
    let x = 6;
}
```

如果试图将 let 语句绑定到一个新的变量，将无法编译，因为 let 语句不能返回值，因此也不能绑定新的变量。因此，在 rust 中与类似于 C 语言的 x=y=1 的行为不同。

``` rust
fn main() {
    let y = (let x = 6); // comile error
}
```

以上的函数定义也是一个 statement。而计算并可得到结果的表达式组成了 rust 程序的绝大部分，诸如 5 + 6、100、调用函数、调用宏等以及使用 "{}" 包裹的多条语句，都是或能够成为表达式，表达式的结尾不包含分号，否则其将转换为 statement，并且将不会返回值。

``` rust
fn main() {
    let x = 1;
    let y = {
        let x = 10;
        x + 1
    }; // 一个表达式，注意 x + 1 后无分号结尾，此 expression 结果为 11
    println!("x out is {}", x);
    println!("y is {}", y);
}
```

### 函数返回值

在 rust 中， 使用 "->" 来指明返回值类型，整个函数体和 "{}" 包裹的表达式是同义的。当然，函数可以使用 return 关键字提前返回结果，大多数函数隐式返回最后一个表达式。

``` rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

如果函数的最后一个语句加上了分号，且指明函数需要返回值或需要使用它的返回值，此时将无法编译，因为现在 expression 因为分号变成了 statement。

## 程序注释

支持 "//" 的行注释和文档注释（后续章节中介绍）。

## 控制流

### 分支

使用 if else 分支，当 if 后的条件为 true，将执行其后 "{}" 包裹的语句，或称 arms。

``` rust
fn main() {
    let x = 7;
    if x > 5 {
        println!("x > 5");
    } else {
        println!("x <= 5");
    }
}
```

需要注意的是，rust 中，if 的条件**必须显式为 bool 类型**，否则不能通过编译，这与 C 的隐式转换不同。

``` rust
fn main() {
    let y = 1;
    if y {
        println!("y is not 0");
    }
} // 不能通过编译，不存在到 bool 的隐式转换。
```

当存在多个条件状态时，使用 else if 语句处理：

``` rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

当代码存在过多的 else if 需要进行重构，后续将介绍 match 来应对这种状况。

### 表达式中的分支

``` rust
fn main() {
    let z = if y { 199 } else { 299 };
    println!("z is {}", z);
}
```

需要注意，各个 arms 的值的类型必须相同，否则无法编译，rust 必须在编译器明确各个变量的类型。

### 循环

使用 loop 执行循环操作，配合 break 和 continue 来实现循环内复杂的跳转。

``` rust
fun main() {
    loop {
        println!("again");
    }
}
```

与 c 语言不同的，rust 的循环也是一个表达式（expression），即其可以返回值：

``` rust
fn main() {
    let mut counter = 1;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {}", result); // result = 20
}
```

### 带有条件的循环

和其他语言类似，rust 提供了带有条件的 while 循环，其行为和其他语言类似：

``` rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");
}
```

### 范围for

当遍历一个集合时，for 循环是一个方便的选择：

``` rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

如使用 for 逆序打印得 3、2、1：

``` rust
for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
```


# 四、所有权（ownership）

所有权是 rust 语言的重要概念，其使 rust 在没有垃圾回收的概念下仍然保证了内存安全。

## 概念

继续学习 rust 之前，需要了解所有权的概念。对于包含 GC 的语言，使用者无需担忧内存的使用和释放，对于类似于 C 的语言，使用者则必须明确在动态分配的内存无用时显式释放。而 rust 则使用了另一种方法：其使用一系列的规则在编译期就明确了内存的所有权，所有权的特性不会在运行时拖慢程序的效率。

> **堆和栈**
>
> 存储在栈上的数据必须在编译器明确了使用内存的尺寸，对于运行期才能确定内存的变量，则分配在堆上。对于堆和栈中的变量的使用，前者的效率明显低于后者，因为前者伴随了内存分配器分配内存等一系列的复杂操作。对于追踪变量到底分配在堆或栈、减少堆中重复数据、及时释放不再使用的内存等，都属于所有权问题。

### 所有权规则

rust 所有权的基本规则如下：

- 每一个值都有一个变量作为它的拥有者（owner）;
- 每一个值只能有一个 owner；
- 当 owner 变量离开其作用域（scope），该值将被丢弃（drop）；

### 变量作用域

此部分并非 rust 独有的概念，其与 C 语言作用域的概念基本相同，变量在创建后生效，离开其所在的作用域失效：

``` rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

### String 类型

为了进一步说明作用域的概念，此处引入了更复杂的数据类型。前面提到的整型等数据类型，均分配在栈中，String 类型则是分配在堆上的一个例子。使用 String 的 from 函数创建一个初始化的字符串，并使用 push_str 来追加。当使用调用 from 函数时，rust 将在堆上分配内存并将一个字母串字面值赋予该变量，当字符串变量离开其作用域，一个类似于 free 的动作则必须且只能被自动执行一次以保证内存安全，这个函数在 rust 中是 **drop** 函数。

``` rust
{
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}
{
    let s = String::from("hello"); // s is valid from this point forward
    // do stuff with s
}                                  // this scope is now over, and s is no
                                    // longer valid
```

### 数据在变量间的移动和拷贝

相同的数据可以在不同的变量间进行交互，对于基本的数据类型

``` rust
let x = 1;
let y = x;
```

x 和 y 的值将同时为 1，因为 1 是一个固定长度的编译期已知的分配在栈上的简单数据。而对于更复杂的数据，其行为可能完全不同：

``` rust
let s1 = String::from("hello");
let s2 = s1;
```

对于字符串而言，其 owner 由三个部分组成：指向堆数据的指针、实际数据长度以及堆预分配内存的长度。当将 s1 赋值给 s2，仅仅操作以上三部分数据，而真实指向的数据却并不会拷贝。

然而，在 rust 中，以上并不是一个简单的浅拷贝，若符合浅拷贝的行为，则 s1 和 s2 两个 owner 将共享一份相同的数据，所以当 s1 和 s2 同时离开所属的作用域后，必然导致了堆相同数据的重复释放。

故，在 s1 赋值给 s2 后，s1 将失效，这也导致当 s1 离开其作用域时，将不会发生任何事情，这是一个**移动**操作，而非拷贝，字符串 "hello" 所占用的内存释放的任务将交由 s2 完成。因此，以下的行为将导致编译错误，因为 s1 已经是一个非法的变量：

``` rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}, world!", s1);
```

> rust 永远不会主动进行数据的深拷贝。

若需要深拷贝的操作，请调用 clone 函数，此时，s1 和 s2 持有的是不同内存上的同值数据，clone 拷贝了堆数据。

``` rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

然而，以上的概念对于只存在于栈上的数据而言，看起来是无效的。正如：

``` rust
let x = 1;
let y = x;
```

此时 x 和 y 同时拥有数值 5，没有调用 clone，也没有移动行为的发生（x 并未失效）。

原因在于，数据 1 是一个尺寸大小已知分配在栈上的整型数据，浅拷贝或深拷贝对于这种数据来说并没有什么不同。因此，对于这种简单数据类型，则忽略移动和克隆语义。

rust 为这种类型提供了 Copy trait，通过调用 Copy 可以将该数据存放在栈上。若一个数据类型实现了 Copy，则其不能实现 Drop，反之也相同。Copy 保证了赋值给新的数据后旧的数据仍然可用。

包括整型、布尔、浮点类型、字符类型、全部元素均含有 Copy 的 元组类型等，都实现了 Copy。

### 所有权和函数

rust 函数的参数和返回值，在使用上和其他语言有很大区别。传值给函数类似于给变量赋值。因此，对于实现了 Copy 的数据类型的数据，传给函数并离开函数作用域后，该数据仍然可用，对于实现了 Drop 的数据，当传递给函数后，相当于执行了移动语义，原始变量无效，此数据的生命周期将交由函数管理。

这个例子可以清晰地说明这个问题：

``` rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

同样，对于函数的返回值，在返回后将移动给调用者，并由其调用者管理生命周期。

``` rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

如果调用函数后仍然希望使用原有的参数呢？可以考虑将参数返回后在继续使用：

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

不过，rust 提供的引用将更好地解决这个问题。

## 引用和借用 （Reference and Borrowing）

如上一小节所述，在函数调用发生后仍然需要使用原有参数变量是常见的需求，除了函数再次返回该参数作为解决方案外，还可以使用引用，使用引用将能够关联一些数据并无需接管其生命周期：

``` rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

> 与引用相反的操作是：解引用 \*，此处不做介绍。

我们传递 "&s1" 作为函数的参数，且，函数参数 "s: &String" 表明其接受一个 String 类型的引用。函数参数在函数内有效，当离开函数作用域后，s 不会释放 s1 所持有的数据。

我们把使用引用作为函数参数成为借用。注意，正如变量的不可变，引用在默认情况下同样不可改变其引用的数据，如下的例子试图修改引用的数据，将无法通过编译：

``` rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

那么如何修改被引用的数据？需引用一个 mut 变量，并在函数签名中使用 "&mut"：

``` rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

但是，rust 要求一个变量在一个作用域中只能接受一个可变引用，否则将编译失败：

``` rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

这种限制防止了数据竞争，尤其在以下几种场景中：

- 多个指针同时指向相同的数据；
- 至少一个指针正在写数据；
- 没有数据同步机制；

rust 通过这种机制避免了数据的竞争，它甚至在有潜在数据竞争发生的可能下禁止编译这份代码。

当然，可以在不同作用域中使用多个可变引用：

``` rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

此外，当存在一个可变引用时，无法存在不可变引用，因为需要在不可变引用存续期间保证变量的不可变性，不过多个不可变引用可以同时存在：

``` rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

只有在不可变引用最后一次使用后，才能定义新的可变引用，因为此时无需保证数据的不变性：

``` rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// r1 and r2 are no longer used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

还有一个问题是，可能存在空悬引用，在使用指针的语言系统中，这是一个常见的问题，不过 rust 的编译器保证了空悬引用不会存在，当存在这种情况将不能通过编译：

``` rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

函数试图返回一个已经离开声明周期的变量的引用是危险的操作，不过 rust 已经在编译器帮我们避免了这些可能。如果返回的是变量，则会通过移动将生命周期移交，不会存在这种问题。

> 引用
>
> 任何时候，只能存在一个可变引用或多个不可变引用，且引用存在期间必须合法。

## 切片类型（The Slice Type）

### 不使用切片可能会产生的问题

除了引用没有所有权外，另一个没有所有权的类型是切片。通过切片可以借用字符串、数组等数据的一部分或全部，从而避免使用索引后原有数据发生改变导致索引无效的问题。

这里给出一个简单的例子，创建一个函数来获得一个字符串的第一个单词。注意声明函数的格式，包括参数、返回值类型以及最后一个语句没有冒号（expression 而非 statement）。在不引入切片时，函数可以返回第一个空白字符的位置作为第一个单词结尾的索引。此处使用 String 的 as_bytes 将 String 转为字符数组，使用数据的 iter 函数获取迭代器，使用迭代器的 enumerate 函数将返回数组的索引和对应索引的元素所组成的元组：

``` rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

然而，存在的一个问题是，当调用函数后，原始字符串发生了改变，则返回的索引将失效，这导致了潜在的 bug：

``` rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

当然，可以来判断 s 的 size，但是，这又如何判断此时的 s 是否是原有的 s 呢？


### 字符串切片

使用切片可以解决上述问题，字符串切片在 rust 中使用 "&str" 来表示，注意，其和 String 并不是相同的类型。

``` rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

字符串切片使用 &string_name\[begin..end\] 来表示，左闭右开。若 begin 为字符串开始，可省略，若 end 为字符串结尾，可省略：

``` rust
let hello = &s[..5];  // hello 的类型为 &str，非 String
let world = &s[6..];
let hello_world = &s[..];
```

字符串切片实际上是对字符串一部分的引用，其所属权的规则与引用相同。需要注意，在使用字符串引用时，需保证其字符串为utf-8有效编码。

使用切片，可以重写上一个例子，需要注意返回值的类型：

``` rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

此时，在调用这个函数时后而未使用 word 前，若尝试修改 s 将无法通过编译：

``` rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

注意在引入引用和借用概念时，提到过**当存在不可变引用时，不能创建可变引用**，调用函数后返回了 s 的一个切片，即一个不可变引用（切片默认为不可变引用），当试图修改字符串 s 时，此时需要使用数据的一个可变引用，故失败。当最后一次使用 word后，我们才能修改 s。

使用切片保持了 word 相对于 s 的状态。

### 字符串字面值是一个切片

``` rust
let s = "Hello, world!"; // the type of s is &str
```

"Hello, world!" 是一个字符串字面值，其真实的数据类型为字符串切片，即 &str，因此字符串字面值是不可变的。

### 字符串切片作为函数参数

除了上述作为返回值，切片同样可以作为函数参数传入:
``` rust
fn first_word(s: &str) -> &str {
    // fn body
}
```

使用字符串切片作为参数具有更高的适用性，如果参数类型是 String，则无法传入切片，相反，却能够简单地将字符串作为参数传入函数:


``` rust
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

### 其他类型的切片

不止 String，还有一些数据类型也有切片的概念，如元素类型为 u32 的数组，其切片类型表示为 &\[u32\]，此处仅简单了解即可，后续会有更详细的介绍。


# 五、数据结构

此章节学习 rust 中数据结构的定义和使用。

## 定义并实例化 struct

使用关键字 struct 可以将许多不同类型的数据组织在一起，例如，定义一个简单的结构：

``` rust
struct User {           // the name of the struct is User
    username: String,   // ver_name: ver_type,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

使用该结构时，使用 struct_name { key: value} 的形式来创建其实例。注意，无需在意变量的顺序，

``` rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

可以使用 "." 方法来读取数据，对于可变的实例，可以使用 "." 方法修改数据，此时整个结构的所有变量均可变，rust 不允许结构部分变量可变：

``` rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

当然可以使用函数来实例化一个结构：

``` rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

此外，rust 提供了更方便的特性来避免函数参数在实例化结构体时需要显式指明的问题，若函数参数名和结构体的元素名相同时，可以省略其value，例如：

``` rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

此时，只需要使用 email 替代 email: email 即可。为了创建与已有 struct 仅存在少量区别时，使用 update 语法可以更简单地实现这个需求。例如，已有 user1，此时需要建立一个 user2，其只有 email 和 username 是不同的，则可以在指明新变量的 key: value 后，使用 ..user1 指明 user2 的其他域元素均和 user1 相同，并从 user1 的value 进行实例化对应参数。注意，此时 user2 是一个新的实例。

``` rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

除了上述 struct 的形式外，rust 还支持 tuple struct 的定义。与上述的普通 struct 相比，其内各个 fields 没有变量名:

``` rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

当我们需要为 tuple 类型指明一个名字时，即可以这样定义，如上述例子，black 是 Color 的一个实例，origin 是一个 Point 的一个实例。但是，尽管 Color 和 Point 的定义形式相同，但它们不是相同的类型，故需要 Color 参数的函数不接受 Point 类型的参数。tuple struct 的其他行为类似于普通的 tuple，如 ".index" 来索引元素、解元组操作等。

struct 也支持空的定义，即无任何 fields。这对于某些类型：不包含任何数据，但是其支持某些函数操作，是有用的。

struct 的元素支持引用类型，但是，此时需要使用 rust 生命周期的特性，来保证 struct 中的元素的生命周期长于 struct 结构，如下的使用方法是无法通过编译的。关于如何修复这个问题后续会有介绍。

``` rust
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

## 在程序中使用 struct

此节使用 struct 实现了一个计算长方形面积的程序，除了使用到了所学到的 struct 外，还使用了数据借用等知识点：

``` rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 50,
        height: 30,
    };

    // need define Display to use {} for Rectangle
    // println!("rect {} area is {}", rect, area(&rect));

    // need define Debug or
    println!("rect {:?} area is {}", rect, area(&rect));
     // add #[derive(Debug)] before struct Rectangle
    println!("rect {:#?} area is {}", rect, area(&rect));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```

例子中，试图打印 Rectangle 结构，我们尚未了解 struct 的方法，若使用 "{}" 来做占位符，则必须实现 Display，此外，我们还可以使用 "{:?}" 和 "{:#?}" 来作为占位符打印调试信息，此时必须定义 Debug 或在定义结构体前添加 "#[derive(Debug)]"，二者的区别在于前者只输出简单的字符串，后者更清晰地显示 struct 结构。

## struct 的方法

方法类似于函数，不同的是其声明于 struct 内部，而且其第一个参数总是 self（想到python了没~）来表示 struct 本身，通过使用方法，我们可以将 上述程序重写如下：

``` rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 50,
        height: 30,
    };

    println!("area is {}", rect.area());
}
```

使用 impl 关键字，将 Rectangle 所拥有的方法定义在其后的 "{}" 中，方法的第一个参数是 "&self"，并未明确指明其类型，如 "self: Rectangle"，因为 rust 可以自行推断。使用了引用，表示了该方法仅仅借用了实例的变量，不拥有其生命周期，若需要更改实例的变量值，必须使用 "&mut self" 作为第一个参数，直接使用无引用的 "self" 作为第一个参数是很少见的，不过在将本实例转换为其他实例时可能会用到。

不论方法的第一个参数是 "self", "&self", "&mut self"，在使用方法时都无需关心是否需要对方法所属的实例的引用问题，rust 自动提供了对应内容，如例所示，二者是等价的：

``` rust
p1.distance(&p2);
(&p1).distance(&p2);
```

当然，也可以为方法提供更多的参数：

``` rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}
```

此外，在 impl 的作用域中，我们还可以定义关联函数（Associated Functions），这些函数不需要 self 参数。他们和 struct 关联在一起。关联函数通常用于返回该结构对应的新实例，例如 String 的 from 函数：

``` rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

如上，可以初始化一个正方形。对于关联函数来讲，使用 "::" 来调用：

``` rust
let sq = Rectangle::square(3);
```

每个 struct 可以有多个 impl 区域，因此多个方法可以分别定义在不同的 impl 中。

# 六、枚举和模式匹配

## 定义枚举

### 定义和使用简单的枚举

我们可以通过枚举来定义事件的所有不同的可能性，如 ip 地址的种类，包括 ipv4 和 ipv6，则为了后续的区分，可以定义一个枚举：

``` rust
enum IpAddrKind {
    V4,
    V6,
}
```

此时，我们可以创建该枚举的实例：

``` rust
let four = IpAddrKind::v4;
let six  = IpAddrKind::v6;
```

枚举中的所有的元素都属于该枚举命名空间，此时，four 和 six 同属于 IpAddrKind 类型，我们可以在函数传入参数来使用枚举：

``` rust
fn use_enum(ip_kind: IpAddrKind) {
    // fn body
}
fn main() {
    let ipv4 = IpAddrKind::v4;
    use_enum(ipv4);
}
```

### 将枚举与数据绑定

与 C 语言中常见的枚举类型不同的是，rust 可以将数据和枚举中的元素绑定到一起。例如，当我们需要明确每个 ip 地址是 v4 版本还是 v6版本的，按照往常的思路：

``` rust
enum IpAddrKind { V4, V6}
struct IpAddr {
    addr: String,
    kind: IpAddrKind,
}
```

当然，上述写法是正确的，但是有更方便的用法：

``` rust
enum IpAddr {
    V4(String),
    V6(String),
}
```

此时，枚举的每个具体值可以和一个 String 类型的数据绑定：

``` rust
let home = IpAddr::V4(String::from("my.home.com"));
let remote = IpAddr::V6(String::from("::1"));
```

因此，就不需要额外的 struct。然而，这并不是 enum 特性的终点：rust 中 enum 的不同元素可以绑定不同的数据类型：

``` rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

例如，标准库 IpAddr 的定义方法如下：

``` rust
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
```

> 在之前的代码中，我们定义了自己的 IpAddr，尽管标准库中也提供了相同的名字，但是由于我们并没有将其引入程序的命名空间，因此并不会发生冲突！


### 枚举与方法 

使用枚举可以有更多的想象空间，如：

``` rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

并且可以为其定义附带的方法：

``` rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```

如果我们为使用 struct 实现上述操作，需要为每种动作定义一个类型，那么，处理起来将不那么方便！枚举方法的定义同 struct。

### Option enum

Option 类型是标准库中定义的另一种枚举类型，该枚举使用广泛，其可以代表 something，也可以表示 nothing。实际上，**rust 不像其他语言一样，它没有 null**，null 表示由于某种原因当前没有值或当前无效的值，在有 null 概念的语言中，其值或是 null，或是非 null，而且，当把 null 作为非 null 来使用的时候，往往会造成不可估量的后果。

rust 不提供 null，但是它有一个枚举可以用来编码值的存在与否，即 "Option\<T\>"，其被标准库定义如下：

``` rust
enum Option<T> {
    Some(T),
    None,
}
```

Option\<T\> 已经包含的程序的作用域中，无需引用，同时，使用 Some 和 None 也无需使用 "Option::" 前缀。"\<T\>" 是 rust 中的泛型参数，此时，我们只需了解 "\<T\>" 表示Some 可以保存任意类型：

``` rust
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;
```

当使用 None 时，需要明确指明该数据的类型，以保证 rust 在编译期可以推断数据类型。此时需要注意的是，T 和 Option\<T\> 属于不同的类型，下面的代码无法通过编译：

``` rust
let a: i8 = 5;
let b: Option<i8> = Some(10);

let c = a + b;
```

当我们使用 T 时，编译器可以保证我们使用的永远都是合法的数据，而无需检查其是否为空，只有当我们使用 Option\<T\> 时，我们才需要担心是否有非法数据的使用风险，因此，我们必须对其进行检查后才能继续使用，即我们必须显式将 Option\<T\> 转换为 T 类型，并且明确指明当其为空值时的处理方式。具体内容阅读[文档](https://doc.rust-lang.org/std/option/enum.Option.html)。
## match 操作符

match 是 rust 提供的一种非常强大的控制流操作符，其可以在一系列的模式（Patterns）中进行匹配，并执行匹配成功后的模式所对应的代码。此中的模式可以是字面值、变量名、通配符（wildcards）以及其他多种类型。使用 match，具有强大的匹配能力，此外，其处理了所有可能性来保证程序的安全性。

文档给出了一个生动的例子：可以将 match 操作符工作的过程理解为硬币分拣的流程，不同面值的硬币其直径不同，这些硬币依次通过尺寸从小到大的孔洞，当某个硬币的直径和对应孔的直径匹配的时候，便筛选出这枚硬币，使用 rust 的 match 来实现一个硬币面值英文名与其数值匹配的程序：

``` rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("10");
            10
        },
        Coin::Quarter => 25,
    }
}
```

match 关键字后跟随待匹配的对象，其与 if 不同的是，if 要求其表达式的值必须为 bool 类型，而 match 的匹配值可以为任意类型。由 "{}" 包裹并由 "," 分隔的是 match 的多个 arms，每个 arm 包含符号 "=>" 左侧的待匹配模式和右侧的匹配后执行的代码。match 按照 arms 的顺序依次匹配检查，如果模式不能匹配，则继续执行下一个匹配，匹配后执行的代码是一个表达式，其表达式的值是 match 操作的返回值。如果匹配后需要执行多行代码，则可以使用 "{}" 将其包围。


# Waiting for update later

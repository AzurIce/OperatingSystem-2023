# 实验1 - 独立的可执行程序

> 21301021 肖斌

> 所用设备及系统：Macbook Pro M2 Max, MacOS Sonoma 14.0.0

> GitHub 仓库：https://github.com/AzurIce/OperatingSystem-2023

## 一、实验步骤

### 1. 创建 Rust 项目

本实验的主要目的是构建一个独立的不依赖于 Rust 标准库的可执行程序。

首先进入到项目目录，然后启动包含上一节配置好的环境的容器：

```bash
docker run mystifying_kowalevski
docker attach mystifying_kowalevski
```

进入 `/mnt` 目录，并创建 Rust 项目：

```bash
cd /mnt
cargo new os --bin
```

> ![image-20231020110038656](./exp1-independent-application.assets/image-20231020110038656.png)

运行查看结果：

```bash
cd os
cargo run
```

> ![image-20231020110055038](./exp1-independent-application.assets/image-20231020110055038.png)

### 2. 移除标准库依赖

首先，修改 target 为 riscv64，在 `os/.cargo/` 目录下创建 `config` 文件，并添加如下内容：

```
# os/.cargo/config
[build]
target = "riscv64gc-unknown-none-elf"
```

然后修改 `main.rs`，在开头加入如下内容，并删除 `main` 函数：

```rust
#![no_std]
#![no_main]
```

同时，因为标准库中提供了 panic 的处理函数 `#[panic_handler]` 所以我们还需要实现 panic handler，添加如下内容：

```rust
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

现在如果直接使用 `cargo build` 可能会出现编译错误：

![image-20231020110808299](./exp1-independent-application.assets/image-20231020110808299.png)

需要执行如下命令添加相关软件包：

```bash
rustup target add riscv64gc-unknown-none-elf
cargo install cargo-binutils
rustup component add llvm-tools-preview
rustup component add rust-src
```

再进行构建：

![image-20231020111843488](./exp1-independent-application.assets/image-20231020111843488.png)

然后可以对独立的可执行程序进行分析：

```bash
file target/riscv64gc-unknown-none-elf/debug/os
rust-readobj -h target/riscv64gc-unknown-none-elf/debug/os
rust-objdump -S target/riscv64gc-unknown-none-elf/debug/os
```

![image-20231020111958020](./exp1-independent-application.assets/image-20231020111958020.png)

分析可以发现编译生成的二进制程序是一个空程序，这是因为编译器找不到入口函数，所以没有生成后续的代码。

### 3. 用户态可执行的环境

#### 1> 实现入口函数

首先增加入口函数：

```rust
#[no_mangle]
extern "C" fn _start() {
    loop{};
}
```

Rust 编译器要找的入口函数为 `_start()`。

然后重新编译。

通过如下命令可以执行编译生成的程序：

```bash
qemu-riscv64 target/riscv64gc-unknown-none-elf/debug/os
```

可以发现似乎是在执行一个死循环，即程序无输出，也不结束：

![image-20231020112401623](./exp1-independent-application.assets/image-20231020112401623.png)

#### 2> 实现退出机制

添加如下代码：

```diff
#![no_std]
#![no_main]

use core::panic::PanicInfo;
+use core::arch::asm;
+
+const SYSCALL_EXIT: usize = 93;
+
+fn syscall(id: usize, args: [usize; 3]) -> isize {
+    let mut ret: isize;
+    unsafe {
+        asm!("ecall",
+             in("x10") args[0],
+             in("x11") args[1],
+             in("x12") args[2],
+             in("x17") id,
+             lateout("x10") ret
+        );
+    }
+    ret
+}
+
+pub fn sys_exit(xstate: i32) -> isize {
+   syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
+}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn _start() {
-    loop{};
+    sys_exit(9);
}
```

再编译运行，发现程序可以直接正常退出：

![image-20231020112631823](./exp1-independent-application.assets/image-20231020112631823.png)

#### 3> 实现输出支持

首先封装一下对 Linux 操作系统内核提供的系统调用 `SYSCALL_WRITE`：

```rust
const SYSCALL_WRITE: usize = 64;

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
  syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}
```

然后声明一个 Stdout 结构体并为其实现 Write Trait：

```rust
struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(1, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}
```

然后给予 `print` 函数，实现本存在于 Rust 语言标准库中的的输出宏 `print!` 和 `println!`：

```rust
use core::fmt::{self, Write};

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
```

然后我们可以在入口函数 `_start` 中使用我们实现的 `println!` 宏打印 `Hello, world!`。

完整 diff 如下：

```diff
-- a/os/src/main.rs
+++ b/os/src/main.rs
@@ -4,6 +4,46 @@
 use core::panic::PanicInfo;
 use core::arch::asm;

+// Wrap of SYSCALL_WRITE
+const SYSCALL_WRITE: usize = 64;
+
+pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
+  syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
+}
+
+// Implement Write trait for Stdout
+struct Stdout;
+
+impl Write for Stdout {
+    fn write_str(&mut self, s: &str) -> fmt::Result {
+        sys_write(1, s.as_bytes());
+        Ok(())
+    }
+}
+
+pub fn print(args: fmt::Arguments) {
+    Stdout.write_fmt(args).unwrap();
+}
+
+// Implement print macro
+use core::fmt::{self, Write};
+
+#[macro_export]
+macro_rules! print {
+    ($fmt: literal $(, $($arg: tt)+)?) => {
+        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
+    }
+}
+
+#[macro_export]
+macro_rules! println {
+    ($fmt: literal $(, $($arg: tt)+)?) => {
+        print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
+    }
+}
+
+
+// Wrap of SYSCALL_EXIT
 const SYSCALL_EXIT: usize = 93;

 fn syscall(id: usize, args: [usize; 3]) -> isize {
@@ -31,5 +71,6 @@ fn panic(_info: &PanicInfo) -> ! {

 #[no_mangle]
 extern "C" fn _start() {
+    println!("Hello, world!");
     sys_exit(9);
 }
```

编译运行：

![image-20231020113256882](./exp1-independent-application.assets/image-20231020113256882.png)

## 二、思考问题

### 1. 为什么称最后实现的程序为独立的可执行程序，它和标准的程序有什么区别？

标准的程序中包含了标准库中实现的一系列内容，可以理解为最终的可执行文件中还包含标准库中的代码。

之所以称最后实现的程序为 *独立的可执行程序*，就是因为它并不依赖于标准库，而是完全通过操作系统内核提供的系统调用来实现一系列功能。

比如 *退出程序* 和 *输出宏*。

`print!` 和 `println!` 在标准库中是有实现的，其实在标准库中的底层部分也是通过 *系统调用* 来实现的，只不过可能有针对各种不同平台的详细代码，比如在 Linux 下通过 Linux 的系统调用实现，在 Windows 下通过 Windows 的 API 实现，等等。

### 2. 实现和编译独立可执行程序的目的是什么？

其实这一次实验相当于从底层，从最基本的积木「系统调用」来与系统交互，搭出了「标准库」的冰山一角。

后面的实验可能会要我们手动去实现一个操作系统，而编程语言的标准库中是不包含对我们自己实现的操作系统的具体实现的，所以我们不可能依赖标准库来写我们的代码，只能同样用最基本的积木来与我们自己的操作系统进行交互。

## 三、Git 提交截图

![image-20231020122900341](./exp1-independent-application.assets/image-20231020122900341.png)

## 四、其他说明

做这次实验之余简单看了一下整体的实验内容与脉络，感觉内容很有意思。

我认为我比较幸运，因为我个人在平时是比较喜欢折腾各种技术之类的小东西的，所以对 Linux、Rust、Git、vim 等一系列东西都有一定的理解，也因此能够从实验中获得更多的收获。

> 最初一次接触 Linux 可能是小学四五年级的时候奔着「国产操作系统」摸索着给自己的笔记本装了个 KyLin，不过当时什么也不知道，简单吧完了两下便换回了 Linux。后面从打 OI 使用 NOI-Linux、自己使用 Ubuntu 来熟悉，到后面自己开始使用、折腾 Linux 系统（Manjaro Linux -> Arch Linux -> NixOS），因此能对 Linux 及相关的工具有一定的理解。
>
> 我自己也很乐于探索新兴的编程语言，同时在自己的小项目里做各种尝试，于是从最初因打 OI 了解的的 C/C++ 到 Python、Java、前端的那套东西和各种框架、Golang、Rust，也因此对 Rust 有一定的理解。

但是有很多人并不像我一样幸运：

---

舍友因为将 `.cargo` 目录创建为 `cargo`，而导致 cargo 读取不到正确的 `config` 文件，进而无法编译。

我说：「你 cat 一下 .cargo 里的 config 我看看」

舍友：「啊？」「cat？」

我说：「cat，空格」

---

舍友：艰难地用着 vim

我：「你可以用 VSCode 在本机编辑，要运行编译哪些命令的时候再到 docker 里运行」

舍友：「哦哦哦哦哦哦哦哦哦哦哦哦！！！！！」

---

舍友：「我发现我都做完了，但是我发现 GardenerOS 里啥都没有」

我：「...」

---

那么我想，「标准库是什么」、「extern "C" 是啥」、「Trait 是什么」、「asm!在做什么」、「宏是什么」等等这些问题，或许更不可能让大家理解。对于很多人来说，机械性地复制-粘贴手册中的指令，对着各种小问题焦头烂额，“碰运气”式地得到了和手册中描述相同的结果便已经皆大欢喜。

但是，似乎也没有什么更好的解决办法，很多人接触相关内容的时间本就很短。这个领域的信息量实在是十分庞大，短时间内将它们全部接受、吸收近乎不可能。

突然有些感悟，简单写了下（）

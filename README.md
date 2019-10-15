# Rust学习

### 目录结构

```
learn_rust  //根目录
    |
    |- src  //源码
    |   |
    |   |- exercise //练习程序模块
    |   |- study    //rust学习内容
    |
    |- tests    //测试程序目录
```

### 使用和测试

- 使用

在main.rs中通过 `use learn_rust::exercise::atoi;`可以使用atoi模块中的方法。同理use其他模块。

- 测试

运行 `cargo test xxx` xxx是需要运行的测试模块或方法名称。

例如： `cargo test test_two_sum`

<img src="https://github.com/a74946443/learn_rust/blob/master/image/example.png?raw=true" />

### 文档

内容可通过文档进行查看

运行 `cargo doc --open`

# return "!" 代表什么意思

```rust
pub extern "C" fn _start() -> ! {
    loop {}
}
```

https://stackoverflow.com/questions/46689785/what-is-the-use-of-as-return-type-in-rust

代表这个函数永远都不返回.

# String [M..N]字符串切片

```rust
self.input[self.pos..]
```

# unwrap_or

```rust
let (next_pos, _) = iter.next().unwrap_or((1, ' '));

assert_eq!(Some("car").unwrap_or("bike"), "car");
assert_eq!(None.unwrap_or("bike"), "bike");
```

# 多行字符串

# 如何写单元测试

# 如何写to_string


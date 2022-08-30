
# return "!" 代表什么意思

```rust
pub extern "C" fn _start() -> ! {
    loop {}
}
```

https://stackoverflow.com/questions/46689785/what-is-the-use-of-as-return-type-in-rust

代表这个函数永远都不返回.


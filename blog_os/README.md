
# ubuntu

```shell
sudo apt install build-essential
```


```shell

rustup override set nightly

rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

cargo build

rustup component add llvm-tools-preview

cargo bootimage

```

run:

```shell
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin
```

# windows 下

```shell
rustup component add rust-src
cargo install cargo-xbuild
rustup override set nightly
cargo xbuild 

cargo install bootimage

rustup component add llvm-tools-preview

cargo bootimage

```


 ```shell
 .\qemu-system-x86_64.exe -drive format=raw,file=D:\workspace\rust\hello\blog_os\target\x86_64-blog_os\debug\bootimage-blog_os.bin
 ```
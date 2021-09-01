
# windows 下

```shell

cargo xbuild 

 cargo install bootimage

rustup component add llvm-tools-preview

cargo bootimage

```


 ```shell
 .\qemu-system-x86_64.exe -drive format=raw,file=D:\workspace\rust\hello\blog_os\target\x86_64-blog_os\debug\bootimage-blog_os.bin
 ```
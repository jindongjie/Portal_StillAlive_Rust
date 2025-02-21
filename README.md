在终端上演示《传送门》片尾曲效果的 rust 程序。

## 使用条件
可在任意主流终端下运行，并可在古老串行终端中使用（已测试： DEC VT220)。

## 使用方法
直接运行打包好的可执行文件

```
./portal_still_alive
```

程序会读取 `TERM`，`COLUMNS` 和 `LINES` 环境变量来调整输出区域大小并决定是否启用终端颜色等
特性。如果希望在一台标准 VT220 终端上演示，应该运行：

```
TERM=vt220 ./portal_still_alive
```

可以使用 `--no-sound` 参数不带音乐进行演示

## Linux 运行效果 / Snapshot on Linux
![](still_alive_linux.jpg)

## 演示视频 / demonstration video
![](still_alive_informer213.jpg)
NEED RECORD VIDEO!

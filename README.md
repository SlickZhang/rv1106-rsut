开发板内核版本和架构：Linux luckfox 5.10.110 armv7l GNU/Linux
gcc编译器：arm-rockchip830-linux-uclibcgnueabihf

步骤 1：将附件同 Dockerfile 放置同一目录下，运行以下指令

```
docker build -t rv1106:V1.0 .
```

生成镜像并启动。

步骤 2：
在用户文件下克隆该仓库后，运行下面指令

```
cargo +rv1106 build -p rv1106-hello --target=armv7-unknown-linux-uclibceabihf --release
```

编译完成后即可在 `target/armv7-.../release` 目录下得到可执行文件。

# 需要的开发环境

- rust rustc 1.76.0 (07dca489a 2024-02-04)
- QEMU emulator version 8.2.0 (v8.2.0-12045-g3d58f9b5c5)

# 命令汇总

启动 qemu

```powershell
qemu-system-riscv64.exe `
    -machine virt `
    -nographic `
    -bios ./bootloader/rustsbi-qemu.bin `
    -device loader,file=./target/riscv64gc-unknown-none-elf/release/sonari.bin,addr=0x80200000
```

启动 gdb 调试

```powershell
riscv64-unknown-elf-gdb.exe `
    -ex 'file ./target/riscv64gc-unknown-none-elf/release/hello-rust' `
    -ex 'set arch riscv:rv64' `
    -ex 'target remote localhost:1234'
```

剪裁二进制文件

```shell
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/sonari -O binary target/riscv64gc-unknown-none-elf/release/sonari.bin
```

# 进度总结

- [x] 配置好内核交叉开发环境
- [x] 实现 hello word 打印
- [x] 初次调用 sbi 接口，关机、打印输出
- [ ] 任务处理
- [ ] 内存管理
- [ ] 文件系统
- [ ] 网络协议栈

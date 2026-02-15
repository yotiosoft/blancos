# 準備
nightly の設定、rust-src のインストール
```bash
$ rustup override set nightly
$ rustup component add rust-src
```

bootimage のインストール
```bash
$ rustup component add llvm-tools-preview
$ cargo install bootimage
```

# ビルド
```bash
$ cargo bootimage
```

# 起動
GUI で起動
```bash
$ qemu-system-x86_64 -drive format=raw,file=target/x86_64-ferrios/debug/bootimage-ferrios.bin
```

CUI で起動
```bash
$ qemu-system-x86_64 -nographic -serial mon:stdio -drive format=raw,file=target/x86_64-ferrios/debug/bootimage-ferrios.bin
```

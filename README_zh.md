# bpf-compatible

## 简介

这个仓库包含一套工具链，用以实现在不依赖于本地 `BTF` 的情况下， `eBPF` 程序的跨内核版本兼容。

我们基于[`btfhub`](https://github.com/aquasecurity/btfhub-archive)来实现。`btfhub`提供了各种发行版各种内核的预编译好的 `BTF` 存档（包括不支持本地 `BTF` 的内核版本）。

通过这个仓库提供的工具链，你可以从`btfhub`下载你所需要的内核版本的`BTF`存档，而后将这些`BTF`存档进行裁剪，使之仅保留你所写的`eBPF`程序中使用到了的类型，而后将这些裁剪后的`BTF`存档与编译好的`package.json`打包成`tar`存档发布。

而后通过这个仓库中提供的`bpf-compatible-rs`库，用户程序可以加载一个按照上述格式打包好的`tar`存档，从中选取适合当前内核的`BTF`存档，并从`tar`中获取`package.json`。

## 设计

### Tar的结构

```plain
|- package.json
|- btfhub-archive
|- ---- ubuntu <ID in os-release>
|- ---- ---- 22.04 <VERSION in os-release>
|- ---- ---- ---- x86_64 <machine in uname>
|- ---- ---- ---- ---- 5.15.0-71-generic.btf <kernel-release in uname>
```

一个由此工具链打包出来的`tar`文件的可能结构如上。其中包括由`ecc`所生成的`package.json`，以及一个`btfhub-archive`文件夹。`btfhub-archive`的结构与仓库[`btfhub-archive`](https://github.com/aquasecurity/btfhub-archive)一致，但只保留了其中部分内容（即我们需要的内核的BTF文件）。

### 具体的功能
- 从[`btfhub-archive`](https://github.com/aquasecurity/btfhub-archive)下载所需的内核版本的BTF
- 根据编译好的eBPF程序，对我们需要的BTF存档进行裁剪
- 将裁剪好的BTF与编译好的eBPF程序进行打包
- 解压以tar存档形式发布的 eBPF 程序，从中选取适合当前内核版本的BTF存档并运行

### 工具链

这个仓库所提供的工具链主要包括两部分。

- `script/btfgen`：一个shell脚本，用于从`btfhub-archive`下载BTF存档，以及生成裁剪过的BTF存档
- `bpf-compatible-rs`: 一个`Rust`库，用以提供解压tar存档的支持，以及选择适合当前内核的`BTF`存档的功能。目前`bpf-loader-rs`和`ecli`基于这个库实现了`tar`存档的加载与运行。

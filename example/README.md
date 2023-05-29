# bpf-compatible examples

Here is the root of the examples about bpf-compatible. The sources of the bpf programs are listed in the folder `examples`.
Other folders (e.g, libbpf, or bpftool) are auxiliary projects which are used to build eBPF program.

## Details

The example build structure was original from `libppf-bootstrap`.

Currently I only provided two examples:

- `bootstrap`: Monitoring the exec and exit of processes, tracking the live time of each process, and passing the data to the userspace program through ringbuf
- `execsnoop`: Monitoring the exec of processes, and pass the data to the userspace program through `perf event`
## Usage

- Run `make bootstrap` or `execsnoop` in the `examples` folder to build the executable
- Run the generated executable on the target machine to see the output

If the target machine has its kerne listed on  [this repo](https://github.com/eunomia-bpf/btfhub-archive), the executable should be successfully executed. Otherwise, it will fail to start.

## More

Theoretically, all programs that use libbpf can be adapted with this toolchain with changes of few lines. (i.e, link the library `libbpf_compatible.a`, and invoke `ensure_core_btf` from `btf_helpers.h`)

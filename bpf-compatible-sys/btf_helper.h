#ifndef _BTF_HELPER_H
#define _BTF_HELPER_H

#include <bpf/libbpf.h>

int ensure_core_btf_with_tar_binary(char** path, char* tar_bin, int tar_len);

int ensure_core_btf_with_linked_tar(char** path);

void clean_core_btf_rs(char* path);

static int ensure_core_btf(struct bpf_object_open_opts* opts) {
    return ensure_core_btf_with_linked_tar(&opts->btf_custom_path);
}

static void cleanup_core_btf(struct bpf_object_open_opts* opts) {
    clean_core_btf_rs(opts->btf_custom_path);
}

#endif  // _BTF_HELPER_H

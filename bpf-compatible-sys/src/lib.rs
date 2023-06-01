//!  SPDX-License-Identifier: MIT
//!
//! Copyright (c) 2023, eunomia-bpf
//! All rights reserved.
//!
#![allow(clippy::not_unsafe_ptr_arg_deref)]
use std::{
    ffi::{c_char, c_int, CStr},
    io::{Read, Write},
    path::PathBuf,
    slice,
};

use bpf_compatible_rs::{generate_current_system_btf_archive_path, tar::Archive};
use flate2::read::GzDecoder;
use libc::{c_void, malloc, EILSEQ, EINVAL, EIO, ENOENT, ENOMEM};

const VMLINUX_BTF_PATH: &str = "/sys/kernel/btf/vmlinux";

#[no_mangle]
pub extern "C" fn ensure_core_btf_with_tar_binary(
    path: *mut *const c_char,
    tar_bin: *const u8,
    tar_len: c_int,
) -> c_int {
    if PathBuf::from(VMLINUX_BTF_PATH).exists() {
        return 0;
    }
    let tar_bytes = unsafe { slice::from_raw_parts(tar_bin, tar_len as usize) };
    let decompressed_bytes = {
        let mut val = vec![];
        let mut gzip_reader = GzDecoder::new(tar_bytes);
        if let Err(e) = gzip_reader.read_to_end(&mut val) {
            eprintln!("Failed to decompress: {}", e);
            return -EINVAL;
        }
        val
    };

    let mut tar = Archive::new(&decompressed_bytes[..]);
    let local_btf_path =
        PathBuf::from("./btfhub-archive").join(match generate_current_system_btf_archive_path() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to generate running kernel btf path: {:?}", e);
                return -ENOENT;
            }
        });
    let entries = match tar.entries() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read entries in the tar: {}", e);
            return -EINVAL;
        }
    };
    let mut btf_path = None;
    for entry in entries {
        let entry = match entry {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to read entry: {}", e);
                return -EIO;
            }
        };
        // path of a entry looks like `./btfhub-archive/ubuntu/20.04/x86_64/5.4.0-40-generic.btf`
        let path = match entry.header().path() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to read path name: {}", e);
                return -EILSEQ;
            }
        };
        if path == local_btf_path {
            let mut temp_file = match mkstemp::TempFile::new("/tmp/eunomia.btf.XXXXXX", false) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Failed to create a tempfile to store the btf: {}", e);
                    return -EIO;
                }
            };
            let file_bytes = &decompressed_bytes[entry.raw_file_position() as usize
                ..(entry.raw_file_position() + entry.size()) as usize];
            if let Err(e) = temp_file.write_all(file_bytes) {
                eprintln!("Failed to write btf things to the tempfile: {}", e);
                return -EIO;
            }
            btf_path = Some(temp_file.path().to_string());
        }
    }
    let btf_path = match btf_path {
        Some(v) => v,
        None => {
            eprintln!("Failed to find the btf archive matching the running kernel");
            return -ENOENT;
        }
    };
    let btf_path_bytes = btf_path.as_bytes();
    // The buffer will be passed to C program, so allocate it with malloc
    let holder = unsafe { malloc(btf_path_bytes.len() + 1) } as *mut u8;
    if holder.is_null() {
        eprintln!("Unable to allocate a buffer for c string");
        return -ENOMEM;
    }
    let holder_slice = unsafe { slice::from_raw_parts_mut(holder, btf_path_bytes.len() + 1) };
    holder_slice[..btf_path_bytes.len()].copy_from_slice(btf_path_bytes);
    // C-Strings require a trailing zero
    holder_slice[btf_path_bytes.len()] = 0;
    *unsafe { &mut *path } = holder as *const c_char;
    0
}

extern "C" {
    static _binary_min_core_btfs_tar_gz_start: c_char;
    static _binary_min_core_btfs_tar_gz_end: c_char;
}

#[no_mangle]
pub extern "C" fn ensure_core_btf_with_linked_tar(path: *mut *const c_char) -> c_int {
    let len = unsafe {
        &_binary_min_core_btfs_tar_gz_end as *const c_char as usize
            - &_binary_min_core_btfs_tar_gz_start as *const c_char as usize
    };
    ensure_core_btf_with_tar_binary(
        path,
        unsafe { &_binary_min_core_btfs_tar_gz_start as *const c_char } as *const u8,
        len as c_int,
    )
}

#[no_mangle]
pub extern "C" fn clean_core_btf_rs(path: *mut c_char) {
    if path.is_null() {
        return;
    }
    let path_buf = PathBuf::from(
        unsafe { CStr::from_ptr(path) }
            .to_string_lossy()
            .to_string(),
    );
    if let Err(e) = std::fs::remove_file(path_buf) {
        eprintln!("Failed to perform clean: {}", e);
    }
    unsafe { libc::free(path as *mut c_void) };
}

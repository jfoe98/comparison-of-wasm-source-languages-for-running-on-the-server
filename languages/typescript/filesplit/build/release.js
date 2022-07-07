import * as __import0 from "wasi_snapshot_preview1";
async function instantiate(module, imports = {}) {
  const __module0 = imports.wasi_snapshot_preview1;
  const adaptedImports = {
    env: Object.assign(Object.create(globalThis), imports.env || {}),
    wasi_snapshot_preview1: Object.assign(Object.create(__module0), {
      fd_write(fd, iovs, iovs_len, nwritten) {
        // ~lib/bindings/wasi_snapshot_preview1/fd_write(u32, usize, usize, usize) => u16
        fd = fd >>> 0;
        iovs = iovs >>> 0;
        iovs_len = iovs_len >>> 0;
        nwritten = nwritten >>> 0;
        return __module0.fd_write(fd, iovs, iovs_len, nwritten);
      },
      proc_exit(rval) {
        // ~lib/bindings/wasi_snapshot_preview1/proc_exit(u32) => void
        rval = rval >>> 0;
        __module0.proc_exit(rval);
      },
      path_open(dirfd, dirflags, path, path_len, oflags, fs_rights_base, fs_rights_inheriting, fs_flags, fd) {
        // ~lib/bindings/wasi_snapshot_preview1/path_open(u32, u32, usize, usize, u16, u64, u64, u16, usize) => u16
        dirfd = dirfd >>> 0;
        dirflags = dirflags >>> 0;
        path = path >>> 0;
        path_len = path_len >>> 0;
        fd = fd >>> 0;
        return __module0.path_open(dirfd, dirflags, path, path_len, oflags, fs_rights_base, fs_rights_inheriting, fs_flags, fd);
      },
      fd_read(fd, iovs, iovs_len, nread) {
        // ~lib/bindings/wasi_snapshot_preview1/fd_read(u32, usize, usize, usize) => u16
        fd = fd >>> 0;
        iovs = iovs >>> 0;
        iovs_len = iovs_len >>> 0;
        nread = nread >>> 0;
        return __module0.fd_read(fd, iovs, iovs_len, nread);
      },
      fd_fdstat_set_flags(fd, flags) {
        // ~lib/bindings/wasi_snapshot_preview1/fd_fdstat_set_flags(u32, u16) => u16
        fd = fd >>> 0;
        return __module0.fd_fdstat_set_flags(fd, flags);
      },
      fd_close(fd) {
        // ~lib/bindings/wasi_snapshot_preview1/fd_close(u32) => u16
        fd = fd >>> 0;
        return __module0.fd_close(fd);
      },
    }),
  };
  const { exports } = await WebAssembly.instantiate(module, adaptedImports);
  return exports;
}
export const {
  filesplit
} = await (async url => instantiate(
  await (
    globalThis.fetch && globalThis.WebAssembly.compileStreaming
      ? globalThis.WebAssembly.compileStreaming(globalThis.fetch(url))
      : globalThis.WebAssembly.compile(await (await import("node:fs/promises")).readFile(url))
  ), {
    wasi_snapshot_preview1: __maybeDefault(__import0),
  }
))(new URL("release.wasm", import.meta.url));
function __maybeDefault(module) {
  return typeof module.default === "object" && Object.keys(module).length == 1
    ? module.default
    : module;
}

import { finish, startup } from "./env";

import "wasi";
import { FileSystem, Descriptor } from 'as-wasi/assembly';
import { fdflags, lookupflags, rights, oflags, path_open, fd, errno } from 'bindings/wasi';

export function filesplit(n: i32): void {
    startup();

    let file = getFile(n);

    for (let i = 0; i < n; i++) {
        let line = file.readLine();
        

        if (line == null) {
            continue;
        }

        writeToNewFile(parseInt(changetype<String>(line)) as i32);
    }

    file.close();

    finish();
}

function writeToNewFile(number: i32): void {
    let outputFilePath = "./" + (number % 10).toString() + ".txt";

    let file = open_with_append(outputFilePath);

    if (file == null) {
        throw new Error("Could not open file " + outputFilePath);
    }

    if (!file.setFlags(fdflags.APPEND)) {
        console.log("Setting flags did not work");
    }
    file.writeStringLn(number.toString());
    file.close();
}

function getFile(n: i32): Descriptor {
    let filePath = "./numbers_" + n.toString() + ".txt";
    let file = FileSystem.open(filePath, "r");

    if (file == null) {
        throw new Error("Could not open file " + filePath);
    }

    return file;
}

// copied from https://github.com/jedisct1/as-wasi/blob/0cb04bc290c0ff6e6dc3d1184e02fd1600eb54e8/assembly/as-wasi.ts#L493
// copied and adjusted because original implementation does not offer any possibility to set the right FD_FDSTAT_SET_FLAGS to be able to append to file 
function open_with_append(path: string): Descriptor | null {
    let dirfd = 3;
    let fd_lookup_flags = lookupflags.SYMLINK_FOLLOW;
    let fd_oflags: u16 = 0;
    let fd_rights: u64 = 0;

    fd_oflags = oflags.CREAT;
    fd_rights =
    rights.FD_WRITE |
    rights.FD_READ | rights.FD_SEEK | rights.FD_TELL | rights.FD_FILESTAT_GET |
    rights.PATH_CREATE_FILE | rights.FD_FDSTAT_SET_FLAGS;

    let fd_rights_inherited = fd_rights;
    let fd_flags: fdflags = 0;
    let path_utf8_buf = String.UTF8.encode(path);
    let path_utf8_len: usize = path_utf8_buf.byteLength;
    let path_utf8 = changetype<usize>(path_utf8_buf);
    let fd_buf = memory.data(8);
    let res = path_open(
      dirfd as fd,
      fd_lookup_flags,
      path_utf8, path_utf8_len,
      fd_oflags,
      fd_rights,
      fd_rights_inherited,
      fd_flags,
      fd_buf
    );
    if (res !== errno.SUCCESS) {
      return null;
    }
    let fd = load<u32>(fd_buf);
    return new Descriptor(fd);
  }
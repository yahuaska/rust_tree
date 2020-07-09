## Usage
```
USAGE:
    tree [OPTIONS] [PATH]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --depth <DEPTH>    

ARGS:
    <PATH>    Path to show

```
Example output:
```
> $ ./target/debug/tree /usr/share/gdb
├─ auto-load
│  ├─ usr
│  │  ├─ lib32
│  │  │  └─ libstdc++.so.6.0.25-gdb.py
│  │  ├─ mips-linux-gnu
│  │  │  ├─ lib32
│  │  │  │  └─ libstdc++.so.6.0.25-gdb.py
│  │  │  ├─ lib64
│  │  │  │  └─ libstdc++.so.6.0.25-gdb.py
│  │  │  └─ lib
│  │  └─ lib
│  ├─ librevenge-stream-0.0.py
│  ├─ librevenge-0.0.py
│  └─ lib
├─ syscalls
│  ├─ mips-o32-linux.xml
│  ├─ mips-n32-linux.xml
│  ├─ sparc64-linux.xml
│  ├─ arm-linux.xml
│  ├─ sparc-linux.xml
│  ├─ s390x-linux.xml
│  ├─ amd64-linux.xml
│  ├─ s390-linux.xml
│  ├─ i386-linux.xml
│  ├─ ppc-linux.xml
│  ├─ mips-n64-linux.xml
│  ├─ gdb-syscalls.dtd
│  ├─ ppc64-linux.xml
│  ├─ freebsd.xml
│  └─ aarch64-linux.xml
├─ python
│  └─ gdb
└─ system-gdbinit
```
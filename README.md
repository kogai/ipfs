Experimenal implementation for to investigate IPFS.

## TODO

* [x] Investigate how to build from source of referencial implementation
* [x] Run go-ipfs from source code

## MEMO

```zsh
# Establish local IPFS node
$ ipfs daemon

# In the other process
$ cargo run # -> Success!

# Build and run go-ipfs from source code
ARGS=version make go/run
```

MVP will support at least `daemon`, `add` and `cat`

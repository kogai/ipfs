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
ARGS="cat /ipfs/QmQ3WBvK3CbjsrfTduFxvrudjDyP59vkp1qa1NpuDugkb8 --debug" make go/run
```

* cat() -> `$GOPATH/src/github.com/ipfs/go-ipfs/core/commands/cat.go:136`
* Emit() -> `$GOPATH/src/gx/ipfs/QmTjNRVt2fvaRFu93keEC7z5M1GS1iH6qZ9227htQioTUY/go-ipfs-cmds/cli/responseemitter.go:136`
* Run() -> `$GOPATH/src/gx/ipfs/QmTjNRVt2fvaRFu93keEC7z5M1GS1iH6qZ9227htQioTUY/go-ipfs-cmds/cli/run.go:27`
* ConstructNode -> `$GOPATH/src/github.com/ipfs/go-ipfs/cmd/ipfs/main.go:107`
* DAG.Get() -> `$GOPATH/src/github.com/ipfs/go-ipfs/merkledag/merkledag.go:64`
* Block.GetBlock() -> `$GOPATH/src/github.com/ipfs/go-ipfs/blockservice/blockservice.go:327`
* BlockStore.Get() -> `$GOPATH/src/gx/ipfs/QmaG4DZ4JaqEfvPWt5nPPgoTzhc1tr1T3f4Nu9Jpdm8ymY/go-ipfs-blockstore/blockstore.go:118`

DAGReader is also as IOReader.

MVP will support at least `daemon`, `add` and `cat`

Experimenal implementation for to investigate IPFS.

## TODO

* [x] Investigate how to build from source of referencial implementation
* [x] Run go-ipfs from source code
* [ ] Investigate what `dshelp.CidToDsKey(hash)` responsible to

## MEMO

```zsh
# Establish local IPFS node
$ ipfs daemon

# In the other process
$ cargo run # -> Success!

# Build and run go-ipfs from source code
ARGS=version make go/run
ARGS="cat /ipfs/QmWm6xANVhca6YNY97bF7TqVtFgEsT2Kperzgwhmvpi88d --debug" make go/run
```

* cat() -> `$GOPATH/src/github.com/ipfs/go-ipfs/core/commands/cat.go:136`
* Emit() -> `$GOPATH/src/gx/ipfs/QmTjNRVt2fvaRFu93keEC7z5M1GS1iH6qZ9227htQioTUY/go-ipfs-cmds/cli/responseemitter.go:136`
* Run() -> `$GOPATH/src/gx/ipfs/QmTjNRVt2fvaRFu93keEC7z5M1GS1iH6qZ9227htQioTUY/go-ipfs-cmds/cli/run.go:27`
* ConstructNode -> `$GOPATH/src/github.com/ipfs/go-ipfs/cmd/ipfs/main.go:107`
* DAG.Get() -> `$GOPATH/src/github.com/ipfs/go-ipfs/merkledag/merkledag.go:64`
* Block.GetBlock() -> `$GOPATH/src/github.com/ipfs/go-ipfs/blockservice/blockservice.go:327`
* BlockStore.Get() -> `$GOPATH/src/gx/ipfs/QmaG4DZ4JaqEfvPWt5nPPgoTzhc1tr1T3f4Nu9Jpdm8ymY/go-ipfs-blockstore/blockstore.go:118`
* Where actual data from blocks appear -> `$GOPATH/src/github.com/ipfs/go-ipfs/unixfs/io/dagreader.go:43`
* Maybe actual getter is here -> `$GOPATH/src/gx/ipfs/QmXRKBQA4wXP7xWbFiZsR1GP4HV6wMDQ1aWFxZZ4uBcPX9/go-datastore/keytransform/keytransform.go:43`
* One of an actual implementation of DataStore `$GOPATH/src/gx/ipfs/QmXRKBQA4wXP7xWbFiZsR1GP4HV6wMDQ1aWFxZZ4uBcPX9/go-datastore/retrystore/retrystore.go:47`
* Actual Repo instantiated(FSRepo) at here `$GOPATH/src/github.com/ipfs/go-ipfs/repo/fsrepo/fsrepo.go:119`
* Where actual Get() -> `$GOPATH/src/gx/ipfs/QmaCTqBCt1aKaGfHfSVzsprqWRXCjHthK8xXrPbUZYCWga/go-ds-flatfs/flatfs.go:321`
* And get actual directory at `$GOPATH/src/gx/ipfs/QmaCTqBCt1aKaGfHfSVzsprqWRXCjHthK8xXrPbUZYCWga/go-ds-flatfs/shard.go:62`
  DAGReader is also as IOReader.

MVP will support at least `daemon`, `add` and `cat`

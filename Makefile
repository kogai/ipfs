GO_SRC := $(shell find ./go/src/github.com/ipfs/go-ipfs -type f -name '*.go')

all: $(GOPATH)/src/github.com/ipfs/ipfs

go/run: $(GOPATH)/src/github.com/ipfs/ipfs
	$(GOPATH)/src/github.com/ipfs/go-ipfs/cmd/ipfs/ipfs $(ARGS)

.PHONY: install
install: go/bin/ipfs
	go get github.com/rogpeppe/godef
	go get golang.org/x/tools/cmd/godoc
	go get -v github.com/uudashr/gopkgs/cmd/gopkgs
	go get -v github.com/sqs/goreturns
	go get -v github.com/ramya-rao-a/go-outline
	go get -v github.com/nsf/gocode
	go get -v golang.org/x/tools/cmd/guru
	go get -u github.com/derekparker/delve/cmd/dlv

go/bin/ipfs: $(GO_SRC)
	go get -u -d github.com/ipfs/go-ipfs
	cd $(GOPATH)/src/github.com/ipfs/go-ipfs && \
		make install

$(GOPATH)/src/github.com/ipfs/ipfs: $(GO_SRC)
	cd $(GOPATH)/src/github.com/ipfs/go-ipfs && \
		make build

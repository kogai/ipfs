.PHONY: install
install: go/bin/ipfs
	go get github.com/rogpeppe/godef
	go get golang.org/x/tools/cmd/godoc
	go get -v github.com/uudashr/gopkgs/cmd/gopkgs

go/bin/ipfs:
	go get -u -d github.com/ipfs/go-ipfs
	cd $(GOPATH)/src/github.com/ipfs/go-ipfs && \
		make install

DEL = rm -rf

all: bootstrap.tar

package.json: 
	ecc bootstrap.bpf.c bootstrap.h

btfhub-cache:
	BTFHUB_REPO=https://github.com/eunomia-bpf/btfhub-archive \
	BTFHUB_CACHE=./btfhub-cache \
	../../script/btfgen fetch

output.tar.gz: btfhub-cache package.json
	BTFHUB_CACHE=./btfhub-cache \
	../../script/btfgen btfgen -j package.json bootstrap.bpf.o

bootstrap.tar: output.tar.gz
	gzip -d output.tar.gz
	mv output.tar bootstrap.tar

ecli-rs:
	cargo install ecli-rs

run: bootstrap.tar ecli-rs
	ecli-rs run bootstrap.tar

clean:
	$(DEL) *.o
	$(DEL) *.json
	$(DEL) btfhub-cache
	$(DEL) output.tar.gz

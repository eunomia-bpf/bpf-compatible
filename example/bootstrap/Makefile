DEL = rm -rf

all: bootstrap.tar

package.json: 
	ecc bootstrap.bpf.c bootstrap.h

btfhub-cache:
	BTFHUB_REPO=https://github.com/eunomia-bpf/btfhub-archive \
	BTFHUB_CACHE=./btfhub-cache \
	../../script/btfgen fetch

bootstrap.tar: btfhub-cache package.json
	BTFHUB_CACHE=./btfhub-cache \
	../../script/btfgen btfgen -j package.json bootstrap.bpf.o

clean:
	$(DEL) *.o
	$(DEL) *.json
	$(DEL) btfhub-cache

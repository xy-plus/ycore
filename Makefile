all: run

run:
	make -C user build
	make -C kernel run

env:
	make -C kernel env

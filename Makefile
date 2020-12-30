.PHONY: all clean run env

all: run

run:
	make -C user build
	make -C kernel run

env:
	make -C kernel env

clean:
	make -C user clean
	make -C kernel clean
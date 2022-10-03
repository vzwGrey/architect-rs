.PHONY: all
all: prepare sim

SIM ?= ghdl
EXTRA_ARGS ?= --std=08
TOPLEVEL_LANG ?= vhdl

VHDL_SOURCES += vhdl/shift_register.vhdl
TOPLEVEL = shiftregister
MODULE = test

include $(shell cocotb-config --makefiles)/Makefile.sim

.PHONY: prepare
prepare:
	mkdir -p vhdl/
	cargo r --example shift_register > vhdl/shift_register.vhdl

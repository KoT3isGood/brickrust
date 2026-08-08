.PHONY: help install build run

help:
	@echo "run"
	@echo "    make install DIR=\"/path/to/common/Brick Rigs\""
	@echo ""
	@echo "for example \"$(HOME)/.steam/steam/steamapps/common/Brick Rigs\""
	@echo "dev=true    enables development builds"

TARGET_RELEASE=target/x86_64-pc-windows-gnu/release
TARGET_DEV=target/x86_64-pc-windows-gnu/debug

ifeq ($(dev),true)
TARGET=$(TARGET_DEV)
else
TARGET=$(TARGET_RELEASE)
endif

CARGO_TARGET = --target x86_64-pc-windows-gnu

doc:
	cargo doc $(CARGO_TARGET) --examples

ifeq ($(dev),true)
build:
	cargo build $(CARGO_TARGET) --examples || true
else
build:
	cargo build $(CARGO_TARGET) -r --target x86_64-pc-windows-gnu --examples || true
endif

ifdef DIR

install: build
	cp "$(TARGET)/deps/xinput1_3.dll" "$(DIR)/BrickRigs/Binaries/Win64" 
	cp "$(TARGET)/deps/brickworks.dll" "$(DIR)/BrickRigs/Binaries/Win64" 
	cp "/usr/x86_64-w64-mingw32/bin/libgcc_s_seh-1.dll" "$(DIR)" 
	cp "/usr/x86_64-w64-mingw32/bin/libwinpthread-1.dll" "$(DIR)" 
	mkdir -p "$(DIR)/brickworks"
	cp "$(TARGET)/examples/function_tests.dll" "$(DIR)/brickworks" 


else
install:
install_examples:
endif

run: install
	steam -applaunch 552100

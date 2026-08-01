.PHONY: help install build run

help:
	@echo "run"
	@echo "\tmake install DIR=\"/path/to/common/Brick Rigs\""
	@echo ""
	@echo "for example \"$(HOME)/.steam/steam/steamapps/common/Brick Rigs\""
	@echo "dev=true\tenables development builds"

TARGET_RELEASE=target/x86_64-pc-windows-gnu/release
TARGET_DEV=target/x86_64-pc-windows-gnu/debug

ifeq ($(dev),true)
TARGET=$(TARGET_DEV)
else
TARGET=$(TARGET_RELEASE)
endif

ifeq ($(dev),true)
build:
	cargo build --target x86_64-pc-windows-gnu --examples || true
else
build:
	cargo build -r --target x86_64-pc-windows-gnu --examples || true
endif

ifdef DIR

install: build
	cp "$(TARGET)/deps/xinput1_3.dll" "$(DIR)/BrickRigs/Binaries/Win64" 
	cp "$(TARGET)/deps/brickworks.dll" "$(DIR)/BrickRigs/Binaries/Win64" 
	mkdir -p "$(DIR)/brickworks"
	cp "$(TARGET)/examples/basic_init.dll" "$(DIR)/brickworks" 
	cp "/usr/x86_64-w64-mingw32/bin/libgcc_s_seh-1.dll" "$(DIR)" 
	cp "/usr/x86_64-w64-mingw32/bin/libwinpthread-1.dll" "$(DIR)" 


else
install: help
endif

run: install
	steam -applaunch 552100

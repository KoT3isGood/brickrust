.PHONY: help install build run

help:
	@echo "make install DIR=\"/path/to/common/Brick Rigs\" # installs for standalone version"
	@echo "make install_brmk DIR=\"/path/to/BRMK/BrickRigs\" # installs for BRMK"
	@echo ""
	@echo "for example \"$(HOME)/.steam/steam/steamapps/common/Brick Rigs\""
	@echo ""
	@echo "dev=true			enables development builds"
	@echo "MINGW=...		mingw libraries directory"

TARGET_RELEASE=target/x86_64-pc-windows-gnu/release
TARGET_DEV=target/x86_64-pc-windows-gnu/debug
MINGW=/usr/x86_64-w64-mingw32/bin

ifeq ($(dev),true)
TARGET=$(TARGET_DEV)
else
TARGET=$(TARGET_RELEASE)
endif

CARGO_TARGET = --target x86_64-pc-windows-gnu --workspace

doc:
	cargo doc $(CARGO_TARGET) --examples

ifeq ($(dev),true)
build:
	cargo build $(CARGO_TARGET) --features=brickworks_impl/impl
build_brmk:
	cargo build $(CARGO_TARGET) --features=brmk,brickworks_impl/impl
else
build:
	cargo build $(CARGO_TARGET) -r --features=brickworks_impl/impl
build_brmk:
	cargo build $(CARGO_TARGET) -r --features=brmk,brickworks_impl/impl
endif

ifdef DIR
PLUGIN_DIR = $(DIR)/BrickRigs/Plugins/BrickRust

xinput: 
	cargo build --package xinput1_3 -r --target x86_64-pc-windows-gnu

install: xinput build
	cp "$(TARGET_RELEASE)/xinput1_3.dll" "$(DIR)/BrickRigs/Binaries/Win64" 
	cp "$(TARGET)/deps/brickworks.dll" "$(DIR)/BrickRigs/Binaries/Win64" 
	cp "$(MINGW)/libgcc_s_seh-1.dll" "$(DIR)" 
	cp "$(MINGW)/libwinpthread-1.dll" "$(DIR)" 
	mkdir -p "$(DIR)/brickworks"

install_brmk: build_brmk
	mkdir -p "$(PLUGIN_DIR)/Binaries/Win64"
	cp "$(TARGET)/brmk_plugin.dll" "$(PLUGIN_DIR)/Binaries/Win64/BrickRigsModKitSteam-BrickRust.dll" 
	cp "brmk_plugin/BrickRigsModKitSteam.module" "$(PLUGIN_DIR)/Binaries/Win64"
	cp "brmk_plugin/BrickRust.uplugin" "$(PLUGIN_DIR)"
	cp "$(TARGET)/brickworks.dll" "$(DIR)/BrickRigs/Binaries/Win64/brickworks.dll" 
	cp "$(MINGW)/libgcc_s_seh-1.dll" "$(DIR)/BrickRigs/Binaries/Win64"
	cp "$(MINGW)/libwinpthread-1.dll" "$(DIR)/BrickRigs/Binaries/Win64" 
	mkdir -p "$(DIR)/BrickRigs/Binaries/Win64/brickworks"

else
install:
install_brmk:
endif

run: install
	steam -applaunch 552100

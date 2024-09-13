# https://github.com/esp-rs/esp-idf-template

.PHONY:
setup:
	cargo install cargo-generate
	cargo install ldproxy
	cargo install espup
	cargo install espflash
	cargo install cargo-espflash # Optional

build: ./src
	cargo build

flash: build
	# espflash flash \
	# 	target/riscv32imc-esp-espidf/debug/hello-rust-embedded \
	# 	--monitor

	espflash flash --list-all-ports --monitor target/xtensa-esp32-espidf/debug/dev
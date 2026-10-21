include .env # HOST=root@<box>
.DEFAULT_GOAL := deploy

# clippy can't run here: v4l is Linux-only
linter:
	cargo fmt --check

provision: upload
	./provision.sh $(HOST)

upload: linter
	rsync -F .rsync-filter --delete -av . $(HOST):/opt/kaleidoscope

# compiles on the box: cross-compiling macroquad + bindgen from macOS isn't worth it
build: upload
	ssh $(HOST) /root/.cargo/bin/cargo build --release --manifest-path /opt/kaleidoscope/Cargo.toml

deploy: build
	ssh $(HOST) 'systemctl daemon-reload && systemctl restart kaleidoscope'

logs:
	ssh $(HOST) journalctl -n100 -f -u kaleidoscope

restart:
	ssh $(HOST) systemctl restart kaleidoscope

stop:
	ssh $(HOST) systemctl stop kaleidoscope

reboot:
	ssh $(HOST) reboot

all: build
.PHONY: all

build:
	cargo build --release
.PHONY: build

install: build
	./install-workflow.sh target/release/alfred-gems
.PHONY: install

update-plist:
	./install-workflow.sh --update-plist
.PHONY: update-plist

clean:
	cargo clean
	rm -f gems.alfredworkflow
.PHONY: clean

release: clean build
	cp target/release/alfred-gems .
	zip -9 -r gems.alfredworkflow icon.png info.plist alfred-gems
	rm alfred-gems


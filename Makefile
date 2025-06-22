IMAGE := nds-rust-dev

build:
	docker run --rm -v $$(pwd):/work $(IMAGE) cargo +nightly nds build

prepare:
	docker build -t $(IMAGE) .

release:
	docker run --rm -v $$(pwd):/work $(IMAGE) cargo +nightly nds build --release

clean:
	docker run --rm -v $$(pwd):/work $(IMAGE) cargo clean

shell:
	docker run --rm -it -v $$(pwd):/work $(IMAGE) bash

autocomplete:
	./autocomplete.sh

.PHONY: build release clean shell

IMAGE := nds-rust-dev

build:
	docker build -t $(IMAGE) .
	docker run --rm -v $$(pwd):/work $(IMAGE) cargo +nightly nds build

release:
	docker build -t $(IMAGE) .
	docker run --rm -v $$(pwd):/work $(IMAGE) cargo +nightly nds build --release

clean:
	docker run --rm -v $$(pwd):/work $(IMAGE) cargo clean

shell:
	docker build -t $(IMAGE) .
	docker run --rm -it -v $$(pwd):/work $(IMAGE) bash

autocomplete:
	./autocomplete.sh

.PHONY: build release clean shell

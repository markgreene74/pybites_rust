.PHONY: build-executable
build-executable:
	cd exercises_downloader && \
	cargo build --release

.PHONY: download-exercises
download-exercises:
	$(MAKE) build-executable && \
	exercises_downloader/target/release/pybites-rust-download && \
	echo ... all done

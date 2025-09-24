.PHONY: build-executable
build-executable:
	cd exercise_downloader && \
	cargo build --release

.PHONY: download-exercises
download-exercises:
	$(MAKE) build-executable && \
	exercise_downloader/target/release/pybites-rust-download && \
	echo ... all done

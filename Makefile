.PHONY: build
build:
	docker build -t role-master .
.PHONY: run
run:
	docker run role-master
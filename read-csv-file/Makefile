## help: print this help message (DEFAULT)
.PHONY: help
help:
	@echo -e "Make commands for ${binary_name}\n"
	@echo 'Usage:'
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/ /' | sort

## build: build the project
.PHONY: build
build:
	@cargo $@

## clean: clean the project
.PHONY: clean
clean:
	@cargo $@

## dev: start the project in dev mode
.PHONY: dev
dev:
	@cargo watch --clear -x "run"

## run: run the project
.PHONY: run
run:
	@cargo $@

## test: run the tests
.PHONY: test
test:
	@cargo $@

help:
	@cat Makefile

edit:
	@nvim ./src/main.rs

git.pushall: git.commitall
	@git push
git.commitall: git.addall
	@if [ -n "$(shell git status -s)" ] ; then git commit -m 'saving'; else echo '--- nothing to commit'; fi
git.addall:
	@git add .

check:
	@cargo rustc -- -Awarnings
test:
	@cargo test

all: test release

release:
	@cargo rustc --release -- -C prefer-dynamic

run: 
	@cargo run


clean:
	@cargo clean

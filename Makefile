binname="learn-Rust"


help:
	@echo 'there is no help.. yet'

run: release size
	@./target/release/$(binname)

edit:
	@vi ./src/main.rs

savetogit:
	@git add . && git commit -m 'saving' && git push

release:
	@cargo rustc --release -- -C prefer-dynamic

size:
	@ls -lAh ./target/release/$(binname)

clean:
	@cargo clean

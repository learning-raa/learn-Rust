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
	
path:
	export LD_LIBRARY_PATH='/home/configurator/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib'

clean:
	@cargo clean

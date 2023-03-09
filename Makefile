
help:
	@echo 'there is no help.. yet'

run:
	@cargo run

edit:
	@vi ./src/main.rs

savetogit:
	@git add . && git commit -m 'saving' && git push



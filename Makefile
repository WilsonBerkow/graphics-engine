compile: clean main

main:
	rustc -C opt-level=3 src/main.rs

d: clean
	rustc src/main.rs

run:
	./main

clean:
	rm -f main

compile: clean main

main:
	rustc src/main.rs

run: main
	./main

clean:
	rm -f main

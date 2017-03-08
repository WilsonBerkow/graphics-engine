compile: clean main

main:
	rustc -A dead_code src/main.rs

run: main
	./main

clean:
	rm -f main

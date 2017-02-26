compile: clean main

main:
	rustc src/main.rs

run: main
	./main
	display img.ppm

clean:
	rm -f main

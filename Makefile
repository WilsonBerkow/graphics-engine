compile: clean main

main:
	rustc src/main.rs

run: main
	./main
	convert img.ppm img.png
	display img.png

clean:
	rm -f main

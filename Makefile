compile: clean main

main:
	rustc -C opt-level=3 src/main.rs

run: main
	mkdir -p anim
	./main
	if [ -f anim/football.gif ]; then rm anim/football.gif; fi
	convert anim/*.png anim/football.gif
	animate -delay 5 anim/football.gif

clean:
	rm -f main

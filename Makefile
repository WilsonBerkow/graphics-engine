compile: clean main

main:
	rustc src/main.rs

run: main
	mkdir -p anim
	./main
	if [ -f anim/football.gif ]; then rm anim/football.gif; fi
	convert anim/* anim/football.gif
	animate -delay 25 anim/football.gif

clean:
	rm -f main

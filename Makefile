all: watch

tailwind:
	NODE_ENV=production bun tailwindcss -c ./tailwind.config.js -o ./style/tailwind.css --minify

format:
	cargo fmt

clean:
	cargo clean

games:
	./build_games

watch: games format tailwind
	cargo leptos watch

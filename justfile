wasm:
	cd client && wasm-pack build
dist: wasm
	cd client/www && npm run build
serve: dist
	sfz -r client/www/dist

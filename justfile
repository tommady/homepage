build-wasm:
	wasm-pack build --release --no-typescript --target web --out-dir ../static/pkg wasm

build-server:
	cargo build --release --package server

run-server:
	cargo run --package server

build-image:
	podman build --tag docker.io/tommady/homepage:latest --format docker --arch arm64 --os linux --platform linux/arm64 --squash-all .

publish-image:
	podman push docker.io/tommady/homepage:latest

deploy-k8s:
	kubectl apply -k kustomize/base

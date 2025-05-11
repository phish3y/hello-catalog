.PHONY: fmt-web
fmt-web:
	cd web && npm run format

.PHONY: fmt-api
fmt-api:
	cargo fmt

.PHONY: run-web
run-web:
	docker compose up --build web

.PHONY: run-api
run-api:
	docker compose up --build api_debug

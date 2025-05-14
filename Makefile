.PHONY: fmt-web
fmt-web:
	cd web && npm run format

.PHONY: fmt-api
fmt-api:
	cargo fmt

.PHONY: run
run:
	docker compose up --build

.PHONY: down
down:
	docker compose down
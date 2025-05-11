.PHONY: format-web
format-web:
	cd web && npm run format

.PHONY: run-web
run-web:
	docker compose up --build web

.PHONY: run-api
run-api:
	docker compose up --build api_debug

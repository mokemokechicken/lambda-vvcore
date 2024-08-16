build-container:
	docker buildx build --platform linux/arm64 -t lambda-vvcore .

run-container:
	docker run --platform linux/arm64 -e LOCAL_MODE=1 -p 9000:8080 lambda-vvcore:latest

test-local-api:
	bash test_api.sh

test-lambda-api:
	API_ENDPOINT=$(LAMBDA_ENDPOINT) bash test_api.sh
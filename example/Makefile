IMAGE_NAME := record-builder

--guard-%:
	@if [ "${${*}}" = "" ]; then \
		echo "Environment variable $* not set: $*=abc123 make $(MAKECMDGOALS)"; \
		exit 1; \
	fi

--check-%:
	@if [ "`command -v ${*}`" = "" ]; then \
		echo "Command ${*} not found in path";  \
		exit 1; \
	fi

# Container
build: --check-podman
	podman build -t $(IMAGE_NAME) -f Containerfile

docker-push: --guard-VERSION build --check-podman
	podman login docker.io
	podman push $(IMAGE_NAME):latest docker.io/$(USER)/$(IMAGE_NAME):$(VERSION)

gitlab-push: --guard-REGISTRY_URL --guard-REGISTRY-PATH --guard-VERSION build --check-podman
	podman login $(REGISTRY_URL)
	podman push --tls-verify=false  \
		$(IMAGE_NAME):latest $(REGISTRY_URL)/$(REGISTRY_PATH)/$(IMAGE_NAME):$(VERSION)

# Docs
generate: --check-mvn
	mvn -P generate-docs exec:exec generate-resources

publish: --guard-USER --guard-TOKEN --check-mvn
	CONFLUENCE_USER=$(USER) CONFLUENCE_TOKEN=$(TOKEN) mvn -P generate-docs-and-publish exec:exec generate-resources

open-adr:
	open ./target/static/documentation/architecture-decision-records.html

open-tdr:
	open ./target/static/documentation/technical-debt-records.html

docs: mvn
	@echo
	@echo "*************************************"
	@echo "*                                   *"
	@echo "* Documentation can be found here:  *"
	@echo "* ./target/static/documentation/	   *"
	@echo "*                                   *"
	@echo "*************************************"
	@echo


APP="my-rkt"
REG="192.168.2.51:5000"

.PHONY: build
build: vmlinux
	docker build -t $(APP) .

.PHONY: push
push: build
	docker tag $(APP) $(REG)/$(APP)
	docker push $(REG)/$(APP)
	docker image remove $(REG)/$(APP)

.PHONY: vmlinux
vmlinux:
	cd sal_app && make clean && make vmlinux

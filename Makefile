APP="my-rkt"
REG="192.168.2.51:5000"

.PHONY: build
build: vmlinux
	docker build -t $(APP) .
	-docker rm -f dummy
	docker create -it --name dummy $(APP) /bin/sh
	docker cp dummy:/root/$(APP) $(APP)
	docker rm -f dummy

.PHONY: push
push: build
	docker tag $(APP) $(REG)/$(APP)
	docker push $(REG)/$(APP)
	docker image remove $(REG)/$(APP)

.PHONY: rund
rund: build
	docker run -it --privileged -p 80:80 -v /sys/kernel/debug:/sys/kernel/debug my-rkt

.PHONY: vmlinux
vmlinux:
	cd sal_app && make clean && make vmlinux

.PHONY: clean
clean:
	-docker system prune -a
	-rm -f $(APP)
	cd sal_app && make clean
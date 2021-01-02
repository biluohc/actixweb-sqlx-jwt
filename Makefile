app=template
version=v0.1
commit_id=$(shell git describe --always --abbrev=8 --dirty=-modified)
src_path = ${PWD}
src_path_docker = /opt

# debug or release
# mode=release
ifneq ($(mode), debug)
    target = --release
else
    target =
endif

docker_file_rust = Dockerfile.rust
docker_file_prod = Dockerfile.prod

image_rust = ${app}-rust
image_prod = ${app}-prod

# println
$(info src: $(src_path))
$(info commit: $(commit_id))
$(info mode: $(target))

$(shell mkdir -p ${HOME}/.cargo/{git,registry})
$(shell touch ${HOME}/.cargo/config)

b:
	cargo build ${target} && rm -frv target/*/*.d

image-rust:
	@if [ `docker images | grep ${image_rust} | wc -l` -eq 0 ]; then \
		echo "build rust docker image "; \
        docker build -t ${image_rust}:latest -f ${docker_file_rust} .; \
    else \
        echo "docker image ${image_rust} already exist!";\
    fi

image: build
	@echo "build docker image"; \
	docker build -t ${image_prod}:latest -f ${docker_file_prod} . && \
	docker tag ${image_prod}:latest ${image_prod}:${version}-${commit_id} && \
	echo ${image_prod}:${version}-${commit_id}

build: image-rust
	@echo "docker build ${mode}"; \
	docker run -i --rm \
		-v ${HOME}/.cargo/git:/root/.cargo/git \
		-v ${HOME}/.cargo/config:/root/.cargo/config \
	    -v ${HOME}/.cargo/registry:/root/.cargo/registry \
	    -v ${src_path}:${src_path_docker} \
	    --workdir ${src_path_docker} \
		--network host \
	    ${image_rust}:latest \
	    bash -c "cd ${src_path_docker}/ && cargo build --target-dir target ${target}"

run:
	docker run -d --restart always --network host -v ${src_path}:/opt --name ${app} ${image_prod}

clean:
	rm -fr target/*

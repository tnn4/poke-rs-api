# How to build dockerfile if name is not Dockerfile or Dockerfile?
# docker build
# docker build -f <docker-file-name> <path-to-context>
# docker build -t tnn4/poke-rs-api:0.1.0 -f <dockerfile-name> <path-to-context>
# path-to-context is usually . , the current working directory

# COPY only supports the basic copying of local files into the container, 
# while ADD has some features 
# (like local-only tar extraction and remote URL support) that are not immediately obvious. 

# Build the backend
# FROM debian:bullseye
# turns out the . was the problem
FROM ubuntu:22.04
ENV PATH="${PATH}:/usr/src/myapp/bin"
# ENTRYPOINT []
WORKDIR /usr/src/myapp
# COPY HOST_PATH DST_PATH
COPY . /usr/src/myapp/
CMD ["bin/poke-rs-api", "--docker"]

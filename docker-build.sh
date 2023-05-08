image="pokeapi" # image to build
name="test1" # container name
dockerfile="Dockerfile-alpine"
this_directory="."
host_port=3001
container_port=3001

# docker image rm $image
docker rm ${name}
docker build -t $image -f $dockerfile ${this_directory}
docker run --name $name -p $host_port:$container_port $image
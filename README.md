# Pkmn API

[![Rust](https://img.shields.io/badge/language-Rust-blue)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/language-Python-blue)](https://www.python.org/)
![bash](https://img.shields.io/badge/language-bash-blue)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)


TODO
- Dockerize/containerize
- Add nginx to docker-compose

Compatibility: This was developed and tested on Ubuntu 22.04. This app should work on most Linux platforms and WSL2 if you're feeling adventurous. 
You can always Dockerize/containerize it to make it more portable.

Requires:
- Docker
- If no docker
    - [python](https://www.python.org/downloads/) v3.10.6
    - [rust](https://www.rust-lang.org/tools/install) (if developing) rustc v1.68.0, cargo v1.68.0

---

This is a [Pokeapi](https://pokeapi.co/) mirror that can be used to serve a subset of the api using Rust Axum backend(hence the name). This was built as an education experience and also serves as a performant tool for backend development and testing REST apis. 

If you're wondering why this does't serve all endpoints, it's because, the primary goal of this app is optimization and performance. Data has to be cached and hashed first to make lookups fast, and serving a subset of the data is required if working on constrained resources.

## Directory Structure
Directory | Description
|---|---
|pokeapi-cache | static json files for serving
|poke-rs-api | source code for rust backend

# Quick Start

1. Download the release [here]()
2. Download the source, go to the root of the project
3. Create a `bin` folder
4. drop the binary into the `bin` folder
5. Run it with: `$ ./bin/poke-rs-api`
    - If you want to use a different port `$./bin/poke-rs-api --port <your-port>`
    

The server should be listening on `http://localhost:3001` on local mode and `http://0.0.0.0:3001` on docker mode.

The default port is `3001` but can be changed with an argument when running the server.

e.g.
```bash
$ bin/poke-rs-api --port <your-port>
```

## With docker


Manually:

Build image: 
- `docker build -t poke-rs-api -f Dockerfile-main .`

Run container from image:
- `docker run --name my_container_name -p 3001:3001 poke-rs-api`

Delete image:
- `docker rm my_container_name`

Or use the script - `./docker-build.sh`

- If you want to change the port in docker: change the line at `CMD` in the `Dockerfile-main` to something like:


```dockerfile
CMD ["bin/poke-rs-api","--docker", "--port", "<your-port>"]
```

## Serving the API over the Internet

The server listens on `localhost:<port>` and serves JSON for the request at its endpoint at `http://localhost:<port>/pokeapi/v2/<endpoint>/<id>` so this has to be combined with a reverse proxy like [Nginx](https://docs.nginx.com/nginx/admin-guide/web-server/reverse-proxy/), or [Apache proxy server](https://httpd.apache.org/docs/2.4/howto/reverse_proxy.html)  to forward requests to localhost and serve data over the internet securely. 
- [nginx example configuration](#nginx-example)



# Development
---

## Populate Cache



You need to populate your cache so the server can use the cached data to serve the REST api.

This repo has zip files with the static json files for some pre-cached endpoints stored in `pokeapi-cache-zip` that you extract to `pokeapi-cache`
Make sure the directory looks like this.
```
pokeapi-cache
|--berry
|--move
|--pokemon
```

Download the rest of the endpoints you want to use with the python script `pokeapi-cacher.py`.

```sh
$ python pokeapi-cacher.py <list-of-endpoints>
```

e.g. Download cache all files from endpoints: pokemon, berry, move, See: [here](https://pokeapi.co/docs/v2) for available endpoints
```sh
$ python pokeapi-cacher.py --endpoint pokemon --endpoint berry --endpoint move
```


### Generate lookup table from cache
Once the files are cached to `pokeapi-cache` you can serve them with the provided backend.

If you want to be able to look up a resource by name you'll have to generate mapping from name to id since the files are stored as `<id>.json`.

`$ map2id.py berry && map2id.py pokemon`

Note: this can't be done for some endpoints like `move` since it currently generates duplicate keys in the toml file. 

```
$ bin/poke-rs-api
```

The server will listen on `http://localhost:3001`
and will serve endpoints like `http://localhost:3000/pokeapi/pokemon/0`

---

## Docker reference
If you have Docker and have available memory and disk space, use the provided `Dockerfile` to do all the work for you.

[Important: Build the cache first](#populate-cache)

```sh
$ docker build . # don't forget the dot(.)
```

If you want to use a custom dockerfile name do:
- `$ docker build <path-to-dockerfile> <path-to-context>`
In this case something like:
- `$ docker build -f yourDockerfile .`

If you want to give your image a name, you'll have to put it from the command line [src](https://stackoverflow.com/questions/38986057/how-to-set-image-name-in-dockerfile). Dockerfiles don't support tagging.
- `$ docker build -t me/myapp:tag -f <path-to-dockerfile> <path-to-context>`
- e.g. `$ docker build -t me/poke-rs-api:v0.1.0 -f mydockerfile .`


Run `docker images` to find the built image
The build command should have given an Image ID like: `ae5528709935...`
```
0:[t@serv2:pokeapi]> docker images
REPOSITORY                       TAG             IMAGE ID       CREATED          SIZE
tnn4/poke-rs-api                 v0.1.0          ae5528709935   23 minutes ago   87.9MB

```


### Available Endpoints

These are the endpoints that the backend can serve. You can always extend the source to add more.

Endpoints that can be retrieved by name means you can pass a name for the item instead of a number: e.g. `http://localhost/pokeapi/v2/pokemon/25` can be retrieved with `http://localhost/pokeapi/v2/pokemon/pikachu`


Endpoint | Description | Example URL | retrieve by name
|---|---|---|---
berry | berry data | http://localhost/pokeapi/v2/berry/1 | yes
move | pokemon moves | http://localhost/pokeapi/v2/move/1 | no
pokemon | pokemon information | http://localhost/pokeapi/v2/pokemon/1 | yes


---


### Nginx example

Nginx example conf in `/etc/nginx/nginx.conf`:

<details>
    <summary>nginx.conf</summary>

```conf
user  t;
worker_processes  auto;

error_log  /var/log/nginx/error.log notice;
pid        /var/run/nginx.pid;


events {
    worker_connections  1024;
}


http {
    include       /etc/nginx/mime.types;
    default_type  application/octet-stream;

    log_format  main  '$remote_addr - $remote_user [$time_local] "$request" '
                      '$status $body_bytes_sent "$http_referer" '
                      '"$http_user_agent" "$http_x_forwarded_for"';

    access_log  /var/log/nginx/access.log  main;

    sendfile        on;
    #tcp_nopush     on;

    keepalive_timeout  65;

    #gzip  on;

    include /etc/nginx/conf.d/*.conf;
    
    # Rate Limiting
    # limit_req_zone defines parameters for rate limiting
    # $binary_remote_addr - store remote ips as binary to save space
    # zone - define shared memory to store state of each IP, 1mb = 16K, 10mb = 160K ips
    # rate- max amount to send per milliseconds 4r/s = 1r / 250 ms
    limit_req_zone $binary_remote_addr zone=my_limit:10m rate=4r/s;


    # root appends to path, alias replaces path
    server{
        # SERVER NAME
        server_name example.com www.example.com;
        root /home/t/siteroot/public/;

        # http
        listen 80;
        
        # https
        listen 443 ssl http2; # ipv4
        listen [::]:443 ssl http2; # ipv6

        # -- Matrix server --
        listen 8448 ssl http2;
        listen [::]:8448 ssl http2;

        merge_slashes off;

        # Put SSL/TLS credentials here to enable HTTPS
        # TODO: figure out where these are in docker container
        ssl_certificate /path/to/cert.pem;
        ssl_certificate_key /path/to/key.pem;
        ssl_trusted_certificate /path/to/cert.pem; 
        include /path/to/options-ssl-nginx.conf;
 
        
        # -- Error Logging
        error_log /path/to/nginx-error.log;
        
        # Nginx defaults to allow 1 Mb uploads
        client_max_body_size 1M;

        # -- Locations --
        # - a location block lives within a server block and is used to define
        # how Nginx handles requests for different resources and URIs for parent server
        # https://www.digitalocean.com/community/tutorials/understanding-nginx-server-and-location-block-selection-algorithms

        index index.html index.htm;
        
        location / {
            root /var/www/public;
            # try_files $uri $uri/ /index.html index.html index.htm;
            try_files index.html index.htm;
        }
        # pokeapi endpoints
        # you can use regex with locaitons
        # use: https://regex101.com/
        location ~ \/pokeapi\/v2\/(berry|move|pokemon)\/[A-Za-z0-9]+ {
            proxy_pass http://127.0.0.1:3001$request_uri;
            default_type application/json;
        }
    }
}
```
</details>
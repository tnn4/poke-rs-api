# Poke-rs API

Requires:
- python

## Directory Structure
Directory | Description
|---|---
|pokeapi-cache | static json files for serving
|poke-rs-api | source code for rust backend


Thi is a [Pokeapi](https://pokeapi.co/) mirror that serves static JSON with a Rust Axum backend(hence the name), so it should be performant. This was built to have a local and performant backend to learn about and test REST apis.

## Quick Start
Download and cache the endpoints you want to use with the python script `pokeapi-cacher.py`.

```sh
$ python pokeapi-cacher.py <list-of-endpoints>
```
e.g. Download all files from endpoints: pokemon, berry, move, See: [here](https://pokeapi.co/docs/v2) for available endpoints


```sh
$ python pokeapi-cacher.py --pokemon --berry --move
```

Once the files are cached to `pokeapi-cache` you can serve them with the provided backend.

```
$ bin/poke-rs-api
```

The server will listen on `http://localhost:3001`
and will serve endpoints like `http://localhost:3000/pokeapi/pokemon/0`

Available Endpoints

Endpoint | Description | Example URL
|---|---|---
move | pokemon move data | http://localhost/pokeapi/v2/move/1
pokemon | pokemon information | http://localhost/pokeapi/v2/pokemon/1


---

## Details

This backend is meant to be used with a reverse proxy like [Nginx](https://docs.nginx.com/nginx/admin-guide/web-server/reverse-proxy/), or [Apache proxy server](https://httpd.apache.org/docs/2.4/howto/reverse_proxy.html) or used standalone for local development. It simply listens on `localhost:<port>` and serves JSON for the request at its endpoint at `http://localhost:<port>/pokeapi/v2/<endpoint>/<id>`.

The default port is `3001` but can be changed with an argument when running the server.

```bash
$ bin/poke-rs-api --port <your-port>
```

If you want to serve the REST api over the internet you should add a reverse proxy to forward requests to this backend. You can tell the backend to listen on all domains with `0.0.0.0` but that would be dangerous.

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
        location ~ \/pokeapi\/v2\/(berry|pokemon)\/[0-9]+ {
            proxy_pass http://127.0.0.1:3001$request_uri;
            default_type application/json;
        }
    }
}
```
</details>
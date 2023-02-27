# ticketland-signdrop
A service that airdrops SOL to newly created accounts


# Build docker image

```bash
docker build --build-arg GITHUB_TOKEN=<github_token> -t registry.digitalocean.com/ticketland/ticketland-signdrop:<version> -f ./operations/signdrop/Dockerfile ./
```

# NFT Scraper

# Development

Before everything modify first the `docker-compose`. Change the environment for scraper
the `NFTPORT_API_KEY`, replaced it with your own API key.

In order to run it through docker you need to build it first. Build the image using
the `docker-compose` build command:

```
docker-compose build scarper
```

Then run the whole docker-compose file which contains `postgres`, `pgadmin` and lastly the
one we built `scarper`. Confirm everything works:

```
docker-compose up -d
docker-compose ps -a
```
If all the docker process is running then its time to open pgadmin, access it through your browser `http://localhost:5480`.
Use the credential which you can find inside the `docker-compose.yaml` file. Then execute the `migrations/dump_postgres.backup_schema.yaml`
in the database named `nft`.

See if everything works by restarting all the process:

```
docker-compose restart
```
# A Test Social Network CRUD API 

## Requirements
 - posgres 
 - docker 
 - docker-compose cli 
 - diesel cli (To setup the database and if you want to run migrations)
 - insomnia (api testing client)

## Running the application 

You must start the docker container with
`docker compose up -d`
then you can run. First you will need to in the api directory
`cd api`
`diesel setup` // only need to do this once 
`make run`

## Testing 
I have included an export file named `insomnia_export` which you and import into 
insomnia and hit the api with insomnia or use the `copy to curl` option to use product curl commands

## Other Notes 
I left the debug binary if you would like to run the binary (note it the binary was compiled on a M2 mac so it may not work but running cargo new should work)

# Newsflash
Newsflash is an aggregate worldwide news app. 

It collects and displays news from multiple sources around the globe, providing users with a unified and up-to-date news experience.

# Setup
Currently this uses a local PostgreSQL database through docker. This will change to include more shortly.
## API Environment Variables
The newsflash-api/.env file is required for configuration. 

It must include:

SERVER_ADDR: The address and port for the API server (e.g., 127.0.0.1:8081)

PG__USER: PostgreSQL username

PG__PASSWORD: PostgreSQL password

PG__HOST: PostgreSQL host

PG__PORT: PostgreSQL port

PG__DBNAME: PostgreSQL database name
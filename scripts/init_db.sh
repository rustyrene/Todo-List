#!/bin/bash

# Check if .env file exists
env_file="../.env"
if [ ! -f "$env_file" ]; then
    echo "Error: .env file not found in the parent directory."
    exit 1
fi

# Load environment variables from .env file
source "$env_file"

# Check if the required environment variables are set
if [ -z "$POSTGRES_USER" ] || [ -z "$POSTGRES_DB" ] || [ -z "$POSTGRES_PASSWORD" ]; then
    echo "Error: Please set POSTGRES_USER, POSTGRES_DB, and POSTGRES_PASSWORD in the .env file."
    exit 1
fi

# Set the PostgreSQL container name and password
container_name="postgres-container-todolist"
password="$POSTGRES_PASSWORD"

# Create and run the PostgreSQL Docker container
docker run -d \
    --name "$container_name" \
    -e POSTGRES_USER="$POSTGRES_USER" \
    -e POSTGRES_DB="$POSTGRES_DB" \
    -e POSTGRES_PASSWORD="$password" \
    -p 5432:5432 \
    postgres:latest

echo "PostgreSQL Docker container created and running."

# Print the DB_URL
db_url="postgresql://$POSTGRES_USER:$password@localhost:5432/$POSTGRES_DB"
echo "DB_URL: $db_url"


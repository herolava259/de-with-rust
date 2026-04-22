echo "Docker compose build for Neo4J"

docker compose -f ../docker-compose.yml \
               --env-file ../.env \
               build

echo "Docker compose up for Neo4J"

docker compose -f ../docker-compose.yml \
               --env-file ../.env \
               up -d
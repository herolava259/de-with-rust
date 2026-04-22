#! /bin/bash/

# Load environment variables from .env file

if [ "$#" -gt 0]; then
    container_name="$1"
else
    container_name = "graphdb"
fi

if [ "$#" -gt 1]; then
    database_name="$2"
else
    database_name=${cat ../.env | grep NEO4J_DB_NAME_DEF | cut -d "=" -f2}
fi


if [ "$#" -gt 2]; then
    backup_file=$3
else
    backup_file=$(find ../backups/ -type f -name "*$database_name*" -printf '%T@ %p\n' | sort -nr | head -1 | cut -d' ' -f2)
fi

pwd=$(cat ../.env | grep NEO4J_PASSWORD | cut -d'=' -f2)

container_id=$(docker ps -qf "name=$container_name")

if [ -z "$container_id" ]; then
    echo "No running container found with name $container_name"
    container_id=$(docker ps -qaf "name=$container_name")
    echo "Running database"
    echo "Container found with name $container_name: $container_id"
    docker start $container_id
    echo "Container $container_name started successfully"
fi


docker exec \
    "$container_id" \
    cypher-shell -u neo4j -p "$pwd" -d system "stop database $database_name;"

docker exec \
    $container_id \
    neo4j-admin database restore --from="$backup_file" --overwrite-destination "$database_name"

docker exec -it $container_id cypher-shell -u neo4j -p "$pwd" -d system "start database $database_name;"
    

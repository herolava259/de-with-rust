echo "Arguments passed to the script"

for arg in "$@"; do
    echo "Argument: $arg"
done

# assgign argument to named variable
if [ "$#" -gt 0 ]; then
    NEO4J_VERSION="$1"
else
    NEO4J_VERSION=$(cat ../.env | grep NEO4J_VERSION | cut -d "=" -f2)

# assign argument second arg as password 
if [ "$#" -gt 1 ]; then
    NEO4J_PASSWORD="$2"
else
    NEO4J_PASSWORD=$(cat ../.env | grep NEO4J_PASSWORD | cut -d "=" -f2)
fi

# assign argument third arg as plugins
if [ "$#" -gt 2 ]; then
    NEO4J_PLUGINS="$3"
else
    NEO4J_PLUGINS="[$(cat ../.env | grep NEO4J_PLUGINS | cut -d "=" -f2)]"
fi

echo "Docker build started for Neo4J"

# TODO: assign version to build argument and use it in the dockerfile
docker build \
    --build-arg NEO4J_VERSION=$NEO4J_VERSION \
    --build-arg NEO4J_PLUGINS=$NEO4J_PLUGINS \
    --build-arg NEO4J_PASSWORD=$NEO4J_PASSWORD \
    -t custom-neo4j \
    -f dockerfiles/Neo4J.Dockerfile .

echo "Docker build completed for Neo4J"

echo "Starting Neo4J container"

docker run \
    --name grapphdb\
    --network=standalone-network \
    --detach \
    --publish 7474:7474 --publish 7687:7687 \
    --publish=6000:6000 --publish=7000:7000 \
    --hostname=graphdb \
    --env NEO4H_AUTH=neo4j/$NEO4J_PASSWORD \
    --env NEO4J_PLUGINS=$NEO4J_PLUGINS \
    --volume=../data:/data \
    --volume=../logs:/logs \
    --volume=../conf:/conf \
    --volume=../plugins:/plugins \
    --volume=../backups:/backups \
    neo4j-$NEO4J_VERSION



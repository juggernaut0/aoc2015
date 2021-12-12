set -e

NAME="aoc2015"

CONTAINER=$(docker ps -f "name=$NAME" -q)
if [ -z "$CONTAINER" ]; then
  echo "starting db"
  CONTAINER=$(docker run --name $NAME --rm -d -v $PWD/input:/input -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres:14)
  sleep 5
fi
echo "$CONTAINER"

docker exec -i "$CONTAINER" psql -U postgres << EOF
DROP DATABASE IF EXISTS ${NAME};
DROP ROLE IF EXISTS ${NAME};
CREATE USER ${NAME} WITH PASSWORD '${NAME}';
CREATE DATABASE ${NAME};
GRANT ALL PRIVILEGES ON DATABASE ${NAME} TO ${NAME};
GRANT pg_read_server_files TO ${NAME};
EOF

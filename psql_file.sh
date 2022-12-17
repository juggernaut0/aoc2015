set -e

docker exec -i "aoc2015" psql -U postgres < $1

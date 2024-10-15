#!/bin/bash
printf "\n## Testing health...\n"
printf "GET /health HTTP/1.0\nHost: localhost\n\n" | nc localhost 3030

printf "\n## Testing GET /transaction...\n"

printf "GET /transaction/1 HTTP/1.0\nHost: localhost\n\n" | nc localhost 3030
printf "\n"


printf "\n--- Testing...\n"
printf "GET /user/1 HTTP/1.0\nHost: localhost\n"
printf "^^^\n"
# printf "GET /user/1 HTTP/1.0\nHost: localhost\n\n" | nc localhost 3030
curl -X GET http://localhost:3030/user/1

printf "\n## Testing POST /user...\n"

curl -X POST \
  http://localhost:3030/user \
  -H 'Content-Type: application/json' \
  -d '{"username":"testuser","email":"testemail"}'


curl -X POST \
  http://localhost:3030/transaction \
  -H 'Content-Type: application/json' \
  -d '{"casino_id": 1,"user_id": 1, "cost": 100, "benefit": 120}'
# printf "POST /transaction/1 HTTP/1.0\nHost: localhost\n\n" | nc localhost 3030
# printf "\n"
# printf "PUT /user/1 HTTP/1.0\nHost: localhost\n\n" | nc localhost 3030
#!/bin/bash
printf "\n## Testing health...\n"
printf "GET /health HTTP/1.0\nHost: localhost\n\n" | nc localhost 3030

printf "\n## Testing GET /transaction...\n"

printf "GET /transaction/1 HTTP/1.0\nHost: localhost\n\n" | nc localhost 3030
printf "\n"

printf "\n## Testing GET /user...\n"

printf "GET /user/1 HTTP/1.0\nHost: localhost\n\n" | nc localhost 3030
printf "\n"

printf "\n## Testing POST /user...\n"

curl -X POST \
  http://localhost:3030/user \
  -H 'Content-Type: application/json' \
  -d '{"username":"testuser","email":"testemail"}'

# printf "POST /transaction/1 HTTP/1.0\nHost: localhost\n\n" | nc localhost 3030
# printf "\n"
# printf "PUT /user/1 HTTP/1.0\nHost: localhost\n\n" | nc localhost 3030
# printf "\n"



printf -- "\n##---\n"
printf "Done testing..."
printf -- "\n##---\n"

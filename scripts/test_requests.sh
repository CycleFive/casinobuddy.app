#!/bin/bash
printf "\n## Testing health...\n"
printf "GET /health HTTP/1.0\nHost: localhost\n\n" | nc localhost 3030
printf "\n## Testing GET /transactions...\n"
printf "GET /transactions/1 HTTP/1.0\nHost: localhost\n\n" | nc localhost 3030
printf -- "\n##---\n"
printf "Done testing..."
printf -- "\n##---\n"

echo -ne "Content-Length: 83\r\n\r\n"
echo -ne "{\"jsonrpc\": \"2.0\", \"method\": \"initialize\", \"id\": 1, \"params\": {\"capabilities\": {}}}"
sleep 3

echo -ne "Content-Length: 57\r\n\r\n"
echo -ne "{\"jsonrpc\": \"2.0\", \"method\": \"initialized\", \"params\": {}}"
sleep 3

echo -ne "Content-Length: 157\r\n\r\n"
echo -ne "{\"jsonrpc\": \"2.0\", \"method\": \"textDocument/definition\", \"id\": 2, \"params\": {\"textDocument\": {\"uri\": \"file://temp\"}, \"position\": {\"line\": 1, \"character\": 1}}}"
sleep 3

echo -ne "Content-Length: 65\r\n\r\n"
echo -ne "{\"jsonrpc\": \"2.0\", \"method\": \"shutdown\", \"id\": 3, \"params\": null}"
sleep 3

echo -ne "Content-Length: 52\r\n\r\n"
echo -ne "{\"jsonrpc\": \"2.0\", \"method\": \"exit\", \"params\": null}"
sleep 3

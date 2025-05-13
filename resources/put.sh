#!/bin/bash

curl -X PUT http://localhost:3000/api/file --header "Content-Type: application/zip" --data-binary "@500Cities_City_11082016.ZIP"

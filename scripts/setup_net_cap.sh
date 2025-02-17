#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <file_name>"
  exit 1
fi

FILE_NAME="$1"

sudo setcap  cap_net_admin,cap_net_raw=eip "$FILE_NAME"
EXIT_CODE=$?  

if [ $EXIT_CODE -eq 0 ]; then
  echo "Capabilities have been successfully set on $FILE_NAME"
else
  echo "Failed to set capabilities on $FILE_NAME"
  exit $EXIT_CODE
fi
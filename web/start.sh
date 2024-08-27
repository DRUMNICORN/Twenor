# start.sh file
# will do following
# 1.1. if screen session is not running, start it
# 1.2 if screen session is running, attach to it
# 2. build the project
# 3. run the project

#!/bin/bash

PROJECT_NAME="website"

# 1.1. if screen session is not running, start it

if ! screen -list | grep -q "$PROJECT_NAME"; then
    screen -dmS $PROJECT_NAME
fi

# 1.2 if screen session is running, attach to it

screen -S $PROJECT_NAME

# 2. build the project

yarn build

# 3. run the project

yarn start
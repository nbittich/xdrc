# XDRC
overkill way of making aliases

# How to

- e.g given the following directory `$HOME/my-projects/project-docker`
- run the following command to add a shortcut for docker-compose: `xdrc a up docker-compose up -d`
  - where `up` is the key, the rest is the command to run
- to check which commands you have within that dir: `xdrc i`
- to execute a saved command: `xdrc r up`
- to delete a saved command: `xdrc d up`
- A file `.xdrc.json` should be created in `$HOME/my-projects/project-docker`. It is used to store your commands
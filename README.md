# bevy_2048
2048 clone created with Bevy game engine.

The game is in development.

# TODOS
- [x] Board
  - [x] Create the background at startup
  - [x] Create the empty tiles grid at startup
- [x] Tiles
  - [x] Tile spawning
    - [x] Create an event system for spawning new tiles
    - [x] Spawn 2 tiles at startup
    - [x] Add tile spawning animation 
  - [ ] Tile movement
    - [ ] Handle the keys input
    - [ ] Move tiles only if the new position:
      - [ ] is not out of bounds and
      - [ ] is empty or
      - [ ] has a moving tile or
      - [ ] has a not-merged-tile with the same level
    - [ ] Add movement animation
- [ ] Score system
  - [ ] Add score to the score system every merge
  - [ ] Highscore system
    - [ ] Every end game save the highscore
    - [ ] Save the highscore in a file
- [ ] Interface
  - [ ] Add score label
  - [ ] Add highscore label
  - [ ] Add new game button
  - [ ] Add a title
  - [ ] Add a link to this repo and bevy repo
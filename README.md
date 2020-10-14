# bevy_2048
2048 clone created with Bevy game engine.

The game is in development.

## TODOS
- [x] Board
  - [x] Create the background at startup
  - [x] Create the empty tiles grid at startup
- [x] Tiles
  - [x] Tile spawning
    - [x] Create an event system for spawning new tiles
    - [x] Spawn 2 tiles at startup
    - [x] Add tile spawning animation 
  - [x] Tile movement
    - [x] Handle the keys input
    - [x] Move tiles only if the new position:
      - [x] is not out of bounds and
        - [x] is empty or
        - [x] has a moving tile or
          - [x] has a not-merged-tile with the same level and
          - [x] self is not merged
    - [x] Add movement animation
    - [x] Spawn new tile if moved
  - [x] Tile despawning
    - [x] Despawn merged tiles with the last level with an animation
  - [x] Add merge animation
- [x] Score system
  - [x] Add score to the score system every merge
  - [x] Highscore system
    - [x] Check for highscore every score
    - [x] Save the highscore in a file
- [x] Game Over
  - [x] When not available moves
- [ ] Interface
  - [x] Add score label
  - [x] Add highscore label
  - [x] Add new game button
  - [x] Add a title
  - [x] Add how to play explanation
  - [ ] Add game over screen
  - [ ] Add a link to this repo and bevy repo
  - [ ] Add explanation of the tiles' colors
    - [ ] Make each color to be revealed when getting to the color in the game
  - [ ] Options
    - [ ] Add an option to set the board's size
  - [ ] Add an option to hide the interface
- [ ] Polish
  - [x] Remove the usage of commands for changing components' values(too slow)
  - [x] Separate the project into files
  - [ ] Cache best.bin file directory at start
  - [ ] When moving while animating save the next direction.
# bevy_2048
2048 clone created with Bevy game engine.

The game is in development.

## TODOS
- [x] Board
  - [x] Create the background at startup
  - [x] Create the empty tiles grid at startup
  - [x] Make the board flexible
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
- [x] Set window's title
- [ ] Interface
  - [x] Add score label
  - [x] Add highscore label
  - [x] Add new game button
  - [x] Add a title
  - [x] Add how to play explanation
  - [x] Make the UI flexible
  - [ ] Add explanation of the tiles' colors
    - [ ] Make each color to be revealed when getting to the color in the game
  - [ ] Add game over screen
  - [ ] Add a link to this repo and bevy repo
- [ ] Finishing
  - [ ] Update README
    - [ ] Add intro to the game 
    - [ ] Add instructions how to run using Cargo
    - [ ] Add gifs of the game.
  - [ ] Add a release to this repo
- [ ] Polish
  - [x] Remove the usage of commands for changing components' values(too slow)
  - [x] Separate the project into files
  - [x] When getting an input while moving, save the direction for the next movement
  - [x] Rewrite the UI into a separated plugin for each widget
  - [ ] Move the todo list into its own file.
  - [ ] Cache best.bin file directory at start

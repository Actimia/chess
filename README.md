# Chess

A simple chess engine written in rust.

## Todo
 - Check for game over:
   - Checkmate/king has been captured
   - Draw by repetition/50 move rule
 - Import boardstate from PGN
 - Display move with proper notation
 - Input move with proper notation
 - Trim illegal moves:
   - Should not be able to move into check, including discoveries
 - Evaluate position
   - Count material
 - Alpha-beta search player
 - Tests

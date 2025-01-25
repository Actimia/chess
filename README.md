# Chess

A simple chess engine written in rust.

## Todo
 - Proper checkmate instead of king capture to end the game
 - Draw by insufficient material
   - Atleast king v king
 - Import boardstate from PGN
 - Display move with proper notation
 - Input move with proper notation
 - Trim illegal moves:
   - Should not be able to move into check, including discoveries
   - This might be covered by the engine if the eval fn works properly
 - Engine
   - Countless optimizations available
   - Memoize alpha-beta search
   - Better eval fn
 - Tests

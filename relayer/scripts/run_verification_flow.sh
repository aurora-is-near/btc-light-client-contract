

# Pick a real transaction from a block and verify it
VERIFY_MODE="true" TRANSACTION_POSITION=0 TRANSACTION_BLOCK_HEIGHT=277136 cargo run

# Try to verify transaction that does not exist, random numbers
VERIFY_MODE="true" TRANSACTION_POSITION=0 TRANSACTION_BLOCK_HEIGHT=277136 FORCE_TRANSACTION_HASH=6471267463 cargo run

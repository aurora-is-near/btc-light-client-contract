# Make sure you are running local bitcoin node with a block content downloaded
# and that block content is not pruned (see Readme for the initial setup)

# Pick a real transaction from a block and verify it
VERIFY_MODE="true" TRANSACTION_POSITION=0 TRANSACTION_BLOCK_HEIGHT=277136 cargo run

# Try to verify transaction that does not exist, random numbers
VERIFY_MODE="true" TRANSACTION_POSITION=0 TRANSACTION_BLOCK_HEIGHT=277136 FORCE_TRANSACTION_HASH="75a25d63da6063b00cb08f794ad0edb81f2fe7cd1f234b6462ff36d137bfaf19" cargo run

TEST := labyrinth
TEST_CASE := 3.in
TEST_BASE := ./testcases

run:
	cat $(TEST_BASE)/$(TEST)/$(TEST_CASE) | cargo run


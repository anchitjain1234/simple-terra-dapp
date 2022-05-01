# Simple Terra Dapp

A simple smart contract for terra blockchain supporting:
1. Instantiation
2. Set scores for users and their tokens. This can only be done by the owner.
3. Retrieve scores for the users.

To run the unit tests, run `cargo test`.

This contract is also deployed on testnet at [terra1p77szenk6wy9kv90ppmzumwmpray6exjgnetre](https://terrasco.pe/testnet/address/terra1p77szenk6wy9kv90ppmzumwmpray6exjgnetre).

Possible operations:
1. Retrivee the owner address using query `{"get_owner": {}}`.
2. Retrieve score for a particular user using query `{"get_score": {"address": "terra1vahwu7rlfhxvztnzqsszjr3xtsvqeytnfw4ymd"}}`. If the score is not set for that user the output will be INT_MIN.
3. Retrieve score for a particular user for a token using query `{"get_score_for_token": {"user_address":"terra1vahwu7rlfhxvztnzqsszjr3xtsvqeytnfw4ymd", "token_address": "terra1vahwu7rlfhxvztnzqsszjr3xtsvqeytnfw4ymd"}}`. If the score is not set for that user and token the output will be INT_MIN.
4. Set score for some user with some token using execute `{"set_score":{"user_address": "terra1mfafaq4fajgm763zyzgqpmguf0swzdphlcd63j", "token_address": "terra1mfafaq4fajgm763zyzgqpmguf0swzdphlcd63j", "score":5}}`. This can only be done by the owner. [Sample execution](https://terrasco.pe/testnet/tx/7B277BE2AB4045AE36D113DF0AA21AE46B58312E92F378F736F46C3B2FE8F893)

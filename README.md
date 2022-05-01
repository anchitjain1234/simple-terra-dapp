# Simple Terra Dapp

A simple smart contract for terra blockchain supporting:
1. Instantiation
2. Set scores for users and their tokens. This can only be done by the owner.
3. Retrieve scores for the users.

To run the unit tests, run `cargo test`.

This contract is also deployed on testnet at [terra1vahwu7rlfhxvztnzqsszjr3xtsvqeytnfw4ymd](https://terrasco.pe/testnet/address/terra1vahwu7rlfhxvztnzqsszjr3xtsvqeytnfw4ymd).

Possible operations:
1. Retrivee the owner address using query `{"get_owner": {}}`.
2. Retrieve score for a particular user using query `{"get_score": {"address": "terra1vahwu7rlfhxvztnzqsszjr3xtsvqeytnfw4ymd"}}`. If the score is not set for that user the output will be INT_MIN.
3. Retrieve score for a particular user for a token using query `{"get_score_for_token": {"user_address":"terra1vahwu7rlfhxvztnzqsszjr3xtsvqeytnfw4ymd", "token_address": "terra1vahwu7rlfhxvztnzqsszjr3xtsvqeytnfw4ymd"}}`. If the score is not set for that user and token the output will be INT_MIN.
4. Set score for some user with some token using execute `{"set_score":{"user_address": "terra1mfafaq4fajgm763zyzgqpmguf0swzdphlcd63j", "token_address": "terra1mfafaq4fajgm763zyzgqpmguf0swzdphlcd63j", "score":5}}`. This can only be done by the owner. [Sample execution](https://terrasco.pe/testnet/tx/D2F643BC41038E86219957635BF82868E713929A91D829DDA32E4369E3C7BF16)

The JSON files enclosed in this folder have been taken from a Sawtooth node running the [IntegerKey](https://sawtooth.hyperledger.org/docs/core/releases/1.0/transaction_family_specifications/integerkey_transaction_family.html) transaction processor. The JSON data comes from the following GET endpoints provide by a REST API on the node:
- /batches
- /state
- /blocks
- /transactions

The data was collected after the following commands had been run on a fresh instance of the sawtooth node:
<!-- 0. 0 - genesis -->
1. `intkey set num1 2`  (create num1 and set to 2)
3. `intkey set num2 2`  (create num2 and set to 2)
4. `intkey inc num2 10` (add 10 to num2)
5. `intkey inc num1 1`  (add 1 to num1)
6. `intkey dec num1 2`  (dec 2 from num1)
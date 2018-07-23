# Description
This is a framework for tracking generic [limit order books](https://en.wikipedia.org/wiki/Central_limit_order_book) and applying a variety of pricing mechanisms.

# Goals
- Track any asset's orderbook in real time
- Be able to roll back to any previous point in time
- Be fast
- Be lossless
- Have a modular solution to pricing models

# Reasons
I thought it might be a good idea to have a pluggable solution to handle limit order books, as there really isn't anything resembling what I want in the broader open source marketplace.

With any luck I will have the opportunity to work on research related to orderbooks, and I think it wise to prebuild the tools I'll need for that ahead of time.

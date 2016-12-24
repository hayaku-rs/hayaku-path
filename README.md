# hayaku-path

The default router for hayaku. Based on [fasthttprouter](https://github.com/buaazp/fasthttprouter).


### Features
 - Handles wildcard parameters; e.g. `/{test}` will match `/one`, `/two`, etc.
 - Allows specification of regex used in parameters; e.g. `/{test:[\d]}` will
 match `/1` but not `/one`.

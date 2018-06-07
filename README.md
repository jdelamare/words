# Words 

## Purpose
Confirms if a word is in `usr/share/dict/words`.

## How to use
(soon to be implemented) add the crate to [dependencies] and specify the lastest version.  Then bring it into scope by placing `extern crate words;` followed by `use words::*` in a source file where the functionality is desired.

## How it works
Go to `/usr/share/dict/words` and add all of those words to a vector of vectors.  Then perform binary search on those words to confirm a query's existence.
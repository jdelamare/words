# Words 

## Purpose
Confirms if a word is in `usr/share/dict/words`.

## How to use
Add the crate to [dependencies] and specify the lastest version.  Then bring it into scope by placing `extern crate words;` followed by `use words::*` in a source file where the functionality is desired.
Note that creating the words object does not populate the object with the fields from `/usr/share/dict/words`.  Call `populate()` on the new object.

## How it works
Go to `/usr/share/dict/words` and add all of those words to a vector of vectors.  Then perform binary search on those words to confirm a query's existence.

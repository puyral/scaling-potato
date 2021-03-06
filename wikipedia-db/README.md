# Preprocessing
## how to use
```
./make_makefile.sh eo nrm br > Makefile && make sqlite
```

Where `eo nrm br` are the [wikipedia codes](https://en.wikipedia.org/wiki/List_of_Wikipedias) for the wikipedias to preprocess.
# Wikipedia dumps

According to [this idea](https://stackoverflow.com/questions/42983236/making-a-tree-of-wikipedia-links) I will put the wikipedia dumps here and there processed-version too. Will not be follow by git because there is a few GBs woth of data

The `xml` files tells which dumps are currently downloaded (those are followed by git btw)


## API usage note
 - [usercontrib](https://en.wikipedia.org/w/api.php?action=help&modules=query%2Busercontribs)

 - [a little tips](https://stackoverflow.com/questions/35826469/how-to-combine-two-wikipedia-api-calls-into-one/35830161)

  - https://en.wikipedia.org/wiki/Help:User_contributions
  - https://www.mediawiki.org/wiki/Manual:Categorylinks_table

# Results

The preprocessed database for `de`, `en`, `eo`, `es`, `fr` and `it` is available [here](https://lufi.rezel.net/r/FNG9Hz9M0v#+j1rtpsQvFM6QLh9cWlU2iCzhKLcwBPiaGtFBO5D4V4=)
# wordler

This is a very simple command-line wordle solver, which uses the strategy of guessing all possible 5-letter words in order of frequency, filtering out ones that are impossible as it learns more information. In some testing on the Wordle archives, I haven't found any that it can't solve in 6, and it can solve most of them in less time.

It can also work alongside a human, as it gives you three guesses to pick from (when testing it by itself, I just always select the first guess).

### Example usage:
```
> cargo run

Guesses:
  0: about
  1: other
  2: which
Pick: 0
Green: ___u 
Yellow: _b

Guesses:
  0: debug
  1: begun
  2: scrub
Pick: 0
Green: _ebu_
Yellow: 

Guesses:
  0: rebus
  1: sebum
  2: webui
Pick: 0
Green: rebus
Got 'rebus' in 3
```

You can also use `n` instead of a number when picking guesses to skip the first guess if it's an invalid word, like a proper name, which Wordler will sometimes guess otherwise.

### Frequency list

Unfortunately, English word frequency lists are pretty hard to find. Wordler uses the frequency list found [on Kaggle here](https://www.kaggle.com/rtatman/english-word-frequency), which isn't limited to 5-letter words, so Wordler loads it and filters those out as it goes through words.

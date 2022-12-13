
<!-- README.md is generated from README.Rmd. Please edit that file -->

# segmntr

<!-- badges: start -->

[![segmntr status
badge](https://paithiov909.r-universe.dev/badges/segmntr)](https://paithiov909.r-universe.dev)
<!-- badges: end -->

An R wrapper of [Vaporetto](https://github.com/daac-tools/vaporetto): a
fast and lightweight pointwise prediction based tokenizer

``` r
segmntr::segment(audubon::polano[4:5])
#> $`1`
#> [1] "宮沢/名詞-固有名詞-人名-姓/ミヤザワ" "賢治/名詞-固有名詞-人名-名/ケンジ"  
#> [3] "訳述"                               
#> 
#> $`2`
#>  [1] "その/連体詞/ソノ"                   "ころ/名詞-普通名詞-副詞可能/コロ"  
#>  [3] "わたくし/代名詞/ワタクシ"           "は/助詞-係助詞/ワ"                 
#>  [5] "、/補助記号-読点"                   "モリー"                            
#>  [7] "オ/接頭辞/オ"                       "市/名詞-普通名詞-一般/シ"          
#>  [9] "の/助詞-格助詞/ノ"                  "博物/名詞-普通名詞-一般/ハクブツ"  
#> [11] "局/名詞-普通名詞-助数詞可能/キョク" "に/助詞-格助詞/ニ"                 
#> [13] "勤め/動詞-一般/ツトメ"              "て/助詞-接続助詞/テ"               
#> [15] "居り"                               "まし/助動詞/マシ"                  
#> [17] "た/助動詞/タ"                       "。/補助記号-句点"
```

## License

3-Clause BSD License.

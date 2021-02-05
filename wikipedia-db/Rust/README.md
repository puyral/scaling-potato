# Rust

I've decided to do the preprocessing in *Rust* first because I wanted to try this language on some real project for a
while and second because we need some speed and memory efficiency that *python*, *Java* (or *Javascript*), this only
other languages we are somewhat comfortable with, don't have. (I don't feel like doing this in *C/C++*, and we haven't
done any *Caml* in a long time)

## Goal

The whole goal here is to pre-process some of Wikipedia's dump to be able to access the results quickly afterwards (and
reduce the memory footprint of the project (I have almost 100Go a Wikipedia graph on my computer!)).

As of right now, we are trying to PageRank the categories to make a Tree out of them in order to extract some
information from them.

## Structure

The files [linesample](linesample) and [linesample_links](linesample_links) are one-line extract of the the Wikipedia in
Esperanto used for the tests.

Otherwise we are trying to keep the Rust part of the project as compliant with the convention as we can. Thus,
understanding its structure should be straightforward.

## Why Wikipedia in Esperanto ?

The goal is to use the medium-sized Wikipedia in Esperanto to test the performances of the scripts before trying on
bigger Wikipedias such as the English and French ones.
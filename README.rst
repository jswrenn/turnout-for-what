================
turnout-for-what
================
.. image:: https://upload.wikimedia.org/wikipedia/commons/a/a2/V%C3%BDm%C4%9Bna_hrotit%C3%A1.jpg

Usage
-----
::

    tfw SWITCH FILES...

``SWITCH``
    A file descriptor yielding a stream of line-separated numbers. For every byte received from stdin, the last number received from SWITCH indicates the index of the FILE that the byte should be redirected to.

``FILES``
    One or more file paths.

Example
-------
A load-balancer for the natural numbers::

  #!/usr/bin/env bash
  i=0;
  echo "$i" > nat
  trap "rm nat *.txt" EXIT

  while true; do
    wc -l *.txt
    sleep 0.2
    clear
  done &

  while true; do
    sleep 0.2;
    i=$(expr $i + 1);
    echo "$i";
  done \
    | tee -a nat \
    | cargo run -- \
      <(while true; do
          echo $(expr "$(tail -n 1 nat)" % 5)
        done) \
      0.txt 1.txt 2.txt 3.txt 4.txt

This script will create five text files, balance the natural numbers 
between them, and continuously print their line counts.

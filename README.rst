================
turnout-for-what
================
.. image:: https://upload.wikimedia.org/wikipedia/commons/a/a2/V%C3%BDm%C4%9Bna_hrotit%C3%A1.jpg

Usage
-----
::

    tfw SWITCH

``SWITCH``
    A file descriptor yielding a stream of line-separated paths. For every line received from stdin, the last path received from SWITCH indicates the file that the line should be redirected to.

Example
-------
A load-balancer for the natural numbers::

    #!/usr/bin/env bash

    trap "rm -f nat {0..4}; pkill -P $$" EXIT

    touch nat {0..4}

    while true; do
      echo $((i++))
      sleep 0.2;
    done > nat &

    tail -f nat | cargo run -- <(tail -f nat | xargs -I{} expr {} % 5) &

    while true; do
      sleep 0.2
      clear
      wc -l {0..4}
    done

This script will create five text files, balance the natural numbers 
between them, and continuously print their line counts.

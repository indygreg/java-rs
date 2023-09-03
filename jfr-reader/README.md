# JFR Reader

This crate aims to implement a no compromises reader for Java Flight Recorder
(JFR) data files. By *no compromises* we mean:

* Is able to fully decode every piece of data in JFR files.
* Is able to do so safely - no panics / all errors can be caught.
* Is able to do so efficiently. No slower than the official JFR reader. Ideally
  faster. Both in terms of wall time and CPU cycles.
* Offers a flexible and useful API that allows consumers to access JFR
  data easily without having to fully understand the internals of the JFR
  format.

The JFR file format relies heavily on reusing references to common values and
data structures. This allows it to achieve high data compression rates: a
fully expanded JFR event often constitutes 100s of bytes but the on-disk
form is often ~20. Many JFR parsers eagerly expand these references. This
crate leans heavily into the *pay for what you use* mindset and lazily expands
referenced data, allowing it to achieve significant speedups versus other
JFR readers.

## Performance

If you already have JFR chunk data in memory, here are some example numbers:

* >3,000 MB/s to iterate raw event records. Just sizes and event IDs.
* ~1,800 MB/s to do the above and load the metadata. This gets you access
  to class names and allows you to do things like scan for specific named
  events or count occurrences of class by name.
* ~360 MB/s to do the above and load all referenced constants.
* ~155 MB/s to decode every event.

This is single threaded performance on a Ryzen 5950X. Chunks are standalone
entities and multiple chunks can be decoded in parallel on separate threads
achieving ~linear speedups.

Constants and event reading has not yet been optimized and will likely become
faster.

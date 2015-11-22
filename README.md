# sdc-rs

Read and write Riegl's `.sdc` file format.

[![Build Status](https://travis-ci.org/gadomski/sdc-rs.svg?branch=master)](https://travis-ci.org/gadomski/sdc-rs)

[Documentation](http://gadomski.github.io/sdc-rs).

This library comes with an executable, named `sdc`, that can perform some operations on `.sdc` files.


## What is an `.sdc` file?

`sdc` is a very simple binary format for storing point cloud data.
Its layout is defined in the documentation that comes with Riegl's `SDCImport` tool.
An `sdc` file has a very brief header that gives the format version and some optional arbitrary ASCII text, and then each point record follows.
The only way to get the total number of points in an `sdc` file is to read to the end.


## License

As much as possible, this code is available under the MIT license.
See `LICENSE.txt` in this repository for the complete license.
We don't use any Riegl libraries, just their published file format, but in case it's unclear their file format is there, and all that.


## Contributing

As always, [issues](https://github.com/gadomski/sdc-rs/issues) and [pull requests](https://github.com/gadomski/sdc-rs/pulls) are welcome.

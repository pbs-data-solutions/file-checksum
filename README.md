# File Checksum

[![Tests Status](https://github.com/pbs-data-solutions/file-checksum/workflows/Testing/badge.svg?branch=main&event=push)](https://github.com/pbs-data-solutions/file-checksum/actions?query=workflow%3ATesting+branch%3Amain+event%3Apush)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/pbs-data-solutions/file-checksum/main.svg)](https://results.pre-commit.ci/latest/github/pbs-data-solutions/file-checksum/main)
[![Coverage](https://codecov.io/github/pbs-data-solutions/file-checksum/coverage.svg?branch=main)](https://codecov.io/gh/pbs-data-solutions/file-checksum)
[![PyPI version](https://badge.fury.io/py/file-checksum.svg)](https://badge.fury.io/py/file-checksum)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/file-checksum?color=5cc141)](https://github.com/pbs-data-solutions/file-checksum)

Generate checksums for files.

CVM requires that submitted data files include a checksum to verify the file has not changed. This
program will generate those checksums for either a single file, or all the files in a directory.

## Installation

Installation with [pipx](https://github.com/pypa/pipx) is recommended.

```sh
pipx install file-checksum
```

Alternatively the program can be installed with pip.

```sh
pip install file-checksum
```

## Usage

### Generate

* Arguments
  * checksum_path: Path to the directory or file for which to generate checksums [required]
* Options
  * --output-file, -o: Path to the file to same the checksums. Must be a .txt file. If no path is
  provided the output will be printed to the screen and not saved. [default: None]
  * --checksum-type, -c: The type of checksum to generate. Supported types are md5, sha1, and sha256. [default: sha256]
  * --verbose: Provides more output while running

#### Example

If we have the files `my_file_1.xml` and `my_file_2.xml` in the `home/my_files` directory, checksums
can be generated for the files by running:

```sh
checksum /home/my_files -o /home/checksums.txt
```

The will create a file called `checksums.txt` in the `/home` directory containing the following
information (hashes made up for example purposes)

```console
sha256 checksum: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 - my_file_1.xml
sha256 checksum: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 - my_file_2.xml
```

Subsequent runs pointing to the same output file will append the results to the file so checksums
from different directores can be save to the same file.

## Contributing

Contributions to this project are welcome. If you are interesting in contributing please see our [contributing guide](CONTRIBUTING.md)

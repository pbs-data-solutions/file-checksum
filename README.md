# File Checksum

[![Tests Status](https://github.com/pbs-data-solutions/file-checksum/actions/workflows/testing.yaml/badge.svg?branch=main&event=push)](https://github.com/pbs-data-solutions/file-checksum/actions?query=workflow%3ATesting+branch%3Amain+event%3Apush)

Generate checksums for files.

This program will generate checksums for either a single file, or all the files in a directory.

## Installation

Install with cargo:

```sh
cargo install file-checksum
```

Install on Debian/Ubuntu:

```sh
curl -LO https://github.com/sanders41/python-project-generator/releases/download/v1.0.16/python-project-generator_1.0.16_amd64.deb
sudo dpkg -i python-project-generator_1.0.16_amd64.deb
```

file-checksum can also be installed with binaries provided with each release
[here](https://github.com/pbs-data-solutions/file-checksum/releases).

## Usage

### Generate

- Arguments:

  - <CHECKSUM_PATH> Path to the directory or file for which to generate checksums

- Options:
  - -o, --output-file <OUTPUT_FILE> Path to the file to same the checksums. Must be a .txt file. If
    no path is provided the output will be printed to the screen and not saved
  - -c, --checksum-type <CHECKSUM_TYPE> The type of checksum to generate. [default: sha256]
    [possible values: md5, sha1, sha256]
  - --overwrite Overwrite the output file rather than appending to it
  - -v, --verbose Provides more output while running
  - -h, --help Print help

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
from different directores can be save to the same file. Passing the `--overwrite` flag will clear
the contents of the file before writing instead of appending to the file.

## Contributing

Contributions to this project are welcome. If you are interesting in contributing please see our [contributing guide](CONTRIBUTING.md)

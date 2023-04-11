from __future__ import annotations

from enum import Enum
from hashlib import md5, sha1, sha256
from pathlib import Path
from typing import Union

from rich.console import Console
from rich.theme import Theme
from typer import Argument, Exit, Option, Typer

__version__ = "0.1.0"

app = Typer()

custom_style = Theme({"error": "red", "date": "green"})
console = Console(theme=custom_style)


class ChecksumType(str, Enum):
    MD5 = "md5"
    SHA1 = "sha1"
    SHA256 = "sha256"


class FileTypeError(Exception):
    pass


def _check_valid_output_file_type(file_name: Path) -> None:
    if file_name.suffix != ".txt":
        raise FileTypeError("Output file must be a .txt file")

    return None


def _process_checksum(
    file_path: Path,
    *,
    output_file: Path | None = None,
    checksum_type: ChecksumType = ChecksumType.SHA256,
    verbose: bool = False,
) -> None:
    if not output_file:
        verbose = True

    try:
        with open(file_path, "rb") as f:
            if checksum_type == ChecksumType.SHA256:
                file_hash = sha256(f.read())
            elif checksum_type == ChecksumType.MD5:
                file_hash = md5(f.read())
            else:
                file_hash = sha1(f.read())
    except IsADirectoryError:
        console.print(f"{file_path} is a directory and cannot be opened as a file", style="error")
        raise Exit(1)

    checksum_output = f"{checksum_type.value} checksum: {file_hash.hexdigest()} - {file_path.name}"

    if output_file:
        file_path = output_file.parent
        if not file_path.exists():
            file_path.mkdir(parents=True, exist_ok=True)

        with open(output_file, "a") as f:
            f.write(f"{checksum_output}\n")

    if verbose:
        console.print(checksum_output)


def _print_successful_generation_message(verbose: bool) -> None:
    if verbose:
        console.print("")  # Prints a blank line between verbose output and final message

    console.print("Checksums successfully generated")


@app.command()
def generate(
    checksum_path: Path = Argument(
        ..., help="Path to the directory or file for which to generate checksums", exists=True
    ),
    output_file: Path = Option(
        None,
        "--output-file",
        "-o",
        help="Path to the file to same the checksums. Must be a .txt file. If no path is provided the output will be printed to the screen and not saved.",
    ),
    checksum_type: ChecksumType = Option(
        ChecksumType.SHA256,
        "--checksum-type",
        "-c",
        help="The type of checksum to generate. [default: sha256]",
        show_default=False,
    ),
    verbose: bool = Option(False, "--verbose", "-v", help="Provides more output while running"),
) -> None:
    if output_file:
        try:
            _check_valid_output_file_type(output_file)
        except FileTypeError:
            console.print("Error: The output file must be a .txt file", style="error")
            raise Exit(1)
    else:
        verbose = True

    print(checksum_path)
    print(checksum_path.is_file())
    if checksum_path.is_file():
        _process_checksum(
            checksum_path, output_file=output_file, checksum_type=checksum_type, verbose=verbose
        )
        _print_successful_generation_message(verbose)
        raise Exit()

    with console.status("Generating checksums"):
        for file_path in checksum_path.iterdir():
            if file_path.is_file():
                _process_checksum(
                    file_path,
                    output_file=output_file,
                    checksum_type=checksum_type,
                    verbose=verbose,
                )

    _print_successful_generation_message(verbose)
    raise Exit()


@app.callback(invoke_without_command=True)
def main(
    version: Union[bool, None] = Option(
        None,
        "--version",
        "-v",
        is_eager=True,
        help="Show the installed version",
    ),
) -> None:
    if version:
        console.print(__version__)
        raise Exit()


if __name__ == "__main__":  # pragma: no cover
    raise SystemExit(app())

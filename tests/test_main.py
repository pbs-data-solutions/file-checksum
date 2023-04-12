from hashlib import md5, sha1, sha256
from pathlib import Path

import pytest
from typer import Exit
from typer.testing import CliRunner

from file_checksum.__main__ import (
    FileTypeError,
    __version__,
    _check_valid_output_file_type,
    _process_checksum,
    app,
)

try:
    import tomli as tomllib  # type: ignore
except ModuleNotFoundError:
    import tomllib  # type: ignore


def test_versions_match():
    pyproject_file = Path().absolute() / "pyproject.toml"
    with open(pyproject_file, "rb") as f:
        data = tomllib.load(f)
        pyproject_version = data["tool"]["poetry"]["version"]
    assert __version__ == pyproject_version


def get_checksum(file_name: Path, checksum_type: str):
    with open(file_name, "rb") as f:
        if checksum_type == "md5":
            return md5(f.read())

        if checksum_type == "sha1":
            return sha1(f.read())

        return sha256(f.read())


@pytest.fixture
def fake_file_path(tmp_path):
    file_path = tmp_path / "test.xml"

    with open(file_path, "w") as f:
        f.write('<?xml version="1.0" encoding="UTF-8"?><test>some test text</test>')

    return file_path


@pytest.fixture
def fake_file_path2(tmp_path):
    file_path = tmp_path / "test.xml"

    with open(file_path, "w") as f:
        f.write('<?xml version="1.0" encoding="UTF-8"?><test>some more test text</test>')

    return file_path


def test_check_valid_output_file_type():
    assert _check_valid_output_file_type(Path("test.txt")) is None  # type: ignore


def test_check_valid_output_file_type_error():
    with pytest.raises(FileTypeError):
        _check_valid_output_file_type(Path("test.xlsx"))


def test_process_checksum_directory_error(tmp_path):
    with pytest.raises(Exit):
        _process_checksum(tmp_path)


def test_process_checksum_exception():
    with pytest.raises(Exit):
        _process_checksum(Path("bad"))


@pytest.mark.parametrize("checksum_type", ["md5", "sha1", "sha256", None])
@pytest.mark.parametrize("checksum_type_flag", ["-c", "--checksum-type"])
@pytest.mark.parametrize("verbose", ["-v", "--verbose", None])
def test_generate_file(checksum_type, checksum_type_flag, verbose, fake_file_path, tmp_path):
    output_file = tmp_path / "output.txt"
    checksum = get_checksum(fake_file_path, checksum_type)

    args = ["generate", str(fake_file_path), "-o", str(output_file)]

    if checksum_type_flag:
        args.append(checksum_type_flag)
        args.append(checksum_type)

    if verbose:
        args.append(verbose)

    runner = CliRunner()
    runner.invoke(app, args, catch_exceptions=False)

    with open(output_file, "r") as f:
        result = f.read()
        assert checksum.hexdigest() in result


def test_generate_file_creates_directory(fake_file_path, tmp_path):
    output_file = tmp_path / "new" / "output.txt"
    checksum = get_checksum(fake_file_path, "sha356")

    args = ["generate", str(fake_file_path), "-o", str(output_file)]

    runner = CliRunner()
    runner.invoke(app, args, catch_exceptions=False)

    with open(output_file, "r") as f:
        result = f.read()
        assert checksum.hexdigest() in result


def test_generate_file_bad_output_file_type(fake_file_path, tmp_path):
    output_file = tmp_path / "output.csv"

    args = ["generate", str(fake_file_path), "-o", str(output_file)]

    runner = CliRunner()
    result = runner.invoke(app, args, catch_exceptions=False)
    out = result.stdout

    assert "must be a .txt file" in out


def test_generate_file_no_output_file(fake_file_path):
    checksum = get_checksum(fake_file_path, "sha256")

    args = ["generate", str(fake_file_path)]

    runner = CliRunner()
    result = runner.invoke(app, args, catch_exceptions=False)
    out = result.stdout

    assert checksum.hexdigest() in out


@pytest.mark.parametrize("checksum_type", ["md5", "sha1", "sha256", None])
@pytest.mark.parametrize("checksum_type_flag", ["-c", "--checksum-type"])
@pytest.mark.parametrize("verbose", ["-v", "--verbose", None])
def test_generate_directory(checksum_type, checksum_type_flag, verbose, tmp_path):
    fake_dir = tmp_path / "fake"
    fake_dir.mkdir()

    fake_file_path = fake_dir / "test.xml"
    with open(fake_file_path, "w") as f:
        f.write('<?xml version="1.0" encoding="UTF-8"?><test>some test text</test>')

    fake_file_path2 = fake_dir / "test2.xml"
    with open(fake_file_path2, "w") as f:
        f.write('<?xml version="1.0" encoding="UTF-8"?><test>some more test text</test>')

    output_file = tmp_path / "output.txt"
    checksum_1 = get_checksum(fake_file_path, checksum_type)
    checksum_2 = get_checksum(fake_file_path2, checksum_type)

    args = ["generate", str(fake_dir), "-o", str(output_file)]

    if checksum_type_flag:
        args.append(checksum_type_flag)
        args.append(checksum_type)

    if verbose:
        args.append(verbose)

    runner = CliRunner()
    runner.invoke(app, args, catch_exceptions=False)

    with open(output_file, "r") as f:
        result = f.read()
        assert checksum_1.hexdigest() in result
        assert checksum_2.hexdigest() in result


@pytest.mark.parametrize("args", [["--version"], ["-v"]])
def test_version(args):
    runner = CliRunner()
    result = runner.invoke(app, args, catch_exceptions=False)
    out = result.stdout
    assert __version__ in out

import pytest
from src.main import read_csv_first_row


def test_read_csv_first_row():
    csv_file_path = "pythons/data/dummy_csv_file.csv"
    expected = "first_name,last_name,age\n"
    actual = read_csv_first_row(csv_file_path)
    print(f"actual: {actual}")
    assert actual == expected

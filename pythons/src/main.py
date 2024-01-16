import agni


def read_csv_first_row(csv_file_path) -> str:
    first_row = agni.read.csv(csv_file_path).first()
    return first_row


def main():
    result = agni.sum_as_string(99, 199)
    print(f"sum_as_string result: {result}")

    print("reading first row of csv file")
    first_row = read_csv_first_row("pythons/data/dummy_csv_file.csv")
    print(f"first row: {first_row}")


if __name__ == "__main__":
    main()

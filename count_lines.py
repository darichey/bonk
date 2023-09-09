import os

def count_lines_in_txt_recursive(directory):
    total_lines = 0

    for root, dirs, files in os.walk(directory):
        for filename in files:
            if filename.endswith(".csv"):
                with open(os.path.join(root, filename), 'r', encoding='utf-8') as file:
                    lines = len(file.readlines())
                    total_lines += lines

    return total_lines

# Replace 'path_to_your_directory' with the actual path to your directory containing the CSV files.
directory = 'data'

total_lines = count_lines_in_txt_recursive(directory)
print(f"Total number of lines in all CSV files treated as text files: {total_lines}")
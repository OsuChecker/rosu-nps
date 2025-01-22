# nps

## Project Description

**nps** is a library that calculates and returns a JSON file with the number of notes per second (**Notes Per Second**, abbreviated as NPS) on sequences based on a given frequency.

The project analyzes `.osu` files (used in the rhythm game [Osu!](https://osu.ppy.sh/)) to determine the density of notes in defined time intervals, making it useful for analytics or tools for players and developers.

Originally, this project was developed in **C++**, but I decided to reimplement it in **Rust** to leverage its memory safety guarantees and integrate it more easily in a **Python** environment (via PyO3). This makes it possible to use the library in a Flask server.

---

## Key Features

- **Download and read `.osu` files** from a provided URL.
- **Analysis**: Extract all "hit objects" (notes) from `.osu` files.
- **Calculate note density per second** across defined time intervals.
- **JSON Output**: The calculations are returned in JSON format for seamless integration into external applications.

---

## Documentation

For more detailed technical instructions, visit the documentation directly on the website in the **doc** section.

---

## Installation and Compilation

### Prerequisites:

- Python 3 installed.
- The **maturin** package:  
  Install it using the following command:
  ```bash
  pip install maturin
  ```

### Steps to Compile and Use the Project:

1. Create a Python virtual environment:
   ```bash
   python3 -m venv venv
   ```

2. Activate the virtual environment:
   ```bash
   source venv/bin/activate
   ```

3. Compile and install the Rust module as a Python extension using **maturin**:
   ```bash
   maturin develop
   ```

### Using the Library in Python:

After compilation, you can import and use the library as follows:
```python
import nps

# Usage example
url = "http://example.com/sample.osu"
frequency = 1.0  # Frequency in intervals per second.

try:
    result = nps.get_nps(url, frequency)
    print(result)
except ValueError as e:
    print(f"Error: {e}")
```

---

## Author

Developed by **Osef0760**  
**Contact**: osefcode@gmail.com
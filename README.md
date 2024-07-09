# Fuzzypicker
Fuzzypicker is a Rust library for enabling fuzzy searching and interactive selection of items in command-line applications. It is designed to assist in building CLI tools where users need to select an item from a list based on fuzzy matching criteria.

## Features
- Fuzzy searching of items in a list based on user input.
- Interactive selection with keyboard and mouse support.
- Seamless integration into Rust-based command-line applications.

## Installation
Add fuzzypicker to your Cargo.toml dependencies:
```bash
cargo add fuzzypicker
```
## Usage
Here's a basic example demonstrating how to use fuzzypicker to implement a fuzzy selection mechanism in a Rust CLI application:

```rust
use fuzzypicker::FuzzyPicker;

fn main() {
    // Example list of items (could be anything implementing Display + Clone)
    let items = vec![
        "apple", "banana", "cherry", "date", "elderberry", "fig", "grape", "honeydew",
    ];

    // Create a new FuzzyPicker instance
    let mut picker = FuzzyPicker::new();
    picker.set_items(&items);

    // Perform interactive selection
    if let Ok(Some(selected_item)) = picker.pick() {
        println!("Selected item: {}", selected_item);
    } else {
        println!("Selection cancelled or no item selected.");
    }
    
    // The picker can be reset to set new items
    picker.reset();
    assert_eq!(picker.num_of_items, 0);
}
```
## API
`struct FuzzyPicker<T: Display + Clone>`
#### Methods
- `new() -> Self`: Constructs a new `FuzzyPicker` instance.
- `set_items(items: &[T])`: Initializes the picker with a slice of items implementing `Display + Clone`.
- `reset()`: Resets the picker.
- `pick() -> Result<Option<T>, Box<dyn Error>>`: Initiates the interactive selection process. Returns Some(selected_item) if an item is selected, or None if selection is cancelled.

## Contributing
Contributions are welcome! If you'd like to contribute to `fuzzypicker`, please fork the repository and submit a pull request. For major changes, please open an issue first to discuss what you would like to change.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments
- This library uses crossterm for terminal handling and input.
- Fuzzy matching is powered by fuzzy-matcher.

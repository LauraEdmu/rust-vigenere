# Vigenère Cipher Program

This is a simple Vigenère cipher program written in Rust that allows the user to encrypt or decrypt text using a provided key.

## Functions

- **`get_cleartext`**: Prompts the user to enter the text to encrypt and returns it as a lowercase, trimmed `String`.
- **`get_ciphertext`**: Prompts the user to enter the text to decrypt and returns it as a lowercase, trimmed `String`.
- **`encrypt`**: Takes a cleartext and a key, and returns the encrypted text using the Vigenère cipher algorithm.
- **`decrypt`**: Takes a ciphertext and a key, and returns the decrypted text using the Vigenère cipher algorithm.
- **`get_key`**: Prompts the user to enter the key and returns it as a lowercase, trimmed `String`.
- **`main`**: The main function that allows the user to choose between encryption and decryption, and displays the result.

## Usage

1. Run the program.
2. Follow the prompts to either encrypt or decrypt a message using the Vigenère cipher.
3. The result will be displayed and saved to a file named `output.txt`.

## Example

To encrypt a message:
```
Enter 1 to encrypt, 2 to decrypt: 1
Enter the text to encrypt: hello
Enter the key: key
Result: riijm
```

To decrypt a message:
```
Enter 1 to encrypt, 2 to decrypt: 2
Enter the text to decrypt: riijm
Enter the key: key
Result: hello
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## Contact

For any questions or feedback, please contact the project maintainer via github or email:  [laura.e.git@pm.me](mailto:laura.e.git@pm.me)
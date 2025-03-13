use xor_cryptor::XORCryptor;

fn main() {
    // Create the key as a String.
    let key = String::from("CSUCKS");
    
    // List of hex strings to convert to bytes.
    let hex_values = ["41", "30", "20", "63", "4a", "45", "54", "76", "01", "1c", "7e", "59", "63", "e1", "61", "25", "7f", "5a", "60", "50", "11", "38", "1f", "3a", "60", "e9", "62", "20", "0c", "e6", "50", "d3", "35"];

    
    // Convert hex values into a buffer of bytes.
    let mut decrypted_buffer: Vec<u8> = Vec::new();
    for hex in hex_values.iter() {
        let byte = u8::from_str_radix(hex, 16).unwrap();
        decrypted_buffer.push(byte);
    }
    
    // Create the XORCryptor by passing a reference to the key.
    let cryptor = XORCryptor::new(&key).unwrap();
    
    // Pass the buffer by value (not as a reference) since decrypt_vec expects Vec<u8>.
    let res = cryptor.decrypt_vec(decrypted_buffer);
    
    // Print the decrypted result.
    println!("{}", String::from_utf8_lossy(&res));

    println!("Decrypted length: {}", res.len());
    println!("{}", String::from_utf8_lossy(&res));
}

use crypto::{symmetriccipher, buffer, aes, blockmodes, aes_gcm};
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use crypto::aead::{AeadEncryptor, AeadDecryptor};
use crypto::symmetriccipher::Encryptor;


// Const
const TAG_LEN: usize = 16;

static AAD: [u8; 16] = [0; 16];


// Encrypt a buffer with the given key and iv using
// AES-128/CBC/Pkcs encryption.
pub fn encrypt_aescbc128(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {

    // Create an encryptor instance of the best performing
    // type available for the platform.
    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize128,
        key,
        iv,
        blockmodes::PkcsPadding);

    // Each encryption operation encrypts some data from
    // an input buffer into an output buffer. Those buffers
    // must be instances of RefReaderBuffer and RefWriteBuffer
    // (respectively) which keep track of how much data has been
    // read from or written to them.
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    // Each encryption operation will "make progress". "Making progress"
    // is a bit loosely defined, but basically, at the end of each operation
    // either BufferUnderflow or BufferOverflow will be returned (unless
    // there was an error). If the return value is BufferUnderflow, it means
    // that the operation ended while wanting more input data. If the return
    // value is BufferOverflow, it means that the operation ended because it
    // needed more space to output data. As long as the next call to the encryption
    // operation provides the space that was requested (either more input data
    // or more output space), the operation is guaranteed to get closer to
    // completing the full operation - ie: "make progress".
    //
    // Here, we pass the data to encrypt to the enryptor along with a fixed-size
    // output buffer. The 'true' flag indicates that the end of the data that
    // is to be encrypted is included in the input buffer (which is true, since
    // the input data includes all the data to encrypt). After each call, we copy
    // any output data to our result Vec. If we get a BufferOverflow, we keep
    // going in the loop since it means that there is more work to do. We can
    // complete as soon as we get a BufferUnderflow since the encryptor is telling
    // us that it stopped processing data due to not having any more data in the
    // input buffer.
    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;

        // "write_buffer.take_read_buffer().take_remaining()" means:
        // from the writable buffer, create a new readable buffer which
        // contains all data that has been written, and then access all
        // of that data as a slice.
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

// Decrypts a buffer with the given key and iv using
// AES-128/CBC/Pkcs encryption.
//
// This function is very similar to encrypt(), so, please reference
// comments in that function. In non-example code, if desired, it is possible to
// share much of the implementation using closures to hide the operation
// being performed. However, such code would make this example less clear.
pub fn decrypt_aescbc128(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize128,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}


// Encrypt a buffer with the given key and iv using
// AES-128/CBC/Pkcs encryption.
pub fn encrypt_aesctr128(input: &[u8], output: &mut [u8], key: &[u8], iv: &[u8]) -> Result<usize, symmetriccipher::SymmetricCipherError> {

    // Create an encryptor instance of the best performing
    // type available for the platform.
    let mut encryptor = aes::ctr(
        aes::KeySize::KeySize128,
        key,
        iv);

    let input_len = input.len();
    encryptor.encrypt(input, &mut output[..input_len], &mut tag[..]);
    output[input_len..].copy_from_slice(&tag.to_vec());
    Ok(input_len + TAG_LEN)
}

// Decrypts a buffer with the given key and iv using
// AES-128/CTR.
//
// This function is very similar to encrypt(), so, please reference
// comments in that function. In non-example code, if desired, it is possible to
// share much of the implementation using closures to hide the operation
// being performed. However, such code would make this example less clear.
pub fn decrypt_aesctr128(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize128,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}


// Encrypt a buffer with the given key and iv using
// AES-128/GCM.
// The output memory needs to be assigned in advance. Usually the size is the length of plaintext + 16(Tag length).
pub fn encrypt_aesgcm(input: &[u8], output: &mut [u8], key: &[u8], iv: &[u8]) -> Result<usize, symmetriccipher::SymmetricCipherError> {
    let mut tag: [u8; TAG_LEN] = [0; TAG_LEN];
    // Create an encryptor instance of the best performing
    // type available for the platform.
    let mut encryptor = aes_gcm::AesGcm::new(
        aes::KeySize::KeySize128,
        key,
        iv,
        &AAD[..]);

    let input_len = input.len();
    encryptor.encrypt(input, &mut output[..input_len], &mut tag[..]);
    output[input_len..].copy_from_slice(&tag.to_vec());
    Ok(input_len + TAG_LEN)
}

// Decrypts a buffer with the given key and iv using
// AES-128/GCM.
// The output memory needs to be assigned in advance. Usually the size is the length of ciphertext - 16(Tag length).
pub fn decrypt_aesgcm(input: &[u8], output: &mut [u8], key: &[u8], iv: &[u8]) -> Result<usize, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes_gcm::AesGcm::new(
        aes::KeySize::KeySize128,
        key,
        iv,
        &AAD[..]);
    let data_len = input.len() - 16;
    let result = decryptor.decrypt(&input[..data_len], &mut output[..], &input[data_len..]);
    if !result {
        println!("Something wrong with the decryption.");
        return Err(symmetriccipher::SymmetricCipherError::InvalidPadding);
    }
    Ok(data_len)
}

#[cfg(test)]
fn gcm_test() {
    use rand::RngCore;
    use rand::rngs::OsRng;
    let message = "This is another test.";
    println!("{}", message);

    let mut key: [u8; 16] = [0; 16];
    let mut iv: [u8; 12] = [0; 12];

    // In a real program, the key and iv may be determined
    // using some other mechanism. If a password is to be used
    // as a key, an algorithm like PBKDF2, Bcrypt, or Scrypt (all
    // supported by Rust-Crypto!) would be a good choice to derive
    // a password. For the purposes of this example, the key and
    // iv are just random values.
    // let mut rng = OsRng::new().ok().unwrap();
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut iv);


    let mut encrypted_data = vec![0; message.len() + 16];
    let mut result = encrypt_aesgcm(message.as_bytes(), &mut encrypted_data, &key, &iv).ok().unwrap_or(0);
    println!("cipher text: {:?}", encrypted_data);

    // encrypted_data[0] = 0u8;

    let mut decrypted_data = vec![0; message.len()];
    result = decrypt_aesgcm(&encrypted_data[..], &mut decrypted_data, &key, &iv).ok().unwrap_or(0);
    let s = match String::from_utf8(decrypted_data) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("decrpyted text: {:}", s);
}
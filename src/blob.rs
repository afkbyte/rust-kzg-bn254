use crate::{errors::BlobError, helpers, polynomial::Polynomial};

/// A blob which is Eigen DA spec aligned.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blob {
    blob_data: Vec<u8>,
    is_padded: bool,
    length_after_padding: usize,
}

impl Blob {
    /// Creates a new `Blob` from the given data.
    pub fn new(blob_data: Vec<u8>, is_padded: bool) -> Self {
        let length_after_padding;
        if is_padded {
            length_after_padding = blob_data.len();
        }
        else {
            length_after_padding = 0;
        }
        Blob { blob_data, is_padded, length_after_padding }
    }

    /// Creates a new `Blob` from the given data.
    pub fn is_padded(&self) -> bool {
        return self.is_padded;
    }

    /// Creates a new `Blob` from the provided byte slice and pads it according
    /// to DA specs.
    pub fn from_bytes_and_pad(input: &[u8]) -> Self {
        let padded_input = helpers::convert_by_padding_empty_byte(input);
        let length_after_padding = padded_input.len();
        Blob {
            blob_data: padded_input,
            is_padded: true,
            length_after_padding,
        }
    }

    /// Returns the blob data
    pub fn get_blob_data(&self) -> Vec<u8> {
        self.blob_data.clone()
    }

    /// Returns the length of the data in the blob.
    pub fn len(&self) -> usize {
        self.blob_data.len()
    }

    /// Pads the blob data in-place if it is not already padded.
    pub fn pad_data(&mut self) -> Result<(), BlobError> {
        if self.is_padded {
            Err(BlobError::AlreadyPaddedError)
        } else {
            self.blob_data = helpers::convert_by_padding_empty_byte(&self.blob_data);
            self.is_padded = true;
            self.length_after_padding = self.blob_data.len();
            Ok(())
        }
    }

    /// Removes padding from the blob data if it is padded.
    pub fn remove_padding(&mut self) -> Result<(), BlobError> {
        if !self.is_padded {
            Err(BlobError::NotPaddedError)
        } else {
            self.blob_data = helpers::remove_empty_byte_from_padded_bytes(&self.blob_data);
            self.is_padded = false;
            self.length_after_padding = 0;
            Ok(())
        }
    }

    /// Converts the blob data to a `Polynomial` if the data is padded.
    pub fn to_polynomial(&self) -> Result<Polynomial, BlobError> {
        if !self.is_padded {
            Err(BlobError::NotPaddedError)
        } else {
            let fr_vec = helpers::to_fr_array(&self.blob_data);
            let poly = Polynomial::new(&fr_vec, self.length_after_padding)
                .map_err(|err| BlobError::GenericError(err.to_string()))?;
            Ok(poly)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_already_padded() {
        let mut blob = Blob::from_bytes_and_pad("hi".as_bytes());
        let mut result = blob.pad_data();
        assert_eq!(result, Err(BlobError::AlreadyPaddedError));
        blob.remove_padding().unwrap();
        result = blob.remove_padding();
        assert_eq!(result, Err(BlobError::NotPaddedError));
        assert_eq!(blob.to_polynomial(), Err(BlobError::NotPaddedError));
    }

    #[test]
    fn test_convert_by_padding_empty_byte() {
        use crate::consts::GETTYSBURG_ADDRESS_BYTES;

        let mut blob = Blob::from_bytes_and_pad("hi".as_bytes());
        assert_eq!(
            blob.get_blob_data(),
            vec![0, 104, 105],
            "testing adding padding"
        );
        assert_eq!(blob.is_padded(), true, "has to be padded");

        blob.remove_padding().unwrap();
        assert_eq!(
            blob.get_blob_data(),
            vec![104, 105],
            "testing removing padding"
        );
        assert_eq!(blob.is_padded(), false, "cannot be padded");

        let result: Vec<u8> = vec![
            0, 70, 111, 117, 114, 115, 99, 111, 114, 101, 32, 97, 110, 100, 32, 115, 101, 118, 101,
            110, 32, 121, 101, 97, 114, 115, 32, 97, 103, 111, 32, 111, 0, 117, 114, 32, 102, 97,
            116, 104, 101, 114, 115, 32, 98, 114, 111, 117, 103, 104, 116, 32, 102, 111, 114, 116,
            104, 44, 32, 111, 110, 32, 116, 104, 0, 105, 115, 32, 99, 111, 110, 116, 105, 110, 101,
            110, 116, 44, 32, 97, 32, 110, 101, 119, 32, 110, 97, 116, 105, 111, 110, 44, 32, 99,
            111, 110, 0, 99, 101, 105, 118, 101, 100, 32, 105, 110, 32, 108, 105, 98, 101, 114,
            116, 121, 44, 32, 97, 110, 100, 32, 100, 101, 100, 105, 99, 97, 116, 101, 0, 100, 32,
            116, 111, 32, 116, 104, 101, 32, 112, 114, 111, 112, 111, 115, 105, 116, 105, 111, 110,
            32, 116, 104, 97, 116, 32, 97, 108, 108, 32, 109, 0, 101, 110, 32, 97, 114, 101, 32,
            99, 114, 101, 97, 116, 101, 100, 32, 101, 113, 117, 97, 108, 46, 32, 78, 111, 119, 32,
            119, 101, 32, 97, 114, 0, 101, 32, 101, 110, 103, 97, 103, 101, 100, 32, 105, 110, 32,
            97, 32, 103, 114, 101, 97, 116, 32, 99, 105, 118, 105, 108, 32, 119, 97, 114, 44, 0,
            32, 116, 101, 115, 116, 105, 110, 103, 32, 119, 104, 101, 116, 104, 101, 114, 32, 116,
            104, 97, 116, 32, 110, 97, 116, 105, 111, 110, 44, 32, 111, 0, 114, 32, 97, 110, 121,
            32, 110, 97, 116, 105, 111, 110, 32, 115, 111, 32, 99, 111, 110, 99, 101, 105, 118,
            101, 100, 44, 32, 97, 110, 100, 32, 0, 115, 111, 32, 100, 101, 100, 105, 99, 97, 116,
            101, 100, 44, 32, 99, 97, 110, 32, 108, 111, 110, 103, 32, 101, 110, 100, 117, 114,
            101, 46, 32, 0, 87, 101, 32, 97, 114, 101, 32, 109, 101, 116, 32, 111, 110, 32, 97, 32,
            103, 114, 101, 97, 116, 32, 98, 97, 116, 116, 108, 101, 45, 102, 105, 0, 101, 108, 100,
            32, 111, 102, 32, 116, 104, 97, 116, 32, 119, 97, 114, 46, 32, 87, 101, 32, 104, 97,
            118, 101, 32, 99, 111, 109, 101, 32, 116, 0, 111, 32, 100, 101, 100, 105, 99, 97, 116,
            101, 32, 97, 32, 112, 111, 114, 116, 105, 111, 110, 32, 111, 102, 32, 116, 104, 97,
            116, 32, 102, 105, 0, 101, 108, 100, 44, 32, 97, 115, 32, 97, 32, 102, 105, 110, 97,
            108, 32, 114, 101, 115, 116, 105, 110, 103, 45, 112, 108, 97, 99, 101, 32, 102, 0, 111,
            114, 32, 116, 104, 111, 115, 101, 32, 119, 104, 111, 32, 104, 101, 114, 101, 32, 103,
            97, 118, 101, 32, 116, 104, 101, 105, 114, 32, 108, 105, 0, 118, 101, 115, 44, 32, 116,
            104, 97, 116, 32, 116, 104, 97, 116, 32, 110, 97, 116, 105, 111, 110, 32, 109, 105,
            103, 104, 116, 32, 108, 105, 118, 0, 101, 46, 32, 73, 116, 32, 105, 115, 32, 97, 108,
            116, 111, 103, 101, 116, 104, 101, 114, 32, 102, 105, 116, 116, 105, 110, 103, 32, 97,
            110, 100, 0, 32, 112, 114, 111, 112, 101, 114, 32, 116, 104, 97, 116, 32, 119, 101, 32,
            115, 104, 111, 117, 108, 100, 32, 100, 111, 32, 116, 104, 105, 115, 46, 0, 32, 66, 117,
            116, 44, 32, 105, 110, 32, 97, 32, 108, 97, 114, 103, 101, 114, 32, 115, 101, 110, 115,
            101, 44, 32, 119, 101, 32, 99, 97, 110, 0, 110, 111, 116, 32, 100, 101, 100, 105, 99,
            97, 116, 101, 44, 32, 119, 101, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 115,
            101, 99, 114, 0, 97, 116, 101, 226, 128, 148, 119, 101, 32, 99, 97, 110, 110, 111, 116,
            32, 104, 97, 108, 108, 111, 119, 226, 128, 148, 116, 104, 105, 115, 32, 103, 0, 114,
            111, 117, 110, 100, 46, 32, 84, 104, 101, 32, 98, 114, 97, 118, 101, 32, 109, 101, 110,
            44, 32, 108, 105, 118, 105, 110, 103, 32, 97, 110, 0, 100, 32, 100, 101, 97, 100, 44,
            32, 119, 104, 111, 32, 115, 116, 114, 117, 103, 103, 108, 101, 100, 32, 104, 101, 114,
            101, 44, 32, 104, 97, 118, 0, 101, 32, 99, 111, 110, 115, 101, 99, 114, 97, 116, 101,
            100, 32, 105, 116, 32, 102, 97, 114, 32, 97, 98, 111, 118, 101, 32, 111, 117, 114, 32,
            0, 112, 111, 111, 114, 32, 112, 111, 119, 101, 114, 32, 116, 111, 32, 97, 100, 100, 32,
            111, 114, 32, 100, 101, 116, 114, 97, 99, 116, 46, 32, 84, 0, 104, 101, 32, 119, 111,
            114, 108, 100, 32, 119, 105, 108, 108, 32, 108, 105, 116, 116, 108, 101, 32, 110, 111,
            116, 101, 44, 32, 110, 111, 114, 32, 0, 108, 111, 110, 103, 32, 114, 101, 109, 101,
            109, 98, 101, 114, 32, 119, 104, 97, 116, 32, 119, 101, 32, 115, 97, 121, 32, 104, 101,
            114, 101, 44, 0, 32, 98, 117, 116, 32, 105, 116, 32, 99, 97, 110, 32, 110, 101, 118,
            101, 114, 32, 102, 111, 114, 103, 101, 116, 32, 119, 104, 97, 116, 32, 116, 0, 104,
            101, 121, 32, 100, 105, 100, 32, 104, 101, 114, 101, 46, 32, 73, 116, 32, 105, 115, 32,
            102, 111, 114, 32, 117, 115, 32, 116, 104, 101, 32, 0, 108, 105, 118, 105, 110, 103,
            44, 32, 114, 97, 116, 104, 101, 114, 44, 32, 116, 111, 32, 98, 101, 32, 100, 101, 100,
            105, 99, 97, 116, 101, 100, 0, 32, 104, 101, 114, 101, 32, 116, 111, 32, 116, 104, 101,
            32, 117, 110, 102, 105, 110, 105, 115, 104, 101, 100, 32, 119, 111, 114, 107, 32, 119,
            104, 0, 105, 99, 104, 32, 116, 104, 101, 121, 32, 119, 104, 111, 32, 102, 111, 117,
            103, 104, 116, 32, 104, 101, 114, 101, 32, 104, 97, 118, 101, 32, 116, 0, 104, 117,
            115, 32, 102, 97, 114, 32, 115, 111, 32, 110, 111, 98, 108, 121, 32, 97, 100, 118, 97,
            110, 99, 101, 100, 46, 32, 73, 116, 32, 105, 0, 115, 32, 114, 97, 116, 104, 101, 114,
            32, 102, 111, 114, 32, 117, 115, 32, 116, 111, 32, 98, 101, 32, 104, 101, 114, 101, 32,
            100, 101, 100, 105, 0, 99, 97, 116, 101, 100, 32, 116, 111, 32, 116, 104, 101, 32, 103,
            114, 101, 97, 116, 32, 116, 97, 115, 107, 32, 114, 101, 109, 97, 105, 110, 105, 0, 110,
            103, 32, 98, 101, 102, 111, 114, 101, 32, 117, 115, 226, 128, 148, 116, 104, 97, 116,
            32, 102, 114, 111, 109, 32, 116, 104, 101, 115, 101, 32, 0, 104, 111, 110, 111, 114,
            101, 100, 32, 100, 101, 97, 100, 32, 119, 101, 32, 116, 97, 107, 101, 32, 105, 110, 99,
            114, 101, 97, 115, 101, 100, 32, 0, 100, 101, 118, 111, 116, 105, 111, 110, 32, 116,
            111, 32, 116, 104, 97, 116, 32, 99, 97, 117, 115, 101, 32, 102, 111, 114, 32, 119, 104,
            105, 99, 0, 104, 32, 116, 104, 101, 121, 32, 104, 101, 114, 101, 32, 103, 97, 118, 101,
            32, 116, 104, 101, 32, 108, 97, 115, 116, 32, 102, 117, 108, 108, 32, 0, 109, 101, 97,
            115, 117, 114, 101, 32, 111, 102, 32, 100, 101, 118, 111, 116, 105, 111, 110, 226, 128,
            148, 116, 104, 97, 116, 32, 119, 101, 32, 104, 0, 101, 114, 101, 32, 104, 105, 103,
            104, 108, 121, 32, 114, 101, 115, 111, 108, 118, 101, 32, 116, 104, 97, 116, 32, 116,
            104, 101, 115, 101, 32, 100, 0, 101, 97, 100, 32, 115, 104, 97, 108, 108, 32, 110, 111,
            116, 32, 104, 97, 118, 101, 32, 100, 105, 101, 100, 32, 105, 110, 32, 118, 97, 105,
            110, 0, 226, 128, 148, 116, 104, 97, 116, 32, 116, 104, 105, 115, 32, 110, 97, 116,
            105, 111, 110, 44, 32, 117, 110, 100, 101, 114, 32, 71, 111, 100, 44, 0, 32, 115, 104,
            97, 108, 108, 32, 104, 97, 118, 101, 32, 97, 32, 110, 101, 119, 32, 98, 105, 114, 116,
            104, 32, 111, 102, 32, 102, 114, 101, 101, 0, 100, 111, 109, 44, 32, 97, 110, 100, 32,
            116, 104, 97, 116, 32, 103, 111, 118, 101, 114, 110, 109, 101, 110, 116, 32, 111, 102,
            32, 116, 104, 101, 0, 32, 112, 101, 111, 112, 108, 101, 44, 32, 98, 121, 32, 116, 104,
            101, 32, 112, 101, 111, 112, 108, 101, 44, 32, 102, 111, 114, 32, 116, 104, 101, 0, 32,
            112, 101, 111, 112, 108, 101, 44, 32, 115, 104, 97, 108, 108, 32, 110, 111, 116, 32,
            112, 101, 114, 105, 115, 104, 32, 102, 114, 111, 109, 32, 0, 116, 104, 101, 32, 101,
            97, 114, 116, 104, 46,
        ];

        blob = Blob::from_bytes_and_pad(GETTYSBURG_ADDRESS_BYTES);
        assert_eq!(blob.get_blob_data(), result, "testing adding padding");
        assert_eq!(blob.is_padded(), true, "has to be padded");
        assert_eq!(blob.get_length_after_padding(), 1515);

        blob.remove_padding().unwrap();
        assert_eq!(blob.is_padded(), false, "cannot be padded");
        assert_eq!(
            blob.get_blob_data(),
            GETTYSBURG_ADDRESS_BYTES,
            "testing removing padding"
        );
    }

    #[test]
    fn test_new_blob_creation() {
        use crate::consts::GETTYSBURG_ADDRESS_BYTES;
        let blob_from = Blob::from_bytes_and_pad(GETTYSBURG_ADDRESS_BYTES);
        let mut blob_raw = Blob::new(GETTYSBURG_ADDRESS_BYTES.to_vec(), false);

        blob_raw.pad_data().unwrap();
        assert_eq!(blob_raw, blob_from, "testing adding padding");
        assert_eq!(blob_raw.is_padded(), true, "has to be padded");
        assert_eq!(blob_from.is_padded(), true, "has to be padded");
    }

}


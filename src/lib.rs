use std::ops::RangeInclusive;

pub use decoding::*;
pub use encoding::*;
pub use errors::*;

mod constants;
mod decoding;
mod encoding;
mod errors;

static CHAR_RANGE_NUMBERS: RangeInclusive<u8> = b'0'..=b'9';
static CHAR_RANGE_LOWERCASE: RangeInclusive<u8> = b'a'..=b'z';
static CHAR_RANGE_UPPERCASE: RangeInclusive<u8> = b'A'..=b'Z';

/// Verifies `content` is a valid G60 encoded string.
pub fn verify(content: &str) -> Result<(), VerificationError> {
    let bytes = content.as_bytes();

    // Check length.
    let remaining_bytes = bytes.len() - bytes.len() / 11 * 11;
    if let 1 | 4 | 8 = remaining_bytes {
        return Err(VerificationError::InvalidLength);
    }

    // Check chars.
    for (index, c) in bytes.iter().enumerate() {
        if CHAR_RANGE_UPPERCASE.contains(c) {
            if let b'O' | b'I' = *c {
                return Err(VerificationError::InvalidByte { index, byte: *c });
            }
        } else if !CHAR_RANGE_NUMBERS.contains(c) && !CHAR_RANGE_LOWERCASE.contains(c) {
            return Err(VerificationError::InvalidByte { index, byte: *c });
        }
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// TESTS ----------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_ok() {
        let test = "0123456789ABCDEFGH";
        assert!(verify(test).is_ok(), "Incorrect for '{}'", test);

        // --------------------------------------------------------------------

        let test = "JKLMNPQRSTUVWXYZab";
        assert!(verify(test).is_ok(), "Incorrect for '{}'", test);

        // --------------------------------------------------------------------

        let test = "cdefghijklmnopqrst";
        assert!(verify(test).is_ok(), "Incorrect for '{}'", test);

        // --------------------------------------------------------------------

        let test = "uvwxyz0123456789AB";
        assert!(verify(test).is_ok(), "Incorrect for '{}'", test);
    }

    #[test]
    fn test_verify_invalid_length() {
        let test = "JKLMNPQRSTUx";
        let error = verify(test).expect_err("The verification must fail");

        assert_eq!(
            error,
            VerificationError::InvalidLength,
            "Incorrect for '{}'",
            test
        );

        // --------------------------------------------------------------------

        let test = "JKLMNPQRSTUxxxx";
        let error = verify(test).expect_err("The verification must fail");

        assert_eq!(
            error,
            VerificationError::InvalidLength,
            "Incorrect for '{}'",
            test
        );

        // --------------------------------------------------------------------

        let test = "JKLMNPQRSTUxxxxxxxx";
        let error = verify(test).expect_err("The verification must fail");

        assert_eq!(
            error,
            VerificationError::InvalidLength,
            "Incorrect for '{}'",
            test
        );
    }

    #[test]
    fn test_verify_invalid_characters() {
        let test = "Hello, world!";
        let error = verify(test).expect_err("The verification must fail");

        assert_eq!(
            error,
            VerificationError::InvalidByte {
                index: 5,
                byte: b',',
            },
            "Incorrect for '{}'",
            test
        );

        // --------------------------------------------------------------------

        let test = "THIS IS A TEST";
        let error = verify(test).expect_err("The verification must fail");

        assert_eq!(
            error,
            VerificationError::InvalidByte {
                index: 2,
                byte: b'I',
            },
            "Incorrect for '{}'",
            test
        );

        // --------------------------------------------------------------------

        let test = "TESTONTEST";
        let error = verify(test).expect_err("The verification must fail");

        assert_eq!(
            error,
            VerificationError::InvalidByte {
                index: 4,
                byte: b'O',
            },
            "Incorrect for '{}'",
            test
        );
    }
}

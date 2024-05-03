//! Convert given values into their representation in memory

/// Convert given values into their representation in memory
///
/// Note that this might not behave as expected for certain types, such as strings or heap allocated structs,
/// as it will only return the data from the struct, not the actual data it points to
pub trait RamBytes {
    /// Convert a given value into it's memory representation in bytes
    ///
    /// # Safety
    /// This function references raw memory. If this is freed, the reference will be invalid
    unsafe fn as_ram_bytes(&self) -> &[u8];

    /// Convert a given value into it's memory representation in bytes
    ///
    /// # Safety
    /// As opposed to [`as_ram_bytes`], this function clones the data into a new vector,
    /// so it will continue to be valid even if the original data is freed
    fn to_ram_bytes(&self) -> Vec<u8> {
        unsafe { self.as_ram_bytes() }.to_vec()
    }
}

impl<T> RamBytes for T {
    unsafe fn as_ram_bytes(&self) -> &[u8] {
        core::slice::from_raw_parts(
            core::ptr::from_ref::<T>(self).cast::<u8>(),
            core::mem::size_of_val(self),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_slice() {
        let bytes: [u8; 6] = [0, 1, 2, 3, 4, 5];

        assert_eq!(bytes.to_ram_bytes(), bytes);
    }

    #[test]
    fn number_struct() {
        #[derive(Debug, Copy, Clone)]
        #[allow(dead_code)]
        struct VecSize {
            // Note that the size of usize is dependent on your architecture
            // Normally, you would use usize, but this test could fail on certain systems because of that fact
            len: u16,
            capacity: u16,
        }

        let size = VecSize {
            len: 10,
            capacity: 128,
        };

        let bytes = [10, 0, 128, 0];

        assert_eq!(unsafe { size.as_ram_bytes() }, bytes);
    }

    #[test]
    fn struct_data() {
        #[derive(Debug, Copy, Clone)]
        struct Person {
            first: [char; 8],
            surname: [char; 6],
            age: u8,
        }

        let person = Person {
            first: ['J', 'u', 'l', 'i', 'e', 't', 't', 'e'],
            surname: ['C', 'o', 'r', 'd', 'o', 'r'],
            age: 19,
        };

        // Rust analyzer says the struct is 60 bytes wide
        // The first 32 bytes are for the first name
        // The second 32 bytes are for the last name
        // The last byte is for the age
        // The last 3 bytes are null
        let person_bytes: Vec<u8> = {
            let mut v = Vec::with_capacity(40);

            v.extend(
                &person
                    .first
                    .iter()
                    .flat_map(|c| (*c as u32).to_ne_bytes())
                    .collect::<Vec<_>>(),
            );
            v.extend(
                &person
                    .surname
                    .iter()
                    .flat_map(|c| (*c as u32).to_ne_bytes())
                    .collect::<Vec<_>>(),
            );
            v.push(person.age);
            v.extend(&[0; 3]);

            v
        };
        assert_eq!(person.to_ram_bytes(), person_bytes);
    }

    #[test]
    fn test_conversions() {
        let data: i32 = 0b0010_0101_1010_0101_0010_0101_1010_0101;

        let bytes = data.to_ram_bytes();

        assert_eq!(bytes, data.to_le_bytes());

        #[allow(clippy::cast_ptr_alignment)]
        let ptr = Box::into_raw(bytes.into_boxed_slice()).cast::<i32>();

        let decoded_data = unsafe { *ptr };

        assert_eq!(data, decoded_data);
    }

    // #[test]
    // fn packed_struct_data() {
    //     #[derive(Debug, Copy, Clone)]
    //     #[repr(packed)]
    //     struct Person {
    //         first: &'static str,
    //         surname: &'static str,
    //         age: u8,
    //     }

    //     let person = Person {
    //         first: "Juliette",
    //         surname: "Cordor",
    //         age: 19,
    //     };

    //     // Rust analyzer says the struct is 40 bytes wide
    //     // The first 16 bytes are for the first name
    //     // The second 16 bytes are for the last name
    //     // The last byte is for the age
    //     let person_bytes: Vec<u8> = {
    //         let mut v = Vec::with_capacity(33);

    //         v.extend(person.first.as_bytes());
    //         v.extend(person.surname.as_bytes());
    //         v.push(person.age);

    //         v
    //     };

    //     assert_eq!(into_bytes(person), person_bytes);
    // }
}

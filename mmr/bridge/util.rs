#![macro_use]

/// Doc with expr
#[macro_export]
macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

/// Convert bytes to hex
#[macro_export]
macro_rules! hex {
    ($bytes:expr) => {{
        let mut s = String::new();
        for i in $bytes {
            s.push_str(&format!("{:02x}", i));
        }
        s
    }};
}

/// Convert hext to `Vec<u8>` or `[u8; n]`
#[macro_export]
macro_rules! bytes {
    // Convert hex to Vec<u8>
    ($hex:expr) => {{
        let mut h = $hex;
        if h.starts_with("0x") {
            h = &h[2..];
        }

        (0..h.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&h[i..i + 2], 16))
            .collect::<Result<Vec<u8>, _>>()
            .unwrap_or_default()
    }};

    // Convert hex to [u8; $bits]
    ($hex:expr, $bits:expr) => {{
        let mut hash = [0_u8; $bits];
        hash.copy_from_slice(&bytes!($hex));
        hash
    }};
}

#[macro_export]
/// Construct hash bytes
macro_rules! construct_hash_bytes {
    ( $(#[$attr:meta])* $visibility:vis struct $name:ident ( $n_words:tt ); ) => {
        doc_comment!{
            concat!("The ", stringify!($n_words), "-bit hash type."),
            $(#[$attr])*
            #[derive(Decode, Encode)]
            $visibility struct $name (pub [u8; $n_words]);
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str(&hex!(self.0.as_ref()))
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.debug_list().entries(self.0.iter()).finish()
            }
        }

        impl Default for $name {
            fn default() -> $name {
                $name([0; $n_words])
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                for i in 0..self.0.len() {
                    if self.0[i] != other.0[i] {
                        return false;
                    }
                }
                true
            }
        }

        impl Eq for $name {}
    };
}

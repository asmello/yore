#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
use crate::EncodeError;

pub trait Encoder {
    fn encode_grapheme(&self, bytes: &mut &[u8]) -> Option<u8>;

    /// Encode a single Unicode `char` to its codepage byte, or `None` if
    /// the character has no mapping. Allocation-free.
    #[inline]
    fn encode_char(&self, c: char) -> Option<u8> {
        let mut buf = [0u8; 4];
        let utf8 = c.encode_utf8(&mut buf).as_bytes();
        let mut slice: &[u8] = utf8;
        self.encode_grapheme(&mut slice)
    }

    #[cfg(feature = "alloc")]
    #[doc(hidden)]
    #[inline(always)]
    fn encode_helper<'a>(
        &self,
        s: &'a str,
        fallback: Option<u8>,
    ) -> Result<Cow<'a, [u8]>, EncodeError> {
        let mut src = s.as_bytes();
        if crate::is_ascii_str(s) {
            return Ok(src.into());
        }
        let len = s.chars().count();
        let mut res = Vec::with_capacity(len);

        // extend uses iterator size hint to skip per-element capacity checks
        res.extend(
            (0..len)
                .map(|_| self.encode_grapheme(&mut src).or(fallback))
                .flatten(),
        );

        // If any encoding failed, we got fewer bytes than expected
        if res.len() != len {
            return Err(EncodeError {});
        }

        Ok(res.into())
    }
}

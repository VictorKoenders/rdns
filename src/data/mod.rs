mod question;
mod request;
mod response;

#[cfg(test)]
mod test;

pub use self::question::*;
pub use self::request::*;
pub use self::response::*;

pub(self) trait SliceExt {
    type Output;
    fn try_split_at(&self, mid: usize) -> Option<(Self::Output, Self::Output)>;
}
impl<'a, T> SliceExt for &'a [T] {
    type Output = &'a [T];
    fn try_split_at(&self, mid: usize) -> Option<(&'a [T], &'a [T])> {
        if mid > self.len() {
            None
        } else {
            Some(self.split_at(mid))
        }
    }
}

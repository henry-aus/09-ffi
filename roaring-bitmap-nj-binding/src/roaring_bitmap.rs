use napi::{Error, Status};
use napi_derive::napi;
use roaring::RoaringBitmap;

#[napi(js_name = "RoaringBitmap")]
pub struct NjRoaringBitmap {
  inner: RoaringBitmap,
}

#[napi]
impl NjRoaringBitmap {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      inner: RoaringBitmap::new(),
    }
  }

  #[napi]
  pub fn insert(&mut self, value: u32) -> bool {
    self.inner.insert(value)
  }

  #[napi]
  pub fn append(&mut self, values: Vec<u32>) -> napi::Result<u32, Status> {
    match self.inner.append(values.into_iter()) {
      Ok(num) => napi::Result::Ok(num as u32),
      Err(_) => napi::Result::Err(Error::new(Status::InvalidArg, "not ordered values")),
    }
  }

  #[napi]
  pub fn clear(&mut self) {
    self.inner.clear()
  }

  #[napi]
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  #[napi]
  pub fn is_full(&self) -> bool {
    self.inner.is_full()
  }

  #[napi]
  pub fn len(&self) -> u32 {
    self.inner.len() as u32
  }

  #[napi]
  pub fn min(&self) -> Option<u32> {
    self.inner.min()
  }

  #[napi]
  pub fn is_disjoint(&self, other: &NjRoaringBitmap) -> bool {
    self.inner.is_disjoint(&other.inner)
  }

  #[napi]
  pub fn is_subset(&self, other: &NjRoaringBitmap) -> bool {
    self.inner.is_subset(&other.inner)
  }

  #[napi]
  pub fn is_superset(&self, other: &NjRoaringBitmap) -> bool {
    self.inner.is_superset(&other.inner)
  }

  #[napi]
  pub fn full() -> Self {
    Self {
      inner: RoaringBitmap::full(),
    }
  }

  #[napi]
  pub fn push(&mut self, value: u32) -> bool {
    self.inner.push(value)
  }

  #[napi]
  pub fn remove(&mut self, value: u32) -> bool {
    self.inner.remove(value)
  }

  #[napi]
  pub fn contains(&self, value: u32) -> bool {
    self.inner.contains(value)
  }

  #[napi]
  pub fn max(&self) -> Option<u32> {
    self.inner.max()
  }

  #[napi]
  pub fn rank(&self, value: u32) -> u32 {
    self.inner.rank(value) as u32
  }

  #[napi]
  pub fn select(&self, n: u32) -> Option<u32> {
    self.inner.select(n as u32).map(|x| x as u32)
  }

  #[napi]
  pub fn remove_smallest(&mut self, n: u32) {
    self.inner.remove_smallest(n as u64)
  }

  #[napi]
  pub fn remove_biggest(&mut self, n: u32) {
    self.inner.remove_biggest(n as u64)
  }
}

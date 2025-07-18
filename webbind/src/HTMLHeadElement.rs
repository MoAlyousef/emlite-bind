use super::*;

/// The HTMLHeadElement class.
/// [`HTMLHeadElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLHeadElement {
    inner: HTMLElement,
}
impl FromVal for HTMLHeadElement {
    fn from_val(v: &Any) -> Self {
        HTMLHeadElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLHeadElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLHeadElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLHeadElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLHeadElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLHeadElement> for Any {
    fn from(s: HTMLHeadElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLHeadElement> for Any {
    fn from(s: &HTMLHeadElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLHeadElement);

impl HTMLHeadElement {
    /// The `new HTMLHeadElement(..)` constructor, creating a new HTMLHeadElement instance
    pub fn new() -> HTMLHeadElement {
        Self {
            inner: Any::global("HTMLHeadElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}

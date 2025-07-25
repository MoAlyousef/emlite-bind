use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IsInputPendingOptions {
    inner: Any,
}
impl FromVal for IsInputPendingOptions {
    fn from_val(v: &Any) -> Self {
        IsInputPendingOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IsInputPendingOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IsInputPendingOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IsInputPendingOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IsInputPendingOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IsInputPendingOptions> for Any {
    fn from(s: IsInputPendingOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IsInputPendingOptions> for Any {
    fn from(s: &IsInputPendingOptions) -> Any {
        s.inner.clone()
    }
}

impl IsInputPendingOptions {
    pub fn include_continuous(&self) -> bool {
        self.inner.get("includeContinuous").as_::<bool>()
    }

    pub fn set_include_continuous(&mut self, value: bool) {
        self.inner.set("includeContinuous", value);
    }
}
/// The Scheduling class.
/// [`Scheduling`](https://developer.mozilla.org/en-US/docs/Web/API/Scheduling)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Scheduling {
    inner: Any,
}
impl FromVal for Scheduling {
    fn from_val(v: &Any) -> Self {
        Scheduling {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Scheduling {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Scheduling {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Scheduling {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Scheduling {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Scheduling> for Any {
    fn from(s: Scheduling) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Scheduling> for Any {
    fn from(s: &Scheduling) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Scheduling);

impl Scheduling {
    /// The isInputPending method.
    /// [`Scheduling.isInputPending`](https://developer.mozilla.org/en-US/docs/Web/API/Scheduling/isInputPending)
    pub fn is_input_pending0(&self) -> bool {
        self.inner.call("isInputPending", &[]).as_::<bool>()
    }
    /// The isInputPending method.
    /// [`Scheduling.isInputPending`](https://developer.mozilla.org/en-US/docs/Web/API/Scheduling/isInputPending)
    pub fn is_input_pending1(&self, is_input_pending_options: &IsInputPendingOptions) -> bool {
        self.inner
            .call("isInputPending", &[is_input_pending_options.into()])
            .as_::<bool>()
    }
}

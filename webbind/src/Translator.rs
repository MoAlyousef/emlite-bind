use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TranslatorCreateOptions {
    inner: Any,
}
impl FromVal for TranslatorCreateOptions {
    fn from_val(v: &Any) -> Self {
        TranslatorCreateOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TranslatorCreateOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TranslatorCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TranslatorCreateOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TranslatorCreateOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TranslatorCreateOptions> for Any {
    fn from(s: TranslatorCreateOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TranslatorCreateOptions> for Any {
    fn from(s: &TranslatorCreateOptions) -> Any {
        s.inner.clone()
    }
}

impl TranslatorCreateOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl TranslatorCreateOptions {
    pub fn monitor(&self) -> Function {
        self.inner.get("monitor").as_::<Function>()
    }

    pub fn set_monitor(&mut self, value: &Function) {
        self.inner.set("monitor", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TranslatorCreateCoreOptions {
    inner: Any,
}
impl FromVal for TranslatorCreateCoreOptions {
    fn from_val(v: &Any) -> Self {
        TranslatorCreateCoreOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TranslatorCreateCoreOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TranslatorCreateCoreOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TranslatorCreateCoreOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TranslatorCreateCoreOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TranslatorCreateCoreOptions> for Any {
    fn from(s: TranslatorCreateCoreOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TranslatorCreateCoreOptions> for Any {
    fn from(s: &TranslatorCreateCoreOptions) -> Any {
        s.inner.clone()
    }
}

impl TranslatorCreateCoreOptions {
    pub fn source_language(&self) -> String {
        self.inner.get("sourceLanguage").as_::<String>()
    }

    pub fn set_source_language(&mut self, value: &str) {
        self.inner.set("sourceLanguage", value);
    }
}
impl TranslatorCreateCoreOptions {
    pub fn target_language(&self) -> String {
        self.inner.get("targetLanguage").as_::<String>()
    }

    pub fn set_target_language(&mut self, value: &str) {
        self.inner.set("targetLanguage", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TranslatorTranslateOptions {
    inner: Any,
}
impl FromVal for TranslatorTranslateOptions {
    fn from_val(v: &Any) -> Self {
        TranslatorTranslateOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TranslatorTranslateOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TranslatorTranslateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TranslatorTranslateOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TranslatorTranslateOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TranslatorTranslateOptions> for Any {
    fn from(s: TranslatorTranslateOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TranslatorTranslateOptions> for Any {
    fn from(s: &TranslatorTranslateOptions) -> Any {
        s.inner.clone()
    }
}

impl TranslatorTranslateOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
/// The Translator class.
/// [`Translator`](https://developer.mozilla.org/en-US/docs/Web/API/Translator)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Translator {
    inner: Any,
}
impl FromVal for Translator {
    fn from_val(v: &Any) -> Self {
        Translator {
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
impl core::ops::Deref for Translator {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Translator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Translator {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Translator {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Translator> for Any {
    fn from(s: Translator) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Translator> for Any {
    fn from(s: &Translator) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Translator);

impl Translator {
    /// The create method.
    /// [`Translator.create`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/create)
    pub fn create(options: &TranslatorCreateOptions) -> Promise {
        Any::global("Translator")
            .call("create", &[options.into()])
            .as_::<Promise>()
    }
}
impl Translator {
    /// The availability method.
    /// [`Translator.availability`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/availability)
    pub fn availability(options: &TranslatorCreateCoreOptions) -> Promise {
        Any::global("Translator")
            .call("availability", &[options.into()])
            .as_::<Promise>()
    }
}
impl Translator {
    /// The translate method.
    /// [`Translator.translate`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/translate)
    pub fn translate0(&self, input: &str) -> Promise {
        self.inner
            .call("translate", &[input.into()])
            .as_::<Promise>()
    }
    /// The translate method.
    /// [`Translator.translate`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/translate)
    pub fn translate1(&self, input: &str, options: &TranslatorTranslateOptions) -> Promise {
        self.inner
            .call("translate", &[input.into(), options.into()])
            .as_::<Promise>()
    }
}
impl Translator {
    /// The translateStreaming method.
    /// [`Translator.translateStreaming`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/translateStreaming)
    pub fn translate_streaming0(&self, input: &str) -> ReadableStream {
        self.inner
            .call("translateStreaming", &[input.into()])
            .as_::<ReadableStream>()
    }
    /// The translateStreaming method.
    /// [`Translator.translateStreaming`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/translateStreaming)
    pub fn translate_streaming1(
        &self,
        input: &str,
        options: &TranslatorTranslateOptions,
    ) -> ReadableStream {
        self.inner
            .call("translateStreaming", &[input.into(), options.into()])
            .as_::<ReadableStream>()
    }
}
impl Translator {
    /// Getter of the `sourceLanguage` attribute.
    /// [`Translator.sourceLanguage`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/sourceLanguage)
    pub fn source_language(&self) -> String {
        self.inner.get("sourceLanguage").as_::<String>()
    }
}
impl Translator {
    /// Getter of the `targetLanguage` attribute.
    /// [`Translator.targetLanguage`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/targetLanguage)
    pub fn target_language(&self) -> String {
        self.inner.get("targetLanguage").as_::<String>()
    }
}
impl Translator {
    /// The measureInputUsage method.
    /// [`Translator.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/measureInputUsage)
    pub fn measure_input_usage0(&self, input: &str) -> Promise {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<Promise>()
    }
    /// The measureInputUsage method.
    /// [`Translator.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/measureInputUsage)
    pub fn measure_input_usage1(
        &self,
        input: &str,
        options: &TranslatorTranslateOptions,
    ) -> Promise {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<Promise>()
    }
}
impl Translator {
    /// Getter of the `inputQuota` attribute.
    /// [`Translator.inputQuota`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/inputQuota)
    pub fn input_quota(&self) -> f64 {
        self.inner.get("inputQuota").as_::<f64>()
    }
}
impl Translator {
    /// The destroy method.
    /// [`Translator.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}

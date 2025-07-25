use super::*;

/// The HTMLTableRowElement class.
/// [`HTMLTableRowElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTableRowElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTableRowElement {
    fn from_val(v: &Any) -> Self {
        HTMLTableRowElement {
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
impl core::ops::Deref for HTMLTableRowElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTableRowElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLTableRowElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLTableRowElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLTableRowElement> for Any {
    fn from(s: HTMLTableRowElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLTableRowElement> for Any {
    fn from(s: &HTMLTableRowElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLTableRowElement);

impl HTMLTableRowElement {
    /// The `new HTMLTableRowElement(..)` constructor, creating a new HTMLTableRowElement instance
    pub fn new() -> HTMLTableRowElement {
        Self {
            inner: Any::global("HTMLTableRowElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTableRowElement {
    /// Getter of the `rowIndex` attribute.
    /// [`HTMLTableRowElement.rowIndex`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/rowIndex)
    pub fn row_index(&self) -> i32 {
        self.inner.get("rowIndex").as_::<i32>()
    }
}
impl HTMLTableRowElement {
    /// Getter of the `sectionRowIndex` attribute.
    /// [`HTMLTableRowElement.sectionRowIndex`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/sectionRowIndex)
    pub fn section_row_index(&self) -> i32 {
        self.inner.get("sectionRowIndex").as_::<i32>()
    }
}
impl HTMLTableRowElement {
    /// Getter of the `cells` attribute.
    /// [`HTMLTableRowElement.cells`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/cells)
    pub fn cells(&self) -> HTMLCollection {
        self.inner.get("cells").as_::<HTMLCollection>()
    }
}
impl HTMLTableRowElement {
    /// The insertCell method.
    /// [`HTMLTableRowElement.insertCell`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/insertCell)
    pub fn insert_cell0(&self) -> HTMLTableCellElement {
        self.inner
            .call("insertCell", &[])
            .as_::<HTMLTableCellElement>()
    }
    /// The insertCell method.
    /// [`HTMLTableRowElement.insertCell`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/insertCell)
    pub fn insert_cell1(&self, index: i32) -> HTMLTableCellElement {
        self.inner
            .call("insertCell", &[index.into()])
            .as_::<HTMLTableCellElement>()
    }
}
impl HTMLTableRowElement {
    /// The deleteCell method.
    /// [`HTMLTableRowElement.deleteCell`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/deleteCell)
    pub fn delete_cell(&self, index: i32) -> Undefined {
        self.inner
            .call("deleteCell", &[index.into()])
            .as_::<Undefined>()
    }
}
impl HTMLTableRowElement {
    /// Getter of the `align` attribute.
    /// [`HTMLTableRowElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/align)
    pub fn align(&self) -> DOMString {
        self.inner.get("align").as_::<DOMString>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLTableRowElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/align)
    pub fn set_align(&mut self, value: &DOMString) {
        self.inner.set("align", value);
    }
}
impl HTMLTableRowElement {
    /// Getter of the `ch` attribute.
    /// [`HTMLTableRowElement.ch`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/ch)
    pub fn ch(&self) -> DOMString {
        self.inner.get("ch").as_::<DOMString>()
    }

    /// Setter of the `ch` attribute.
    /// [`HTMLTableRowElement.ch`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/ch)
    pub fn set_ch(&mut self, value: &DOMString) {
        self.inner.set("ch", value);
    }
}
impl HTMLTableRowElement {
    /// Getter of the `chOff` attribute.
    /// [`HTMLTableRowElement.chOff`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/chOff)
    pub fn ch_off(&self) -> DOMString {
        self.inner.get("chOff").as_::<DOMString>()
    }

    /// Setter of the `chOff` attribute.
    /// [`HTMLTableRowElement.chOff`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/chOff)
    pub fn set_ch_off(&mut self, value: &DOMString) {
        self.inner.set("chOff", value);
    }
}
impl HTMLTableRowElement {
    /// Getter of the `vAlign` attribute.
    /// [`HTMLTableRowElement.vAlign`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/vAlign)
    pub fn v_align(&self) -> DOMString {
        self.inner.get("vAlign").as_::<DOMString>()
    }

    /// Setter of the `vAlign` attribute.
    /// [`HTMLTableRowElement.vAlign`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/vAlign)
    pub fn set_v_align(&mut self, value: &DOMString) {
        self.inner.set("vAlign", value);
    }
}
impl HTMLTableRowElement {
    /// Getter of the `bgColor` attribute.
    /// [`HTMLTableRowElement.bgColor`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/bgColor)
    pub fn bg_color(&self) -> DOMString {
        self.inner.get("bgColor").as_::<DOMString>()
    }

    /// Setter of the `bgColor` attribute.
    /// [`HTMLTableRowElement.bgColor`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/bgColor)
    pub fn set_bg_color(&mut self, value: &DOMString) {
        self.inner.set("bgColor", value);
    }
}

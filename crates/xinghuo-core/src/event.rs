use std::fmt::Debug;

#[derive(Default)]
pub struct EventListener {
    pub onclick: Option<Box<dyn FnMut(Click)>>,
    pub onmouseenter: Option<Box<dyn FnMut(MouseEnter)>>,
    pub onmouseleave: Option<Box<dyn FnMut(MouseLeave)>>,
    pub onmousemove: Option<Box<dyn FnMut(MouseMove)>>,
    pub onmouseout: Option<Box<dyn FnMut(MouseOut)>>,
    pub onmouseover: Option<Box<dyn FnMut(MouseOver)>>,
    pub onmouseup: Option<Box<dyn FnMut(MouseUp)>>,
}

impl EventListener {
    pub fn is_empty(&self) -> bool {
        let Self {
            onclick,
            onmouseenter,
            onmouseleave,
            onmousemove,
            onmouseout,
            onmouseover,
            onmouseup,
        } = self;
        onclick.is_none()
            && onmouseenter.is_none()
            && onmouseleave.is_none()
            && onmousemove.is_none()
            && onmouseout.is_none()
            && onmouseover.is_none()
            && onmouseup.is_none()
    }
}

macro_rules! debug_event_field {
    ($debug:ident, $($name:ident),*) => {
        $(
            if $name.is_some() {
                $debug.field(stringify!($name), &"Some");
            }
        )*
    };
}

impl Debug for EventListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            onclick,
            onmouseenter,
            onmouseleave,
            onmousemove,
            onmouseout,
            onmouseover,
            onmouseup,
        } = self;

        // let a = &format("{:?}", self.onclick.as_ref().map(|_| ()))[..4];
        let mut debug = f.debug_struct("Events");
        debug_event_field!(
            debug,
            onclick,
            onmouseenter,
            onmouseleave,
            onmousemove,
            onmouseout,
            onmouseover,
            onmouseup
        );
        debug.finish()
    }
}

pub trait GlobalEventHandler: Sized {
    fn onclick(self, _callback: impl FnMut(Click) + 'static) -> Self {
        self
    }
    fn onmouseenter(self, _callback: impl FnMut(MouseEnter) + 'static) -> Self {
        self
    }
    fn onmouseleave(self, _callback: impl FnMut(MouseLeave) + 'static) -> Self {
        self
    }
    fn onmousemove(self, _callback: impl FnMut(MouseMove) + 'static) -> Self {
        self
    }
    fn onmouseout(self, _callback: impl FnMut(MouseOut) + 'static) -> Self {
        self
    }
    fn onmouseover(self, _callback: impl FnMut(MouseOver) + 'static) -> Self {
        self
    }
    fn onmouseup(self, _callback: impl FnMut(MouseUp) + 'static) -> Self {
        self
    }
}

pub trait Event {
    const NAME: &'static str;
}

#[derive(Debug)]
pub struct Click {}
impl Event for Click {
    const NAME: &'static str = "Click";
}

#[derive(Debug)]
pub struct MouseEnter {}
impl Event for MouseEnter {
    const NAME: &'static str = "MouseEnter";
}

#[derive(Debug)]
pub struct MouseLeave {}
impl Event for MouseLeave {
    const NAME: &'static str = "MouseLeave";
}

#[derive(Debug)]
pub struct MouseMove {}
impl Event for MouseMove {
    const NAME: &'static str = "MouseMove";
}

#[derive(Debug)]
pub struct MouseOut {}
impl Event for MouseOut {
    const NAME: &'static str = "MouseOut";
}

#[derive(Debug)]
pub struct MouseOver {}
impl Event for MouseOver {
    const NAME: &'static str = "MouseOver";
}

#[derive(Debug)]
pub struct MouseUp {}
impl Event for MouseUp {
    const NAME: &'static str = "MouseUp";
}

// onabort              <- Abort,
// onblur               <- Blur,
// oncancel             <- Cancel,
// oncanplay            <- CanPlay,
// oncanplaythrough     <- CanPlayThrough,
// onchange             <- Change,
// onclick              <- Click,
// onclose              <- CloseWebsocket,
// oncontextmenu        <- ContextMenu,
// oncuechange          <- CueChange,
// ondblclick           <- DoubleClick,
// ondrag               <- Drag,
// ondragend            <- DragEnd,
// ondragenter          <- DragEnter,
// ondragexit           <- DragExit,
// ondragleave          <- DragLeave,
// ondragover           <- DragOver,
// ondragstart          <- DragStart,
// ondrop               <- Dropped,
// ondurationchange     <- DurationChange,
// onemptied            <- Emptied,
// onended              <- PlaybackEnded,
// onerror              <- ErrorEvent,
// onfocus              <- Focus,
// ongotpointercapture  <- GotPointerCapture,
// oninput              <- Input,
// oninvalid            <- Invalid,
// onkeydown            <- KeyDown,
// onkeypress           <- KeyPress,
// onkeyup              <- KeyUp,
// onload               <- ResourceLoad,
// onloadeddata         <- DataLoaded,
// onloadedmetadata     <- MetadataLoaded,
// onloadend            <- LoadEnd,
// onloadstart          <- LoadStart,
// onlostpointercapture <- LostPointerCapture,
// onmouseenter         <- MouseEnter,
// onmouseleave         <- MouseLeave,
// onmousemove          <- MouseMove,
// onmouseout           <- MouseOut,
// onmouseover          <- MouseOver,
// onmouseup            <- MouseUp,
// onpause              <- Pause,
// onplay               <- Play,
// onplaying            <- Playing,
// onpointercancel      <- PointerCancel,
// onpointerdown        <- PointerDown,
// onpointerenter       <- PointerEnter,
// onpointerleave       <- PointerLeave,
// onpointermove        <- PointerMove,
// onpointerout         <- PointerOut,
// onpointerover        <- PointerOver,
// onpointerup          <- PointerUp,
// onprogress           <- Progress,
// onratechange         <- PlaybackRateChange,
// onreset              <- FormReset,
// onresize             <- ViewResize,
// onscroll             <- Scroll,
// onseeked             <- Seeked,
// onseeking            <- Seeking,
// onselect             <- Select,
// onselectionchange    <- SelectionChange,
// onselectstart        <- SelectionStart,
// onshow               <- ContextMenuShow,
// onstalled            <- Stalled,
// onsubmit             <- Submit,
// onsuspend            <- Suspend,
// ontimeupdate         <- TimeUpdate,
// onvolumechange       <- VolumeChange,
// onwaiting            <- Waiting,
// onwheel              <- Wheel,

use ratatui::layout::Rect;
use ratatui::Frame;
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State,
};

/// A wrapper that adds an activatable action to any MockComponent.
///
/// When Enter key is pressed or Submit command is received,
/// the stored closure is invoked.
pub struct WithAction<C: MockComponent> {
    props: Props,
    content: C,
    action: Box<dyn FnMut()>,
}

impl<C: MockComponent> WithAction<C> {
    pub fn new<F: FnMut() + 'static>(content: C, action: F) -> Self {
        Self {
            props: Props::default(),
            content,
            action: Box::new(action),
        }
    }

    /// Get a reference to the wrapped component
    pub fn inner(&self) -> &C {
        &self.content
    }

    /// Get a mutable reference to the wrapped component
    pub fn inner_mut(&mut self) -> &mut C {
        &mut self.content
    }
}

impl<C: MockComponent> MockComponent for WithAction<C> {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        self.content.view(frame, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr).or_else(|| self.content.query(attr))
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.content.attr(attr, value);
    }

    fn state(&self) -> State {
        self.content.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        if matches!(cmd, Cmd::Submit) {
            (self.action)();
        }
        self.content.perform(cmd)
    }
}

impl<C: MockComponent> Component<Event<NoUserEvent>, NoUserEvent> for WithAction<C> {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        if matches!(ev, Event::Keyboard(KeyEvent { code: Key::Enter, .. })) {
            (self.action)();
        }
        None
    }
}

/// Extension trait that adds `.with_action()` method to all MockComponents
pub trait InteractableExt: MockComponent + Sized {
    fn with_action<F: FnMut() + 'static>(self, action: F) -> WithAction<Self> {
        WithAction::new(self, action)
    }
}

impl<C: MockComponent> InteractableExt for C {}

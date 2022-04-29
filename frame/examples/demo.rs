#[derive(Default)]
struct State {
    width: f32,
    height: f32,
    children: Vec<Box<dyn Widget>>,
}

struct Context {
    state: State,
}

struct Event {
    clicked: bool,
    dbclicked: bool,
    hovered: bool,
}

impl Event {
    pub fn clicked(&self) -> bool {
        self.clicked
    }
    pub fn dbclicked(&self) -> bool {
        self.dbclicked
    }
    pub fn hovered(&self) -> bool {
        self.hovered
    }
}

trait Widget {
    /// 组件id
    fn id(&self) -> &str;

    /// 视图
    fn view(&mut self);
}

struct DemoWidget {
    param: u32,
}

impl DemoWidget {
    fn clicked(&self) {}
}

impl Widget for DemoWidget {
    view! {
        div {
            width: 100,
            
            span {
                "hello"
            }
            div {
                
            }
        }
        <div id="demo_widget1" width="100%" on_click=self.clicked>
            <block width=100 height=20 font_size=20 on_click=self.clicked>
                    <widget1 width=50 height=20 >
                        hello
                    </widget1>
            </block>
        </div>
    }

    fn id(&self) -> &str {
        "demo_widget1"
    }

    fn view(&mut self) {
        let mut state = State::default();
        state.width = div_width;
        state.children = Vec::new();

        let mut block = Block::default();
        block.width = block_width;
        block.height = height;
        block.font_size = font_size;

        // ...
        // ...

        state.children.append(block);
    }
}

fn main() {}

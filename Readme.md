星火UI

## UI架构

    Command:
        实现节点的删除, 触发重绘

    在每一个 update cycle 中:

        处理 Command 队列

        生成一个 RenderObject Tree, 绑定数据 和 事件处理, 节点的修改, 并生成 渲染树.

        同时生成一个 Render tree, 用于 渲染

        事件发生时, 对 View tree 遍历, 每一个节点处理事件, 处理函数的bool返回值: true为已处理, 不继续向下传递

<flex>

</flex>

注册事件,

    data hashmap
    ctx.register_data(widget path, Event, dyn Any);

    event tree
    ctx.register_event(widget path, Event, dyn Any);

    render tree

    ctx:
        draw_interface
        window_handle


    enum Event {
        Click(Fn(Clicked, dyn Any)->bool),
    }

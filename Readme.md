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

2022.05/17

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

2022.05/19

    底层结构:

        Element 渲染树, 每一个节点保存着 Element对象, Element中定义着页面渲染的 盒子模型和布局信息

        数据树: 保持渲染树的形状, 节点是 (Node<Element>, Data), 每一次重绘时, 检索数据是否changed, 如果changed, 当前节点及所有子节点就会重绘
            根据 Node<Element> 可以得到当前节点和所有子节点
            Data 需要实现: trait DataChanged {}

        事件树: 保持渲染树的形状, 节点是 (Node<Element>, EventHandler)
            根据 Node<Element> 可以得到当前节点和所有子节点

    根据刷新率, 每次执行 数据数遍历, 如果 Data Changed, 则 该节点执行重绘

    当 Click 事件发生时, 遍历 事件树, 如果事件发生的位置 刚好在 目标节点的 区域(Area), 则执行 交互函数, 同时可以 从 Node<Data> 中得到数据, 根据 Node<Event> 可以进行事件的 "冒泡" 与 "捕获"

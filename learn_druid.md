ui 자체를 static 하게 만드는 것을 목표로 하고 있다. 

rust 를 이상적으로 쓰기 위해서는 dyn 자체를 적게 사용해야 해서, rust 에서는 gui 들이 static 하게 만들기 위해 다양한 시도들이 있으며, 이로 인해 다양한 프레임워크가 등장했으나, 이게 적합하지 않다면 core 개발자들이 다른 곳으로 이동하는 기현상이 있네.. 흠

## container widget
container widget 은 자식들을 가질 수 있다.
container widget 은 자식들을 어떻게 배치할지 알고 있다.

`WindowDesc::new(ui_builder());` 에서 ui_builder 는 위젯 트리를 만들 책임을 가지고 있다.

Split, Flex, Container

## select a structures' filed with lenses

# UnderthRust

# About Projects
Implementing Data-structures and Algorithms in Rust.

# References
- Intruduction to Algorithms(구현중).
- TAOCP
- ...

# Conventions
## Branch Strategy
모든 topic은 별도의 issue와 그에 따른 branch를 가지며, 반드시 PR을 통해서 Review되어야 한다.

## Project Directory
모든 topic의 구현은 적절한 directory에 library crate를 생성해 구현하며, 이는 모두 yt42 crate에서 public use 된다.

모든 binary crate 및 테스트 script는 yt42 바깥에 위치한다.

이하 새롭게 생성되는 최상단 directory는 아래에 기술할 것.

### Collections
범용 자료구조들을 구현하는 directory.

## Code convention
Standard Rust Linter(Default).

## Comment
적절한 Review를 위해서 docs를 작성한다. 기본적으로 표준 Rust doc 양식을 따른다.

각 crate는 lib.rs의 상단에 //! 주석을 작성하여 crate를 설명할 것. 함수, 구조체, 모듈, 열거형, 트레이트 등 모든 component는 documentation을 하여야 한다.

다만, 구현에 대한 표준 문서화를 위한 주석을 먼저 작성하고, 구현 과정의 시행착오를 위한 주석을 작성할 것성성

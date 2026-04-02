# 트레이트 (Trait) 소개

`trait`는 특정 타입이 구현해야 하는 공통된 동작을 정의하는 방법입니다. 서로 다른 타입이라도 같은 `trait`를 구현하면 공통된 특징을 갖게 됩니다. 함수 인자로 객체를 넘길 때 특정 구조체 타입 대신, 해당 `trait`를 구현한 타입이라면 무엇이든 전달할 수 있어 다형성을 구현하는 데 핵심적인 역할을 합니다.

다른 객체지향 언어의 추상 클래스나 인터페이스와 유사하다고 이해하면 쉽습니다. 다양한 타입의 객체를 추상화하여 묶거나 코드를 재사용할 수 있게 해주므로, 러스트로 규모 있는 프로그램을 만들기 위해서는 필수적으로 익혀야 할 문법입니다.

간단한 예제를 통해 `trait`의 기본 사용법을 알아보겠습니다. 서로 다른 두 구조체 `Person`과 `Book`에 공통의 `trait`인 `Printable`을 구현하는 예제입니다.

```rust
// code/trait/main.rs
trait Printable {
    type Age;
    fn print(&self);
    fn get_age(&self) -> Self::Age;
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

impl Printable for Person {
    type Age = u32;
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.get_age());
    }
    fn get_age(&self) -> Self::Age {
        self.age
    }
}

struct Book {
    title: String,
    author: String,
    published: u32,
}

impl Printable for Book {
    type Age = u32;
    fn print(&self) {
        println!(
            "Title: {}\nAuthor: {}\nPublished: {}",
            self.title,
            self.author,
            self.get_age()
        );
    }
    fn get_age(&self) -> Self::Age {
        self.published
    }
}

fn print_info(item: &dyn Printable<Age = u32>) {
    item.print();
}

fn main() {
    let person = Person::new("Alice", 22);
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };

    print_info(&person);
    print_info(&book);
}
```

### 트레이트 선언과 구현

먼저 `Printable`이라는 `trait`를 선언합니다.

```rust
trait Printable {
    type Age;
    fn print(&self);
    fn get_age(&self) -> Self::Age;
}
```

`Printable` `trait`에는 두 개의 함수와 하나의 연관 타입(`associated type`)이 포함되어 있습니다. 이 `trait`를 구현하는 구조체는 `print`와 `get_age` 함수를 정의해야 하며, `Age`가 어떤 타입이 될지도 결정해야 합니다.

`Person` 구조체에 `Printable`을 구현할 때는 다음과 같이 작성합니다.

```rust
impl Printable for Person {
    type Age = u32;
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.get_age());
    }
    fn get_age(&self) -> Self::Age {
        self.age
    }
}
```

여기서 `type Age = u32;`는 `Age`라는 이름을 `u32` 타입의 별칭처럼 사용하겠다는 의미입니다. `get_age` 함수는 이 `Age` 타입을 반환하므로 최종적으로 `u32`를 반환하게 됩니다. `Book` 구조체 또한 동일한 방식으로 `Printable`을 구현할 수 있습니다.

### 트레이트 객체와 dyn 키워드

이제 `dyn` 키워드를 사용하는 **트레이트 객체**를 살펴보겠습니다.

```rust
fn print_info(item: &dyn Printable<Age = u32>) {
    item.print();
}
```

`print_info` 함수의 인자 타입은 `&dyn Printable<Age = u32>`입니다. 이는 "Age 타입을 u32로 정의하고 Printable trait를 구현한 모든 타입의 참조"를 인자로 받는다는 의미입니다. 덕분에 서로 다른 타입인 `Person`과 `Book`의 참조를 동일한 함수에 전달할 수 있습니다.

여러 `trait`를 동시에 만족해야 하는 타입을 지정하고 싶다면 다음과 같이 작성합니다.

```rust
fn some_function(param: &(dyn Trait1 + Trait2)) {
    // 구현 내용
}
```

## 연관 타입과 연관 함수

- **연관 타입 (Associated Type)**: `trait` 내부에서 사용하는 범용적인 타입입니다. 위 예제의 `type Age;`가 이에 해당하며, 실제 구현 시점에 구체적인 타입을 결정합니다.
- **연관 함수 (Associated Function)**: `self` 인자를 받지 않는 함수로, 보통 `Person::new`처럼 객체를 생성하는 용도로 많이 사용됩니다.

## 트레이트 객체의 동작 원리: vtable

`dyn` 키워드를 사용하면 컴파일 타임에 타입을 확정하는 것이 아니라, 런타임에 실제 구현된 함수를 찾아 호출합니다. 이를 **동적 디스패치 (Dynamic Dispatch)**라고 부릅니다. 컴파일러는 `vtable`이라는 함수 포인터 테이블을 만들어 관리하며, `item.print()`를 호출할 때 실제 객체에 맞는 함수의 주소를 찾아 실행합니다. 약간의 성능 오버헤드가 있을 수 있지만, 대개는 무시할 수 있는 수준입니다.

## 트레이트 객체의 다운캐스팅

트레이트 객체로 묶인 데이터에서 원래의 구체적인 타입을 다시 찾아야 할 때가 있습니다. 이를 **다운캐스팅 (Downcasting)**이라고 합니다. 러스트에서는 `std::any::Any`를 사용하여 이를 구현할 수 있습니다.

```rust
use std::any::Any;

trait Printable {
    // ... 기존 메서드 생략 ...
    fn as_any(&self) -> &dyn Any;
}

impl Printable for Person {
    // ... 생략 ...
    fn as_any(&self) -> &dyn Any { self }
}

// ... 사용 예시 ...
if let Some(p) = item.as_any().downcast_ref::<Person>() {
    println!("Found person: {}", p.name);
}
```

`Collection`과 같은 구조체에 서로 다른 타입을 `Box<&dyn Printable>` 형태로 담아두고, 루프를 돌며 원래 타입이 무엇인지 확인하여 각 타입의 고유한 기능을 실행할 때 유용하게 쓰입니다.

## 표준 라이브러리의 주요 트레이트

러스트 표준 라이브러리에는 범용적으로 사용되는 중요한 `trait`들이 미리 정의되어 있습니다.

### std::fmt::{Display, Debug}

객체의 데이터를 문자열로 출력할 때 사용합니다.

- **Debug**: 개발자를 위한 디버깅용 출력입니다. `{:?}` 포맷으로 출력하며, `#[derive(Debug)]`로 자동 생성할 수 있습니다.
- **Display**: 사용자에게 보여줄 정식 출력입니다. `{}` 포맷으로 출력하며, 직접 `fmt` 함수를 구현해야 합니다.

```rust
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

### Clone과 Copy

- **Clone**: 객체를 명시적으로 복사(`deep copy`)할 때 사용합니다. `clone()` `method`를 통해 소유권을 완전히 가진 새 객체를 생성합니다.
- **Copy**: 대입 시 자동으로 복사가 일어나는 타입에 부여합니다. `i32` 같은 기본 타입은 `Copy`가 구현되어 있습니다.

### Default

`Default` `trait`는 구조체의 필드를 기본값으로 초기화하여 새 객체를 생성할 때 사용합니다. `Book::default()`와 같이 호출합니다.

### PartialEq와 Eq

- **PartialEq**: 두 객체의 값이 같은지 비교할 때 사용합니다. `==` 연산자를 가능하게 합니다.
- **Eq**: `PartialEq`를 상속하며, 모든 값이 항상 자기 자신과 같다는 수학적 성질을 보장합니다. 부동소수점(`f32::NAN`) 같은 예외가 없는 경우에만 사용합니다. `HashMap`의 키로 쓰이려면 `Eq`와 `Hash`가 구현되어야 합니다.

### PartialOrd와 Ord

객체 간의 크기를 비교할 때 사용합니다. `PartialOrd`는 `partial_cmp`를 통해 `Option<Ordering>`을 반환하며, 이를 구현하면 `<`, `>`, `<=` 등의 연산자를 쓸 수 있습니다. 이를 바탕으로 `Vec`의 `sort()` 등을 활용해 정렬을 수행할 수 있습니다.

### From과 Into

타입 간의 변환을 정의합니다. `From<T>`를 구현하면 자동으로 `Into<U>`가 구현됩니다.

```rust
let isbn: u32 = book.into(); // Book 타입을 u32로 변환
```

성공 여부가 불확실한 변환에는 `TryFrom`과 `TryInto`를 사용하며, 이들은 결과를 `Result` 타입으로 반환합니다.

### Iterator와 반복자

데이터 집합을 하나씩 순회할 때 사용합니다. `next` `method`를 구현해야 하며, 이를 통해 `for` 루프에서 객체를 바로 사용할 수 있게 됩니다. 읽기 전용 순회에는 `iter()`, 값 수정이 필요한 경우 `iter_mut()`를 활용합니다.

`trait`는 러스트의 타입 시스템을 지탱하는 핵심 기둥입니다. 다양한 표준 `trait`들을 잘 활용하고 직접 필요한 `trait`를 정의해 사용함으로써, 코드의 안전성과 재사용성을 동시에 높일 수 있습니다.

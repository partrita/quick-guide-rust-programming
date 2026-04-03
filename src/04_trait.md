# 트레이트 (Trait) 소개

`Trait`는 특정 타입이 구현해야 하는 공통된 동작을 정의하는 명세입니다. 서로 다른 타입이라도 같은 `Trait`를 구현하면 공통적인 특징을 가지게 됩니다. 함수 인자로 특정 구조체 대신 해당 `Trait`를 구현한 타입이라면 무엇이든 받을 수 있어 다형성을 구현하는 데 핵심적인 역할을 합니다.

다른 언어의 인터페이스(`Interface`)나 추상 클래스와 유사하지만, `Rust`만의 고유한 특징이 있습니다. 대규모 프로그램을 설계할 때 반드시 익혀야 할 필수 문법입니다.

간단한 예제로 `Trait`의 기본 사용법을 알아보겠습니다. `Person`과 `Book`이라는 서로 다른 구조체에 `Printable`이라는 공통 인터페이스를 적용하는 예제입니다.

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

먼저 `Printable` 트레이트를 선언합니다.

```rust
trait Printable {
    type Age;
    fn print(&self);
    fn get_age(&self) -> Self::Age;
}
```

이 트레이트를 구현하는 구조체는 반드시 `print`와 `get_age` 함수를 정의해야 하며, 연관 타입(`Associated Type`)인 `Age`가 어떤 타입이 될지도 결정해야 합니다.

`Person` 구조체에 구현할 때는 다음과 같이 작성합니다.

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

`type Age = u32;`는 이 트레이트 안에서 사용할 타입 별칭입니다.

### 트레이트 객체와 dyn 키워드

이제 `dyn` 키워드를 사용하는 트레이트 객체를 살펴보겠습니다.

```rust
fn print_info(item: &dyn Printable<Age = u32>) {
    item.print();
}
```

`item`의 타입인 `&dyn Printable<Age = u32>`는 "`Age` 타입을 `u32`로 정의하고 `Printable` 트레이트를 구현한 모든 타입의 참조"를 의미합니다. 덕분에 서로 다른 타입인 `Person`과 `Book`을 동일한 함수에서 처리할 수 있습니다.

여러 트레이트를 동시에 만족해야 한다면 다음과 같이 선언합니다.

```rust
fn some_function(param: &(dyn Trait1 + Trait2)) { ... }
```

## 연관 타입과 연관 함수

*   연관 타입 (`Associated Type`): 트레이트 내부에서 사용하는 범용 타입입니다. 실제 구현 시점에 구체적인 타입을 결정합니다.
*   연관 함수 (`Associated Function`): `self` 인자를 받지 않는 함수로, 대개 `Person::new`처럼 객체 생성 용도로 쓰입니다.

## 트레이트 객체의 동작 원리: vtable

`dyn` 키워드를 사용하면 컴파일 타임이 아닌 런타임에 실제 구현된 함수를 찾아 호출하는데, 이를 동적 디스패치 (`Dynamic Dispatch`)라고 합니다. 컴파일러는 내부적으로 `vtable`이라는 함수 포인터 테이블을 만들어 관리하며, 실행 시점에 적절한 함수 주소를 찾아갑니다. 아주 미세한 성능 차이가 있을 수 있지만 실무에서는 무시해도 좋을 수준입니다.

## 트레이트 객체의 다운캐스팅

트레이트 객체로 묶인 데이터에서 원래의 구체적인 타입을 다시 찾아야 할 때가 있는데, 이를 다운캐스팅 (`Downcasting`)이라고 합니다. `Rust`에서는 `std::any::Any`를 사용하여 구현할 수 있습니다.

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

// 사용 예시
if let Some(p) = item.as_any().downcast_ref::<Person>() {
    println!("Found person: {}", p.name);
}
```

여러 타입을 하나의 컬렉션(`Vec<Box<dyn Printable>>`)에 담아두고 루프를 돌며 각 타입 고유의 기능을 실행할 때 유용합니다.

## 표준 라이브러리의 주요 트레이트

`Rust` 표준 라이브러리에는 범용적으로 쓰이는 중요한 트레이트들이 미리 정의되어 있습니다.

### std::fmt::{Display, Debug}

데이터를 문자열로 출력할 때 사용합니다.
*   `Debug`: 개발자용 디버깅 출력입니다. `{:?}` 포맷을 쓰며 `#[derive(Debug)]`로 자동 생성 가능합니다.
*   `Display`: 사용자에게 보여줄 정식 출력입니다. `{}` 포맷을 쓰며 직접 `fmt` 함수를 구현해야 합니다.

### Clone과 Copy

*   `Clone`: 객체를 명시적으로 깊은 복사(`Deep Copy`)할 때 사용합니다. `clone()` 메서드를 호출합니다.
*   `Copy`: 대입 시 자동으로 복사가 일어나는 타입에 부여합니다. `i32` 같은 기본 타입은 이미 구현되어 있습니다.

### Default

구조체 필드를 기본값으로 초기화해 새 객체를 생성할 때 씁니다. `Book::default()`와 같이 호출합니다.

### PartialEq와 Eq

*   `PartialEq`: 두 객체의 값이 같은지 비교합니다. `==` 연산자를 쓸 수 있게 해줍니다.
*   `Eq`: 모든 값이 자기 자신과 같음을 보장하는 수학적 성질을 더합니다. 부동소수점(`f32::NAN`) 같은 예외가 없을 때 사용합니다. `HashMap`의 키로 쓰이려면 필수입니다.

### From과 Into

타입 간 변환을 정의합니다. `From<T>`를 구현하면 `Into<U>`는 자동으로 구현됩니다.

```rust
let isbn: u32 = book.into(); // Book 타입을 u32로 변환
```

성공 여부가 불확실한 변환에는 `TryFrom`과 `TryInto`를 쓰며, 결과로 `Result`를 반환합니다.

### Iterator

데이터 집합을 순회할 때 사용합니다. `next()` 메서드를 구현하면 `for` 루프에서 바로 사용할 수 있습니다.

트레이트는 `Rust` 타입 시스템의 척추와 같습니다. 표준 트레이트들을 잘 활용하고 직접 필요한 트레이트를 정의해 사용함으로써 코드의 안전성과 재사용성을 극대화해 보세요.

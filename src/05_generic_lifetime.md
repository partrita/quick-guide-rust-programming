# 제네릭 (Generic) 프로그래밍 소개

제네릭 프로그래밍은 하나의 코드를 다양한 타입에 재사용할 수 있게 해주는 기법입니다. 제네릭을 이용해 구조체(`struct`), 열거형(`enum`), 함수, 트레이트(`trait`) 등을 만들 수 있습니다. 다음 예제를 통해 제네릭의 핵심을 알아보겠습니다.

```rust
// code/generic_struct/main.rs
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

impl<T> std::ops::Add for Pair<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            first: self.first + rhs.first,
            second: self.second + rhs.second,
        }
    }
}

fn main() {
    let left_pair = Pair { first: 5, second: 10 };
    let right_pair = Pair { first: 10, second: 5 };

    let result = add(left_pair, right_pair);
    println!("Sum: {:?}", result);
}
```

### 코드 분석

가장 먼저 `Pair`라는 구조체를 만들었습니다.

```rust
struct Pair<T> {
    first: T,
    second: T,
}
```

여기서 `T`는 타입 파라미터입니다. `i32` 같은 기본 타입부터 사용자 정의 구조체까지 어떤 타입이든 올 수 있습니다. `C++`의 템플릿이나 `Java`의 제네릭과 문법 및 개념이 매우 유사합니다.

다음은 일반 함수 `add`입니다.

```rust
fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}
```

이 함수는 같은 타입 `T`인 두 인자를 받아 합계를 반환합니다. 여기서 중요한 점은 `where` 절을 이용한 트레이트 바운드 (`Trait Bound`)입니다. `T` 타입이 반드시 `std::ops::Add` 트레이트를 구현하고 있어야 한다는 제약 조건을 걸어준 것입니다. 그래야만 함수 내부에서 `+` 연산을 수행할 수 있기 때문입니다.

우리가 만든 `Pair` 구조체도 `add` 함수에서 사용하려면 `std::ops::Add` 트레이트를 구현해야 합니다.

```rust
impl<T> std::ops::Add for Pair<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            first: self.first + rhs.first,
            second: self.second + rhs.second,
        }
    }
}
```

여기서 `Copy` 트레이트가 추가된 이유는 `add` 메서드 호출 시 인자가 참조가 아닌 값으로 전달되어 소유권 이동이 발생하기 때문입니다. `T` 타입이 비트 단위로 복사 가능한 타입이어야만 안전하게 연산을 수행할 수 있습니다. `i32` 같은 기본 타입은 이미 `Copy`를 구현하고 있습니다.

## 트레이트에 제네릭 사용하기

트레이트 자체에도 제네릭을 적용할 수 있습니다. 이전 장의 `Printable` 예제를 제네릭 버전으로 바꿔보겠습니다.

```rust
// code/generic_trait/main.rs
trait Printable<AGE> {
    fn print(&self);
    fn get_age(&self) -> AGE;
}

impl Printable<u32> for Person {
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.age);
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn print_info(item: &dyn Printable<u32>) {
    item.print();
}
```

연관 타입(`Associated Type`)을 쓰는 방식과 제네릭을 쓰는 방식 중 무엇이 더 낫다고 단정할 수는 없지만, 보통 대상 타입이 또 다른 구조체라면 제네릭을 쓰는 것이 구조적으로 더 깔끔할 때가 많습니다.

## 실전 에러 처리와 제네릭 트레이트 활용

`Rust`의 에러 처리를 더 우아하게 하려면 프로젝트 전용 에러 타입을 만들고 `Result`를 재정의하는 것이 좋습니다.

```rust
#[derive(Debug, PartialEq)]
pub enum MyError {
    UnsupportedCommand,
    WrongInput(String),
    UnknownValue { name: String, expected: String, found: String },
}

pub type Result<T, E = MyError> = std::result::Result<T, E>;
```

이렇게 하면 함수의 반환 타입이 `Result<usize>`처럼 단순해집니다. 에러 타입은 암묵적으로 `MyError`가 되기 때문입니다. 또한 `?` 연산자(`try` 연산자)를 활용해 에러를 상위 함수로 쉽게 전파할 수 있습니다.

계층 구조를 가진 에러를 만들고 싶다면 `From` 트레이트를 구현하면 됩니다.

```rust
impl From<MyError> for SuperError {
    fn from(err: MyError) -> Self {
        Self::CommandError(err)
    }
}
```

이제 하위 모듈에서 발생한 `MyError`가 `?` 연산자를 만나는 순간 자동으로 `SuperError`로 변환되어 상위 모듈로 전달됩니다.

## 객체의 수명을 알리는 라이프타임 (Lifetime)

라이프타임은 댕글링 레퍼런스 (`Dangling Reference`), 즉 이미 해제된 메모리를 가리키는 포인터 문제를 방지하기 위한 기법입니다.

보통 한 구조체가 다른 구조체의 필드를 참조할 때 필요합니다.

```rust
struct Statistics<'a> {
    items: &'a HashMap<String, Item>,
}
```

`'a`라는 이름표가 바로 라이프타임 매개변수입니다. 이는 "`Statistics` 객체는 자신이 참조하는 `items` 데이터보다 더 오래 살아남을 수 없다"는 것을 컴파일러에게 명시적으로 알려주는 약속입니다.

대부분의 경우 컴파일러가 수명을 알아서 판단하지만, 참조 관계가 복잡해지면 직접 수명을 지정해 주어야 합니다. 이때 컴파일러 에러 메시지가 매우 친절하게 안내해 주므로 너무 겁먹을 필요는 없습니다.

# 데이터를 자동으로 해제하는 스마트 포인터

스마트 포인터는 단순한 메모리 주소 그 이상입니다. 데이터 관리를 위한 메타데이터와 자동 메모리 관리 기능을 갖춘 특별한 구조체입니다.

핵심 트레이트 두 가지를 기억하세요:
1.  `Deref` 트레이트: `*` 연산자를 사용해 내부 데이터에 접근할 수 있게 해줍니다.
2.  `Drop` 트레이트: 객체가 스코프를 벗어날 때 자동으로 호출되어 메모리나 자원(파일, 소켓 등)을 정리합니다.

## 로우 포인터 (Raw Pointer)

스마트하지 않은 순수 포인터입니다. 주로 외부 라이브러리 연동이나 로우레벨 시스템 개발 시 사용합니다.

```rust
let mut mutablestr = String::from("hello");
let mutptr: *mut String = &mut mutablestr; // 가변 로우 포인터
```

로우 포인터를 역참조하려면 반드시 `unsafe` 블록을 사용해야 합니다. 포인터가 유효한지 컴파일러가 보장할 수 없기 때문입니다.

## Box<T>: 힙 영역 저장소

가장 기본이 되는 스마트 포인터입니다. 데이터를 힙 (`Heap`) 영역에 저장하고, 그 주소값과 메타데이터만 스택 (`Stack`)에 보관합니다.

*   스택: 크기가 고정된 데이터. 할당/해제가 매우 빠릅니다.
*   힙: 실행 중에 크기가 결정되거나 변할 수 있는 데이터.

`Box<T>`를 직접 구현해 보면 그 원리를 쉽게 이해할 수 있습니다.

```rust
// code/smart_pointer_basic/main.rs
use std::ops::{Deref, DerefMut};

struct MySmartPointer<T>(T);

impl<T> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

impl<T> Drop for MySmartPointer<T> {
    fn drop(&mut self) { println!("Dropping MySmartPointer"); }
}
```

`deref`가 내부 데이터의 레퍼런스를 반환하므로 `*` 연산자로 값을 읽을 수 있고, `drop`이 자동으로 자원을 정리합니다.

## 실무 활용: 트레이트 객체와 Box

서로 다른 구조체들을 하나의 배열에 담고 싶을 때 `Box<dyn Trait>` 형식을 사용합니다.

```rust
let mut items: Vec<Box<dyn GenSerialData>> = vec![
    Box::new(userid),
    Box::new(productid)
];
```

서로 다른 타입이라도 같은 트레이트를 구현했다면 `Box`에 담아 하나의 컬렉션으로 관리할 수 있습니다. 이때 타입 지정을 명확히 해주어야 컴파일러가 트레이트 객체임을 인식할 수 있습니다.

그 외에도 동시성 프로그래밍을 위한 `Arc`, `Mutex`, `RwLock` 등 다양한 스마트 포인터가 존재합니다. `Rust`의 안전한 메모리 관리 철학은 바로 이 스마트 포인터들 위에서 완성됩니다.

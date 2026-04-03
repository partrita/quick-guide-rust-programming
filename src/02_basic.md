# 기본 문법

`Rust` 언어의 기본 문법을 소개하겠습니다. 이 책은 변수나 함수 같은 일반적인 프로그래밍 개념에 대한 설명은 생략하고, `Rust` 언어에 빠르게 적응하기 위해 필요한 핵심 사항 위주로 안내하겠습니다.

## 함수와 for 루프

`FizzBuzz`라는 함수를 만들어 보겠습니다. 1부터 100까지의 숫자 중 3의 배수면 `Fizz`, 5의 배수면 `Buzz`, 3과 5의 공배수면 `FizzBuzz`를 출력하는 프로그램입니다.

기존 `C` 언어 스타일의 `for`문에 익숙하시다면 다음과 같은 형태를 먼저 떠올리실 겁니다.

```c
for (i = 1; i < 100; i++)
```

이런 방식은 종료 조건(`i < 100`)을 실수하기 쉬워 버그에 취약합니다. 100까지 처리해야 하는데 실수로 99까지만 처리하는 식이죠.

`Rust`의 `for` 루프는 오직 반복자(`Iterator`)만 사용합니다. 이는 많은 최신 언어가 채택하고 있는 방식입니다. `for`와 `if` 구문을 사용해 `FizzBuzz` 예제를 구현해 보았습니다.

> 예제 코드는 [https://github.com/gurugio/quick-guide-rust-programming](https://github.com/gurugio/quick-guide-rust-programming)에서 내려받을 수 있습니다.

```rust
// code/function_for/main.rs
fn fizzbuzz_if_else(max: i32) {
    for i in 1..=max {
        let rem_three: i32 = i % 3;
        let rem_five: i32 = i % 5;

        if rem_three == 0 && rem_five == 0 {
            println!("{} - FizzBuzz", i);
        } else if rem_three == 0 {
            println!("{} - Fizz", i);
        } else if rem_five == 0 {
            println!("{} - Buzz", i);
        } else {
            /* do nothing */
        }
    }
}

fn main() {
    println!("Hello, function_for!");
    fizzbuzz_if_else(10);
}
```

아래와 같이 `cargo run` 명령으로 빌드하고 실행할 수 있습니다.

```bash
$ cargo run --bin function_for
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/function_for`
Hello, function_for!
3 - Fizz
5 - Buzz
6 - Fizz
9 - Fizz
10 - Buzz
```

문법 자체는 다른 언어들과 매우 유사합니다. 몇 가지 핵심적인 특징만 살펴보겠습니다.

프로그램은 `main` 함수에서 시작합니다. `println!` 매크로를 호출해 메시지를 출력하고 `fizzbuzz_if_else` 함수를 호출합니다.

함수 정의는 `fn` 키워드로 시작합니다.

```rust
fn 함수이름(인자: 타입, 인자: 타입, ...) -> 반환값 {
    함수 라인1;
    라인2;
    결과값
}
```

각 라인은 `;`으로 끝나야 하며, 함수의 반환값은 마지막에 `;` 없이 작성합니다. 이는 `Scala` 같은 함수형 언어와 비슷한 특징입니다. 반환값이 없으면 생략하거나 `()`를 사용합니다.

`Rust`에서는 `Range` 타입으로 루프 범위를 지정합니다. `1..10`은 10을 포함하지 않는 1부터 9까지를 의미하며, 10을 포함하려면 `1..=10`과 같이 `=`를 추가해야 합니다.

`if`문에서 마지막 `else`는 아무것도 하지 않더라도 명시해 주는 것이 좋습니다. 이는 모든 케이스를 완벽히 처리한다는 의지를 보여주며 보안상으로도 권장되는 습관입니다.

변수는 `let` 키워드로 정의합니다.

```rust
let 변수이름: 타입 = 초기값;
```

## if-else

`Rust`의 `if-else`는 문장이 아닌 표현식(Expression)입니다. 즉, 결과를 반환할 수 있습니다.

```rust
// code/if/main.rs
fn main() {
    let num = 5;
    let var = if num % 3 == 0 {
        3
    } else {
        if num % 5 == 0 {
            5
        } else {
            0
        }
    };
    println!("var = {}", var);
}
```

첫 번째 `if` 블록 안의 `3` 뒤에 `;`가 없는 것에 주목하세요. 이는 `3`이 블록의 결과값이 되어 `var`에 저장된다는 의미입니다. `C` 언어의 삼항 연산자(`? :`)와 비슷한 역할을 수행하면서도 훨씬 명확합니다.

왜 `if`가 값을 가지도록 설계했을까요? 가장 큰 장점은 초기화되지 않은 변수를 줄일 수 있다는 점입니다. `C` 언어 등에서 변수 선언만 하고 나중에 `if-else` 내에서 초기화하다가 실수로 특정 경로를 빠뜨리면, 쓰레기 값이 담긴 변수가 프로그램 전체를 위협할 수 있습니다. `Rust`에서는 컴파일러가 이를 엄격히 체크하여 모든 경로에서 변수가 확실히 초기화되도록 강제합니다.

참고로 `Rust`에서는 `match`, `map`, `filter` 등을 활용하는 함수형 스타일을 선호하므로, `if-else`가 너무 많아진다면 더 `Rust`다운 코드로 개선할 여지가 있는지 살펴보는 것이 좋습니다.

## Mutable 변수와 타입 추론

`Rust`의 모든 변수는 기본적으로 불변(Immutable)입니다. 값을 변경하려면 `mut` 키워드를 붙여야 합니다.

```rust
// code/mutable_var/main.rs
fn fib(mut index: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut t;

    loop {
        t = a + b;
        a = b;
        b = t;

        index -= 1;
        if index <= 0 {
            break;
        }
    }
    b
}

fn main() {
    println!("{}", fib(10));
}
```

이 예제에는 세 가지 중요한 개념이 담겨 있습니다.

1.  `mut` 키워드: 값이 변하는 변수와 인자에 사용합니다.
2.  `loop`: 조건 없는 무한 루프를 생성하며 `break`로 탈출합니다.
3.  타입 추론: 컴파일러가 변수 사용 맥락을 보고 타입을 알아서 결정합니다. 덕분에 모든 곳에 일일이 타입을 적을 필요가 없습니다. 다만, 함수 정의에서는 타입을 생략할 수 없습니다.

## as를 이용한 타입 변환

`Rust`는 암묵적인 타입 변환을 허용하지 않습니다. 타입 변환은 항상 명시적이어야 하며, 이때 `as` 키워드를 사용합니다.

```rust
fn string_to_digit(input: String) -> i32 {
    let mut ret = 0;
    for c in input.chars() {
        ret = ret * 10;
        ret += c as i32 - '0' as i32; // char를 i32로 명시적 변환
    }
    ret
}
```

`C` 언어처럼 문자와 숫자를 섞어서 연산할 수 없습니다. 논리적으로 문자와 숫자는 다른 타입이기 때문입니다. `Rust`는 이런 엄격함을 통해 보안 사고를 미연에 방지합니다.

또한 `Rust`는 에러 처리를 위해 `Result`와 `Option` 타입을 적극적으로 활용합니다. 이는 에러를 단순한 숫자나 `NULL`로 표현하던 방식에서 벗어나, 논리적으로 완벽한 에러 처리를 가능하게 합니다.

## match를 이용한 패턴 매칭

`Rust`의 가장 강력한 기능 중 하나인 패턴 매칭(Pattern Matching)입니다. `switch`문보다 훨씬 강력하며 유연합니다.

```rust
// code/match/main.rs
fn fizzbuzz_2(max: i32) {
    for i in 1..=max {
        match (i % 3, i % 5) {
            (0, 0) => println!("{} - FizzBuzz", i),
            (0, _) => println!("{} - Fizz", i),
            (_, 0) => println!("{} - Buzz", i),
            _ => (),
        }
    }
}
```

`(i % 3, i % 5)`와 같이 튜플(`tuple`)을 만들어 매칭할 수도 있습니다. 언더바(`_`)는 "나머지 모든 경우"를 의미합니다. `match` 또한 표현식이므로 결과값을 반환할 수 있으며, 컴파일러는 모든 케이스가 처리되었는지(Exhaustive check)를 완벽히 검사합니다.

## 값을 가지는 표현식(Expression)

`Rust`는 거의 모든 것이 표현식입니다. `{}` 블록 자체도 값을 가집니다.

```rust
let y = {
    let x_squared = x * x;
    let x_cube = x_squared * x;

    // 마지막 줄에 세미콜론이 없으므로 이 값이 y에 할당됨
    x_cube + x_squared + x
};
```

이러한 특성을 이용하면 변수 초기화 코드를 매우 깔끔하게 정리할 수 있습니다. 표현식 끝에 `;`을 붙이면 문장(Statement)이 되어 결과값이 무시됩니다.

## 배열과 슬라이스

배열은 고정된 크기의 연속된 데이터 구조입니다. `[타입; 크기]` 형태로 정의합니다.

슬라이스(`Slice`)는 배열의 일부 또는 전체를 가리키는 참조(&)입니다.

```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
let slice: &[i32] = &numbers[1..4]; // 1번부터 3번 인덱스까지 참조
```

슬라이스는 내부적으로 길이 정보를 포함하고 있어 안전합니다. `C` 언어의 포인터 연산에서 발생하는 메모리 범위를 벗어난 접근(Buffer Overflow) 문제를 언어 차원에서 방지합니다. 함수 인자로 배열 데이터를 전달할 때는 대개 슬라이스(`&[T]`) 형식을 사용합니다.

## 문자열 String과 &str

`Rust`에서 문자열은 크게 두 가지 타입을 사용합니다.

1.  `String`: 힙(`heap`)에 저장되는 동적 문자열. 크기를 늘릴 수 있습니다.
2.  `&str`: 문자열 슬라이스. 문자열 리터럴(`"hello"`)이나 `String`의 일부를 가리킵니다.

`Rust` 문자열은 기본적으로 `UTF-8` 형식을 따릅니다. 따라서 `ASCII`처럼 1바이트 고정 크기가 아니므로 인덱스를 통한 직접 접근(`s[0]`)을 막아두었습니다. 대신 `chars()` 반복자를 사용해 안전하게 접근해야 합니다.

`String` 객체를 함수에 그냥 넘기면 소유권(Ownership)이 이동하여 더 이상 사용할 수 없게 됩니다. 따라서 함수 인자로는 주로 `&str` 형식을 사용하여 읽기 권한만 빌려주는 방식을 선호합니다.

## 소유권(Ownership)

`Rust` 최고의 강점이자 입문자가 가장 먼저 만나는 장벽입니다. 하지만 소유권 덕분에 가비지 컬렉터 없이도 메모리 안전성을 완벽히 보장할 수 있습니다.

소유권의 세 가지 규칙:
1. 모든 값은 고유한 소유자(Owner)가 있습니다.
2. 소유자는 한 번에 단 하나만 존재합니다.
3. 소유자가 스코프(`scope`)를 벗어나면 값은 자동으로 해제됩니다.

이 규칙을 통해 메모리 누수나 이중 해제(Double Free) 같은 고전적인 버그들을 컴파일 타임에 차단합니다. 데이터를 다른 곳에 빌려줄 때는 참조(&)를 사용하며, 이를 빌림(Borrowing)이라고 부릅니다.

## 구조체(Struct)

`Rust`는 클래스 대신 구조체를 사용합니다. 데이터와 메서드를 분리하여 정의하며, 상속보다는 구성을 선호합니다.

```rust
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Self {
        Person { name: name.to_string(), age }
    }

    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}
```

`impl` 블록을 통해 구조체의 메서드를 정의합니다. `&self`는 인스턴스 메서드를, `self` 없이 정의하면 연관 함수(정적 메서드)가 됩니다.

## 열거형(Enum)

`Rust`의 열거형은 단순히 숫자를 나열하는 것을 넘어, 내부에 데이터를 담을 수 있는 강력한 도구입니다.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

`match` 구문과 결합하면 복잡한 상태를 안전하고 명확하게 처리할 수 있습니다. 특히 `Option<T>`와 `Result<T, E>`는 `Rust`에서 가장 중요한 열거형으로, 값의 부재나 에러 상황을 다루는 핵심 기법입니다.

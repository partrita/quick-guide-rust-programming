# 함수형 프로그래밍

## 함수형 프로그래밍 소개

러스트는 기본적으로 절차형 프로그래밍 언어(Imperative programming language)입니다. 함수형 프로그래밍 언어의 장점들을 도입했을 뿐, 러스트가 하스켈 같은 순수 함수형 프로그래밍 언어라고 말할 수는 없습니다. 그렇다면 함수형 프로그래밍이란 무엇이며, 왜 러스트에 함수형 언어의 패러다임을 도입했을까요?

위키백과에 따르면 "패러다임(영어: paradigm)은 어떤 한 시대 사람들의 견해나 사고를 근본적으로 규정하고 있는 테두리로서의 인식 체계, 또는 사물에 대한 이론적인 틀이나 체계를 의미하는 개념이다." (출처: https://ko.wikipedia.org/wiki/패러다임)라고 합니다. 설명이 다소 어렵지만, 핵심은 프로그램을 어떻게 하면 더 잘 만들 수 있을지 고민하는 여러 방식 중 하나가 바로 함수형 프로그래밍 패러다임이라는 점입니다. 저는 Scheme나 Scala를 1~2년 정도 공부한 경험이 있는데, 좋은 개발자가 되기 위해 한 번쯤 깊게 빠져볼 만한 패러다임이라고 생각합니다. 특정 언어가 좋다는 접근보다는 함수형 프로그래밍 패러다임 그 자체를 접하기 위해 언어를 도구로 활용하시길 바라는 마음에서 '패러다임'이라는 용어를 강조해 보았습니다.

함수형 언어의 역사나 종류 등 상세한 내용은 다른 자료를 참고해 주시기 바랍니다. 여기서는 러스트에도 적용되는 함수형 언어의 주요 조건들만 설명하겠습니다.

1. **순수 함수를 지원한다**

순수 함수는 같은 값을 입력하면 항상 같은 반환값을 얻을 수 있고, 외부의 데이터를 수정하지 않는 함수입니다. 다음 예제를 통해 차이를 확인해 보겠습니다.

```rust
fn imperative_add_one(x: &mut i32) {
    *x += 1;
}

fn functional_add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let mut y = 1;
    imperative_add_one(&mut y);
    println!("y={}", y);
    
    let y = 1;
    println!("y={}", functional_add_one(y));
}
```

`imperative_add_one` 함수는 비순수 함수입니다. 외부 인자인 `y`를 받아서 직접 수정하기 때문입니다. 반면 `functional_add_one` 함수는 순수 함수입니다. 어떤 인자를 받아도 수정하지 않으며, 1을 전달하면 언제나 2를 반환합니다. 가능한 한 많은 함수를 순수 함수로 만드는 연습을 하는 것이 좋습니다. 순수 함수는 버그 발생 위험이 적고, 병렬 실행 시에도 문제가 생기지 않기 때문입니다. 코드를 짤 때 최대한 순수 함수를 활용하고 불가피한 경우에만 상태를 갖는 함수로 만든다면, 문제가 생겼을 때 비순수 함수 위주로 확인하여 더 빠르게 해결할 수 있습니다.

2. **상태를 가지지 않거나 변하지 않는 데이터를 사용한다**
러스트의 모든 변수는 별도의 표시가 없으면 불변(`immutable`) 타입입니다. 또한 함수 내부의 마지막 줄이 곧 반환값이 되는 방식도 여러 함수형 언어가 가지는 특징입니다.

3. **함수가 1급 객체가 되어 고차 함수의 인자나 반환값으로 사용될 수 있다**
러스트에서는 함수를 변수에 저장하거나, 다른 함수의 인자나 반환값으로 사용할 수 있습니다. 뒤에서 함수 포인터와 `closure`를 소개하며 자세히 다루겠습니다.

4. **모든 블록 구문이 값을 반환한다**
이미 `if-else` 예제에서 확인했듯이, `{`로 시작해서 `}`로 끝나는 블록은 값을 반환합니다. 함수도 블록이므로 값을 반환하고, `if-else`도 블록이므로 값을 반환합니다. 보통 다음과 같이 `let` 구문으로 값을 할당할 때 블록의 반환값을 사용합니다. 이 블록 안에는 `if-else`와 같은 복잡한 코드를 넣을 수도 있습니다.

```rust
fn main() {
    let _y = { 1 };
}
```

그렇다면 러스트는 왜 함수형 언어의 패러다임을 도입했을까요?

1. 순수 함수와 불변 데이터를 사용하면 프로그램의 구조를 적절히 나누기 쉬워집니다. 또한 순수 함수로 구성된 코드는 예측 가능하므로 버그를 잡기 쉽고, 코드 재사용성도 높아집니다.
2. 성능 면에서 유리합니다. `closure`와 `iterator`를 잘 조합하면 컴파일러가 최적화할 수 있는 여지가 많아집니다.
3. 가장 큰 장점은 코드가 간결해지고 가독성이 좋아진다는 것입니다. `closure`, `map`, `iterator` 등을 잘 활용하면 `if-else`나 `for` 루프를 거의 사용하지 않고도 프로그램을 만들 수 있습니다. 보통 버그는 복잡한 조건 체크나 루프 처리에서 자주 발생하는데, 함수형 스타일을 사용하면 이러한 위험을 줄일 수 있습니다.
4. `thread`나 `async`를 사용하는 비동기 프로그래밍에 활용하기 좋습니다.

물론 이러한 장점을 누리려면 함수형 스타일과 `iterator`, `map` 등의 특징에 익숙해져야 합니다. 처음에는 사고 과정이 낯설어 어렵게 느껴질 수 있지만, 연습을 통해 익숙해지면 어느 순간 그 간결함과 강력함을 느끼게 될 것입니다.

## 이터레이터 (Iterator)

사실 `iterator`가 함수형 프로그래밍에 속하는지에 대해서는 논쟁이 있습니다. 구현 방식에 따라 내부 상태를 저장하기도 하므로 엄격하게는 함수형 스타일에 맞지 않을 수 있기 때문입니다. 예를 들어 소켓에서 데이터를 단순히 읽기만 하는 `iterator`는 상태 관리가 없을 수 있지만, 파일에서 데이터를 읽어오는 경우 현재 위치를 저장해야 하므로 상태가 필요합니다. 하지만 순수 함수를 만들기 위해 `iterator`가 필요한 경우가 많으므로, 보통 함수형 프로그래밍과 함께 소개됩니다.

이전 장에서 `iterator`를 사용하는 예제를 여러 번 만들어 보았습니다. 여기서는 `iterator`를 직접 구현해 보며 러스트에서 어떻게 동작하는지 알아보겠습니다. 정확히는 `trait`라는 기능을 사용해야 하는데, 이는 다음 장에서 자세히 설명하겠습니다. 이번 장에서는 내가 만든 구조체에 `next`라는 `method`가 추가된다는 점에 집중해 주시기 바랍니다. 파이썬의 `next` `method`를 사용해 보셨다면 익숙하실 것입니다.

`iterator`는 단순히 데이터를 순서대로 처리하는 것뿐만 아니라 지연 처리(Lazy evaluation)를 위한 중요한 기법입니다. 모든 데이터를 미리 만들어 놓으면 메모리와 프로세서 자원이 낭비될 수 있지만, `iterator`를 사용하면 필요한 시점에 데이터를 생성하므로 자원을 효율적으로 사용할 수 있습니다.

지연 처리의 대표적인 예인 피보나치 수열 생성을 `iterator`로 구현해 보겠습니다.

```rust
// code/functional_iterator/main.rs
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

fn main() {
    let mut fib_iter = Fibonacci { curr: 0, next: 1 };
    println!("next returns: {}", fib_iter.next().unwrap());
    println!("next returns: {}", fib_iter.next().unwrap());
    println!("next returns: {}", fib_iter.next().unwrap());

    for i in fib_iter {
        println!("In a loop: {}", i);
        if i > 100 {
            break;
        }
    }
}
```

피보나치 수열을 생성하는 코드는 간단합니다. `Iterator` `trait`에 맞춰 `next` `method`를 구현해 주기만 하면 됩니다.

```rust
impl Iterator for <구조체 이름> {
    type Item = <데이터 타입>;

    fn next(&mut self) -> Option<Self::Item> {
        // 구현 내용
        // 반환값은 Some(<Item 타입의 값>) 또는 None이어야 함
    }
}
```

이렇게 구현하면 러스트 컴파일러가 `for` 루프 등에 필요한 코드를 자동으로 생성해 줍니다. `next`가 호출될 때마다 `fib_iter` 내부의 `curr`와 `next` 값을 이용해 새 값을 만들어 냅니다. `for` 루프에서 사용할 때도 이전까지 생성된 위치 이후부터 수열을 이어 나갑니다. 현재 구현은 메모리 효율을 위해 이전 값을 저장하지 않으므로, 지나간 값을 다시 확인하려면 별도의 버퍼를 두는 등의 추가 구현이 필요합니다.

## 클로저 (Closure)

### 함수 포인터 타입 fn

러스트는 C/C++처럼 함수 포인터를 지원합니다. 다음은 `fizzbuzz` 함수를 함수 포인터로 구현한 예제입니다.

```rust
// code/functional_function_pointer/main.rs
fn fizzbuzz_fn(fizzfn: fn(i32) -> bool, buzzfn: fn(i32) -> bool) {
    for i in 1..=100 {
        if fizzfn(i) && buzzfn(i) {
            println!("FizzBuzz");
        } else if fizzfn(i) {
            println!("Fizz");
        } else if buzzfn(i) {
            println!("Buzz");
        }
    }
}

fn fizzcheck(n: i32) -> bool {
    n % 3 == 0
}

fn buzzcheck(n: i32) -> bool {
    n % 5 == 0
}

fn main() {
    fizzbuzz_fn(fizzcheck, buzzcheck);
}
```

`fizzbuzz_fn` 함수의 인자를 보면 `fn`이라는 키워드가 있는데, 이것이 함수 포인터를 나타내는 타입입니다. 러스트에서는 명확하게 `fn`이라는 타입 이름을 붙여 사용합니다. `i32` 인자를 하나 받아 `bool`을 반환하는 함수 포인터는 `fn(i32) -> bool`로 선언합니다. 함수 포인터는 단순히 함수 코드가 위치한 메모리 주소일 뿐이며, 다음에 설명할 `closure`와 비교하기 위해 소개했습니다.

### 클로저 타입 Fn

`closure`는 이름이 없는 익명 함수입니다. `fizzbuzz` 예제를 `closure`를 사용하도록 바꿔 보겠습니다.

```rust
// code/functional_closure_nocapture/main.rs
fn fizzbuzz_fn(fizzfn: fn(i32) -> bool, buzzfn: fn(i32) -> bool) {
    for i in 1..=100 {
        if fizzfn(i) && buzzfn(i) {
            println!("FizzBuzz");
        } else if fizzfn(i) {
            println!("Fizz");
        } else if buzzfn(i) {
            println!("Buzz");
        }
    }
}

fn main() {
    fizzbuzz_fn(|x| x % 3 == 0, |y| y % 5 == 0);
}
```

`|` 사이에 인자를 넣고 그 뒤에 실행 코드를 적습니다. 외부 변수를 캡처하지 않는 `closure`는 함수 포인터(`fn`) 타입으로 자동 변환될 수 있습니다.

### 외부 변수를 캡처하는 Fn과 FnMut

만약 `closure`가 영역 밖의 변수를 참조(캡처, capture)한다면 더 이상 단순한 `fn` 타입으로 처리할 수 없습니다.

```rust
fn main() {
    let fizz = 3;
    let buzz = 5;
    // 아래 코드는 컴파일 에러가 발생합니다.
    // fizzbuzz_fn(|x| x % fizz == 0, |y| y % buzz == 0);
}
```

에러 메시지는 `closure`가 변수를 캡처했으므로 `fn` 타입이 기대되는 곳에 전달할 수 없다고 알려줍니다. 이를 해결하려면 제네릭과 `Fn` `trait`를 사용해야 합니다.

```rust
// code/functional_closure_capture/main.rs
fn fizzbuzz_fn<FA, FB>(fizzfn: FA, buzzfn: FB)
where
    FA: Fn(i32) -> bool,
    FB: Fn(i32) -> bool,
{
    for i in 1..=100 {
        if fizzfn(i) && buzzfn(i) {
            println!("FizzBuzz");
        } else if fizzfn(i) {
            println!("Fizz");
        } else if buzzfn(i) {
            println!("Buzz");
        }
    }
}

fn main() {
    let fizz = 3;
    let buzz = 5;
    fizzbuzz_fn(|x| x % fizz == 0, |y| y % buzz == 0);
}
```

`Fn` `trait`는 외부 변수를 불변 참조로 캡처할 때 사용합니다. 만약 외부 변수를 수정해야 한다면 `FnMut`를 사용해야 하지만, 이는 함수형 프로그래밍 패러다임에는 잘 맞지 않습니다. 가능한 한 변수를 수정하지 않는 방향으로 설계하는 것을 권장합니다.

## Map

`map`, `filter`, `reduce`는 함수형 프로그래밍의 핵심 개념입니다. `map`은 데이터 집합의 각 요소에 동일한 함수를 적용하여 새로운 결과 집합을 만드는 작업입니다. 러스트에서는 `iterator`뿐만 아니라 `Option`과 `Result`에서도 `map` `method`를 지원합니다.

### 이터레이터의 map 사용법

`for` 루프 대신 `map`을 사용하면 코드가 간결해지고, 컴파일러 최적화 덕분에 성능이 더 좋아지기도 합니다.

```rust
// code/functional_map/main.rs
fn fizzbuzz_3(max: i32) {
    let ret = (1..=max)
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => format!("{} - FizzBuzz\n", i),
            (0, _) => format!("{} - Fizz\n", i),
            (_, 0) => format!("{} - Buzz\n", i),
            _ => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("");
    println!("{}", ret);
}
```

`map` `method`는 새로운 `iterator`를 반환할 뿐, 실제 연산을 즉시 수행하지 않습니다(지연 처리). 따라서 마지막에 `collect`를 호출하여 실제 연산을 수행하고 결과를 수집해야 합니다. `collect::<Vec<String>>()`처럼 반환할 타입을 명시적으로 지정해 주어야 하는 경우가 많습니다.

### Option과 Result의 map 사용법

`Option`이나 `Result` 타입에서도 `map`을 자주 사용합니다.

```rust
// code/functional_map_option/main.rs
fn main() {
    let some_number = Some(5);
    let double_some = some_number.map(|x| x * 2); // Some(10)
    
    let none_number: Option<i32> = None;
    let double_none = none_number.map(|x| x * 2); // None
}
```

`Option`의 `map`은 값이 `Some`일 때만 내부 값을 꺼내 `closure`를 적용하고 결과를 다시 `Option`으로 감싸줍니다. `None`일 때는 아무 작업도 하지 않고 `None`을 반환합니다. 이 방식을 사용하면 `match`나 `if let`을 사용해 값을 일일이 꺼내지 않아도 되므로 코드가 훨씬 깔끔해집니다. `Result` 역시 `Ok`일 때만 `map`이 동작하며, `Err`일 때는 에러를 그대로 반환합니다.

### map 디버깅: inspect

`map` 체이닝 중간에 값을 확인하고 싶을 때는 `inspect` `method`를 사용합니다. `inspect`는 데이터를 소비하거나 변경하지 않고 그대로 다음 단계로 전달하면서, 중간에 필요한 작업(예: 로그 출력)을 수행할 수 있게 해줍니다.

```rust
// code/functional_map_inspect/main.rs
let ret = (1..=max)
    .map(|i| /* ... */)
    .inspect(|s| println!("map returns {}", s))
    .collect::<Vec<String>>();
```

### map 사용 시 주의사항: 소유권 소비(Consume)

`map` `method`는 기본적으로 값을 소비(`consume`)합니다. 즉, 연산 후에는 원본 변수를 다시 사용할 수 없습니다. 만약 원본 데이터를 유지해야 한다면 `as_ref`나 `as_mut`를 사용하여 참조 타입으로 변환한 뒤 `map`을 호출해야 합니다.

```rust
let maybe_some_string = Some(String::from("Hello"));
let maybe_len = maybe_some_string.as_ref().map(|s| s.len());
println!("{:?}", maybe_some_string); // as_ref를 썼으므로 사용 가능
```

## Filter

`filter`는 특정 조건을 만족하는 데이터만 걸러내는 역할을 합니다.

```rust
let fizz = (1..=50)
    .into_iter()
    .filter(|i| i % 3 == 0)
    .collect::<Vec<_>>();
```

`filter`에 전달되는 `closure`의 인자는 데이터의 참조 타입(`&T`)입니다. 따라서 값을 비교할 때 역참조(`*i`)가 필요할 수 있습니다.

## Reduce

`reduce`는 모든 요소를 하나로 합치는 연산을 수행합니다.

```rust
let sum = (1..=10)
    .reduce(|a, b| a + b); // Some(55)
```

`reduce`는 `iterator`를 소비하며 최종 결과값을 반환하므로 별도의 `collect`가 필요하지 않습니다. 첫 번째 요소를 초기값으로 사용하며, `iterator`가 비어있을 수 있으므로 결과는 `Option` 타입으로 반환됩니다.

함수형 프로그래밍 스타일은 처음에는 낯설 수 있지만, `map`, `filter`, `reduce`를 적절히 활용하면 버그가 적고 가독성이 뛰어난 코드를 작성할 수 있습니다. 특히 러스트의 강력한 타입 시스템과 결합하여 안전하고 효율적인 프로그램을 만드는 데 큰 도움이 됩니다.

# 함수형 프로그래밍

## 함수형 프로그래밍 소개

`Rust`는 기본적으로 명령형 프로그래밍 언어(`Imperative programming language`)입니다. 함수형 프로그래밍의 장점들을 대거 도입했을 뿐, `Haskell` 같은 순수 함수형 언어라고 볼 수는 없습니다. 그렇다면 함수형 프로그래밍이란 무엇이며, `Rust`는 왜 이 패러다임을 받아들였을까요?

위키백과에 따르면 "패러다임(`Paradigm`)은 어떤 한 시대 사람들의 견해나 사고를 근본적으로 규정하고 있는 테두리로서의 인식 체계, 또는 사물에 대한 이론적인 틀이나 체계를 의미하는 개념이다." ([출처](https://ko.wikipedia.org/wiki/패러다임))라고 합니다. 말이 좀 어렵지만, 핵심은 프로그램을 더 잘 만들기 위해 고민하는 여러 방식 중 하나가 바로 함수형 프로그래밍 패러다임이라는 점입니다. 저는 `Scheme`나 `Scala`를 공부한 경험이 있는데, 좋은 개발자가 되기 위해 한 번쯤 깊게 빠져볼 만한 가치가 있는 분야라고 생각합니다. 특정 언어의 우수성보다는 '함수형 패러다임' 자체를 도구로 활용해 보시길 권합니다.

함수형 언어의 역사나 상세 이론은 다른 자료를 참고해 주시고, 여기서는 `Rust`에도 적용되는 함수형 언어의 주요 특징들을 살펴보겠습니다.

1.  순수 함수(`Pure Function`) 지원
    순수 함수는 같은 입력에 대해 항상 같은 결과값을 반환하며, 함수 외부의 데이터를 수정하지 않는 함수입니다.

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

    `imperative_add_one`은 인자로 받은 `y`를 직접 수정하는 비순수 함수입니다. 반면 `functional_add_one`은 입력값을 수정하지 않고 언제나 1을 더한 새 값을 반환하는 순수 함수입니다. 순수 함수는 버그 발생 위험이 적고 병렬 처리에 유리하므로, 가능한 한 많은 로직을 순수 함수로 구성하는 것이 좋습니다.

2.  불변 데이터 사용
    `Rust`의 모든 변수는 기본적으로 불변(`Immutable`)입니다. 또한 함수 내부의 마지막 줄 결과가 곧 반환값이 되는 방식도 함수형 언어의 전형적인 특징입니다.

3.  함수가 일급 객체(`First-class Citizen`)
    함수를 변수에 저장하거나 다른 함수의 인자 또는 반환값으로 사용할 수 있습니다. 뒤에서 다룰 함수 포인터와 클로저(`Closure`)가 이에 해당합니다.

4.  모든 블록 구문의 값 반환
    앞서 살펴봤듯이 `{}`로 둘러싸인 블록은 값을 반환합니다. `if-else` 블록도 값을 가지며, 이를 `let` 구문에 직접 할당할 수 있습니다.

`Rust`가 함수형 패러다임을 도입한 이유:
*   안전성: 순수 함수와 불변 데이터를 사용하면 프로그램 구조가 명확해지고 버그를 잡기 쉬워집니다.
*   최적화: 클로저와 반복자(`Iterator`)를 조합하면 컴파일러가 최적화할 여지가 많아져 성능 면에서 유리합니다.
*   간결함: `map`, `filter`, `reduce` 등을 활용하면 복잡한 `if-else`나 `for` 루프 없이도 훨씬 가독성 좋은 코드를 짤 수 있습니다.
*   동시성: 스레드(`Thread`)나 비동기(`Async`) 프로그래밍에서 데이터 경합(Race Condition) 문제를 피하기 좋습니다.

물론 함수형 스타일에 익숙해지려면 시간이 조금 필요하지만, 그 간결함과 강력함을 한 번 경험하고 나면 다시 예전으로 돌아가기 어려울 것입니다.

## 반복자 (Iterator)

사실 반복자(`Iterator`)를 함수형 프로그래밍의 일부로 볼 것인지는 논쟁의 여지가 있습니다. 구현 방식에 따라 내부 상태를 가지기도 하기 때문입니다. 하지만 순수 함수형 로직을 짜는 데 반복자가 필수적인 역할을 하므로 보통 함께 소개됩니다.

이미 `for` 루프 예제에서 반복자를 사용해 보았습니다. 여기서는 반복자를 직접 구현하며 `Rust`에서 어떻게 동작하는지 알아보겠습니다. 정확히는 트레이트(`Trait`) 기능을 사용해야 하는데, 자세한 내용은 다음 장에서 다루고 이번에는 내 구조체에 `next`라는 메서드가 추가된다는 점에 집중해 주세요.

반복자는 데이터를 순서대로 처리할 뿐만 아니라, 필요한 시점에만 데이터를 생성하는 지연 처리(`Lazy Evaluation`)를 위한 핵심 도구입니다.

피보나치 수열 생성을 반복자로 구현해 보겠습니다.

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

`Iterator` 트레이트의 `next` 메서드를 구현하면 `Rust` 컴파일러가 `for` 루프 등에 필요한 코드를 자동으로 연결해 줍니다. 수열의 다음 값을 계산하고 `Some(값)`을 반환하며, 끝내고 싶을 때 `None`을 반환하면 됩니다.

## 클로저 (Closure)

### 함수 포인터 타입 fn

`Rust`는 `C`/`C++`처럼 함수 포인터를 지원합니다. `fn` 키워드가 그 역할을 합니다.

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

fn fizzcheck(n: i32) -> bool { n % 3 == 0 }
fn buzzcheck(n: i32) -> bool { n % 5 == 0 }

fn main() {
    fizzbuzz_fn(fizzcheck, buzzcheck);
}
```

`fn(i32) -> bool`처럼 명시적인 타입을 가지며, 단순한 함수의 주소값을 나타냅니다.

### 클로저 타입 Fn

클로저(`Closure`)는 이름 없는 익명 함수입니다. `|인자| 실행코드` 형태로 작성합니다.

```rust
// code/functional_closure_nocapture/main.rs
fn main() {
    // 외부 변수를 캡처하지 않는 클로저는 fn 타입으로 자동 변환됩니다.
    fizzbuzz_fn(|x| x % 3 == 0, |y| y % 5 == 0);
}
```

### 외부 변수를 캡처하는 Fn과 FnMut

클로저가 주변 영역의 변수를 참조(캡처)하면 더 이상 단순한 `fn` 타입으로 취급할 수 없습니다. 이때는 제네릭(`Generic`)과 `Fn` 트레이트를 사용해야 합니다.

```rust
// code/functional_closure_capture/main.rs
fn fizzbuzz_fn<FA, FB>(fizzfn: FA, buzzfn: FB)
where
    FA: Fn(i32) -> bool,
    FB: Fn(i32) -> bool,
{
    // ... 루프 로직 ...
}

fn main() {
    let fizz = 3;
    let buzz = 5;
    // fizz와 buzz 변수를 캡처하여 사용합니다.
    fizzbuzz_fn(|x| x % fizz == 0, |y| y % buzz == 0);
}
```

`Fn`은 불변 참조로 캡처할 때, `FnMut`은 값을 수정해야 할 때 사용합니다. 가급적 변수를 수정하지 않는 설계를 지향하는 것이 좋습니다.

## Map, Filter, Reduce

이 세 가지는 함수형 프로그래밍의 삼신기와 같습니다.

### Map

데이터 집합의 각 요소에 함수를 적용해 새로운 결과 집합을 만듭니다. 반복자뿐만 아니라 `Option`과 `Result`에서도 지원합니다.

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

`map`은 지연 처리되므로 마지막에 `collect()`를 호출해야 실제 연산이 일어납니다.

### Filter

조건을 만족하는 요소만 골라냅니다.

```rust
let fizz = (1..=50)
    .into_iter()
    .filter(|i| i % 3 == 0) // 3의 배수만 필터링
    .collect::<Vec<_>>();
```

### Reduce

모든 요소를 하나로 합칩니다. 합계나 평균 등을 구할 때 유용합니다. 결과는 `Option` 타입으로 반환됩니다.

```rust
let sum = (1..=10).reduce(|a, b| a + b); // Some(55)
```

이러한 함수형 기법들을 적절히 활용하면 버그는 줄어들고 코드는 훨씬 아름다워집니다. `Rust`의 강력한 타입 시스템 위에서 안전하고 우아한 프로그램을 만들어 보세요.

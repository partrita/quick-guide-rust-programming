# 기본 문법

`Rust` 언어의 기본 문법을 소개하겠습니다. 이 책은 변수가 무엇인지, 함수가 무엇인지 등 일반적인 프로그래밍에 대한 설명은 생략하고, `Rust` 언어에 빠르게 적응하기 위해 필요한 사항들만 소개하겠습니다.

## 함수와 for 루프

`FizzBuzz`라는 함수를 만들어 보겠습니다. 1부터 100까지의 숫자 중 3의 배수를 만나면 `Fizz`라고 출력하고, 5의 배수를 만나면 `Buzz`라고 출력합니다. 만약 3과 5의 공배수이면 `FizzBuzz`라고 출력합니다.

만약 아래와 같이 `for`문에 시작값과 종료 조건 등을 지정하는 문법만을 사용해 보셨다면, 이런 형태의 `for`문은 버그에 굉장히 취약하다는 것을 경험해 보셨을 것입니다. "1부터 100까지의 숫자"라는 조건을 봤다면 다음처럼 생각하기 쉽습니다.

```c
for (i = 1; i < 100; i++)
```

이렇게 하면 `i`가 100일 때 처리를 하지 못하는데요, 이런 식으로 실수하기 쉬운 문법을 가지고 있습니다.

러스트의 `for`문에는 오직 이터레이터(`Iterator`)만 사용합니다. 대부분의 최신 언어들이 이미 제공하는 기능입니다. `for`와 `if` 구문을 사용해서 3의 배수이면 `Fizz`라는 메시지를 출력하고, 5의 배수이면 `Buzz`라는 메시지를 출력하며, 3과 5의 공배수이면 `FizzBuzz`를 출력하는 예제를 만들어 보았습니다.

>
> 예제 코드는 <https://github.com/gurugio/quick-guide-rust-programming>에서 다운로드할 수 있습니다.
>

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

아래와 같이 `function_for/main.rs` 파일을 빌드하고 실행할 수 있습니다. `cargo run` 명령은 빌드와 실행을 모두 수행하는 명령입니다.

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

예제 자체는 아주 간단하고 다른 많은 언어에서 사용되는 문법들과 유사합니다. 예제에 사용된 러스트 문법만 간단히 소개하겠습니다.

프로그램은 `main` 함수에서 시작합니다. 프로그램이 시작된 직후 `println!` 매크로를 호출해서 간단한 메시지를 출력합니다. 그리고 `fizzbuzz_if_else` 함수를 호출합니다.

함수의 정의는 `fn` 키워드로 시작합니다.

```rust
fn 함수이름(인자: 타입, 인자: 타입, ...) -> 반환값 {
    함수 라인1;
    라인2;
    결과값
}
```

코드의 각 라인은 `C`/`C++`과 같이 `;`로 끝나야 합니다. 그리고 마지막에 함수 반환값은 `;` 없이 씁니다. `Scala` 등의 함수형 언어와 유사한 점입니다. 반환값이 없으면 명시하지 않거나, 구문의 특성상 반드시 반환값을 지정해야 하는 경우(`if-else`만 있는 함수 등)에는 `()`라고 써주기도 합니다.

러스트에서는 `Range`라는 타입으로 `for`문의 범위를 지정합니다. `Python`과 유사합니다. 아래와 같이 작성하면 10을 제외한 9까지만 처리하는 코드가 됩니다. `Bash`나 몇몇 언어에서는 `1..10`이 10을 포함하지만, 러스트에서는 10을 포함하지 않습니다.

```rust
for i in 1..10 {
......
}
```

1부터 10까지 처리하도록 하려면 다음과 같이 `=`를 추가해야 합니다.

```rust
for i in 1..=10 {
......
}
```

`if`문에서 마지막 `else`는 아무것도 하지 않지만, `else`를 꼭 넣어줘야 모든 케이스를 완벽하게 처리하는 코드가 됩니다. `else`로 따로 할 일이 없다 해도 `else`를 넣고 아무 처리도 하지 않는다는 주석이라도 넣어줘야 보안에 신경 쓴 코드가 됩니다. 러스트는 이런 보안 허점들을 방지하기 위한 문법들을 가지고 있습니다. `if`문에 대한 세부적인 설명은 바로 다음 장에서 이야기하겠습니다.

변수의 정의는 `let` 키워드를 사용합니다.

```rust
let 변수이름: 타입 = 초기값;
```

## if-else

이미 사용했지만 `if-else`만 사용하는 코드를 다시 만들어 보겠습니다.

```c
#include <stdio.h>

int main() {
    int num = 5;
    int var;
    if (num % 3 == 0) {
        var = 3;
    } else if (num % 5 == 0) {
        var = 5;
    } else {
        var = 0;
    }
    printf("var=%d\n", var);
}
```

이 코드를 `Rust`로 변환하면 다음과 같이 만들 수 있습니다. 미리 주의해야 할 점은 `Rust`에서 `if-else`문은 값을 가진다는 것입니다.

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

```bash
$ cargo run --bin if_return_value
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/if_return_value`
var = 5
```

첫 번째 `if`문 블록 안에 `3`이 써 있는데, `3` 옆에 `;`가 없습니다. `3`이 `{ ... }`로 둘러싸인 블록의 값이 된다는 의미입니다. 만약 `num` 변수에 저장된 값이 3의 배수라면 `var`에 블록의 값인 `3`을 저장하게 됩니다.

`C`에 익숙한 분들은 아마 아래와 같은 삼항 연산자가 생각나실 겁니다.

```c
#include <stdio.h>

int main() {
    int num = 5;
    int var = num % 3 == 0 ? 3 : num % 5 == 0 ? 5 : 0;
    printf("var=%d\n", var);
}
```

`C` 언어에서 위와 같은 삼항 연산자는 구문 자체가 값을 반환합니다. `C` 언어의 `if`문이 러스트와 같이 반환값을 가질 수는 없지만, 삼항 연산자를 사용하면 러스트와 같이 좀 더 짧고 에러 처리가 확실한 코드가 될 수 있습니다.

물론 러스트도 아래와 같이 처음에 만든 `C` 코드와 완전히 동일하게 구현할 수 있지만, 추천하는 방식은 아닙니다.

```rust
fn main() {
    let num = 5;
    let var;
    if num % 3 == 0 {
        var = 3;
    } else if num % 5 == 0 {
        var = 5;
    } else {
        var = 0;
    }

    println!("var = {}", var);
}
```

왜 굳이 `if`문이 값을 가지도록 만들었을까요? 프로그래밍 언어의 철학에 대해서 본격적으로 이야기하자면 저도 잘 모르는 깊은 이론들이 있겠지만, 제가 개발하면서 느꼈던 장점은 값이 초기화되지 않은 변수를 최소화할 수 있다는 것입니다. 초기화되지 않은 변수를 일부러 만드는 사람은 없을 것입니다. 하지만 코드가 길어지고 다른 사람의 코드를 유지 보수하다 보면 실수로 변수의 초기화 코드를 제거하기도 합니다. 그리고 아주 심각한 문제는 초기화되지 않은 변수를 알아차리지 못하고 계속 사용하는 경우가 생길 수 있다는 것입니다.

```c
#include <stdio.h>

int main() {
    int num = 5;
    int var;
    if (num % 3 == 0) {
        var = 3;
    } else if (num % 5 == 0) {
        printf("This is error, please enter 3!\n");
    } else {
        var = 0;
    }
    printf("var=%d\n", var);
}
```

위 코드는 `var` 변수를 초기화하는 것을 깜빡한 사례입니다. `if-else`가 한두 개이거나 처음 코딩할 때는 초기화를 깜빡하지 않겠지만, 개발이 점점 진행되고 코드가 길어지고 복잡해지면 초기화를 깜빡하는 경우가 자주 생깁니다. 운이 좋으면 그냥 쓰레기 값이 출력되는 것으로 끝나겠지만, 대부분 의미 없는 값이 이리저리 돌아다니다가 엉뚱한 곳에서 패닉(`panic`)을 발생시키고 디버깅하는 데 며칠이 걸리게 만드는 경험을 해보셨을 것입니다.

러스트에서 `if-else`로 변수를 초기화하면 다음과 같이 값을 반환하지 않는 경우를 방지할 수 있습니다.

```rust
fn main() {
    let num = 5;
    let var = if num % 3 == 0 {
        3
    } else {
        if num % 5 == 0 {
            //5 missing by mistake
            println!("Wrong number");
        } else {
            0
        }
    };
    println!("var = {}", var);
}
```

```bash
error[E0308]: `if` and `else` have incompatible types
  --> code/main.rs:10:13
   |
6  | /         if num % 5 == 0 {
7  | |             //5 missing by mistake
8  | |             println!("Wrong number");
   | |             ------------------------ expected because of this
9  | |         } else {
10 | |             0
   | |             ^ expected `()`, found integer
11 | |         }
   | |_________- `if` and `else` have incompatible types
```

추가로 한 가지 팁을 드리자면, 러스트는 다양한 함수형 프로그래밍 기법을 사용하고 있어서 `if-else`를 사용할 일이 많이 줄어듭니다. `C` 언어 같은 절차형 언어에서 `if-else`를 사용하는 처리를 `Rust`에서는 함수형 언어와 같이 `match`나 `map`, `filter` 등을 사용하는 경우가 훨씬 많습니다. 만약 내 코드에 `if-else`가 많이 보인다면 `Rust`다운 코드를 만들지 못하고 있는 것은 아닌지 자문해 봐야 합니다.

## Mutable 변수와 타입 추론

지금까지 우리가 만들어본 예제들은 변수가 생성될 때 값을 정하고 그 후에는 값을 바꾸지 않았었습니다.

```rust
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
```

위 예제를 보면 3개의 변수가 있습니다. `let`으로 생성한 변수 2개(`rem_three`, `rem_five`)와 함수 인자인 `max`입니다. 이렇게 총 3개의 변수가 하나의 함수를 이루는 컨텍스트(Context)에서 생성되었습니다.

함수의 컨텍스트는 간단하게 이해하자면 함수가 동작하기 위한 모든 메모리, 코드 등을 합쳐서 부르는 말입니다. 로컬 변수는 함수의 스택(`Stack`)에 저장되고, 함수 인자는 함수를 호출하는 상위 함수의 메모리에 있겠지요. 그리고 함수 자체를 이루는 코드도 있을 것입니다. 이런 것들을 컨텍스트라고 생각할 수 있습니다.

어쩌면 지금까지의 예제 코드를 보면서 모든 변수가 값이 고정된 변수였다는 것을 알아차리지 못한 분들도 계셨을 것입니다. 그것이 바로 러스트가 변수 선언의 기본을 변하지 않는 속성(불변, `Immutable`)으로 정한 이유입니다. 생각보다 많은 변수가 한 번 초기화되면 값이 바뀌지 않습니다. 특히 함수형으로 코드를 작성하면 더욱 그런 경우가 많고, 함수를 잘 분리해서 간결하게 작성하면 함수의 로컬 변수 값이 자주 바뀌지 않습니다.

어쨌든 값이 변하는 변수를 만들고자 할 때는 추가로 `mut`라는 키워드를 넣어주어야 합니다. 이것은 `let`으로 변수를 선언할 때뿐 아니라 함수 인자에서도 마찬가지입니다.

`mut` 키워드를 사용하는 예제를 보겠습니다.

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

```bash
$ cargo run --bin mutable_var
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/mutable_var`
144
```

이 예제에서 세 가지 새로운 개념(Concept)을 소개하고 있습니다.

1. `mut` 키워드를 사용해서 값이 변하는 변수와 함수 인자를 사용
2. `loop` 사용 방법
3. 각 변수마다 일일이 타입을 쓰지 않아도 러스트가 타입을 추론할 수 있음 (컴파일러가 변수가 사용되는 상황을 확인해서 적당한 타입으로 만들어 줍니다. `Python` 언어와 같이 완전한 덕 타이핑(`Duck typing`) 언어는 아니지만, 어느 정도 타입을 알아서 판단해 줍니다. 하지만 강타입 언어이므로 컴파일러가 판단하지 못하는 경우가 자주 있고, 타입을 명확하게 지정하라는 컴파일 에러를 자주 만날 수 있습니다.)

`mut` 키워드는 `mutable`의 약자입니다. 값이 변하는 변수라는 표시인데요, 보통의 프로그래밍 언어들이 값이 변하지 않는 상수를 위한 키워드가 있는 것에 반해 러스트에서는 값이 변하지 않는 변수가 기본이고, 값이 변하는 변수에 `mut` 키워드를 추가해 주어야 합니다. 실제로 러스트로 프로그래밍하다 보면 의외로 자주 `mut` 키워드를 빼먹어서 컴파일 에러가 나는 경우를 겪을 것입니다. 그런데 생각보다 더 많은 경우에 `mut` 키워드를 무의식적으로 빼먹고도 문제없이 동작하는 것을 경험하게 될 것입니다. 저도 처음에는 `mut` 키워드를 써줘야 한다는 것이 번거로울 것 같았지만, 곧 제 자신이 값이 변하지 않는 변수를 생각보다 많이 써왔다는 것을 깨달았습니다. 그리고 좀 더 경험이 쌓일수록 값이 바뀌지 않는 변수를 많이 사용하게 되고, 결국 코드가 조금 더 간결해지는 것을 경험하게 되었습니다. 물론 러스트가 가진 함수형 프로그래밍 기능들을 사용해서 그렇기도 합니다만, 지금은 값이 변하지 않는 변수가 기본이라는 것이 코드의 안정성을 위해서 괜찮은 선택이었다고 생각합니다.

`loop` 사용법은 개인적으로 다른 언어의 `while`보다 더 편리하다고 생각합니다. `while`을 사용하다 보면 `while(..)`의 `()` 안에 탈출 표현식을 써주고, 또 `while() {..}` 바디 안에 또 다른 탈출식을 쓰게 되는 경우가 많습니다. 이것보다는 `loop`처럼 기본 탈출식이 없는 것이 좀 더 읽기 편하다고 생각합니다.

러스트는 타입 추론 기능을 가지고 있습니다. `C++`에서는 `auto`라는 타입을 써주면 컴파일러가 최적의 타입을 찾아주는데, 러스트는 아예 타입을 안 써줄 수도 있습니다. 러스트는 강타입 언어이므로 타입이 분명 존재하지만, 타입을 명시하지 않으면 컴파일러가 변수의 사용을 보고 타입을 지정해 줍니다. 이것이 편리한 이유는 러스트의 문법 특성상 타입이 아주 긴 경우가 자주 있기 때문입니다.

아래와 같은 타입을 변수 타입에 써야 한다면 번거로울 수밖에 없겠지요.

```rust
Box<dyn Fn() + Send + 'static>
Arc<Vec<Box<dyn Collector<dyn CollectorModel> + 'static>>>,
```

출처 링크: <https://doc.rust-lang.org/book/ch19-04-advanced-types.html>

참고로 함수를 정의할 때는 타입을 생략할 수 없습니다. 함수를 호출하기 위해서는 입출력의 타입을 정확하게 알아야 컴파일러가 기계 코드를 생성할 수 있기 때문입니다.

## as를 이용한 타입 바꾸기

러스트는 값의 타입을 바꿀 때 항상 명확하게 선언해 주어야 합니다. `C` 언어 등 타입 변환을 암묵적으로 지원하는 언어도 있지만, 그런 언어는 버그를 만들기 쉽고 보안에도 좋지 않습니다. 러스트는 그런 보안적인 요소에 특히 잘 대비하고 있습니다.

다음 예제 코드는 문자열로 된 숫자를 정수로 반환하는 함수입니다.

```rust
fn string_to_digit(input: String) -> i32 {
    let mut ret = 0;
    for c in input.chars() {
        ret = ret * 10;
        ret += c - '0'; // 컴파일 에러 발생
    }
    ret
}
```

```bash
error[E0369]: cannot subtract `char` from `char`
  --> code/main.rs:10:19
   |
10 |         ret += c - '0'; // 컴파일 에러 발생
   |                - ^ --- char
   |                |
   |                char
```

혹시 코드만 보고도 바로 문제를 알아차리셨다면 타입에 대해 많이 고민해 보셨다는 의미입니다. 이 코드를 빌드해 보면 `char` 타입 변수에서 `char` 타입의 값을 뺄 수 없다는 에러가 발생합니다. 아래는 임시방편으로 `char` 타입의 변수를 `i32` 타입의 정수로 변환한 후 빼기 연산을 수행한 코드입니다.

```rust
fn string_to_digit(input: String) -> i32 {
    let mut ret = 0;
    for c in input.chars() {
        ret = ret * 10;
        ret += c as i32 - '0' as i32;
    }
    ret
}
```

이 코드는 컴파일 에러 없이 잘 동작합니다.

위 예제를 통해 우리는 `for`문에서 생성되는 `c`라는 변수가 `char` 타입임을 알 수 있습니다. 러스트의 `for`문에는 항상 이터레이터만 사용할 수 있다고 설명했는데, `String` 구조체의 `chars` 메서드가 문자열의 각 문자를 반환하는 이터레이터를 생성해 주는 메서드라고 생각하면 됩니다.

러스트에서는 `char` 타입의 변수 `c`에서 `char` 타입의 문자 `'0'`을 뺄 수는 없습니다. 현대 언어에 익숙한 분들에게는 당연한 사항일 수 있지만, `C` 언어에 익숙한 분들에게는 당황스러운 일일 수도 있습니다.

`C` 언어는 사실 어셈블리(`Assembly`)로 개발하던 프로젝트의 생산성을 높이기 위해 나온 언어입니다. 어셈블리에 익숙한 개발자의 관점에서 디자인된 언어이다 보니 모든 것이 숫자입니다. 참과 거짓도 숫자이고, 오류를 나타내는 `NULL`이나 에러 값도 숫자이며, 포인터(`Pointer`)도 숫자, `char` 타입도 숫자입니다. 그러니 타입이 다른 변수 간에도 더하기 빼기가 가능합니다. 하지만 이것이 논리적으로 좋은 디자인인지는 의문입니다. 숫자가 아닌 타입을 더하거나 뺀다는 것은 논리적으로 잘못된 연산이 되는 게 더 옳지 않을까요?

잡설이 좀 길었지만, 어쨌든 러스트에서 타입 변환은 `as`라는 키워드를 사용합니다. 추후에 몇 가지 타입 변환과 관련된 키워드를 보겠지만, 가장 기본적인 것이 바로 `as`입니다. 이렇게 언어 자체에 키워드가 있어서 타입이 변환되는 것이 논리적으로 옳은지는 모르겠습니다만, 신택스 슈거(`Syntax sugar`)라고 생각해도 될 듯합니다.

```rust
// code/as/main.rs
use std::num::ParseIntError;

fn string_to_digit(input: String) -> i32 {
    let mut ret = 0;
    for c in input.chars() {
        ret = ret * 10;
        ret += c as i32 - '0' as i32;
    }
    ret
}

fn parse_example(input: &str) -> Result<i32, ParseIntError> {
    input.parse()
}

fn main() {
    println!("{}", string_to_digit("1234".to_string()));
    let ret = parse_example("1234");
    match ret {
        Ok(value) => {
            println!("Parsed integer: {}", value);
        }
        Err(_) => {
            println!("Failed to parse the string as an integer");
        }
    }
}
```

```bash
$ cargo run --bin as
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/as`
1234
Parsed integer: 1234
```

`string_to_digit` 함수의 문제점은 문자열이 숫자 외의 문자를 포함하는 등의 에러 상황에서 에러 값을 반환할 수 없다는 것입니다. `i32` 타입의 값을 반환하고 있는데, 어떤 값이 에러인지 아니면 정상적인 결과인지 알 수 없습니다. 다른 프로그래밍 언어들도 에러를 표현하는 타입이 별도로 존재하지 않는 경우가 많습니다. 만약 `0`을 에러 값으로 정하면 `input`에 `"0"`을 받는 경우를 처리할 수 없습니다. `-1`도 마찬가지이고, `i32`의 최댓값이나 최솟값인 `-2,147,483,648`, `2,147,483,647` 등도 에러 값으로 사용할 수 없습니다. 사용자가 `"-2,147,483,648"`이라고 입력할 수도 있으니까요. 이런 사소해 보이는 문제들이 중대한 보안 이슈의 원인이 되는 경우가 많습니다.

또한 대부분의 프로그래밍 언어가 에러인 경우와 반환값이 없는 경우를 명확하게 구분하지 못하고 있습니다. 함수를 정의할 때 반환값이 없다고 명시하고 에러를 처리하기 위해 예외를 반환하는 경우도 많지만, 사실 논리적으로 생각하면 이상한 코드입니다. 반환값이 없다고 정의된 함수가 예외를 반환하는 것이 어쩔 수 없다고는 해도, 엄밀히 말해 좋은 방법인지는 의문입니다.

러스트는 함수의 반환값으로 `Result`나 `Option` 같은 타입을 사용합니다. `Result`나 `Option` 안에 에러 값이나 정상적인 결과 값을 담아서 처리합니다. 에러와 정상 반환값의 타입을 따로 지정할 수 있습니다. `Result<i32, ParseIntError>`와 같이 두 개의 타입을 지정하는데, 첫 번째는 정상적인 반환값의 타입이고 두 번째는 에러 값의 타입입니다. 정상적인 상황에 반환값이 없다면 `Result<(), ParseIntError>`라고 지정할 수도 있습니다. 따라서 함수에 반환값이 없는 경우에도 에러 상황을 판단할 수 있습니다. `Result`/`Option` 안에 에러가 있으면 에러 상황이 발생한 것이고, 아무런 값도 없으면 정상적으로 실행되었으나 반환값이 없는 경우입니다. 추후 좀 더 자세히 알아볼 것이지만, 미리 이런 상황에서 필요하다는 것을 알고 넘어가면 나중에 이해하기 쉬울 것 같아 예제를 만들어 보았습니다.

## match를 이용한 패턴 매칭

저는 `for`, `while`, `if`, 함수 호출 등 러스트의 기본 문법에 대해서는 모든 프로그래밍 언어에서 봐왔던 것들이라 금방 익숙해질 수 있었습니다. 그러다 처음으로 흥미를 느낀 문법이 바로 패턴 매칭(Pattern Matching) 부분이었습니다. `if-else`가 여러 개 있는 사례는 다른 언어에서도 `switch`문으로 작성하기 때문에 별다른 것이 있을까 생각했었지만, 패턴 매칭의 의미를 이해하게 되면서 감탄했던 기억이 있습니다.

패턴 매칭은 사실 정확한 정의가 무엇인지 따지기보다는, 직접 써보면서 적응해 나가는 것이 더 효율적인 접근 방법이라고 생각합니다. 일단 가장 쉬운 예제를 하나 보겠습니다.

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

fn main() {
    fizzbuzz_2(15);

    let age = 44;
    let gen = match age {
        0..=20 => "MZ",
        21..=50 => "X",
        51..=100 => "A",
        _ => "?",
    };
    println!("generation={}", gen);

    for i in 1..=30 {
        let msg = match i {
            n if n % 15 == 0 => format!("{} - FizzBizz", n),
            n if n % 3 == 0 => format!("{} - Fizz", n),
            n if n % 5 == 0 => format!("{} - Buzz", n),
            _ => format!("{}", i),
        };
        println!("{}", msg);
    }
}
```

```bash
gkim@gkim-laptop:~/study/my-rust-book$ cargo run --bin match
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/match`
3 - Fizz
5 - Buzz
6 - Fizz
9 - Fizz
10 - Buzz
12 - Fizz
15 - FizzBuzz
generation=X
1
2
3 - Fizz
4
5 - Buzz
6 - Fizz
7
8
9 - Fizz
10 - Buzz
11
12 - Fizz
13
14
15 - FizzBizz
16
17
18 - Fizz
19
20 - Buzz
21 - Fizz
22
23
24 - Fizz
25 - Buzz
26
27 - Fizz
28
29
30 - FizzBizz
```

`fizzbuzz_2` 함수는 이전에 만든 `FizzBuzz` 예제를 패턴 매칭으로 바꾼 함수입니다. `match` 키워드 뒤에 있는 `(i % 3, i % 5)`는 하나의 튜플(`tuple`)을 만듭니다. 이 튜플의 값이 `match` 뒤에 나오는 각 패턴에 해당할 때 각기 다른 메시지를 출력하도록 만든 것입니다. 패턴에서 밑줄(`_`)은 모든 값을 의미합니다. 튜플의 값을 비교해야 하는데 두 개의 값이 있어야 하므로, 하나는 0이고 다른 값은 무엇이든 상관없을 때 언더바를 사용한 것입니다. 패턴 매칭이므로 2개의 값이 있는 튜플의 패턴과 매칭되려면 각각의 매칭 케이스마다 2개의 값이 있어야 합니다.

`if-else`를 사용할 때와 마찬가지로 패턴의 비교 순서가 바뀌면 전혀 다른 결과를 만들어냅니다. 만약 아래처럼 패턴 순서를 바꿨다면 어떤 일이 벌어질까요?

```rust
fn fizzbuzz_2(max: i32) {
    for i in 1..=max {
        match (i % 3, i % 5) {
            (0, _) => println!("{} - Fizz", i),
            (_, 0) => println!("{} - Buzz", i),
            (0, 0) => println!("{} - FizzBuzz", i),
            _ => (),
        }
    }
}
```

만약 튜플 값이 `(0, 0)`이면 `FizzBuzz`를 출력하는 게 아니라 `Fizz`를 출력할 것입니다.

흔히 사용하는 패턴을 하나 더 살펴보겠습니다. `main` 함수 안에 다음과 같이 어떤 변수의 값이 특정 범위에 속하는지 판단하는 코드가 있습니다.

```rust
let gen = match age {
    0..=20 => "MZ",
    21..=50 => "X",
    51..=100 => "A",
    _ => "?",
};
```

`age` 변수를 `0..=20`, `21..=50`, `51..=100`이라는 세 가지 `Range` 타입과 비교하는 것인데, 세밀하게 따지면 `age`가 `Range` 타입이 아니므로 패턴으로 매칭되는 것이 조금 이상할 수도 있습니다. 문법적으로 세세하게 따지면 복잡할 수도 있는 코드입니다만, 지금은 이런 방식도 가능하다는 정도로 생각하고 일단 자주 활용하면서 익숙해지면 될 듯합니다. 단순히 `age` 변수의 범위만 따져서 어떤 동작을 수행하는 게 아니라, 각 경우에 따른 반환값을 `gen` 변수를 생성하는 데 사용한 점을 주의 깊게 보시기 바랍니다. `match` 구문도 결국 `if`와 같이 반환값을 가집니다.

단순히 패턴에 일치하는 것만 확인하는 게 아니라, 아래와 같이 `if`와 결합해서 조건식을 작성할 수도 있습니다.

```rust
for i in 1..=30 {
    let msg = match i {
        n if n % 15 == 0 => format!("{} - FizzBizz", n),
        n if n % 3 == 0 => format!("{} - Fizz", n),
        n if n % 5 == 0 => format!("{} - Buzz", n),
        _ => format!("{}", i),
    };
    println!("{}", msg);
};
```

`format!`은 문자열 객체를 반환해 주는 매크로 함수입니다. 조건식에 따라 다른 문자열을 생성해서 `msg` 변수에 저장하게 됩니다.

러스트 관련 소개 자료나 공식 문서 등을 보면 러스트의 패턴 매칭이 '강력하다'는 설명이 많습니다. 보통 강력하다는 말은 다양한 방식으로 활용될 수 있다는 의미입니다. 이 책에서 모든 사례를 일일이 보여드릴 수는 없지만, 러스트로 개발하면서 조금씩 시도하다 보면 "이런 것도 가능하네" 하며 감탄하는 경험을 하게 될 것입니다. 보통의 `switch` 문은 변수의 값을 비교해서 사용하지만, 패턴 매칭은 말 그대로 패턴이나 코드의 값을 비교하므로 많은 장점이 있습니다.

## 값을 가지는 표현식

지금까지 `if-else`나 패턴 매칭의 예제를 보면 변수를 선언하는 부분에 복잡한 코드를 넣은 것을 볼 수 있었습니다. 이러한 표현식에 대해 좀 더 자세히 이야기해 보겠습니다. 다음 예제는 지금까지 살펴본 값을 반환하는 표현식들을 모아놓은 것입니다.

참고로 아래 예제를 실행해 보면 `ret_zero` 함수를 호출하고 있지 않다는 경고 메시지가 나옵니다. 에러는 아니므로 코드에 문제는 없습니다. `_var`와 같은 변수도 마찬가지로 생성만 하고 사용하지 않지만 경고 메시지가 없습니다. 변수 이름을 `_`로 시작했기 때문입니다. 이와 같이 임시로 만들어 두고 나중에 사용하게 될 변수는 이름을 `_`로 시작하면 컴파일 시 경고 없이 깔끔하게 처리할 수 있습니다. `ret_zero` 함수 이름 앞에도 `_`를 붙여보면 경고 메시지가 사라지는 것을 볼 수 있습니다.

```rust
// code/expr/main.rs
fn ret_zero() -> i32 {
    0
}

fn main() {
    let age = 44;
    let gen = match age {
        0..=20 => "MZ",
        21..=50 => "X",
        51..=100 => "A",
        _ => "?",
    };
    println!("generation={}", gen);

    let num = 45;
    let _var = if num % 3 == 0 {
        3
    } else {
        if num % 5 == 0 {
            5
        } else {
            0
        }
    };

    let x = 9;
    let _y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };
}
```

```bash
$ cargo run --bin expr
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
warning: function `ret_zero` is never used
 --> code/expr/main.rs:1:4
  |
1 | fn ret_zero() -> i32 {
  |    ^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `my-rust-book` (bin "expr") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/expr`
generation=X
```

위 예제의 `match`를 이용한 패턴 매칭 구문에서 `{`로 시작하고 `}`로 끝나는 하나의 블록을 표현식(`expression`)이라고 합니다. 마찬가지로 `if-else`도 하나의 표현식입니다.

그리고 표현식은 값을 반환합니다.

값을 반환하는 것에는 또 무엇이 있을까요? 함수가 있습니다. 함수도 하나의 표현식입니다.

```rust
fn ret_zero() -> i32 {
    0
}
```

함수도 `{`와 `}`로 시작과 끝을 나타내며, 반환값을 마지막에 작성한 표현식입니다. 위 예제의 `if-else`도 각 `{}` 블록 안에 반환값이 정해져 있습니다. 또한 `match` 구문에서도 각 상황에 따른 반환값이 있습니다. 이러한 표현식은 세미콜론(`;`)을 만나면 그 반환값이 무시됩니다.

반환값을 가지는 표현식을 이용하면 다음과 같은 변수 초기화 코드를 작성할 수 있습니다. 중간의 코드들은 `;`로 끝나므로 반환값이 없고, 마지막 코드에만 `;`이 없으므로 마지막 줄 수식의 결과값이 `y`의 값이 됩니다.

```rust
let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };
```

`y` 변수의 값을 계산하기 위한 복잡한 코드가 나열되는 대신 `y`의 선언부에 모여 있을 수 있습니다. 물론 앞에서 살펴본 `match`, `if-else` 등도 올 수 있습니다. 다른 언어와 마찬가지로 함수 호출도 가능합니다.

참고로 영어로 표현식은 `expression`이고, 표현식에 `;`가 붙은 것을 문장(`statement`)이라고 구분하여 부르기도 합니다.

## 배열과 배열을 참조하는 슬라이스

문자열을 사용하는 예제를 몇 가지 살펴보았으니, 우선 배열과 슬라이스에 대해 알아본 후 본격적으로 문자열(정확히는 `String` 타입의 객체)에 대해 알아보겠습니다.

배열은 같은 타입의 데이터가 메모리에 연속적으로 나열된 데이터 구조를 말합니다.

```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Example array of numbers
```

위와 같이 `i32`라는 동일한 타입의 데이터가 메모리에 연속적으로 5개 저장되어 있습니다. 메모리에 연속적으로 위치하므로, `numbers[0]` 다음에 있는 데이터가 `numbers[0]`의 위치(포인터)에서 `i32` 타입의 크기인 4바이트만큼 더한 곳에 위치한다는 것을 알 수 있습니다.

```code
numbers[0] ==> 0x100
numbers[1] ==> 0x100 + 1 * 4 = 0x104
...
numbers[i] ==> 0x100 + i * 4
```

이처럼 배열의 인덱스 `[0]`, `[1]` 등을 사용하여 각 데이터에 빠르게 접근할 수 있다는 것이 배열의 특징입니다. 연결 리스트나 트리 등보다 접근 속도가 빠릅니다.

슬라이스는 이러한 배열의 일부(또는 전체)에 접근하기 위해 만든 참조(`reference`) 타입입니다.

```rust
let slice: &[i32] = &numbers[1..4]; // Create a slice from index 1 to 3 (inclusive)
```

위 슬라이스는 `numbers` 배열의 1, 2, 3번 데이터에만 접근할 수 있습니다. `[i32]`는 `i32` 타입의 배열에 접근한다는 표시이며, 참조 연산자 `&`를 사용하여 배열에 대한 참조임을 나타냅니다. 내부적으로는 슬라이스의 길이 정보도 함께 저장하고 있습니다.

여기서 배열이나 구조체처럼 여러 데이터가 묶여 있는 타입을 디버깅할 때 유용한 방법을 하나 소개하겠습니다.

```rust
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Example array of numbers

    let slice: &[i32] = &numbers[1..4]; // Create a slice from index 1 to 3 (inclusive)

    println!("Array: {:?}", numbers);
    println!("Slice: {:?}", slice);
}
```

`"{:?}"` 출력 포맷을 사용하면 다음과 같이 배열의 데이터를 모두 출력해 줍니다.

```rust
Array: [1, 2, 3, 4, 5]
Slice: [2, 3, 4]
```

참고로 나중에 배울 구조체도 마찬가지로 `"{:?}"`를 사용하여 각 필드를 출력할 수 있으므로, 디버깅 시 유용하게 활용해 보시기 바랍니다.

실행 결과를 보면 슬라이스를 통해 3개의 데이터에만 접근할 수 있음을 알 수 있습니다. 슬라이스는 배열의 참조이지만, 내부적으로 몇 개의 데이터를 참조하는지에 대한 정보가 함께 저장됩니다. 그래서 슬라이스 데이터를 출력하면 3개만 나타나는 것입니다.

슬라이스가 무엇인지는 알았는데, 왜 필요한지 의문이 들 수 있습니다. 파이썬과 같은 최신 언어들은 대부분 슬라이스를 지원하지만, `C` 언어처럼 오래된 언어들은 이를 지원하지 않는 경우가 많습니다. 이러한 언어들은 배열이나 구조체를 읽을 때 할당된 영역을 벗어나 메모리를 읽거나 쓰는 것을 방지하기 어렵습니다. `numbers[4]`까지만 접근해야 하는데 실수로 `numbers[5]`를 읽으려 한 경험이 한 번쯤은 있을 것입니다.

슬라이스는 바로 이러한 실수를 방지하기 위해 존재합니다. 함수나 스레드를 호출할 때 배열의 일부분에만 접근해야 한다면, 배열 전체를 전달하는 대신 슬라이스를 전달합니다.

앞서 슬라이스는 배열의 일부 또는 전체에 접근하기 위한 타입이라고 설명했습니다. 다음 예제를 살펴보겠습니다.

```rust
// code/array_slice/main.rs
fn sum_array_ref(nums: &[i32]) -> i32 {
    let mut s = 0;
    let len = nums.len();
    let mut index = 0;
    loop {
        if index >= len {
            return s;
        }
        s += nums[index];
        index += 1;
    }
}

fn sum_slice(nums: &[i32]) -> i32 {
    let mut s = 0;
    for i in nums.iter() {
        s += i;
    }
    s
}

fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];

    println!("Array: {:?}", numbers);
    println!("Slice: {:?}", slice);

    println!("{}", sum_array_ref(&numbers));
    println!("{}", sum_slice(slice));
}
```

```bash
$ cargo run --bin array_slice
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/array_slice`
Array: [1, 2, 3, 4, 5]
Slice: [2, 3, 4]
15
9
```

`sum_array_ref`는 배열을 참조로 전달받는 함수이고, `sum_slice`는 슬라이스를 전달받는 함수입니다. 슬라이스가 배열의 참조이므로 형태가 동일합니다. 각 함수는 전달받은 데이터가 배열 전체의 참조인지, 아니면 일부분을 가리키는 슬라이스인지 구분할 필요가 없습니다. 내부적으로는 동일하게 처리되기 때문입니다.

배열이나 문자열을 처리하는 함수를 작성할 때는 항상 슬라이스로 인자를 받는 습관을 들이는 것이 좋습니다.

물론 슬라이스와 같은 참조가 아니라 배열이나 문자열을 그대로 전달하면 소유권이 함수로 넘어가게 됩니다. 그러면 함수를 호출한 코드에서는 더 이상 해당 데이터에 접근할 수 없으므로, 슬라이스를 사용하여 처리하는 것이 일반적입니다. 러스트의 소유권 개념은 나중에 자세히 다루겠지만, 러스트는 이처럼 개발자가 실수할 수 있는 부분을 언어 차원에서 최대한 방지하려는 철학을 가지고 있습니다.

비관리형(`unmanaged`) 언어의 대표 주자인 `C` 언어를 예로 들어 보겠습니다. `C` 언어는 근본적인 문제점을 해결하기 위해 `C++`을 출시하고, 스마트 포인터 등 다양한 최신 기법을 도입하며 발전해 왔습니다. 하지만 개발자가 모든 것을 직접 제어해야 한다는 철학 때문에 메모리 안전성 문제를 완벽히 해결하기는 어려웠습니다. 러스트는 이러한 `C`/`C++`의 최신 안전 기법들을 집대성하여, 이를 강제하도록 설계된 언어라고 이해하면 도움이 될 것입니다.

## 문자열을 저장하는 `String` 타입

많은 언어에서 문자열은 문자를 표현하는 `char` 타입의 배열입니다. `char` 타입이 8비트 부호 없는 정수를 의미한다면, 결국 바이트 배열과 같습니다. 하지만 러스트를 비롯한 현대적인 언어들은 문자열을 `String`이라는 구조체나 별도의 타입으로 표현합니다. 단순히 1바이트 `ASCII` 코드 배열이 아니라, `UTF-8` 인코딩을 사용하여 각 문자를 관리하기 때문입니다.

러스트에서 슬라이스와 함께 문자열을 설명하는 경우가 많은데, 그 이유를 알아보겠습니다.

우선 `String` 타입에 대해 알아보겠습니다.

매뉴얼을 보면 `String`은 결국 하나의 `struct`임을 알 수 있습니다. 단순한 문자의 배열이 아닙니다. 따라서 `String`을 사용하려면 먼저 객체를 생성해야 합니다. 다음 예제는 흔히 사용되는 `String` 생성 방법들입니다. 참고로 `String`은 러스트 표준 라이브러리(`std`)에 포함되어 있어 별도의 `import` 없이 바로 사용할 수 있습니다.

`String`을 사용하는 몇 가지 방식에 대해 예제를 통해 살펴보겠습니다.

```rust
// code/string/main.rs
fn get_moved_string(data: &str) {
    println!("{}", data);
}

fn main() {
    let _hello = String::from("Hello, world!");
    let mut _s = String::new();
    let _s = "initial contents".to_string();
    let _hello = String::from("안녕하세요");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{}-{}-{}", s1, s2, s3);

    let moving_string = String::from("hello");

    get_moved_string(&moving_string);
    println!("{}", moving_string);

    let mut mutable_string = String::from("hello");
    mutable_string.push_str(" world");
    println!("{}", mutable_string.chars().nth(0).unwrap());

    {
        let hello = "hell".to_string();
        let _r1 = &hello;
        //let mut r2 = &mut hello; // Build Error!!!
    }
}
```

```bash
gkim@gkim-laptop:~/study/my-rust-book$ cargo run --bin string
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/string`
hello
hello
h
```

가장 먼저 주의할 점은 큰따옴표로 감싸진 문자열 리터럴(`literal`)은 `String` 타입이 아니라는 것입니다. 리터럴은 컴파일 시점에 크기가 고정되며, 프로그램 실행 중에 변하지 않습니다. 따라서 컴파일러는 이를 힙(`heap`) 영역이 아닌 실행 파일의 데이터 영역에 저장합니다. 반면 `String` 객체는 실행 중에 힙 영역에 메모리를 할당하고 데이터를 복사하여 생성됩니다. 예제 코드의 `String::from`이나 `to_string` 메서드가 실행될 때 리터럴 데이터가 `String` 객체의 메모리로 복사됩니다.

각 메서드를 설명하면 다음과 같습니다.

- `String::from("msg")`: 문자열 리터럴을 이용하여 `String` 객체를 생성합니다.
- `String::new()`: 빈 `String` 객체를 생성합니다.
- `"msg".to_string()`: 문자열 리터럴을 `String` 객체로 변환합니다. `String::from`과 동일한 역할을 합니다.

또한 `format!` 매크로를 사용하여 `String`을 생성하는 방식도 자주 쓰입니다.

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

`String`은 매우 자주 사용되는 타입이므로 다양한 메서드를 제공합니다. `push_str`, `insert`, `len` 등 문자열을 조작하거나 정보를 얻는 메서드들은 다른 고급 언어의 라이브러리와 유사하여 쉽게 익숙해질 수 있습니다.

그렇다면 문자열 리터럴은 정확히 무엇이며, `String`과는 어떤 관계일까요?

`String` 구조체의 정의를 살펴보겠습니다.

```rust
pub struct String {
    vec: Vec<u8>,
}
```

출처: <https://doc.rust-lang.org/code/alloc/string.rs.html#365>

현대적인 언어를 사용해 보았다면 익숙할 `Vec`(벡터)가 등장합니다. 벡터는 크기가 동적으로 조절되는 배열입니다. 일반 배열은 생성 시 크기가 고정되지만, `Vec`는 데이터를 추가하거나 삭제함에 따라 크기가 유연하게 변합니다. 벡터에 대한 자세한 내용은 다음 장에서 다루겠습니다.

다시 `String`으로 돌아가면, `String`은 내부적으로 `u8`(8비트 정수)의 배열인 `Vec<u8>`을 감싸고 있음을 알 수 있습니다. 따라서 `String` 슬라이스도 결국 바이트 배열에 대한 참조입니다. 이러한 `String` 슬라이스를 나타내는 타입이 바로 `&str`입니다.

예제 코드에 다음 두 줄을 추가해 보겠습니다.

```rust
let greeting: String = String::from("Hello, world");
let gretting_slice: &String = &greeting[1..3];
```

이 코드는 다음과 같은 컴파일 에러를 발생시킵니다.

```rust
error[E0308]: mismatched types
   --> code/main.rs:173:35
    |
173 |     let gretting_slice: &String = &greeting[1..3];
    |                         -------   ^^^^^^^^^^^^^^^ expected `&String`, found `&str`
    |                         |
    |                         expected due to this
    |
    = note: expected reference `&String`
               found reference `&str`
```

에러 메시지의 도움말을 따라 코드를 수정하면 정상적으로 빌드되는 것을 확인할 수 있습니다.

```rust
let greeting: String = String::from("Hello, world");
let gretting_slice: &str = &greeting[1..3];
```

이처럼 `String` 객체의 슬라이스를 다룰 때는 `&String`이 아닌 `&str` 타입을 사용해야 합니다.

`String`은 이른바 '팻 포인터(fat pointer)'입니다. 실제 문자열이 저장된 메모리 주소뿐만 아니라, 데이터의 길이와 할당된 버퍼의 크기 같은 추가 정보를 포함하고 있습니다. 슬라이스는 데이터의 일부분을 가리키는 참조입니다. `String`을 `C` 언어의 구조체로, 슬라이스를 그 내부 데이터를 가리키는 포인터(`char*`)로 생각하면 이해에 도움이 될 것입니다.

```rust
struct String {
    int buffer_len;
    int data_len;
    char *buffer;
};
```

그렇다면 `&str`이 아니라 `&` 없이 `str`만 사용할 수는 없을까요? 결론부터 말하면 불가능합니다. 러스트는 메모리 안전성을 위해 컴파일 시점에 크기를 알 수 있는 객체만을 허용하기 때문입니다.

`str` 타입은 `UTF-8` 데이터가 저장된 메모리 버퍼 그 자체를 의미합니다. 하지만 문자열의 길이는 고정되어 있지 않습니다. 일반적인 구조체나 배열은 컴파일 시점에 그 크기를 정확히 알 수 있지만, 힙에 저장된 문자열 데이터의 크기는 실행 중에 변할 수 있습니다. 따라서 `str` 타입 단독으로는 크기를 확정할 수 없어 변수로 사용할 수 없습니다.

하지만 참조 연산자 `&`를 붙이면 상황이 달라집니다. `&str`은 포인터이며, 포인터의 크기는 컴파일 시점에 고정되어 있기 때문입니다. 64비트 시스템이라면 8바이트로 크기가 일정합니다. 따라서 아래와 같이 선언하면 컴파일러는 스택 메모리에 고정된 크기의 변수를 생성할 수 있으므로 문제가 없습니다.

```rust
let ref_literal: &str = "hello";
let ref_string: &str = &string_object[2..5];
```

`ref_literal`이나 `ref_string` 모두 스택에 저장된 고정 크기의 포인터입니다. 이들이 가리키는 데이터 또한 참조하는 동안에는 크기가 변하지 않는 것으로 간주됩니다. 덕분에 컴파일러는 문자열 슬라이스를 사용할 때 메모리 안전성을 검사하고, 할당된 범위를 벗어난 접근을 방지할 수 있습니다.

반대로 "hello"와 같은 리터럴을 이용해 `String`을 생성하는 과정을 생각해 봅시다. 먼저 힙 메모리에 문자열을 저장할 공간을 할당하고 데이터를 복사해야 합니다. 또한 길이나 용량 같은 정보를 관리하기 위한 구조체 데이터도 생성해야 합니다. `format!`, `to_string`, `String::from` 등은 이러한 일련의 과정을 수행하여 `String` 객체를 만들어냅니다.

이러한 세부 사항은 처음 접할 때 다소 혼란스러울 수 있습니다. 지금은 `String`의 참조나 슬라이스 타입이 `&str`이라는 사실만 기억하고 넘어가도 충분합니다. 학습을 진행하며 실제 문제에 부딪히다 보면 자연스럽게 이해되는 순간이 올 것입니다. 더 깊은 내용이 궁금하다면 공식 문서나 관련 서적을 참고해 보시기 바랍니다.

마지막으로 `String` 객체를 다른 함수에 전달할 때 슬라이스를 사용해야 하는 이유를 설명하겠습니다. `String` 객체를 그대로 전달하면 '소유권'이 넘어가 버려, 함수 호출 이후에는 해당 객체를 다시 사용할 수 없게 됩니다. 반면 슬라이스(`&str`)를 넘기는 것은 데이터의 참조권을 잠시 빌려주는 것과 같습니다. 이 경우 함수가 종료되어도 소유권은 호출한 쪽에 남아있으므로 계속해서 객체를 사용할 수 있습니다.

예제 코드의 `get_moved_string` 함수를 살펴보겠습니다.

```rust
fn get_moved_string(data: &str) {
    println!("{}", data);
}

fn main() {
......
    let moving_string = String::from("hello");

    get_moved_string(&moving_string);
    println!("{}", moving_string);
......
}
```

이 함수를 참조가 아닌 객체 자체를 전달받도록 수정해 보겠습니다. `get_moved_string` 함수의 인자 타입을 `&str`에서 `String`으로 변경합니다.

```rust
fn get_moved_string(data: String) {
    println!("{}", data);
}

fn main() {
......
    let moving_string = String::from("hello");

    get_moved_string(moving_string);
    println!("{}", moving_string);
......
}
```

빌드하면 다음과 같은 컴파일 에러가 발생합니다.

```bash
> 161 | fn get_moved_string(data: String) {
|        ------------       ^^^^^^ this parameter takes ownership of the value
|        |
|        in this function
> 
```

이는 객체를 함수에 전달하면서 소유권이 이전되었음을 의미합니다. 본래 `main` 함수가 `moving_string`의 소유권을 가지고 있었으나, 이를 `get_moved_string` 함수에 넘겨주었기 때문에 더 이상 `main` 함수에서는 해당 변수를 사용할 수 없게 된 것입니다.

따라서 특별한 경우가 아니라면 객체를 함수에 전달할 때는 참조를 사용하고, 특히 `String`의 경우 `&str` 타입을 전달하는 것이 바람직합니다.

### `String`을 배열처럼 참조할 수 없는 이유

다음과 같이 `String` 객체에서 첫 번째 글자를 인덱스로 접근하여 출력할 수 있을까요?

```rust
let mut mutable_string = String::from("hello");
println!("{}", mutable_string[0]);
```

불가능합니다. `String` 타입은 배열이 아니므로 `[0]`과 같은 인덱스 접근을 지원하지 않습니다. 러스트는 예기치 않은 동작을 방지하기 위해 이러한 암묵적인 구현을 지양합니다. 코드를 빌드하면 다음과 같은 에러가 발생합니다.

```bash
> error[E0277]: the type `String` cannot be indexed by `{integer}`
--> code/main.rs:167:20
|
167 |     println!("{}", mutable_string[0]);
|                    ^^^^^^^^^^^^^^^^^ `String` cannot be indexed by `{integer}`
> 
```

해결 방법은 `chars` 메서드를 호출하여 이터레이터를 만든 후, `nth` 메서드로 특정 인덱스의 문자를 가져오는 것입니다.

```rust
let mut mutable_string = String::from("hello");
println!("{}", mutable_string.chars().nth(0).unwrap());
```

`nth` 메서드는 `Option` 타입을 반환하므로, 최종적으로 문자를 얻기 위해 `unwrap` 메서드를 호출합니다.

`Option` 타입에 대해서는 나중에 자세히 다루기로 하고, 왜 인덱스를 통한 직접 접근을 막아두었는지 그 이유를 알아보겠습니다.

주된 이유는 `UTF-8`을 완벽하게 지원하기 위해서입니다. 단순히 `[0]`이 0번째 바이트를 반환하게 할 수도 있었지만, 이는 `ASCII`가 아닌 다국어 문자(UTF-8)를 처리할 때 문제를 일으킵니다. 특정 인덱스의 문자가 몇 바이트를 차지할지 미리 알 수 없기 때문입니다. 따라서 러스트는 항상 이터레이터를 통해 문자열을 분석하며 한 문자씩 접근하도록 설계되었습니다. `String`의 `chars` 메서드가 다른 방식에 비해 상대적으로 느린 이유도 바로 이 때문입니다.

만약 바이트 단위로 쪼개고 싶다면 `as_bytes`라는 메서드를 호출하면 됩니다. 문자열 데이터가 반드시 `ASCII` 문자열이라는 상황이라면 사용할 수 있는 `Option`입니다.

## 변수를 읽고 쓸 수 있는 권한을 의미하는 `소유권(Ownership)`

배열에서의 `슬라이스(Slice)`나 `String`과 `&str`의 관계를 보면서 소유권을 넘기지 않기 위해 `참조(Reference)`를 사용한다는 이야기를 수차례 했습니다. 슬라이스도 그렇지만 그 외에 러스트의 문법적인 특징 상당수가 소유권 개념을 구현하기 위해 만들어진 것이라고 해도 과언이 아닙니다. "왜 이런 문법을 정했을까?", "왜 이건 이렇게 복잡할까?" 등등 러스트를 공부하면서 겪게 되는 의문과 진입장벽 대부분이 소유권과 연관이 있습니다. 러스트의 가장 큰 장점으로 꼽히는 `메모리 안전성`이 바로 이 소유권 덕분에 가능한 것입니다.

소유권이 무엇인지, 그리고 러스트가 데이터를 메모리에 어떻게 배치하고 관리하는지 알아보겠습니다.

### 소유권의 의미

소유권은 단어 그대로 생각하면 변수를 마음대로 다룰 수 있는 권한, 즉 변수에 데이터를 할당하고 읽고 쓰고 해제할 수 있는 권리입니다. 함수의 인자로 전달받은 데이터에 대한 소유권도 있을 수 있으므로 여러 함수나 여러 `스레드(Thread)`에서 공유되는 변수나 메모리에 대한 권한을 의미합니다.

`가비지 컬렉터(Garbage Collector)`가 있는 자바 등의 언어는 메모리를 해제할 수 있는 권한이 프로그램 코드가 아닌 가비지 컬렉터에게 있습니다. 프로그램은 메모리를 할당받아 객체를 만들고 읽고 쓸 수 있지만, 직접 해제하지는 않습니다. 더 이상 접근하지 않고 있으면 가비지 컬렉터가 알아서 메모리를 해제해 줍니다.

러스트는 `컴파일러(Compiler)`가 코드를 컴파일할 때 모든 메모리의 소유권을 추적합니다. 러스트가 정한 규칙에 어긋나게 메모리에 접근하는 코드가 있으면 친절한 안내 `메시지`를 출력하고 더 이상 컴파일을 진행하지 않습니다. 그래서 러스트 코드의 컴파일 시간이 오래 걸린다는 불평이 많습니다. 수십~수백 줄의 간단한 코드도 몇 초 정도 시간이 걸리는 것을 보며 답답할 때도 있을 것입니다. 하지만 빌드를 여러 번 할 필요가 없는 것이, `VSCode` 등 대부분의 개발 도구에서 러스트 언어를 동적으로 분석해 주고 코드를 작성할 때마다 에러를 체크해 주기 때문입니다. 빌드하기 전에 미리 모든 컴파일 에러를 고칠 수 있습니다. 또한 `cargo check` 같은 명령을 사용하면 컴파일 에러가 있는지 확인하는 시간을 줄일 수 있습니다.

`VSCode`를 예로 들면 `Inlay hints` <https://code.visualstudio.com/docs/languages/rust#_inlay-hints> 나 `Linting` <https://code.visualstudio.com/docs/languages/rust#_linting> 등의 기능이 있어서, `cargo`를 호출하기 전에 코드를 작성하는 단계에서 미리 거의 모든 컴파일 에러를 잡을 수 있습니다.

또한 러스트 언어는 한번 빌드가 되고 나면 좀처럼 메모리 관련 에러는 발생하지 않습니다. 기타 `로우레벨(Low-level)` 언어로 만든 코드들이 빌드되어 실행은 되더라도 오랜 시간 동안 에러가 없는지 검증해야 하고, `정적 분석 도구` 등을 돌려야 하는 시간을 생각해 보면 전체적인 개발 시간은 확실히 줄어드는 것입니다.

The Rust Programming Language(<https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html>)에서는 소유권이라는 것이 세 가지 규칙을 의미한다고 설명합니다.

- Each value in Rust has an *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

제가 나름대로 번역하고 그 의미에 대해 설명을 붙이면 다음과 같습니다.

- 모든 값(메모리나 변수, 데이터라고 생각해도 좋습니다)은 누군가에게 소유되어야 합니다. 특정한 누군가에게 소유되지 않고 누구나 마음대로 쓸 수 있는 값은 없습니다.
- 한 번에 하나의 소유권자만 존재할 수 있습니다. 여럿이 하나의 변수를 동시에 소유할 수 없습니다.
- 소유권이 존재하는 범위(스코프, `Scope`라고 하며 보통 `{`로 시작하고 `}`로 끝나는 구역을 의미합니다)가 끝나면 변수는 메모리에서 해제되고 더 이상 사용할 수 없게 됩니다.

함수가 대표적인 하나의 스코프입니다. 몇 가지 스코프를 실험하는 예제를 만들어봤습니다.

```rust
// code/ownership_scope/main.rs
struct MyStruct {}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct now!");
    }
}

fn internal_scope() {
    let hello_string = String::from("hello");
    {
        let world_string = String::from("world");
        println!("{}", hello_string);
        println!("{}", world_string);
    }
    println!("{} again", hello_string);
}

fn duplicated_names() {
    let hello_string = String::from("hello");
    {
        let hello_string = String::from("world");
        println!("{}", hello_string);
    }
    println!("{}", hello_string);
}

fn main() {
    internal_scope();
    duplicated_names();

    println!("main starts");
    {
        println!("inner-scope starts");
        let _my: MyStruct = MyStruct {};
        println!("inner-scope ends");
    }
    println!("main ends");
}
```

```bash
$ cargo run --bin ownership_scope
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/ownership_scope`
hello
world
hello again
world
hello
main starts
inner-scope starts
inner-scope ends
Dropping MyStruct now!
main ends
```

간단하게 스코프에 대한 실험을 하는 `internal_scope`라는 함수를 보겠습니다.

```rust
fn internal_scope() {
    let hello_string = String::from("hello");
    {
        let world_string = String::from("world");
        println!("{}", hello_string);
        println!("{}", world_string);
    }
    println!("{} again", hello_string);
}
```

함수 시작 부분에서 생성된 `hello_string`이라는 변수는 함수가 끝나는 `}`를 만나면서 해제됩니다. 함수의 스코프가 끝나는 `}`에서 함수가 사용했던 `hello_string`이라는 변수가 해제되는 것입니다. `"hello"`가 저장된 메모리가 해제되고, `hello_string`이라는 변수를 더 이상 사용할 수 없게 됩니다.

`hello_string`이라는 변수는 `internal_scope` 함수가 소유하며, 따라서 스코프는 `internal_scope` 함수가 끝날 때까지입니다. `world_string`이라는 변수의 소유권은 `internal_scope` 함수 안에 새로 만들어진 블록에 있습니다. 그 새로운 블록의 시작 지점은 두 번째 `{`이고 끝 지점은 첫 번째 `}`가 있는 곳입니다. 따라서 아래와 같이 `world_string`을 소유한 블록 밖에서 `world_string`을 사용할 수 없습니다.

```rust
fn internal_scope() {
    let hello_string = String::from("hello");
    {
        let world_string = String::from("world");
        println!("{}", hello_string);
    }
    println!("{}", world_string);
    println!("{} again", hello_string);
}
```

```rust
$ cargo build
error[E0425]: cannot find value `world_string` in this scope
   --> code/main.rs:15:20
    |
  7 |     println!("{}", world_string);
    |                    ^^^^^^^^^^^^ help: a local variable with a similar name exists: `hello_string`
```

`world_string`이 사용된 위치는 `internal_scope` 함수의 스코프이지만, `world_string`은 내부 블록에서 선언되어 이미 해제되었으므로 해당 변수를 찾을 수 없다는 에러 `메시지`를 확인할 수 있습니다. 반면 `hello_string`은 `internal_scope` 함수의 스코프 내에 존재하므로 내부 블록에서도 문제없이 사용할 수 있습니다.

그럼 `duplicated_names` 함수에서와 같이 같은 이름의 변수가 중첩된 스코프에 존재할 때는 어떨까요?

```rust
fn duplicated_names() {
    let hello_string = String::from("hello");
    {
        let hello_string = String::from("world");
        println!("{}", hello_string);
    }
    println!("{}", hello_string);
}
```

Cargo를 이용해서 코드를 실행해 보면 다음과 같이 출력됩니다.

```rust
$ cargo run --bin ownership_scope
...
world
hello
...
```

두 개의 변수가 동일한 이름으로 생성되지만, `"hello"`라는 값을 가진 변수는 `duplicated_names` 함수의 스코프에 소유권을 가지고 있고, `"world"`라는 데이터를 가진 변수는 그 내부에서 새로 생성된 작은 블록이 소유권을 가지고 있는 것입니다. 작은 스코프가 끝날 때 `"world"`라는 데이터를 가진 변수(혹은 객체)는 해제됩니다.

참고로 스코프가 끝날 때 자신이 소유한 변수들의 `drop` 메서드를 호출합니다. 예제에 `MyStruct`라는 아무런 데이터를 가지지 않는 구조체를 선언하고, `drop` 메서드를 구현해 준 코드가 있습니다. (아직 구조체에 대한 문법을 알아보지 않았지만, 구조체의 선언만 보면 C 언어와 거의 동일합니다. 구조체의 메서드를 정의하는 문법은 아직 모르지만, 일단 `drop`이라는 메서드가 호출되는 시점만 생각해 보겠습니다.)

```rust
struct MyStruct {}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct now!");
    }
}
......
fn main() {
    ......
    println!("main starts");
    {
        println!("inner-scope starts");
        let my: MyStruct = MyStruct{};
        println!("inner-scope ends");
    }
    println!("main ends");
}
```

```bash
$ cargo run --bin ownership_scope
......
main starts
inner-scope starts
inner-scope ends
Dropping MyStruct now!
main ends
```

`drop` 메서드가 호출되는 지점이 곧 변수의 메모리가 해제되는 지점인데, `"inner-scope ends"`라는 메시지 후에 `drop` 메서드가 호출되는 것을 볼 수 있습니다. 즉, 스코프 안의 모든 코드가 끝나고 스코프가 없어지는 최후의 순간에 스코프가 소유한 변수들을 해제하는 것을 확인할 수 있습니다.

### 소유권의 이동

사실 개념 설명만 들으면 "그래서 어쩌라는 건가"라는 생각이 들 수도 있습니다. 제가 자주 겪어본 몇 가지 사례를 소개하겠습니다. 이 정도만 알고 시작해도 작은 프로젝트를 진행하는 데는 큰 문제가 없을 것입니다.

#### 변수 할당에서 소유권이 이동하는 경우

가장 간단한 예는 변수 간 할당이 발생할 때 소유권이 이동하는 경우입니다.

```rust
let s1 = String::from("foo");
println!("{}", s1);
let s2 = s1;
println!("{} {}", s1, s2);
```

이 예제에서 러스트는 `s1`을 `s2`로 이동시킵니다. 보통의 언어들에서는 객체의 복사가 일어나거나 `포인터 복사`가 일어날 것입니다. 러스트에서는 내부적으로 `포인터 복사`만 일어나며, 여기에 더해 소유권 이동까지 발생합니다. 객체 데이터를 복사하지 않기 때문에 속도는 빠르면서 소유권이 이동하므로 데이터가 의도하지 않게 공유되는 것을 방지합니다.

그런데 실제로 무언가를 만드는 경우에 예제와 같이 단순하게 변수 사이에 값을 옮기는 경우는 거의 없습니다. 실제로는 변수값의 이동이 일어나는지 잘 보이지 않는 경우가 대부분입니다.

```rust
let mut user_input = String::from("페리스");
println!("{}", user_input);
let mut greeting = user_input + "씨 안녕하세요";
println!("{}", greeting);
println!("{}", user_input); // Compile error
```

기존 언어에 익숙하다 보면 이 코드에서 문제가 보이지 않을 수 있습니다. 사실 보이지 않는 게 당연합니다. 하지만 러스트에서는 `user_input`의 소유권 이동이 일어나고, 거기에 메시지가 추가되어 `greeting` 변수에 저장된다는 차이가 있습니다.

```rust
error[E0382]: borrow of moved value: `user_input`
   --> code/main.rs:175:20
    |
171 |     let mut user_input = String::from("아이유");
    |         -------------- move occurs because `user_input` has type `String`, which does not implement the `Copy` trait
172 |     println!("{}", user_input);
173 |     let mut greeting = user_input + "씨 안녕하세요";
    |                        ---------------------------- `user_input` moved due to usage in operator
174 |     println!("{}", greeting);
175 |     println!("{}", user_input); // Compile error
    |                    ^^^^^^^^^^ value borrowed here after move
```

이와 같이 소유권 이동이 보이지 않는 경우가 많긴 하지만, 변수 간의 소유권 이동은 `컴파일러`가 어디에서 이동이 발생했는지, 그리고 소유권이 없는 변수에 어디에서 접근하여 에러가 발생했는지를 매우 친절하게 알려줍니다. 그래서 에러를 찾기 쉽고 고치기도 어렵지 않습니다.

#### 함수 인자로 전달되고 반환값을 받을 때 소유권이 이동하는 경우

```rust
fn make_greeting(name: String) -> String {
    let greeting = format!("{}씨 안녕하세요", name);
    greeting
}

fn main() {
    let user = "페리스".to_string();
    let greeting = make_greeting(user);
    println!("{}", greeting);
}
```

이번 예제도 크게 어렵지 않습니다. `user` 변수가 `make_greeting` 함수의 `name` 매개변수에 바인딩되었습니다. 이는 앞서 살펴본 변수 할당 과정과 유사합니다. `user` 변수가 가진 값의 소유권이 `name`으로 이동했기 때문에, `make_greeting` 함수 호출이 끝난 뒤에는 `user` 변수를 더 이상 사용할 수 없습니다. 한편 `greeting` 변수는 `make_greeting` 함수 내에서 생성되었지만, `main` 함수로 소유권이 반환(이동)된 경우에 해당합니다.

```rust
fn make_greeting(name: String) -> String {
    let greeting = format!("{}씨 안녕하세요", name);
    greeting
}

fn main() {
    let mut user = "페리스".to_string();
    user = make_greeting(user);
    println!("{}", user);
}
```

조금 비효율적으로 보일 수 있지만, 위 예제는 `user` 변수의 소유권을 `main`에서 `make_greeting`으로 이동시킨 후 다시 `main`으로 되돌려받는 과정을 보여줍니다. 이런 방식의 활용도 가능하다는 점을 참고해 주세요.

실제로 함수 간에 소유권을 직접 이동시키는 경우는 드뭅니다. 함수를 호출할 때는 보통 객체의 참조(`Reference`)를 전달하여 소유권을 유지하는 방식을 주로 사용합니다.

```rust
fn make_greeting(name: &str) -> String {
    let greeting = format!("{}씨 안녕하세요", name);
    greeting
}

fn main() {
    let mut user = "아이유".to_string();
    user = make_greeting(&user);
    println!("{}", user);
}
```

앞서 살펴본 참조를 이용하면 변수의 소유권 이동 없이도 다른 스코프에서 값을 사용할 수 있습니다. 내부적으로는 포인터만 전달하므로 `C`/`C++`와 같은 저수준 언어와 대등한 성능을 냅니다. 또한, 러스트 컴파일러는 컴파일 단계에서 소유권 이동을 엄격히 체크하고 안전하지 않은 메모리 공유를 차단하므로 성능과 메모리 안전성을 동시에 확보할 수 있습니다.

러스트에서는 이처럼 참조를 생성하는 것을 **빌림(`Borrowing`)**이라고 표현합니다. 소유권 이동 없이 다른 스코프에서 값을 사용할 수 있게 해주므로 매우 적절한 용어입니다.

위 예제에서는 읽기 전용인 **불변 참조(`Immutable reference`)**를 사용했습니다. 데이터를 수정할 수 있는 **가변 참조(`Mutable reference`)**도 존재합니다.

```rust
fn make_greeting(name: &mut String) {
    name.push_str("씨 안녕하세요");
}

fn main() {
    let mut user = "페리스".to_string();
    make_greeting(&mut user);
    println!("{}", user);
}
```

`mut` 키워드를 함수 호출 시와 함수 매개변수 선언부 모두에 명시해야 한다는 점에 유의하세요.

아래 예제처럼 불변(`Immutable`) 변수에 대한 가변 참조를 생성하는 것은 허용되지 않습니다.

```rust
fn main() {
    let user = "페리스".to_string();
    make_greeting(&mut user);
    println!("{}", user);
}
```

데이터의 소유자가 수정을 허용하지 않은 변수를 빌린 쪽에서 마음대로 변경하는 것은 안전하지 않기 때문입니다.

러스트의 참조 규칙을 요약하면 다음과 같습니다.

- `가변 참조(Mutable reference)`는 동시에 단 하나만 존재할 수 있습니다.
- `불변 참조(Immutable reference)`는 동시에 여러 개 존재할 수 있습니다.
- 참조는 항상 유효한 데이터를 가리켜야 합니다 (대상을 잃은 포인터 금지).

이는 데이터 정합성을 고려하면 매우 합리적인 규칙입니다. 데이터를 변경할 수 없는 `불변 참조`가 여러 개 있더라도 데이터의 일관성은 유지됩니다. 반면 데이터를 수정할 수 있는 `가변 참조`가 존재한다면 데이터가 언제든 바뀔 수 있으므로, 다른 참조가 동시에 존재해서는 안 됩니다.

#### `이터레이터(Iterator)`에서의 소유권 이동

마지막으로 `벡터(Vector)`나 배열처럼 여러 데이터를 담은 컬렉션을 `이터레이터`로 순회할 때의 소유권 이동을 살펴보겠습니다. 이 부분은 실제로 실수가 가장 빈번하게 발생하는 지점이며, 소유권 개념에 익숙해지기 전까지는 꽤 당혹스러울 수 있습니다.

배열이나 `벡터`에서 `이터레이터`를 생성하는 메서드는 크게 두 가지가 있습니다.

- `iter()`: `불변 참조` 기반의 `이터레이터`를 생성합니다.
- `into_iter()`: 컬렉션의 소유권을 소비하여 값 기반의 `이터레이터`를 생성합니다.

참고: <https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#tymethod.into_iter>

두 메서드의 차이점을 예제로 확인해 보겠습니다. 먼저 `into_iter()`를 사용하여 값 기반의 `이터레이터`를 만들어 보겠습니다.

```rust
fn main() {
    let user: [String;3] = ["My".to_string(),
                            "Bloody".to_string(),
                            "Valentine".to_string()];
    for c in user.into_iter() {
        println!("{}", c);
    }
    println!("{:?}", user);
}
```

```rust
error[E0382]: borrow of moved value: `user`
   --> code/main.rs:8:22
    |
2   |     let user: [String;3] = ["My".to_string(),
    |         ---- move occurs because `user` has type `[String; 3]`, which does not implement the `Copy` trait
...
5   |     for c in user.into_iter() {
    |                   ----------- `user` moved due to this method call
...
8   |     println!("{:?}", user);
    |                      ^^^^ value borrowed here after move
    |
note: `into_iter` takes ownership of the receiver `self`, which moves `user`
   --> /Users/user/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/code/rust/library/core/code/iter/traits/collect.rs:262:18
    |
262 |     fn into_iter(self) -> Self::IntoIter;
    |                  ^^^^
    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: you can `clone` the value and consume it, but this might not be your desired behavior
    |
5   |     for c in user.clone().into_iter() {
    |                   ++++++++
```

`Copy` 트레이트나 `self`, `clone()` 등 생소한 키워드들이 등장하여 당혹스러울 수 있습니다. 특히 `into_iter()`의 정확한 동작을 모르는 상태에서 이 에러 메시지를 마주하면 해결 방법을 찾기 어려울 수 있습니다.

우선 `into_iter()` 대신 `iter()` 메서드를 사용해 보겠습니다.

```rust
// code/ownership_move/main.rs
fn main() {
    let user: [String; 3] = [
        "My".to_string(),
        "Bloody".to_string(),
        "Valentine".to_string(),
    ];
    for c in user.iter() {
        println!("{}", c);
    }
    println!("{:?}", user);
}
```

```bash
$ cargo run --bin ownership_move
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/ownership_move`
My
Bloody
Valentine
["My", "Bloody", "Valentine"]
```

아무 문제 없이 실행됩니다. 과연 어떤 차이가 있을까요?

`into_iter()`를 사용했을 때 발생한 에러 메시지를 다시 살펴보면 다음과 같은 노트를 확인할 수 있습니다.

> note: `into_iter` takes ownership of the receiver `self`, which moves `user`

값으로 `이터레이터`를 만든다는 것은 컬렉션 내부의 값을 이동시킨다는 의미입니다. 즉, `이터레이터`가 소유권을 가져갑니다. `for` 루프에서 `c` 변수의 타입은 `String`이 되며, 루프를 돌 때마다 배열의 요소를 하나씩 소유하게 됩니다. 루프의 반복이 끝날 때마다 `c`의 스코프가 종료되어 해당 객체는 메모리에서 해제됩니다. 결과적으로 루프가 모두 끝나면 배열 전체가 해제되어 더 이상 사용할 수 없게 됩니다.

> `Visual Studio Code` 등의 `IDE`를 사용하면 타입 추론 기능을 통해 `c`가 어떤 타입인지 쉽게 확인할 수 있습니다.

반면 `iter()`는 컬렉션의 슬라이스를 생성한 뒤 그 슬라이스에 대한 `이터레이터`를 만듭니다. `참조(Reference)`를 이용하므로 소유권 이동이 발생하지 않습니다. 결론적으로 소유권 이동 없이 배열의 각 요소에 참조로 접근하며, 이때 `c` 변수의 타입은 `&String`이 됩니다.

`이터레이터` 사용 팁을 드리자면, `iter()`와 `into_iter()`는 용도가 다음과 같이 다릅니다.

- `iter()`: 루프 종료 후에도 원본 데이터를 계속 사용해야 할 때 사용합니다.
- `into_iter()`: 컬렉션을 소비하여 요소를 해제하거나, 완전히 새로운 형태의 데이터로 변환할 때 사용합니다.

다른 언어에서는 배열을 순회하며 요소를 하나씩 해제하고 마지막에 배열 자체를 해제하는 패턴을 자주 사용합니다. 러스트에서 `into_iter()`를 사용하면, 각 요소가 `for` 루프의 스코프가 끝날 때마다 자동으로 해제되므로 매우 편리합니다.

참고로 `iter()` 메서드는 `불변 참조`를 생성합니다. 따라서 `for` 루프 안에서 데이터를 수정할 수 없습니다. 데이터를 수정하려면 `iter_mut()` 메서드를 사용하여 `가변 참조`를 생성해야 합니다. 자세한 내용은 [`iter_mut()` 문서](https://doc.rust-lang.org/std/slice/struct.IterMut.html)를 참고하세요.

### 복제(`Clone`)와 소유권

앞서 `into_iter()`를 사용했을 때 발생한 컴파일 에러 메시지에는 `user.clone().into_iter()`로 수정해 보라는 제안이 있었습니다.

```rust
fn main() {
    let user: [String; 3] = [
        "My".to_string(),
        "Bloody".to_string(),
        "Valentine".to_string(),
    ];
    for c in user.clone().into_iter() {
        println!("{}", c);
    }
    println!("{:?}", user);
}
```

이 코드는 컴파일 에러 없이 정상적으로 동작합니다. `clone()`은 데이터를 물리적으로 복제하여 새로운 사본을 만듭니다(`Deep copy`). 위 예제에서는 루프를 돌기 전 `user`의 복사본을 생성하고 그 복사본의 `into_iter()` 메서드를 호출합니다. 덕분에 원본 `user` 객체는 유지되고 복사본만 소진됩니다. 하지만 실제 개발 환경에서 불필요한 복제를 남발하는 것은 좋지 않으며, 대부분의 경우 `iter()` 메서드를 사용하는 것이 더욱 효율적입니다.

### 메모리 할당 위치와 소유권

러스트의 내부 동작을 더 깊이 이해하기 위해, 변수가 저장되는 위치와 소유권의 관계를 잠시 살펴보겠습니다.

이전에 작성했던 피보나치 함수를 다시 살펴보겠습니다.

```rust
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
```

`t = a + b` 코드에서 `t` 변수는 `a`와 `b` 중 어느 쪽의 소유권을 가져올까요?

결론부터 말하면 정수 타입은 소유권 이동이 발생하지 않습니다. 정수, 부동 소수점, `bool`과 같은 기본 타입들은 소유권 이동 대신 **값의 복사**가 일어납니다. 함수 인자로 전달되거나 변수에 대입될 때도 값이 그대로 복사됩니다.

소유권 이동 여부를 결정하는 주요 기준은 데이터가 **스택(`Stack`)**에 할당되는지, 아니면 **힙(`Heap`)**에 할당되는지입니다. `스택`에 할당되는 변수는 소유권 이동 대신 복사가 수행됩니다. 반면 `힙`에 할당되는 변수는 명시적인 복사(`clone()`) 없이는 소유권이 이동됩니다.

정수, 부동 소수점, 참/거짓(`bool`) 타입은 메모리 크기가 고정되어 있습니다. 예를 들어 `i32`는 4바이트, `u8`은 1바이트입니다. 이처럼 컴파일 시점에 크기를 알 수 있는 타입들은 `스택`에 할당됩니다. `스택`은 할당과 해제가 매우 빠르고 효율적입니다. 함수가 종료되면 `스택 포인터`가 이동하면서 해당 영역의 데이터들이 자연스럽게 무효화되므로 메모리 누수 걱정이 없으며, 크기가 작아 복사 비용도 저렴합니다. 따라서 소유권을 복잡하게 관리할 필요가 없습니다.

반면 `String`이나 `벡터(Vector)` 같은 타입은 `힙` 영역을 사용합니다. 실행 시점에 데이터 크기가 변할 수 있기 때문입니다. 예를 들어 사용자 입력이나 네트워크 응답처럼 런타임에 크기가 결정되는 데이터는 `힙`에 동적으로 할당해야 합니다. 또한, 나중에 데이터를 추가할 수 있는 경우나 사용자 입력을 받아 `String` 객체를 생성할 때, 혹은 네트워크를 통해 받은 데이터로 객체를 생성할 때처럼 프로그램 실행 중에만 데이터 크기를 알 수 있는 경우도 마찬가지입니다.

```rust
fn main() {
    let s = String::new();
}
```

위와 같이 `s`라는 변수를 만들었습니다. 이 `s`는 `스택`에 생성된 포인터 변수입니다. 64비트 `CPU`를 가진 시스템에서 동작한다면 `스택`에 8바이트 메모리 영역을 할당하고, `힙` 영역에 `String` 객체를 생성한 후 `스택`에 있는 8바이트 메모리 영역에 `힙` 영역의 주소를 저장하게 됩니다. 우리가 `s`라는 변수를 통해 객체에 저장된 데이터를 읽으면 다음과 같은 과정을 거칩니다.

1. `s` 변수에서 `힙` 영역의 주소 값을 읽음
2. 해당 `힙` 영역 주소에서 실제 데이터를 읽음

이와 같이 두 번의 메모리 접근이 일어납니다. `String` 객체가 변수 대입이나 함수 호출을 통해 소유권이 이동된다는 것은, 물리적으로 따지면 포인터 값(64비트 정수 값)을 복사하는 것과 같습니다. 컴파일러는 변수 대입이나 함수 호출 등 소유권 규칙에 따른 동작이 일어날 때마다 포인터 값의 이동을 감시하고 규칙에 부합하는지를 확인할 뿐입니다. 결과적으로 안정적인 메모리 관리를 수행하면서도 성능 저하가 없는 프로그램을 만들 수 있습니다.

정리하자면 러스트에서 `원시 타입(Primitive type)`으로 분류된 타입들은 이동이 아니라 복사가 일어납니다. 어떤 타입들이 `원시 타입`인지는 러스트의 `표준 라이브러리(Standard Library)` 매뉴얼을 참고하시기 바랍니다.

<https://doc.rust-lang.org/std/#primitives>

`C`나 예전 `C++`을 사용해 본 개발자라면 다음과 같이 생각하면 쉽습니다.

> `malloc`/`new` 등으로 할당하고 `free`로 해제해 줘야 하는 메모리나 객체를 자동으로 해제해 주는 대신, 소유권을 관리해 줘야 한다. `원시 타입(Primitive type)`은 복사가 일어나고 그 외의 타입은 이동이 발생한다.

모던 `C++`을 아는 개발자라면 다음과 같이 이해하면 더욱 쉽습니다.

> `RAII`가 권장이 아니라 강제 사항이며, 모든 포인터는 `스마트 포인터(Smart Pointer)`이다.

나중에 `Copy` 트레이트(`Copy trait`)라는 개념이 나오는데, 미리 간단히 설명하자면 데이터 타입의 크기를 컴파일러가 미리 알 수 있어 데이터의 이동 대신 복사를 수행하는 타입들의 속성이라고 생각하면 됩니다. 컴파일러가 크기를 안다는 것은 `원시 타입(Primitive type)`이 기본적으로 `Copy` 트레이트를 구현하고 있다는 의미입니다. 그 외의 타입들은 동적으로 크기가 바뀔 수 있으므로 컴파일러가 `Copy` 트레이트를 자동으로 구현해 주지 못합니다. 동적으로 크기가 바뀌거나 또 다른 객체를 포함하고 있는 등의 데이터는 `clone()`을 사용해야 합니다.

## 구조체

러스트에는 클래스가 없고 `구조체(struct)`만 존재합니다. `구조체`에 메서드를 추가할 수 있지만 상속 기능은 없기 때문에 완전한 `OOP` 언어는 아닙니다. `구조체`의 형태는 대부분의 다른 언어와 크게 다르지 않습니다.

아래 예제는 다양한 `구조체` 형태를 소개합니다.

```rust
// code/struct/main.rs
// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
```

출처: <https://doc.rust-lang.org/rust-by-example/custom_types/structs.html>

조금이라도 프로그래밍을 해보신 분들이라면 이미 잘 알고 계실 만한 `구조체`와 `튜플`의 모습 그대로입니다. 그나마 `유닛 구조체(Unit struct)`라는 것이 조금 특이한데, 아무런 내부 데이터가 없는 `구조체`입니다. 이는 나중에 `트레이트(Trait)`라는, 클래스의 메서드와 유사한 기능을 사용하기 위한 용도로 쓰입니다. 내부 변수는 없고 메서드만 있는 클래스라고 생각할 수도 있습니다.

다른 언어와 확실히 다른 점은 `구조체`를 만들 때 인자로 사용된 객체의 소유권이 이동한다는 것입니다. 다음 예제를 실행해 보겠습니다.

```rust
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{}", peter.name);
    println!("{}", name);
}
```

```rust
error[E0382]: borrow of moved value: `name`
 --> code/main.rs:6:20
  |
2 |     let name = String::from("Peter");
  |         ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
3 |     let age = 27;
4 |     let peter = Person { name, age };
  |                          ---- value moved here
5 |     println!("{}", peter.name);
6 |     println!("{}", name);
  |                    ^^^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
4 |     let peter = Person { name.clone(), age };
  |                              ++++++++
```

이전에 소유권 이동에 대해 설명하며 소유권이 없는 변수에 접근했을 때 보여드린 에러 메시지와 거의 동일한 형태를 다시 보게 됩니다. 각 에러 메시지의 의미를 살펴보겠습니다.

1. `move occurs because name has type String, which does not implement the Copy trait`: `String` 타입은 `Copy` 트레이트를 구현하지 않습니다. 컴파일러가 `String` 타입의 메모리 크기를 미리 알 수 없기 때문입니다. 현재 예제 코드는 "Peter"라는 리터럴을 `String`으로 만들기 때문에 크기를 알 수 있는 것처럼 보이지만, 동적으로 `String`을 생성하는 경우 문자열이 얼마나 길어질지 예측할 수 없습니다.
2. `value moved here`: `name` 변수의 소유권이 `Person` 구조체를 생성할 때 이동했습니다.
3. `value borrowed here after move`: `println!`으로 소유권이 없는 변수에 접근했으므로 에러가 발생한 것입니다.
4. `consider cloning the value if the performance cost is acceptable`: `name.clone()`으로 복사본을 만들어 `Person`에 전달하는 것도 하나의 해결책이지만, 불필요하게 메모리를 더 사용하게 됩니다.

요약하자면 `Person` 객체를 만들기 위해 `name`이라는 `String` 객체를 사용했고, 이 과정에서 `name`의 소유권이 `peter` 변수 내부의 필드로 넘어갔다는 것입니다. 따라서 `peter` 변수가 생성된 이후로는 `name` 변수를 더 이상 사용할 수 없습니다.

### 메서드 정의

`구조체`를 생성하는 방법을 살펴보았으니, 이번에는 `구조체`의 메서드를 정의하는 예제를 보겠습니다.

```rust
// code/struct_define_main.rs
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let width = f32::abs(self.top_left.x - self.bottom_right.x);
        let height = (self.top_left.y - self.bottom_right.y).abs();
        width * height
    }
}

fn main() {
    let point1: Point = Point { x: 10.3, y: 0.4 };
    let point2: Point = Point { x: 22.5, y: 2.4 };
    let rect = Rectangle {
        top_left: point1,
        bottom_right: point2,
    };
    println!("area size={}", rect.area());
}
```

`Point`와 `Rectangle`이라는 `구조체`를 생성합니다. 그 아래에는 `Rectangle` `구조체`의 메서드를 정의하는 `impl` 구문이 있습니다. 메서드를 정의할 때는 `impl` 키워드 뒤에 `구조체` 이름을 쓰고 블록을 생성합니다. 그리고 그 블록 안에서 `&self`를 첫 번째 인자로 받는 함수를 정의하면 메서드가 됩니다. 이는 다른 언어에서 클래스 메서드를 만드는 것과 유사합니다.

눈여겨볼 점은 `f32` 타입의 절댓값을 구하는 `abs()` 메서드가 다음과 같이 두 가지 형태로 사용된다는 것입니다.

1. `타입::메서드이름(..인자..)`
2. `변수.메서드이름(..인자..)`

1번 `타입::메서드이름` 형태는 보통 정적 메서드 또는 `연관 함수(Associated function)`라고 부릅니다. `구조체` 타입 자체에 종속되는 함수이므로, 인스턴스를 생성하지 않고도 호출할 수 있습니다. 2번 `변수.메서드이름` 형태는 `인스턴스 메서드`라고 하며, 반드시 객체를 생성한 후에 해당 객체를 통해 호출할 수 있습니다. 따라서 첫 번째 인자로 항상 `&self`를 받습니다.

메서드의 첫 번째 인자로는 `&self`뿐만 아니라 `&mut self`도 사용할 수 있습니다. `구조체` 내부의 값을 변경하는 메서드라면 `&mut self`를 사용해야 합니다. 또한, 자기 자신의 메모리를 해제하는(원문에서는 `consume`이라고 표현합니다) 메서드라면 `self` 인자를 가집니다. `self` 앞에 `&`가 붙지 않는 것은 메서드가 자기 자신의 소유권을 직접 전달받는다는 의미입니다.

```rust
// code/struct_method/main.rs
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn new() -> Rectangle {
        Rectangle {
            top_left: Point { x: 0.0, y: 0.0 },
            bottom_right: Point { x: 0.0, y: 0.0 },
        }
    }

    fn area(&self) -> f32 {
        let width = f32::abs(self.top_left.x - self.bottom_right.x);
        let height = (self.top_left.y - self.bottom_right.y).abs();
        width * height
    }

    fn destroy(self) {
        // do nothing but free myself
        println!("destroyer");
    }
}

fn main() {
    let rect = Rectangle::new();

    {
        let point1: Point = Point { x: 10.3, y: 0.4 };
        let point2: Point = Point { x: 22.5, y: 2.4 };
        let rect2 = Rectangle {
            top_left: point1,
            bottom_right: point2,
        };
        rect2.destroy();

        //println!("area size={} {:?}", rect2.area(), rect2); // compile error!!!
    }

    println!("area size={} {:?}", rect.area(), rect);
}
```

```bash
$ cargo run --bin struct_method
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/struct_method`
destroyer
area size=0 Rectangle { top_left: Point { x: 0.0, y: 0.0 }, bottom_right: Point { x: 0.0, y: 0.0 } }
```

`new`라는 이름의 메서드는 러스트의 코딩 관례상 빈 객체를 생성하는 메서드의 이름으로 많이 쓰입니다. 그래서 보통 정적 메서드로 구현됩니다.

`destroy`라는 메서드는 인자를 `self`로 받아오므로 객체의 소유권을 가져옵니다. 따라서 메서드가 종료된 후부터는 객체를 더 이상 쓸 수 없습니다. `new`와 같이 특별히 정해진 이름이 있는 것은 아닙니다. 그리고 `destroy`와 같이 명시적으로 객체를 해지하는 메서드를 만드는 것은 특별한 일이 아니라면 잘 쓰지 않는 방법입니다.

메서드에서 `self`를 이용해 소유권을 받아오는 것을 확인하기 위해, 주석 처리된 부분을 다시 코드로 바꾸고 빌드해 보겠습니다.

```rust
......

fn main() {
    let rect = Rectangle::new();

    {
        let point1: Point = Point { x: 10.3, y: 0.4 };
        let point2: Point = Point { x: 22.5, y: 2.4 };
        let rect2 = Rectangle {
            top_left: point1,
            bottom_right: point2,
        };
        rect2.destroy();

        println!("area size={} {:?}", rect2.area(), rect2); // compile error!!!
    }

    println!("area size={} {:?}", rect.area(), rect);
}
```

```bash
gkim@gkim-laptop:~/study/my-rust-book$ cargo run --bin struct_method
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
error[E0382]: borrow of moved value: `rect2`
  --> code/struct_method/main.rs:45:39
   |
39 |         let rect2 = Rectangle {
   |             ----- move occurs because `rect2` has type `Rectangle`, which does not implement the `Copy` trait
...
43 |         rect2.destroy();
   |               --------- `rect2` moved due to this method call
44 |
45 |         println!("area size={} {:?}", rect2.area(), rect2); // compile error!!!
   |                                       ^^^^^ value borrowed here after move
   |
note: `Rectangle::destroy` takes ownership of the receiver `self`, which moves `rect2`
  --> code/struct_method/main.rs:27:16
   |
27 |     fn destroy(self) {
   |                ^^^^

For more information about this error, try `rustc --explain E0382`.
error: could not compile `my-rust-book` (bin "struct_method") due to 1 previous error
```

이제는 조금 익숙해진 에러 메시지들이 보입니다.

### 구조체 디버깅 방법

이전 예제를 보면 `Point` 구조체와 `Rectangle` 구조체의 정의 윗줄에 `#[derive(Debug)]`라는 코드가 있습니다.

```rust
// code/struct_method/main.rs
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

......

fn main() {
    let rect = Rectangle::new();

......

    println!("area size={} {:?}", rect.area(), rect);
}
```

이 예제를 실행하면 구조체 이름과 각 필드의 이름, 그리고 값까지 출력해 줘서 굉장히 편리합니다.

```bash
$ cargo run --bin struct_method
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/struct_method`
destroyer
area size=0 Rectangle { top_left: Point { x: 0.0, y: 0.0 }, bottom_right: Point { x: 0.0, y: 0.0 } }
```

`#[derive(Debug)]`라는 구문은 `std::fmt::Debug` (Standard library에 속한 `fmt`라는 모듈에 정의된 `Debug`라는 `trait`)를 자동으로 구현하라는 의미입니다. 나중에 `Trait`에 대해서 설명할 때 정확한 의미를 알아보겠지만, 지금은 일단 `"{:?}"`라는 표현식을 써서 구조체의 각 필드 값을 출력한다고 생각하면 됩니다. 구조체의 필드가 `String` 같은 `std`에 정의된 타입이면 대부분 동작합니다. 만약 구조체의 한 필드가 또 다른 구조체 타입이라면, 그 다른 구조체도 `#[derive(Debug)]`를 선언해 주면 됩니다. `Rectangle`에만 `#[derive(Debug)]`를 사용한 게 아니라 `Point`에도 `#[derive(Debug)]`를 선언한 이유는 `Rectangle`의 디버깅 메시지를 출력할 때 `Point`의 디버깅 메시지도 같이 출력되어야 하기 때문입니다.

## 열거형 Enums

### 기본 열거형

열거형도 패턴 매칭과 마찬가지로 러스트를 처음 접한 개발자들이 낯설어하는 특징 중 하나입니다. 하지만 조금만 쓰다 보면 너무나 편리하기 때문에 자주 쓰게 됩니다.
러스트 언어다운 프로그래밍을 하려면 이 열거형을 잘 활용하는 게 중요합니다.
C 언어나 자바, Go 등에서 보통 열거형을 쓰는 이유는 특정 값만을 가지는 타입을 새로 만들기 위해서입니다. 아래 C 언어 예제를 보겠습니다.

```c
// code/enum_basic/enum.c
#include <stdio.h>

enum WEEK {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
};

int main()
{
    enum WEEK today;
    today = Sunday;
    printf("%d\n", today);
    today = 22;
    printf("%d\n", today);
    return 0;
}
```

위와 같이 `WEEK`이라는 새로운 타입을 만들었습니다. `WEEK` 타입의 변수는 `Sunday`부터 `Saturday`라는 값만을 갖도록 만드는 게 목표입니다. 하지만 사실 C 언어의 대부분의 타입이 그렇듯이 `Sunday`부터 `Saturday`가 사실상 모두 정수값이기 때문에, `today` 변수에 아무 정수값이나 넣어도 문제가 없습니다. `today` 변수에 22라는 아무 정수값이나 저장하고 사용해도 컴파일 에러가 없고 잘 동작합니다. 에러를 방지할 수 있는 방법이 전혀 없습니다.

러스트의 열거형도 마찬가지로 가장 기본적인 사용법은 특정 값만을 갖는 새로운 타입을 만드는 것입니다.

```rust
// code/enum_basic/main.rs
enum WEEK {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

fn main() {
    let today: WEEK = WEEK::Sunday;
    // let tomorrow: WEEK = 1; // compile error!!!

    match today {
        WEEK::Sunday => println!("Sunday: Sleep for 10 hours"),
        WEEK::Monday => println!("Monday: Work"),
        WEEK::Tuesday => println!("Tuesday: Work"),
        WEEK::Wednesday => println!("Wednesday: Work"),
        WEEK::Thursday => println!("Thursday: Work"),
        WEEK::Friday => println!("Friday: Work"),
        WEEK::Saturday => {
            println!("Saturday: Party at Club from 22")
        }
    }
}
```

```bash
/my-rust-book$ cargo run --bin enum_basic
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
warning: variants `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, and `Saturday` are never constructed
 --> code/enum_basic/main.rs:3:5
  |
1 | enum WEEK {
  |      ---- variants in this enum
2 |     Sunday,
3 |     Monday,
  |     ^^^^^^
...
  = note: `#[warn(dead_code)]` on by default

warning: `my-rust-book` (bin "enum_basic") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/enum_basic`
Sunday: Sleep for 10 hours
```

정의하고 사용하는 방법은 C 언어와 거의 유사합니다. 하지만 가장 큰 차이는 `WEEK` 타입의 변수에 정말로 `WEEK` 타입의 값인 `WEEK::Sunday`부터 `WEEK::Saturday` 외의 값을 저장하려고 하면 컴파일 에러가 발생한다는 것입니다. `WEEK` 타입의 인자를 받는 함수를 사용할 때도 `WEEK` 타입의 값 외에 잘못된 값을 전달할 수 없습니다. 의도하지 않은 잘못된 값을 사용하는 것을 방지해 줍니다. 주석 처리된 13번째 줄을 코드로 바꾸고 빌드해 보겠습니다.

```rust
......
fn main() {
    let today: WEEK = WEEK::Sunday;
    let tomorrow: WEEK = 1; // compile error!!!
......
```

```bash
$ cargo run --bin enum_basic
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
error[E0308]: mismatched types
  --> code/enum_basic/main.rs:13:26
   |
13 |     let tomorrow: WEEK = 1; // compile error!!!
   |                   ----   ^ expected `WEEK`, found integer
   |                   |
   |                   expected due to this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `my-rust-book` (bin "enum_basic") due to 1 previous error
```

`WEEK` 타입의 변수에 `WEEK` 타입이 아닌 정수값을 저장할 수 없으므로 에러가 발생합니다. 타입을 확실히 구분하기 때문에 에러가 나는 것입니다.

그리고 컴파일러가 주는 경고 메시지를 보면 `WEEK` 타입으로 선언된 값들 중에 사용되지 않는 값이 있는 것도 알려줍니다. 또한 아주 중요한 기능이 있는데, 패턴 매칭에서 처리가 안 되는 경우가 있으면 컴파일 에러가 난다는 것입니다. 예제에 있는 패턴 매칭을 보면 현재는 컴파일이 잘 되도록 만들기 위해 모든 요일을 다 처리하고 있습니다만, 그중 하나라도 지우면 어떻게 될까요?

```rust
......
    match today {
        WEEK::Sunday => println!("Sunday: Sleep for 10 hours"),
        //WEEK::Monday => println!("Monday: Work"),
        WEEK::Tuesday => println!("Tuesday: Work"),
        WEEK::Wednesday => println!("Wednesday: Work"),
        WEEK::Thursday => println!("Thursday: Work"),
        WEEK::Friday => println!("Friday: Work"),
        WEEK::Saturday => {
            println!("Saturday: Party at Club from 22")
        }
    }
......
```

```bash
$ cargo run --bin enum_basic
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
error[E0004]: non-exhaustive patterns: `WEEK::Monday` not covered
  --> code/enum_basic/main.rs:15:11
   |
15 |     match today {
   |           ^^^^^ pattern `WEEK::Monday` not covered
   |
note: `WEEK` defined here
  --> code/enum_basic/main.rs:1:6
   |
1  | enum WEEK {
   |      ^^^^
2  |     Sunday,
3  |     Monday,
   |     ------ not covered
   = note: the matched value is of type `WEEK`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
24 ~         },
25 +         WEEK::Monday => todo!()
   |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `my-rust-book` (bin "enum_basic") due to 1 previous error
```

위와 같이 `WEEK` 타입에 여러 가지 값들이 있는데, 그중에서 `WEEK::Monday`가 처리되지 않고 있다는 에러 메시지를 보여줍니다. 친절하게 어떻게 케이스를 추가하라고도 알려줍니다.

저는 프로젝트가 거대해지고 다른 사람이 만든 코드를 유지보수할 경우에, 실수로 모든 경우에 대한 처리를 하지 않아서 잘 드러나지 않는 에러가 나는 경우를 많이 겪어봤습니다. 해결하기 어려운 문제는 아닙니다만, 릴리스된 후에 이런 문제를 발견하면 새로운 버전을 출시해야 하는 번거로움이 있고, 사용자에게 새로운 버전을 설치하라고 안내해야 하는 등 후속 처리가 쉽지 않습니다. 이렇게 개발자의 실수를 컴파일러가 방지하는 것이 러스트의 디자인 철학입니다. 사람은 사람이 잘하는 것을 하고, 기계는 기계가 잘하는 것을 할 수 있어서 정말 편리합니다.

### 데이터를 포함하는 열거형

러스트 언어의 열거형(`Enums`)은 다음과 같이 데이터를 포함할 수도 있습니다.

이전에 만든 열거형 예제에서는 각 요일마다 해야 할 일이 사용자에게 출력할 메시지 안에 저장되어 있어서 동적으로 바꿀 수 없게 되어 있었습니다. 다음과 같이 각 요일마다 해야 할 일 등의 정보를 저장하도록 바꿀 수 있습니다.

```rust
// code/enum_data/main.rs
#[derive(Debug)]
enum WEEK {
    Sunday(String, i32),
    Monday(String),
    Tuesday(String),
    Wednesday(String),
    Thursday(String),
    Friday(String),
    Saturday {
        what: String,
        place: String,
        when: i32,
    },
}

fn main() {
    let schedule: [WEEK; 2] = [
        WEEK::Sunday("Sleep".to_string(), 10),
        WEEK::Saturday {
            what: "Party".to_string(),
            place: "Club".to_string(),
            when: 22,
        },
    ];

    for day in schedule.into_iter() {
        match day {
            WEEK::Sunday(todo, hours) => println!("Sunday: do {} for {} hours", todo, hours),
            WEEK::Monday(todo) => println!("Monday: do {}", todo),
            WEEK::Tuesday(todo) => println!("Tuesday: do {}", todo),
            WEEK::Wednesday(todo) => println!("Wednesday: do {}", todo),
            WEEK::Thursday(todo) => println!("Thursday: do {}", todo),
            WEEK::Friday(todo) => println!("Friday: do {}", todo),
            WEEK::Saturday { what, place, when } => {
                println!("Saturday: do {} at {} from {}", what, place, when)
            }
        }
    }
}
```

이제 각 요일에 해당하는 타입은 각 요일마다 해야 할 일에 대한 정보를 `String` 타입으로 저장할 수 있습니다. 토요일, 일요일에는 추가 정보를 저장할 수 있습니다. 각 요일에 할 일을 동적으로 지정할 수 있게 되었습니다.

일요일에는 `String`과 `i32` 두 가지 데이터를 저장했습니다. 튜플처럼 각 데이터는 이름을 가지지 않습니다. 패턴 매칭에서 `todo`, `hours`라고 임시로 이름을 지어서 각 데이터를 지정해 줬습니다만, 아무 이름이나 사용할 수 있습니다. 하지만 토요일에는 할 일, 장소, 시간을 저장하는데, 마치 구조체처럼 각 필드마다 이름을 지정했습니다. 패턴 매칭에서 토요일을 패턴 매칭할 때 `WEEK::Saturday`에서 정의된 각 필드 이름 `what`, `place`, `when`을 그대로 똑같이 사용해야 한다는 것에 주의하세요.

한 가지 더 생각해 볼 것은 `schedule` 배열을 순회할 때 `into_iter` 메서드를 사용했다는 것입니다. C 언어였다면 각 요일마다 메시지 출력 후에 내부 데이터를 해지하고, 배열을 해지하는 등 메모리를 일일이 신경 써줘야 했지만, 러스트에서는 그냥 이렇게 소유권을 가져가서 처리하고 스코프를 닫기만 하면 사용한 모든 데이터가 자동으로 해지됩니다. 여러 스레드 간에 메시지를 주고받는 경우를 생각해 보세요. 최종적으로 메시지를 해지해야 하는 스레드는 그냥 데이터의 소유권을 전달받으면 됩니다. 다른 스레드에는 참조만 전달하면 절대로 데이터를 해지할 수 없습니다. 이렇게 하면 개발자가 잘못된 스레드에서 데이터를 해지하는 실수도 방지되고, 반대로 데이터를 해지해야 하는데 해지하지 않는 실수도 방지할 수 있습니다. 소유권을 전달할지, 참조를 전달할지, 참조를 전달하되 가변 참조를 전달할지 불변 참조를 전달할지 설계 단계에서만 잘 결정하면 구현 단계에서는 잘못될 일이 없어지는 것입니다.

## 에러 처리를 위한 Result

열거형의 기본 정의에 대해서 알아봤으니, 열거형 타입의 데이터 구조 중에 가장 많이 사용되는 `Result`에 대해서 이야기하겠습니다.

`Result`가 실제로 어떻게 정의된 것인지 소스 코드부터 보겠습니다.

```rust
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```

출처: <https://doc.rust-lang.org/std/result/>

`Result`는 프로그램 실행 중에 발생한 에러를 표현하는 타입입니다. 그중 가장 대표적인 예가 함수의 반환값입니다. `Result`에는 2개의 타입이 존재합니다. (영어로는 variant라고 부르지만, 이 책에서는 타입이라고 부르겠습니다.) `Ok`는 함수가 동작에 성공했을 때 함수가 반환하는 값을 내장하는 타입이고, `Err`는 함수가 실패했음을 나타내는 값을 내장하는 타입입니다. 함수의 실패를 나타내는 에러 메시지가 될 수도 있고, 에러 상태를 나타내는 데이터가 될 수도 있겠지요.

아주 간단한 예제부터 보겠습니다.

```rust
// code/result_enum/main.rs
fn divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        return Err(String::from("denominator cannot be zero"));
    }
    Ok(numerator / denominator)
}

fn main() {
    let result = divide(10, 0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(message) => println!("Error: {}", message),
    }
}
```

```bash
$ cargo run --bin result_enum
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
     Running `target/debug/result_enum`
Error: denominator cannot be zero
```

`divide` 함수는 나눗셈이 정상적으로 처리되었으면 `Ok` 안에 결과 값을 전달하고, 나눗셈을 실행할 수 없는 에러 상황을 만나면 `Err` 타입에 에러 메시지를 넣어서 전달합니다. `main` 함수는 반환값의 타입을 보고 `divide` 함수가 반환한 값이 정상적인 결과인지 문제가 발생한 상황인지를 알 수 있습니다. 타입을 확인하는 것은 패턴 매칭을 이용하면 항상 모든 에러 값을 놓치지 않고 처리할 수 있습니다. 여기서 패턴 매칭의 편리함과 강력함을 다시 느끼게 됩니다.

사실 C/C++ 언어에서 포인터를 반환하는 함수들이 에러 상황에 `NULL` (사실은 정수 0을 다른 이름으로 바꾸기만 한 것)을 반환하는 게 보통인데, 이게 에러 상황인 것은 나타낼 수 있지만 왜 에러가 발생했는지를 표현할 수도 없고 실수하기도 쉬운 불편한 방식이었습니다. `NULL`이라는 개념을 처음 만들었다는 Tony Hoare님이 후회한다고(<https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/>) 이야기한 것도 그렇고, 모던 C++ (C++ 17)에서 `optional`, `expected` 등을 도입하는 것 등을 보면 `Result`를 잘 활용하는 것이 프로그램의 안정성에 얼마나 필수적인지 알 수 있습니다.

반드시 반환값을 갖는 함수는 최대한 전부 `Result` 타입으로 반환하도록 작성하려고 노력해 보세요. 참고로 `Result`에서는 한 가지 타입의 에러만 반환할 수 있습니다. `divide` 함수에서 반환할 수 있는 에러는 `String` 타입뿐입니다. 만약에 좀 더 긴 함수를 작성하고 있고 이 함수가 몇 가지 라이브러리를 호출하는데, 각 라이브러리마다 반환하는 에러의 타입이 다르다면 어떻게 해야 할까요? 각 라이브러리마다 자신의 에러를 표현하기 위한 구조체를 만들어서 사용한다면, 모든 에러 값들을 하나의 타입으로 또다시 바꿔야 할까요? 뒤에 나올 `trait`라는 것을 사용해서 다양한 에러 타입들을 하나의 타입으로 표현할 수 있습니다. 지금은 어떤 상황에서도 `Result`를 사용할 수 있다는 것만 기억하시기 바랍니다.

### 반환값이 없는 함수에서 Result를 사용하는 방법

그럼 반환값이 없는 함수는 `Result`를 쓸 필요가 없을까요? 다음과 같은 경우를 생각해 보겠습니다.

```rust
fn check_command_valid(cmd: &str) -> Result<(), String> {
    match cmd {
        "good" => Ok(()),
        "unsupported" => Err("Unsupported command".to_owned()),
        "bad" => Err("Bad command".to_owned()),
        _ => Err("Wierd command".to_owned()),
    }
}

fn main() {
    match check_command_valid("blabla") {
        Ok(_) => (),
        Err(error_msg) => println!("Command failed because it is a {}", error_msg),
    }
}
```

```bash
$ cargo run --bin result_noreturn
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/result_noreturn`
Command failed because it is a Wierd command
```

`cmd`로 전달받은 명령어에 문제가 있다면 에러 메시지를 반환하는 함수입니다. 그리고 문제가 없을 때는 아무 반환값도 없습니다. 이렇게 반환값이 없는 함수라 하더라도 성공했는지 실패했는지, 실패했으면 어떤 에러 상황인지 등의 정보를 전달해야 할 때가 많습니다. 이럴 때는 위 예제와 같이 비어 있는 값 `()`를 반환하도록 하면 됩니다. 그리고 패턴 매칭에서는 `()`와 매칭되도록 하면 아무런 처리도 하지 않게 됩니다.

## 함수 결과값 반환을 위한 Option

`Result`는 특정 처리가 성공했나 실패했나를 표현할 수 있었습니다. 그런데 모든 게 다 성공과 실패로 판단되는 것은 아닙니다. 예를 들어 어떤 프로그램이 이전에 기록했던 파일을 다시 읽는 경우를 생각해 보겠습니다. 프로그램이 종료될 때마다 어디까지 실행했었고 결과값이 무엇이었는지 등을 기록합니다. 그리고 프로그램을 다시 시작하면 이전 결과 파일을 읽어서 이어서 처리하게 됩니다. 그런데 프로그램이 설치된 후 최초로 실행되는 경우는 어떨까요? 프로그램이 처음 실행될 때는 파일이 없을 수 있습니다. 그런 경우는 실패도 아니고 에러 상황도 아닙니다. 굳이 따지자면 에러 상황으로 처리할 수도 있지만, 좋은 방법은 아닙니다. 왜냐하면 프로그램의 설치가 잘못되어서 파일 시스템을 못 읽거나, 다른 에러 때문에 파일이 있어도 못 읽는 것과는 다른 것이기 때문입니다. 이와 같이 에러는 아니지만 예외적인 경우가 있을 수 있습니다. 러스트는 이런 경우의 처리를 위해 `Option`이라는 열거형 타입을 제공합니다.

`Option`의 정의는 값이 있고 없고를 표현하는 타입입니다. 실제로 어떻게 정의된 것인지 소스 코드를 먼저 확인해 보겠습니다.

```rust
enum Option<T> {
    Some(T),
    None,
}

```

출처: <https://doc.rust-lang.org/std/option/enum.Option.html>

값이 있을 때는 `Some` 타입 안에 존재하는 값을 저장하고, 값이 없을 때는 `None`으로 표현합니다. 가장 많이 사용하는 경우가 함수 반환 값을 `Option`으로 반환하는 것입니다. `Result`와 마찬가지로 되도록 모든 함수의 반환값을 `Option`으로 처리할 수 있도록 노력해야 합니다.

이제 사용 예제를 한번 보겠습니다.

```rust
fn second(s: &[i32]) -> Option<i32> {
    if s.len() == 0 {
        None
    } else {
        Some(s[1])
    }
}

fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    match x {
        Some(n) => println!("x is {}", n),
        None => println!("x is not present"),
    }

    match y {
        Some(n) => println!("y is {}", n),
        None => println!("y is not present"),
    }

    if let Some(n) = x {
        println!("x is {}", n);
    }

    if let Some(n) = y {
        println!("y is {}", n);
    } else {
        println!("y is not present");
    }

    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    println!("x is {}", x.unwrap());
    //println!("y is {}", y.unwrap()); // panic!!!

    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    println!("x is {}", x.unwrap_or(-1));
    println!("y is {}", y.unwrap_or_default());

    let y: Option<i32> = second(&[]);
    let item = y.expect("An argument of second should not be empty");
    println!("This line is not reachable because item is {}", item);
}
```

```bash
$ cargo run --bin option_enum
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/option_enum`
x is 5
y is not present
x is 5
y is not present
x is 5
x is 5
y is 0
thread 'main' panicked at code/option_enum/main.rs:46:18:
An argument of second should not be empty
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

사용법 자체는 크게 어렵지 않습니다. 패턴 매칭을 사용해서 결과 값을 확인하는 것도 `Result`에서 해본 방식입니다. 패턴 매칭을 해서 `Some` 타입이면 내부 데이터를 꺼내서 사용하면 됩니다. 만약에 `None`이면 아무런 데이터도 없는 것이므로 데이터가 없는 경우에 대한 처리를 하면 됩니다.

```rust
......
    match x {
        Some(n) => println!("x is {}", n),
        None => println!("x is not present"),
    }

    match y {
        Some(n) => println!("y is {}", n),
        None => println!("y is not present"),
    }
......
```

`x`는 `Some` 타입이면서 `5`라는 값을 내장하고 있습니다. 그러므로 `5`를 출력하게 됩니다. `y`는 아무런 값도 없는 `None` 타입입니다. 어떤 값도 들어있지 않으므로, 값이 없다는 안내 메시지를 출력합니다.

>
> C 언어를 오래 사용하다 보면 에러 값과 값이 없는 상태를 혼동할 수 있습니다. C 언어에서는 초기화되지 않은 변수라 해도 `0`이나 쓰레기 값이 들어있을 수밖에 없습니다. 하지만 그것이 실제 데이터가 있는 것인지, 아니면 초기화되지 않은 상태인 건지 구분할 수 없다는 점을 이해하실 것입니다. 러스트는 초기화되지 않은 변수를 허용하지 않습니다. 만약 데이터가 없는 상태일 수도 있는 변수나 함수의 결과 값을 저장할 때는 `Option`을 사용하면 됩니다.
> 이와 관련해서 인터넷에 떠도는 화장실 유머가 이런 상태를 가장 잘 표현한다고 생각합니다. 화장실에 갔는데 화장지가 1칸 남아있다면 결과 값은 `1`입니다. 만약 화장지를 앞사람이 다 써서 화장지 심만 남아있다면 결과 값은 `0`입니다. 만약에 화장지 심도 없고 심지어 화장지 걸이도 없다면? 결과 값은 `None`입니다. C 언어에서는 화장지 걸이가 없는 상태를 나타낼 수 있는 방법이 없습니다. `NULL`은 사실상 정수 `0`이며, `0`은 `0`이라는 값 자체를 의미합니다. 흔히 에러를 표시할 때 쓰는 `-1`은 '화장지'라는 물질을 나타낼 수 없으므로 부적절한 값입니다. 이러한 상태를 가장 정확하게 표현할 수 있는 것은 `None`뿐입니다.
>

그 외에 `if let`을 사용해서 값을 확인하는 방법이 있습니다. `if let`을 사용하면 값이 존재할 때의 처리를 할 수 있고, `else`에서는 값이 없을 때의 처리를 할 수 있습니다.

```rust
    if let Some(n) = x {
        println!("x is {}", n);
    }

    if let Some(n) = y {
        println!("y is {}", n);
    } else {
        println!("y is not present");
    }
```

`x`는 값이 있을 때의 처리만을 수행하며, 값이 없다면 무시합니다. `y`는 값이 들어있다면 그 값을 출력하고, 없다면 값이 없다는 메시지를 출력합니다. `if let` 구문에서 `Some` 안에 있는 값을 `n`으로 바인딩하므로, 내부 스코프에서 `n`은 `i32` 타입이 됩니다. `n`이라는 변수는 항상 유효한 값을 가지므로, 또다시 `None`인지 확인할 필요 없이 안전하게 데이터를 사용할 수 있습니다.

### Option이 제공하는 메서드들

러스트를 처음 접할 때 `Option`을 사용하면, 값을 읽을 때마다 매번 `if let`이나 패턴 매칭으로 값이 있는지 확인하는 과정이 번거롭게 느껴질 수 있습니다. 그래서 간단한 코드를 작성할 때는 `unwrap` 메서드를 자주 사용하곤 합니다.

>
> 열거형도 구조체와 마찬가지로 메서드를 가질 수 있습니다. C 언어에서는 열거형을 자주 사용하지 않지만, 러스트에서는 자신만의 타입을 만들어 데이터를 명확하게 표현하도록 권장하며, 열거형도 매우 빈번하게 사용됩니다.
>

`unwrap` 메서드는 `Option` 안에 존재하는 값을 꺼내는 역할을 합니다. 만약 `Some` 안에 값이 있다면 그 값을 반환하지만, `None`이라면 패닉(`panic`)을 발생시키고 프로그램을 종료합니다. 따라서 반드시 값이 있는 상황에서만 사용해야 합니다. C 언어의 `assert`와 유사한 역할을 한다고 볼 수 있지만, `assert`를 남용하거나 실제 제품 코드에 사용하는 것이 바람직하지 않은 것과 같은 이치입니다.

```rust
let x: Option<i32> = Some(5);
let _y: Option<i32> = None;

println!("x is {}", x.unwrap());
//println!("y is {}", y.unwrap()); // panic!!!
```

사용법은 간단합니다. `unwrap` 메서드를 호출하기만 하면 됩니다. 물론 실제 제품 개발 시에는 `unwrap` 사용을 지양해야 하며, 사용하더라도 최대한 상위 레이어나 `main` 함수에서만 사용하는 것이 좋습니다. 만약 `Option`에서 안전하게 값을 꺼내야 한다면 `unwrap_or`나 `unwrap_or_default` 등을 사용하면 됩니다.

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

println!("x is {}", x.unwrap_or(-1));
println!("y is {}", y.unwrap_or_default());
```

`i32` 타입의 기본값은 `0`입니다. 따라서 "y is 0"이라는 메시지가 출력됩니다. 또한 `unwrap`보다 더 권장되는 방식은 `expect` 메서드를 사용하는 것입니다.

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;
    
let item = y.expect("slice should not be empty");
```

`unwrap`은 단순히 패닉만을 발생시킵니다. 패닉이 발생한 소스 코드의 위치는 알 수 있지만, 어떤 상황인지 판단하기 위한 정보가 부족한 경우가 많습니다. 반면 `expect`를 사용하면 에러 메시지를 직접 추가할 수 있어 문제 해결에 큰 도움이 됩니다.

>
> '굳이 Option에서 값을 꺼내야 한다면'이라고 표현한 이유는, 대부분의 경우 `Option`에서 값을 직접 꺼낼 필요가 없기 때문입니다. `Option`이 담긴 변수를 그대로 사용하면서, 내부 값이 필요할 때만 `if let`이나 패턴 매칭으로 접근하면 됩니다. 또한 나중에 설명할 `map`과 같은 메서드를 사용하여 내부 값에 대한 연산을 수행한 후 다시 `Option`으로 저장할 수도 있습니다. 변수가 `i32`나 `String` 같은 타입을 직접 가지기보다, `Some`이나 `Ok` 타입 내부에 값을 유지하도록 관리하는 것이 더 러스트다운 방식입니다.
>

## ? 연산자

`Result`와 `Option` 타입을 배우고 나면 보통 다음과 같이 코드를 작성하게 됩니다.

```rust
fn foo() -> Result<i32, String> {
    let r = bar();
    match r {
        Ok(n) => {
            println!("Do something with {}", n);
            return Ok(1);
        }
        Err(s) => {
            println!("Do error handling with {}", s);
            return Err(s);
        }
    }
}

fn bar() -> Result<i32, String> {
    let r = foobar();
    match r {
        Ok(n) => {
            println!("Do something with {}", n);
            return Ok(1);
        }
        Err(s) => {
            println!("Do error handling with {}", s);
            return Err(s);
        }
    }
}

fn foobar() -> Result<i32, String> {
    let r = "foobar error".to_string();
    Err(r)
}

fn main() {
    let r = foo();
    match r {
        Ok(n) => println!("Do something with {}", n),
        Err(s) => println!("Do error handling with {}", s),
    }
}
```

함수나 라이브러리를 호출할 때마다 매번 패턴 매칭을 수행하고, 하위 레벨에서 받은 에러 값을 그대로 상위 레벨로 전달하는 과정이 번거롭게 느껴지지 않나요? 러스트를 접하기 전까지 저는 C/C++, 파이썬 등을 사용하면서 하위 레이어에서 발생한 에러를 일일이 확인하여 상위로 전달하는 것을 어쩔 수 없는 필요악이라 생각했습니다. 개발자가 에러 전달을 누락하는 실수를 원천적으로 방지할 방법은 없다고 믿어왔습니다.

하지만 러스트는 물음표(`?`) 연산자(흔히 try 연산자라고도 함)를 제공하여 `Result`나 `Option`의 에러 값(`Err` 또는 `None`)을 편리하게 전달할 수 있게 해줍니다. 함수의 반환 값을 암묵적으로 무시할 수 없는 러스트에서 이 연산자는 매우 필수적입니다. 이 연산자가 없었다면 위 예제처럼 수많은 함수에서 동일한 패턴 매칭 코드를 반복해서 작성해야 했을 것입니다.

위 예제는 `?` 연산자를 사용하여 다음과 같이 간결하게 바꿀 수 있습니다.

```rust
// code/try_operator/main.rs
fn foo() -> Result<i32, String> {
    let r = bar()?;
    println!("Do something with {}", r);
    return Ok(1);
}

fn bar() -> Result<i32, String> {
    let r = foobar()?;
    println!("Do something with {}", r);
    return Ok(1);
}

fn foobar() -> Result<i32, String> {
    let r = "foobar error".to_string();
    Err(r)
}

fn main() {
    let r = foo();
    match r {
        Ok(n) => println!("Do something with {}", n),
        Err(s) => println!("Do error handling with {}", s),
    }
}
```

```bash
$ cargo run --bin try_operator
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.55s
     Running `target/debug/try_operator`
Do error handling with foobar error
```

에러를 확인하는 패턴 매칭 코드를 모두 제거할 수 있었습니다. `?` 연산자는 값이 `None`이거나 `Err` 타입이면 즉시 현재 함수의 결과로 반환하고, `Ok`나 `Some`인 경우에는 그 내부 값을 꺼내어 다음 처리를 계속 진행하게 합니다. 즉, 에러 처리와 `unwrap`의 기능을 동시에 수행하는 것입니다. 코드로 표현하자면 다음과 같은 로직을 `?` 하나로 처리하는 셈입니다.

```rust
let r = match expr {
    Ok(value) => value,
    Err(err) => return Err(err),
}
```

`?` 연산자의 전형적인 사용 예제를 하나 더 살펴보겠습니다.

```rust
use std::fs::File;
use std::io::prelude::*;

fn read_file_contents(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => println!("Error reading file: {}", error),
    }
}
```

`read_file_contents` 함수는 `std::fs::File` 라이브러리의 함수를 호출할 때마다 `?` 연산자를 사용하여 에러를 처리합니다. 에러가 발생하면 즉시 상위 함수로 반환됩니다. 만약 `?` 연산자가 없었다면 `read_file_contents` 함수 내에 최소 두 개의 `match` 표현식이 필요했을 것입니다.

물론 단점도 있습니다. 에러를 단순히 상위 레벨로 전달하기보다 중간에 직접 처리해야 하는 경우도 많기 때문입니다. 위 예제에서도 에러 발생 시 별도의 처리가 필요하다면 `?` 연산자를 그대로 사용하기 어려울 수 있습니다. 하지만 실제 경험상 `?` 연산자 덕분에 에러 처리 코드를 대폭 줄일 수 있었습니다. 이를 잘 활용하기 위해 함수를 적절히 분리하고 구조를 설계하다 보면 더 유연한 코드를 작성할 수 있으니 적극적으로 활용해 보시기 바랍니다.

## 프로젝트 관리

이번 장에서는 여러 개의 파일에 코드를 나누어서 관리하는 방법을 알아보겠습니다.

### 크레이트(Crate)와 패키지(Package)

러스트 컴파일러(rustc)가 한 번에 처리하는 코드를 크레이트라고 정의한다고 합니다. (출처: <https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html>) 사실 정의만 보면 잘 이해가 안 되는데 쉽게 말해서 지금 내가 만들고 있는 게 하나의 실행 파일이나, 하나의 라이브러리이면 각각이 바로 하나의 크레이트입니다.

우리는 지금까지 하나의 실행 파일이 생성되는 예제들을 만들었습니다. 그럼 지금까지 하나의 크레이트를 만들었다는 것입니다. 바이너리 크레이트(Binary crate)는 말 그대로 실행 파일 하나를 만드는 코드입니다. 라이브러리 크레이트(Library crate)는 라이브러리를 만들기 위한 코드입니다. 코드가 파일 하나에만 있던지 여러 개에 있던지는 상관없습니다. 여러 코드 파일들이 하나의 결과물을 만들면, 모든 파일이 하나의 크레이트를 구현하는 것입니다.

패키지는 필요에 따라 여러 크레이트를 모아놓은 것입니다. Cargo를 이용해서 빌드를 하면 Cargo.toml파일에 가장 먼저 [package]라고 패키지 정보를 셋팅합니다. 그것은 내가 Cargo를 이용해서 하나의 패키지를 만든다는 뜻입니다.

그런데 왜 패키지일까요? 예제 프로그램만 만들다 보면 다른 라이브러리를 사용할 일이 없었을 것입니다. 그럼 하나의 크레이트만 있는 패키지를 만드신 것입니다. 그리고 그 하나의 크레이트가 하나의 패키지입니다. Cargo.toml파일의 [dependendies] 섹션에 외부 라이브러리를 추가하게 되면, 하나의 바이너리 크레이트와 여러 개의 외부 라이브러리 크레이트로 이루어진 패키지를 만들게 되는 것입니다. 당연히 여러 개의 바이너리를 하나의 Cargo.toml에서 빌드할 수 있습니다. 그럼 여러 개의 라이브러리 크레이트와 여러 개의 바이너리 크레이트로 구성된 패키지를 만드는 것입니다.

이전에 Cargo를 사용해서 패키지 디렉토리를 생성하는 방법을 이야기했었습니다. cargo new &lt;package-name&gt; 명령을 사용하면 된다고 이야기했었는데요 사실은 --bin옵션을 생략한 것입니다.

```bash
$ cargo new mybin --bin
     Created binary (application) `mybin` package
$ ls -R mybin
Cargo.toml  src

mybin/src:
main.rs
```

내가 지정한 mybin이라는 이름의 디렉토리를 만들고, mybin이라는 패키지를 만드는 Cargo.toml을 생성합니다. 최종 생성할 실행 파일의 이름도 mybin이 됩니다.

라이브러리 패키지를 만들 때는 --lib 옵션을 사용합니다. code/main.rs대신에 code/lib.rs를 만들어줍니다.

```bash
$ cargo new mylib --lib
     Created library `mylib` package
$ ls -R mylib
Cargo.toml  src

mylib/src:
lib.rs
```

새로 생성되는 디렉토리 이름도 mylib이고 패키지의 이름도, 라이브러리 파일의 이름도 mylib이 됩니다.

### Modules 모듈

패키지와 크레이트는 라이브러리나 실행 파일등의 최종 결과물을 생성하는 단위입니다. 하나의 프로젝트 안에서 여러 개의 파일이 있을 때, 다른 파일의 코드를 참조하는 방법은 모듈이라는 방식을 사용합니다. 네임스페이스에 익숙한 분들은 비슷한 것이라고 생각해도 될듯합니다.

아래 예제를 보면 네임스페이스나 기타 언어들이 다른 파일의 함수나 변수 등에 접근하는 방식과 유사하다는 것을 알 수 있습니다.

```rust
fn main() {
    my_module::test_my_mod();
}

mod my_module {
    pub fn test_my_mod() {
        println!("This is my_module::test_my_mod()");
    }
}
```

예제에서 my_module이라는 모듈 안에 구현된 test_my_mod 함수는 pub이라는 키워드를 붙여야 모듈 밖에서도 참조가 가능합니다. 그리고 특정 모듈 안의 함수 등을 참조할 때는 <모듈이름>::<이름> 같은 방식으로 접근이 가능합니다.

만약 모듈 이름이 길거나 모듈 안에 다른 모듈이 있거나 해서 이름이 길어지는 경우 아래와같이 use 키워드를 사용해서 모듈 경로를 생략할 수도 있습니다.

```rust
use my_module::test_my_mod;

fn main() {
    test_my_mod();
}

mod my_module {
    pub fn test_my_mod() {
        println!("This is my_module::test_my_mod()");
    }
}
```

그럼 다른 파일에 있는 함수 등은 어떻게 접근할까요? 실험을 위해 아래와 같이 my_module.rs 파일을 새로 추가합니다.

```bash
% ls src
main.rs         my_module.rs
```

my_module.rs 파일에 아래와 같이 my_module에 정의했던 함수들을 옮겨줍니다. 주의할 것은 mod my_module 선언을 따로 해주지 않고 바로 함수 정의를 시작한다는 것입니다. 파일 하나가 하나의 모듈이 되기 때문입니다. 파일 이름이 my_module.rs이기 때문에 my_module이라는 모듈이 자동으로 선언된 것입니다.

```rust
pub fn test_my_mod() {
    println!("This is my_module::test_my_mod()");
}
```

main.rs에서 my_module을 참조하기 위해서는 아래와같이 mod <모듈 이름>을 사용합니다.

```rust
mod my_module;

fn main() {
    my_module::test_my_mod();
}
```

use 키워드를 사용할 수도 있는데 mod로 모듈 참조를 선언한 이후에 use 키워드를 사용할 수 있습니다.

```rust
mod my_module;
use my_module::test_my_mod;

fn main() {
    test_my_mod();
}
```

만약 소스 디렉토리를 분리하고 싶다면 아래와 같이 각 하위 디렉토리마다 mod.rs라는 파일을 만들어야 합니다. 그리고 mod.rs에 같은 디렉토리에 있는 파일들을 참조해야 합니다.

간단한 실험을 위해 아래와 같이 code/second_mod 라는 디렉토리를 만듭니다. 그리고 code/second_mod 디렉토리 안에 mod.rs파일과 sec_mod_file.rs 파일을 만듭니다.

```bash
% ls -R
main.rs      my_module.rs second_mod

./second_mod:
mod.rs          sec_mod_file.rs
```

mod.rs 파일을 자신과 같은 디렉토리에 있는 모듈들을 모아서 참조하는 일을 합니다. 현재는 sec_mod_file.rs파일뿐이므로 아래와 같이 sec_mod_file.rs 파일을 public으로 참조합니다.

```rust
// code/project/second_mod/mod.rs
pub mod sec_mod_file;
```

sec_mod_file.rs에는 main에서 호출된 함수를 하나 만들어줍니다.

```rust
// code/project/second_mod/sec_mod_file.rs
pub fn second_module() {
    println!("Here second-module");
}
```

이제 main.rs에서 어떻게 참조할 수 있는지 확인해 보겠습니다.

```rust
// code/project/main.rs
mod my_module;
mod second_mod;

use my_module::test_my_mod;

fn main() {
    test_my_mod();
    second_mod::sec_mod_file::second_module();
}
```

```bash
$ cargo run --bin project
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/project`
This is my_module::test_my_mod()
Here second-module
```

main.rs에서 가장 먼저 "mod second_mod" 와 같이 디렉토리 이름으로 모듈을 선언해 줍니다. 그리고 main 함수 안에서 second_module 함수를 호출하기 위해 <모듈이름>::<파일이름>::<함수이름>으로 호출해 줍니다.

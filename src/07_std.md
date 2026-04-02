# 표준 라이브러리와 트레이트

러스트의 `std` 패키지에 있는 표준 라이브러리 중에 이 책에서 사용한 벡터(`Vector`), 해시맵(`HashMap`), 파일 입출력만 좀 더 소개하겠습니다.

## 벡터 타입 `std::vec::Vec`

저는 러스트로 프로그래밍을 할 때 가장 많이 사용하는 라이브러리가 벡터(`Vector`) 타입이었습니다. 말 그대로 같은 타입의 데이터들을 배열같이 저장하는 것인데, 크기에 제한이 없고 접근이 빠른 등 사용하기 편리하고 다양한 메서드들을 지원하고 있어서 데이터를 저장할 때 가장 많이 사용하는 타입입니다.

>
> 참고로 문자열을 나타내는 `String`도 사실은 `u8` 타입 데이터를 벡터에 저장한 것입니다. ([링크](https://doc.rust-lang.org/code/alloc/string.rs.html#365)) 어떤 타입이나 트레이트가 실제로 어떻게 정의되어 있는지 알면 좀 더 사용하기 편리합니다. 온라인 매뉴얼을 통해 쉽게 확인해 볼 수 있는 방법이 있습니다. `String` 타입의 매뉴얼([링크](https://doc.rust-lang.org/std/vec/struct.Vec.html))을 열어보시면 페이지 오른쪽 위에 소스를 볼 수 있는 링크 `source`가 있습니다. 만약 매뉴얼에서 특정 메서드의 코드를 보고 싶다면 매뉴얼에서 해당 메서드에 대한 설명에서 `source` 링크를 누르면 소스 페이지로 넘어갑니다.
>

벡터는 사실 거의 모든 언어마다 다 있는 것이니 굳이 길게 설명하지 않고 러스트에서 자주 사용하는 패턴을 이용한 예제를 보겠습니다.

```rust
// code/std_library_vec/main.rs
use std::vec::Vec;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn find_rust<'a>(books: &'a Vec<Book>) -> Vec<&'a Book> {
    let mut found: Vec<&Book> = Vec::new();
    for b in books.iter() {
        if b.title.contains("Rust") {
            found.push(b);
        }
    }
    found
}

fn main() {
    let rust_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };

    let rust_in_action = Book {
        title: String::from("Rust in Action"),
        author: String::from("Tim McNamara"),
        published: 20210810,
    };

    let another = Book {
        title: String::from("The another book"),
        author: String::from("Unknown"),
        published: 20111111,
    };

    let mut library: Vec<Book> = Vec::new();
    library.push(rust_book);
    library.push(rust_in_action);
    library.push(another);

    let rust_books = find_rust(&library);
    let mut only_titles: Vec<String> = Vec::new();

    if rust_books.is_empty() {
        println!("Cannot find any Rust book");
    } else {
        for b in rust_books.into_iter() {
            let mut title = b.title.clone();
            title.push('\n');
            only_titles.push(title);
        }
    }

    let collect = only_titles.into_iter().collect::<String>();
    println!("{}", collect);
}
```

```bash
$ cargo run --bin std_library_vec
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/std_library_vec`
The Rust Programming Language
Rust in Action

```

`main` 함수부터 살펴보겠습니다. 먼저 세 권의 책에 대한 객체를 생성하고, `library`라는 벡터에 책 데이터를 추가합니다. 다음과 같이 `Vec` 타입의 `new` 메서드를 사용하려면 벡터 객체를 생성 후 `push` 메서드로 데이터를 추가하면 됩니다. 여기에 러스트 언어의 특성상 주의할 것이 있습니다. `push()` 메서드의 인자를 보면 벡터에 저장할 객체의 참조를 사용하는 게 아니라 값을 전달합니다. 바로 여기에서 러스트 언어의 소유권 이동이 발생합니다. 아래와 같이 벡터에 저장한 `rust_book` 객체를 다시 사용하려고 해보면 "value borrowed here after move" 에러가 발생하는 것을 알 수 있습니다.

```rust
let mut library: Vec<Book> = Vec::new();
library.push(rust_book);
library.push(rust_in_action);
library.push(another);
println!("{:?}", rust_book);
```

벡터를 최초로 만들 때 `Vec` 타입의 `new`라는 메서드를 사용할 수도 있지만, 다음과 같이 `vec!`이라는 매크로를 사용해서 변수 선언과 데이터 추가를 동시에 할 수 있습니다.

```rust
let mut library = vec![rust_book, rust_in_action, another];
```

그리고 `find_rust` 함수를 호출합니다. 함수 인자로는 벡터의 참조를 전달합니다. `books.iter()`는 `books` 벡터에 저장된 각 `Book` 타입 객체의 참조 포인터를 반환합니다. 따라서 `b`는 `&Book` 타입입니다. 결국 `find_rust` 함수가 하는 일은 `books`에 저장된 각 책의 제목 중에 "Rust"라는 단어가 있는 책들의 참조 포인터만 골라서 `found` 벡터에 저장하는 것입니다.

이제 `find_rust` 함수에서 받은 결과값을 `rust_books` 변수에 저장합니다. `rust_books`는 책 제목에 "Rust"가 들어간 책들의 참조 포인터가 저장되어 있는 벡터입니다. `find_rust`에서 `books` 인자에 `'a`라는 수명 이름이 있습니다. 그리고 반환값에도 `'a`라는 수명 인자가 지정되어 있습니다. 이 말은 인자로 받은 객체의 레퍼런스와 반환값으로 반환하는 벡터 속에 있는 레퍼런스들의 수명이 같다는 의미입니다. 우리는 여러 `Book` 타입 객체에 대한 레퍼런스를 `rust_books` 변수에 저장합니다. 같은 객체에 대한 레퍼런스가 함수 인자로 전달되어 다른 객체 `rust_books`에 저장되었습니다. 만약 `rust_book`이나 `rust_in_action` 객체가 해제된 후 `rust_books`에 접근한다면 이미 해제된 객체에 접근하게 되므로 메모리 에러가 발생할 것입니다. 러스트 컴파일러는 `rust_books`에 저장된 레퍼런스들의 수명이 `library`에 저장된 객체들의 수명과 같다는 것을 알기 때문에, 한쪽이 해제된 후에 다른 쪽이 접근되는 것을 방지해 줄 것입니다.

그리고 그다음 `for` 루프에서는 그렇게 찾은 책들의 제목만 `only_titles`라는 벡터로 저장합니다. 여기에 책 제목의 `String` 타입 참조 포인터를 저장할 수도 있습니다만, 나중에 책 제목을 출력할 때 한 줄에 하나씩 출력하기 위해 `\n`을 추가해야 하므로 참조가 아니라 `String` 객체를 저장했습니다.

마지막으로 `only_titles` 벡터의 이터레이터에 `collect()` 메서드를 호출합니다. 그러면 각 `String` 타입 객체들이 하나로 합쳐져서 하나의 `String` 타입 객체가 됩니다. 프로그램의 출력은 하나의 `String` 객체가 됩니다. 마지막에 불필요하게 빈 줄이 출력되었습니다.

```rust
% cargo run
The Rust Programming Language
Rust in Action

%
```

제가 이 예제에서 보여드리고자 하는 벡터의 사용 패턴은 다음 3가지입니다.

1. `iter()`/`into_iter()`를 사용하여 각 데이터에 접근하기
2. 원본 벡터 안의 객체 중 일부를 다른 벡터로 저장할 때 참조 포인터와 라이프타임을 사용하기
3. 벡터 안의 객체들을 합치기 위해 이터레이터와 `collect()` 메서드 사용하기

그 외에 자주 사용하는 메서드들을 보자면:

1. `len`: 현재 벡터에 몇 개의 데이터가 있는지 알려줍니다.
2. `&[index]`: `[]`를 사용해서 `index` 위치의 객체에 접근할 수 있습니다. `index`는 `usize` 타입만 허용됩니다. 그냥 `library[index]` 같은 형태로 객체에 접근하려고 하면 소유권 이동이 발생하므로 허용되지 않습니다. 그래서 보통 `&library[index]` 같이 특정 위치의 객체에 대한 참조 포인터를 얻을 때 사용합니다.
3. `push`/`pop`: 벡터의 마지막에 데이터를 추가하거나 빼는 메서드입니다. 스택(`stack`)과 같다고 생각하면 됩니다.
4. `insert`/`remove`: 특정 위치에 데이터를 넣거나 빼는 메서드입니다. 벡터 내부의 데이터를 이동해야 하므로 상황에 따라 실행 속도가 느려질 수 있습니다.
5. `as_ptr`/`as_mut_ptr`: 데이터 배열의 로우(`raw`) 포인터를 얻습니다. 일반적인 상황에서는 사용할 필요가 없지만, 만약 C/C++ 코드와 같이 사용하게 된다면 이 메서드를 자주 사용하게 될 것입니다.

마지막으로 한 가지 더 팁을 드리자면, 벡터의 참조 포인터를 함수로 전달하면 슬라이스(`slice`)가 됩니다. `find_rust` 함수를 다음과 같이 슬라이스를 전달받도록 만들 수도 있습니다. 이렇게 만들면 벡터뿐 아니라 배열을 같이 처리할 수 있는 함수로 만들 수 있기 때문에 좀 더 유연한 코드를 만들 수 있습니다.

```rust
fn find_rust<'a>(books: &'a [Book]) -> Vec<&'a Book> {
    let mut found: Vec<&Book> = Vec::new();
    for b in books.iter() {
        if b.title.contains("Rust") {
            found.push(b);
        }
    }
    found
}
```

### 연습문제

- `Vec` 타입의 소스 코드를 확인해 보세요. 매뉴얼에 소스 코드를 볼 수 있는 링크가 있습니다. 생각보다 단순하게 구현된 타입인데 어떻게 그렇게나 많은 메서드를 지원할 수 있을까요? 데이터 크기를 늘릴 수 있는 배열이 데이터 크기가 고정된 배열에 비해 어떤 장단점이 있는지 조사해 보세요. 그리고 트리나 스택 등 다른 데이터 구조와도 비교해 보세요. 데이터 구조를 공부하는 데 중요한 배경지식이 될 것입니다.
- 예제 코드에서 책 제목이 아니라 저자 이름을 찾는 함수를 만들어 보세요.
- 예제 코드에서 책 출판 날짜를 나타내는 `published` 필드의 타입을 문자열로 바꿔 보세요.
- 예제 코드를 실행하면 마지막에 불필요한 빈 줄이 출력됩니다. 이 빈 줄을 출력되지 않도록 바꿔 보세요.
- [벡터의 내부 구조](https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees)에 대해서 더 자세히 이해하고 싶다면 이 문서를 참고하세요.

## 해시맵 `std::collections::HashMap`

아마도 벡터(`Vector`)만큼이나 많이 사용되는 자료구조를 들자면 해시맵(`HashMap`)을 꼽을 수 있을 것입니다. 이전에 벡터를 사용했던 예제를 해시맵으로 바꿔보겠습니다.

```rust
// code/std_library_hashmap_simple/main.rs
use std::collections::HashMap;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let mut library: HashMap<String, Book> = HashMap::new();
    let the_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };
    library.insert("718503105".to_owned(), the_book);
    library.insert(
        "1617294551".to_owned(),
        Book {
            title: String::from("Rust in Action"),
            author: String::from("Tim McNamara"),
            published: 20210810,
        },
    );
    library.insert(
        "0000000000".to_owned(),
        Book {
            title: String::from("The another book"),
            author: String::from("Unknown"),
            published: 20111111,
        },
    );

    let found = library.get("0000000000");
    println!("{:?}", found);
    let not_found = library.get("xxxxxxxxxx");
    println!("{:?}", not_found);
}
```

```bash
$ cargo run --bin std_library_hashmap_simple
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/std_library_hashmap_simple`
Some(Book { title: "The another book", author: "Unknown", published: 20111111 })
None
```

`library` 변수를 해시맵 타입으로 만들어서 책을 저장하고 있습니다. 키는 문자열로 된 ISBN 값이고 값은 `Book` 타입의 객체입니다. 해시맵에 데이터를 저장하기 위해 `insert` 메서드를 사용하고, 키에 해당하는 데이터를 얻기 위해 `get` 메서드를 사용합니다.
해시맵 자체적으로 데이터 검색 기능이 있으니 이전 예제에서 만들었던 `find_rust` 함수가 필요 없습니다.

주의해야 할 것은 `insert` 메서드에 값을 저장할 때 객체의 값을 전달한다는 것입니다. 만약 `the_book` 객체를 `library`에 저장한 후에는 `the_book` 객체에 대한 소유권이 `library`로 이동합니다. 따라서 `the_book`이라는 변수를 다시는 사용할 수 없습니다. 그리고 하나 더 주의할 것이 있는데, `insert` 메서드는 이미 키가 존재할 경우 데이터를 덮어쓴다는 것입니다. 그러니 실제 제품 개발에서는 항상 데이터가 있는지를 먼저 확인하는 게 필요합니다. 데이터가 있는지 확인하는 방법은 연습문제로 남겨놓았습니다.

그리고 한 가지 예제를 더 보겠습니다. 이전 예제는 키 값이 간단한 스트링 타입이었습니다. 이번 예제는 구조체 타입 `Book`을 키로 사용하는 예제입니다.

```rust
use std::collections::HashMap;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let mut library: HashMap<Book, String> = HashMap::new();
    library.insert(
        Book {
            title: String::from("The Rust Programming Language"),
            author: String::from("Steve Klabnik and Carol Nichols"),
            published: 20230228,
        },
        "1718503105".to_owned(),
    );
    library.insert(
        Book {
            title: String::from("Rust in Action"),
            author: String::from("Tim McNamara"),
            published: 20210810,
        },
        "1617294551".to_owned(),
    );
    library.insert(
        Book {
            title: String::from("The another book"),
            author: String::from("Unknown"),
            published: 20111111,
        },
        "0000000000".to_owned(),
    );

    let found = library.get(&Book {
        title: String::from("The another book"),
        author: String::from("Unknown"),
        published: 20111111,
    });
    println!("{:?}", found);
}
```

이전 예제와 반대로 책의 정보를 가지고 ISBN 값을 찾는 예제입니다. 이 예제를 빌드하면 아래와 같은 에러 메시지를 볼 수 있습니다.

```rust
error[E0599]: the method `insert` exists for struct `HashMap<Book, String>`, but its trait bounds were not satisfied
  --> code/main.rs:12:13
   |
4  | struct Book {
   | -----------
   | |
   | doesn't satisfy `Book: Eq`
   | doesn't satisfy `Book: Hash`
   | doesn't satisfy `Book: PartialEq`
...
12 |     library.insert(
   |     --------^^^^^^
   |
   = note: the following trait bounds were not satisfied:
           `Book: Eq`
           `Book: PartialEq`
           which is required by `Book: Eq`
           `Book: Hash`
help: consider annotating `Book` with `#[derive(Eq, Hash, PartialEq)]`
   |
4  + #[derive(Eq, Hash, PartialEq)]
5  | struct Book {
   |

error[E0599]: the method `insert` exists for struct `HashMap<Book, String>`, but its trait bounds were not satisfied
  --> code/main.rs:20:13
   |
4  | struct Book {
   | -----------
   | |
   | doesn't satisfy `Book: Eq`
   | doesn't satisfy `Book: Hash`
   | doesn't satisfy `Book: PartialEq`
...
20 |     library.insert(
   |     --------^^^^^^
   |
   = note: the following trait bounds were not satisfied:
           `Book: Eq`
           `Book: PartialEq`
           which is required by `Book: Eq`
           `Book: Hash`
help: consider annotating `Book` with `#[derive(Eq, Hash, PartialEq)]`
   |
4  + #[derive(Eq, Hash, PartialEq)]
5  | struct Book {
   |

error[E0599]: the method `insert` exists for struct `HashMap<Book, String>`, but its trait bounds were not satisfied
  --> code/main.rs:28:13
   |
4  | struct Book {
   | -----------
   | |
   | doesn't satisfy `Book: Eq`
   | doesn't satisfy `Book: Hash`
   | doesn't satisfy `Book: PartialEq`
...
28 |     library.insert(
   |     --------^^^^^^
   |
   = note: the following trait bounds were not satisfied:
           `Book: Eq`
           `Book: PartialEq`
           which is required by `Book: Eq`
           `Book: Hash`
help: consider annotating `Book` with `#[derive(Eq, Hash, PartialEq)]`
   |
4  + #[derive(Eq, Hash, PartialEq)]
5  | struct Book {
   |

error[E0599]: the method `get` exists for struct `HashMap<Book, String>`, but its trait bounds were not satisfied
  --> code/main.rs:37:25
   |
4  | struct Book {
   | -----------
   | |
   | doesn't satisfy `Book: Eq`
   | doesn't satisfy `Book: Hash`
   | doesn't satisfy `Book: PartialEq`
...
37 |     let found = library.get(&Book {
   |                 --------^^^ method cannot be called on `HashMap<Book, String>` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `Book: Eq`
           `Book: PartialEq`
           which is required by `Book: Eq`
           `Book: Hash`
help: consider annotating `Book` with `#[derive(Eq, Hash, PartialEq)]`
   |
4  + #[derive(Eq, Hash, PartialEq)]
5  | struct Book {
   |
```

`insert`와 `get` 메서드를 사용하기 위해서 `Book` 구조체에 `Eq`, `Hash`, `PartialEq` 트레이트의 구현이 있어야 한다는 에러 메시지입니다. 에러를 해결하는 것은 컴파일러가 안내하는 대로 아래와 같이 `Book` 구조체에 `derive`를 이용해서 `Eq`, `Hash`, `PartialEq` 트레이트의 구현을 추가해 주면 됩니다.

```rust
// code/std_library_hashmap/main.rs
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let mut library: HashMap<Book, String> = HashMap::new();
    library.insert(
        Book {
            title: String::from("The Rust Programming Language"),
            author: String::from("Steve Klabnik and Carol Nichols"),
            published: 20230228,
        },
        "1718503105".to_owned(),
    );
    library.insert(
        Book {
            title: String::from("Rust in Action"),
            author: String::from("Tim McNamara"),
            published: 20210810,
        },
        "1617294551".to_owned(),
    );
    library.insert(
        Book {
            title: String::from("The another book"),
            author: String::from("Unknown"),
            published: 20111111,
        },
        "0000000000".to_owned(),
    );

    let found = library.get(&Book {
        title: String::from("The another book"),
        author: String::from("Unknown"),
        published: 20111111,
    });
    println!("{:?}", found);
}
```

그럼 왜 이런 에러가 발생하는 것일까요? 키 값으로 우리가 직접 만든 타입을 사용할 때 키 값을 비교할 수가 없기 때문입니다. `insert`나 `get`에서 키 값을 비교할 때 두 개의 객체를 비교해야 하는데, 구조체 타입의 경우 어떻게 두 객체의 값을 비교해야 하는지 알지 못합니다. 그래서 `PartialEq`와 `Eq`가 모두 구현되어야 합니다.

그리고 `Hash`라는 트레이트의 구현도 필요합니다. `Hash` 트레이트는 해당 타입의 해시(`Hash`) 값을 계산하기 위한 트레이트입니다. `Book` 구조체에는 `String` 타입 2개와 `u32` 타입 1개의 데이터가 들어 있습니다. `Hash` 트레이트의 구현을 추가해 주면 각 데이터들의 해시 값을 조합해서 최종 `Book` 타입 객체의 해시 값을 계산합니다. 러스트의 해시맵 라이브러리가 내부적으로 객체의 해시 값을 이용해서 데이터를 검색하고 저장하기 때문에, 우리가 직접 만든 타입을 사용해서 키를 지정하고 싶다면 해시 값을 생성하는 방법도 해시맵 라이브러리에 알려주어야 합니다.

물론 각 트레이트의 구현을 직접 구현할 수도 있습니다. 책을 비교할 때 제목, 저자, 출판일을 모두 비교하는 게 아니라 제목만 비교할 수도 있으니까요. 저는 해시맵의 소개를 위해서 최대한 간단하게 소스를 구현해 봤지만, 직접 각 트레이트를 구현해 보는 것도 좋은 연습이 될 듯합니다.

### 연습문제

- 해시맵의 `insert` 메서드는 이미 같은 키가 존재하면 새로운 값으로 기존 값을 덮어씁니다. 실제 제품에서는 이렇게 기존 데이터가 지워지면 안 되는 경우가 많을 것입니다. 해시맵의 매뉴얼을 읽어보고 이미 같은 키가 존재하는지를 확인하는 메서드를 찾아보세요. 그리고 이미 같은 키가 존재하면 에러를 반환하도록 예제를 고쳐보세요.
- ISBN이 "0000000000"인 책의 제목은 "The another book"입니다. `library`에서 `entry` 메서드를 이용해서 이 책의 제목을 "Not released book"으로 바꾸는 코드를 만들어 보세요. `entry` 메서드는 `library`에 있는 데이터 중에 특정한 키의 값이 있는지를 확인하는 메서드이고, `Entry`라는 열거형(`enum`) 타입의 값을 반환합니다. `Entry`가 무엇인지 확인해 보고, 값이 이미 있을 때의 반환 값과 값이 없을 때의 반환 값이 어떻게 다른지 확인해 보세요. 그리고 `Entry`의 메서드 중에 어떤 메서드를 사용하면 찾는 값이 없을 때만 데이터를 넣을 수 있는지 매뉴얼을 검색해 보세요.

## 파일 읽고 쓰기 위한 `std::{fs::File, io::Read, path::PathBuf}`

러스트에서는 파일을 읽고 쓰기 위해 3가지 모듈(`module`)을 알아야 합니다. 일단 각각이 무엇인지 간단하게 설명하고 어떻게 사용하는지를 알아보겠습니다.

1. `std::fs` 모듈: 로컬 파일 시스템에 있는 파일을 처리하기 위한 모듈입니다. 일반적으로 운영체제에 상관없이 사용할 수 있는 기능들을 모아놓은 것입니다. 그 중에서 `File` 구조체가 일반 파일에 접근할 때 사용됩니다.
2. `std::io` 모듈: 입출력을 위한 타입, 라이브러리, 에러 타입 등을 모아놓은 모듈입니다. `Read`, `Write`라는 트레이트(`trait`)가 있습니다. 주의할 것은 `std::io::Read`라는 트레이트가 있다는 것의 의미를 알아야 합니다. 트레이트가 있다는 것은 다른 어딘가에 구현체(`implementation`)가 있어야 한다는 것입니다. 코드 파일에 `use std::io::Read`라고 선언을 해서 사용은 하지만 사실 구현체는 다른 곳에 있습니다. 예를 들면 `std::fs::File` 구조체가 `std::io::Read` 트레이트를 구현하고 있습니다. 아래 예제에서 사용 방법을 보겠습니다.
3. `std::path` 모듈: 파일을 처리하기 위해서는 파일의 경로를 알아야 합니다. 보통 문자열로 사용할 수도 있겠지만, 문자열로 경로를 표현하면 운영체제에 종속적으로 동작할 수밖에 없습니다. 윈도우에서는 디렉터리 간 구분을 `\`로 하지만 유닉스에서는 `/`로 하니까요. 어느 운영체제나 플랫폼에서도 동작하기 위해서는 파일의 경로를 추상화해야 합니다. `std::path`에 있는 `PathBuf`와 `Path` 타입이 파일의 경로를 추상화하는 것들입니다.

이제 이 세 가지를 가지고 파일을 읽는 예제를 한번 만들어 보겠습니다. 설명은 길었지만 코드는 간단합니다.

```rust
// code/std_library_file/main.rs
use std::{
    env::current_dir,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

/// &Path is immutable form of PathBuf (same to &str and String)
fn grep(filename: &Path, word: &str) -> std::io::Result<()> {
    let mut f: File = File::open(filename)?;
    let mut text_buffer = String::new();

    f.read_to_string(&mut text_buffer)?;
    for line in text_buffer.split('\n') {
        if line.contains(word) {
            println!("{line}");
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut filename: PathBuf = current_dir()?;

    filename.push("code/std_library_file/main.rs");
    grep(&filename, "main")?; // show lines that include specific word
    Ok(())
}
```

```bash
$ cargo run --bin std_library_file
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/std_library_file`
fn main() -> std::io::Result<()> {
    filename.push("code/std_library_file/main.rs");
    grep(&filename, "main")?; // show lines that include specific word
```

`main` 함수에서 첫 번째로 하는 일은 `current_dir()` 함수를 호출해서 프로그램이 실행되는 위치를 알아내는 것입니다. 우리는 `cargo run` 명령으로 프로그램을 실행할 것이므로 현재 위치에 "code/std_library_file/main.rs" 경로를 추가하면 `main.rs` 파일의 위치가 될 것입니다.

한 가지 주의해야 할 것은 `filename`의 `push()` 메서드에 "/code/std_library_file/main.rs"와 같이 절대 경로를 전달하면 안 된다는 것입니다. `push()` 메서드는 `filename`에 저장된 경로에 이어서 하위 경로를 추가하는 일을 하지만, 만약에 전달된 경로가 "/"로 시작하는 절대 경로라면 `filename`에 저장된 경로를 지우고 전달된 경로로 바꾸게 됩니다. 결국 "/code/std_library_file/main.rs" 파일을 읽으려고 하고 에러가 발생할 것입니다.

그리고 `grep`이라는 함수를 호출합니다. `grep` 함수의 인자에는 `filename`의 참조를 전달합니다. 그런데 특이한 게 있습니다. `PathBuf` 타입의 변수의 참조를 전달하는데 `grep` 함수의 인자는 `&Path`가 되는 것입니다. 주석에도 써놓았듯이 `PathBuf`는 일반적으로 경로를 추가하거나 바꿀 수 있는 타입입니다. 그리고 참조를 해서 레퍼런스만 전달하게 되면 `&Path` 타입이 됩니다. `String`과 `&str`의 관계와 동일합니다. 이렇게 하는 이유는 컴파일의 효율성을 높여서 성능을 높이기 위한 것입니다. `PathBuf`를 처음 생성해서 `PathBuf` 타입으로 가지고 있을 때 최대한 모든 처리를 실행해서 경로를 추가하거나 지우거나 해서 최종 경로를 만들어내고, 다른 함수에 전달할 때는 `&Path` 타입으로 전달하도록 하면 성능을 높일 수 있습니다. 프로그램이 보통 설정 파일이나 특정 파일을 찾을 때는 경로를 저장하는 변수의 값을 바꾸지만, 파일을 찾고 나면 경로를 바꿀 일이 드뭅니다. 이미 있는 파일의 경로를 바꿀 일이 많지는 않으니까요. 파일을 지우더라도 경로는 바뀌지 않습니다. 그래서 읽기 전용으로 경로를 관리할 일이 많으므로 이렇게 별도의 타입을 만들었습니다.

`grep` 함수에 전달된 파일 경로를 이용하여 `File::open` 함수를 호출합니다. `File::open` 함수는 파일을 열고 파일을 관리하기 위한 `std::fs::File` 타입의 객체를 반환합니다. `File` 객체는 `std::io::Read` 트레이트를 구현하고 있어서 `read()`나 `read_to_string()` 메서드를 가지고 있습니다. `read_to_string()` 메서드는 설명에서 보는 것과 같이 파일 전체의 내용을 `String` 타입 변수에 추가하는 것입니다. 그렇게 파일 전체를 한꺼번에 읽어서 각 라인별로 찾고자 하는 문자열을 가지고 있는 라인만 출력하게 됩니다.

아래는 위의 예제에서 파일의 경로를 일부러 존재하지 않는 파일 경로를 전달하는 예제입니다. 일부러 에러를 발생시켜서 어떻게 동작하는지 확인해 보기 위한 예제입니다.

```rust
...
fn main() -> std::io::Result<()> {
    let mut filename: PathBuf = current_dir()?;

    filename.push("code/std_library_file/wrongfile.rs");
    grep(&filename, "main")?; // show lines that include specific word
    Ok(())
}
```

```bash
% cargo run
   Compiling ex v0.1.0 (/Users/user/ex)
    Finished dev [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/ex`
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

`grep` 함수에서 `File::open` 함수에서 파일이 없다는 것을 확인하고 에러를 반환합니다. 그러면 `?` 연산자가 에러를 `main` 함수로 전달합니다. 최종적으로 `grep` 함수 호출에도 `?` 연산자를 사용하므로, `File::open` 함수에서 반환한 에러가 이 예제를 실행한 터미널에까지 전달됩니다. `main` 함수의 결과값이 `grep` 함수의 결과값과 동일한 `std::io::Result<()>`인 것을 눈여겨 보시기 바랍니다.

프로그램의 실행 결과를 보면 어떤 에러가 발생했는지 에러의 번호와 에러 타입, 에러를 좀 더 설명하는 메시지를 출력합니다. 에러의 번호는 운영체제가 해당 에러를 어떤 번호로 처리하는지에 따라 달라지는데, 리눅스의 경우 `errno -l` 명령으로도 확인할 수 있고, `man errno` 명령으로도 확인할 수 있습니다. `kind`는 에러의 타입인데 숫자로 된 번호를 일종의 타입으로 바꾼 것이므로 에러 번호나 마찬가지입니다.

### 연습문제

1. 에러를 처리할 때 기본 `std::io::Result`와 같이 러스트의 기본 모듈이 제공하는 에러 타입을 사용할 수도 있지만 그렇게되면 여러 가지 에러 타입들을 처리하기 위해 에러 처리 코드가 복잡해질 수밖에 없습니다. 다음과 같이 `std::io::Result`를 반환하는 함수와 `std::core::Result`를 반환하는 함수를 호출하는 함수는 어떤 에러를 반환해야 할까요?

```rust
fn multiple_errors(filename: &str, number: &str) -> std::io::Result<()> {
    let mut f = File::open(filename)?; // std::io::Result
    let num: u32 = number.parse()?; // https://doc.rust-lang.org/std/str/trait.FromStr.html#associatedtype.Err
    Ok(())
}

fn multiple_errors(
    filename: &str,
    number: &str,
) -> core::result::Result<(), core::num::ParseIntError> {
    let _ = File::open(filename)?; // std::io::Result
    let _ = number.parse::<u32>()?; // https://doc.rust-lang.org/std/str/trait.FromStr.html#associatedtype.Err
    Ok(())
}
```

`std::io::Result<()>`를 반환하도록 하면 `parse()`의 에러를 처리하지 못하고, `parse()`의 에러를 반환하도록 하면 `open()`의 에러를 처리하지 못합니다. 두 가지 타입의 에러를 모두 가질 수 있는 상위 에러를 구현해 보세요. 정수 값이 저장된 텍스트 파일을 읽어서 정수 값을 파싱하는 프로그램을 만들어 보세요. 같은 프로그램에 문자열만 있는 텍스트 파일을 읽도록 해서 파싱에서 에러가 발생하도록 만들어 보세요.

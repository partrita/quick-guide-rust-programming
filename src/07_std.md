# 표준 라이브러리와 트레이트

`Rust`의 `std` 패키지에 포함된 표준 라이브러리 중 가장 빈번하게 사용되는 벡터(`Vector`), 해시맵(`HashMap`), 그리고 파일 입출력(`I/O`)에 대해 더 자세히 알아보겠습니다.

## 벡터 타입 `std::vec::Vec`

`Rust` 프로그래밍에서 데이터를 저장할 때 가장 많이 사용되는 타입입니다. 동일한 타입의 데이터를 배열처럼 연속적으로 저장하며, 크기가 유연하게 늘어난다는 장점이 있습니다.

> 참고: 문자열을 나타내는 `String` 또한 내부적으로는 `u8` 타입 데이터를 벡터(`Vec<u8>`)로 관리합니다. 특정 타입이나 트레이트의 상세 구현이 궁금하다면 [공식 매뉴얼](https://doc.rust-lang.org/std/vec/struct.Vec.html)의 `source` 링크를 통해 실제 소스 코드를 직접 확인해 볼 수 있습니다.

벡터의 주요 사용 패턴을 다음 예제로 살펴보겠습니다.

```rust
// code/std_library_vec/main.rs
use std::vec::Vec;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn find_rust<'a>(books: &'a [Book]) -> Vec<&'a Book> {
    let mut found = Vec::new();
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

    let mut library = Vec::new();
    library.push(rust_book); // 소유권 이동 발생!

    // vec! 매크로를 사용하면 선언과 동시에 초기화할 수 있습니다.
    let rust_books = find_rust(&library);

    // 이터레이터와 collect를 활용해 데이터를 가공합니다.
    let titles: String = rust_books.iter()
        .map(|b| format!("{}\n", b.title))
        .collect();
    println!("{}", titles);
}
```

### 벡터 사용 시 주의사항

1.  소유권 이동: `push()` 메서드로 객체를 벡터에 담으면 해당 객체의 소유권이 벡터로 넘어갑니다. 이후 원래 변수를 사용하려 하면 컴파일 에러가 발생합니다.
2.  인덱스 접근: `&library[index]`와 같이 참조를 얻어와야 합니다. 직접 접근(`library[index]`)은 소유권 이동을 시도하므로 기본적으로 허용되지 않습니다.
3.  슬라이스 활용: 함수 인자로 벡터 전체의 참조(`&Vec<T>`) 대신 슬라이스(`&[T]`)를 사용하면, 벡터뿐만 아니라 일반 배열도 처리할 수 있어 코드가 훨씬 유연해집니다.

## 해시맵 `std::collections::HashMap`

키-값(`Key-Value`) 쌍으로 데이터를 관리할 때 사용합니다. 검색 속도가 매우 빠르며, `Rust`에서는 키로 사용될 타입에 대해 몇 가지 트레이트 구현을 요구합니다.

```rust
// code/std_library_hashmap/main.rs
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Book {
    title: String,
    author: String,
}

fn main() {
    let mut library = HashMap::new();
    let the_book = Book {
        title: "Rust in Action".to_string(),
        author: "Tim McNamara".to_string(),
    };

    // 키와 값 모두 소유권이 이동합니다.
    library.insert("1617294551".to_string(), the_book);

    // get()은 Option<&V>를 반환합니다.
    if let Some(book) = library.get("1617294551") {
        println!("Found: {:?}", book);
    }
}
```

구조체나 사용자 정의 타입을 해시맵의 키로 쓰려면 `Eq`, `Hash`, `PartialEq` 트레이트가 반드시 구현되어 있어야 합니다. 이는 해시맵이 내부적으로 객체의 해시값을 계산하고 동등성을 비교하여 데이터를 관리하기 때문입니다. `#[derive(...)]`를 통해 간단히 해결할 수 있습니다.

## 파일 시스템과 입출력 `std::{fs, io, path}`

파일을 다룰 때는 다음 세 가지 모듈의 조화가 중요합니다.

1.  `std::fs`: 실제 파일 시스템에 접근하여 파일을 열거나 생성합니다.
2.  `std::io`: `Read`, `Write` 트레이트를 통해 실제 데이터를 읽고 씁니다. `File` 구조체는 이 트레이트들을 구현하고 있습니다.
3.  `std::path`: `PathBuf`(가변 경로)와 `Path`(불변 슬라이스)를 통해 운영체제에 독립적인 경로 처리를 지원합니다.

```rust
// code/std_library_file/main.rs
use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

fn read_file_content(path: &Path) -> Result<String> {
    let mut file = File::open(path)?; // ? 연산자로 에러 전파
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
```

`PathBuf`와 `&Path`의 관계는 `String`과 `&str`의 관계와 같습니다. 경로를 조작할 때는 `PathBuf`를 쓰고, 함수의 인자로 넘길 때는 읽기 전용인 `&Path`를 사용하는 것이 관례입니다.

표준 라이브러리를 잘 다루는 것이 `Rust` 정복의 첫걸음입니다. 각 메서드가 반환하는 `Result`나 `Option`을 `?` 연산자와 패턴 매칭으로 우아하게 처리하는 연습을 해보세요.

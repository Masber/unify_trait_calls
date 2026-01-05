trait MyTrait {
    fn my_method(&self) -> String {
        "Default implementation".to_string()
    }
}

trait MyTrait2 {
    fn another_method(&self) -> String {
        "Another default implementation".to_string()
    }
}

trait MySuperTrait: MyTrait + MyTrait2 {}

struct MyStruct1;

impl MyTrait for MyStruct1 {
    fn my_method(&self) -> String {
        "MyStruct1 implementation".to_string()
    }
}

impl MyTrait2 for MyStruct1 {
    fn another_method(&self) -> String {
        "MyStruct1 another implementation".to_string()
    }
}

struct MyStruct2;

impl MyTrait for MyStruct2 {
    fn my_method(&self) -> String {
        "MyStruct2 implementation".to_string()
    }
}

impl MyTrait2 for MyStruct2 {
    /* fn another_method(&self) -> String {
        "MyStruct2 another implementation".to_string()
    } */
}

// Implement MySuperTrait for all types that implement MyTrait and MyTrait2
// Option 1: Blanket implementation.
// ref 1: https://users.rust-lang.org/t/what-are-blanket-implementations/49904
// ref 2: https://users.rust-lang.org/t/extention-traits-on-traits/55764/10
// ref 3: https://doc.rust-lang.org/std/string/trait.ToString.html#implementors
impl<T: MyTrait + MyTrait2> MySuperTrait for T {}

// Option 2: Manual implementation.
/* impl MySuperTrait for MyStruct1 {}
impl MySuperTrait for MyStruct2 {} */

fn create_trait_object(trait_type: &str) -> Box<dyn MySuperTrait> {
    match trait_type {
        "MyStruct1" => Box::new(MyStruct1),
        "MyStruct2" => Box::new(MyStruct2),
        _ => panic!("Unknown trait type"),
    }
}

/* enum MyEnum {
    Variant1(MyStruct1),
    Variant2(MyStruct2),
}

impl MyTrait for MyEnum {
    fn my_method(&self) -> String {
        match self {
            MyEnum::Variant1(s) => s.my_method(),
            MyEnum::Variant2(s) => s.my_method(),
        }
    }
} */

fn main() {
    // Option 1: Using Box<dyn MyTrait>
    let trait_object = create_trait_object("MyStruct2");
    println!("{}", trait_object.my_method());
    println!("{}", trait_object.another_method());

    /* // Option 2: Using enum
    let my_enum = MyEnum::Variant1(MyStruct1);

    println!("{}", my_enum.my_method()); */
}

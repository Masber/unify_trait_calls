trait MyTrait {
    fn my_method(&self) -> String;
}

struct MyStruct1;

impl MyTrait for MyStruct1 {
    fn my_method(&self) -> String {
        "MyStruct1 implementation".to_string()
    }
}

struct MyStruct2;

impl MyTrait for MyStruct2 {
    fn my_method(&self) -> String {
        "MyStruct2 implementation".to_string()
    }
}

fn create_trait_object(trait_type: &str) -> Box<dyn MyTrait> {
    match trait_type {
        "MyStruct1" => Box::new(MyStruct1),
        "MyStruct2" => Box::new(MyStruct2),
        _ => panic!("Unknown trait type"),
    }
}

enum MyEnum {
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
}

fn main() {
    // Option 1: Using Box<dyn MyTrait>
    let trait_object = create_trait_object("MyStruct2");
    println!("{}", trait_object.my_method());

    // Option 2: Using enum
    let my_enum = MyEnum::Variant1(MyStruct1);

    println!("{}", my_enum.my_method());
}

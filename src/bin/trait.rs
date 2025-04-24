trait Mytrait{
    type AssociatedType;
}

struct MyStruct;

impl Mytrait for MyStruct{
    type AssociatedType = i32;
}
use dynomite::{Attributes};

#[derive(Attributes)]
struct Test1 {
    #[dynomite(skip_serializing_if = "true")]
    field: u32,
}

#[derive(Attributes)]
struct Test2 {
    #[dynomite(skip_serializing_if = "2 + 2")]
    field: u32,
}

#[derive(Attributes)]
struct Test3 {
    #[dynomite(skip_serializing_if = "|| true")]
    field: u32,
}

fn main() {}

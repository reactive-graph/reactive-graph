use std::alloc::System;

#[global_allocator]
static ALLOCATOR: System = System;

#[tokio::main]
async fn main() {
    inexor_rgf_rt::main().await;
}

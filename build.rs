use ethers::contract::Abigen;

fn main() {
    Abigen::new("AnchorContract", "contracts/Anchor.json")
        .unwrap()
        .rustfmt(true)
        .generate()
        .unwrap()
        .write_to_file("src/contracts/anchor.rs")
        .unwrap();
}

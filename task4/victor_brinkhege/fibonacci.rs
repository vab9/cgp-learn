struct MyIterator {

}


fn main() {
    for i in MyIterator::new().take(20) {
        println!("{}", i);
    }
}

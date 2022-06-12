struct MyCalendarThree {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        MyCalendarThree {}
    }

    fn book(&self, start: i32, end: i32) -> i32 {}
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(start, end);
 */
fn main() {}

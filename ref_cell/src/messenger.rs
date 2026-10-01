use std::cell::RefCell;
use std::rc::Rc;

pub trait Logger {
fn warning(&self, msg: &str);
fn info(&self, msg: &str);
fn error(&self, msg: &str);
}

pub struct Tracker {
pub messages: RefCell<Vec<String>>,
pub value: RefCell<usize>,
pub max: usize,
}

impl Logger for Tracker {
fn warning(&self, msg: &str) {
    self.messages.borrow_mut().push(msg.to_string());
}

fn info(&self, msg: &str) {
    self.messages.borrow_mut().push(msg.to_string());
}

fn error(&self, msg: &str) {
    self.messages.borrow_mut().push(msg.to_string());
}
}

impl Tracker {
pub fn new(max: usize) -> Self {
    Tracker {
        messages: RefCell::new(Vec::new()),
        value: RefCell::new(0),
        max,
    }
}

pub fn set_value<T>(&self, val: &Rc<T>) {
    let count = Rc::strong_count(val);
    let percentage = (count * 100) / self.max;

    if count > self.max {
        self.error("Error: You can't go over your quota!");
    } else {
        *self.value.borrow_mut() = count;
        if percentage >= 70 {
            self.warning(&format!(
                "Warning: You have used up over {}% of your quota!",
                percentage
            ));
        }
    }
}

pub fn peek<T>(&self, val: &Rc<T>) {
    let count = Rc::strong_count(val);
    let percentage = (count * 100) / self.max;
    self.info(&format!("Info: This value would use {}% of your quota", percentage));
}
}

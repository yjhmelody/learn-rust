#!
#![allow(unused_variables, dead_code)]

use std::collections::linked_list::LinkedList;

/// 带GC标记的对象
#[derive(Debug, Clone)]
pub struct GCObject<T> {
    marked: bool,
    next: *mut GCObject<T>,
    obj: Object<T>,
}

impl<T> GCObject<T> {
    #[inline]
    pub fn new(vm: &mut VM<T>, obj: Object<T>) -> *mut Self {
        if vm.obj_num == vm.max_num {
            vm.gc();
        }

        // 指向root
        let gc_obj = Self {
            marked: false,
            next: vm.root,
            obj,
        };

        // 新对象作为root
        let mut gc_obj = Box::new(gc_obj);
        vm.root = (&mut *gc_obj) as *mut GCObject<T>;
        vm.obj_num += 1;
        &mut *gc_obj as *mut Self
    }

    #[inline]
    fn mark(&mut self, b: bool) {
        self.marked = b;
    }

    #[inline]
    fn is_marked(&self) -> bool {
        self.marked
    }
}

/// 不同类型的对象
#[derive(Debug, Clone)]
pub enum Object<T> {
    Obj(T),
    ObjPair { head: *mut GCObject<T>, tail: *mut GCObject<T> },
}

impl<T> Object<T> {
    #[inline]
    pub fn new(t: T) -> Self {
        Object::Obj(t)
    }
}

/// 运行垃圾回收的环境
#[derive(Debug, Clone)]
pub struct VM<T> {
    root: *mut GCObject<T>,
    stack: LinkedList<*mut GCObject<T>>,
    obj_num: i32,
    max_num: i32,
}

impl<T> VM<T> {
    #[inline]
    pub fn new(max_num: i32) -> Self {
        VM { root: std::ptr::null_mut(), stack: LinkedList::new(), obj_num: 0, max_num }
    }

    #[inline]
    pub fn push(&mut self, obj: *mut GCObject<T>) {
            self.stack.push_back(obj);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<*mut GCObject<T>> {
        self.stack.pop_back()
    }

    #[inline]
    fn mark_all(&mut self) {
        for &mut obj in self.stack.iter_mut() {
            unsafe {
                Self::mark(obj);
            }
        }
    }

    unsafe fn mark(obj: *mut GCObject<T>) {
        if (*obj).is_marked() {
            return;
        }
        match (*obj).obj {
            Object::Obj(_) => { (*obj).mark(true) }
            Object::ObjPair { head, tail } => {
                Self::mark(head);
                Self::mark(tail)
            }
        }
    }

    unsafe fn sweep(&mut self) {
        let mut ptr = self.root as *mut GCObject<T>;
        while ptr != std::ptr::null_mut() {
            if !(*ptr).is_marked() {
                /* This object wasn't reached, so remove it from the list and free it. */
                let unreached = ptr;
                ptr = (*unreached).next;
                self.obj_num -= 1;
            } else {
                /* This object was reached, so unmark it (for the next GC) and move on to the next. */
                (*ptr).mark(false);
                ptr = (*ptr).next;
            }
            println!("{:?}", ptr);
        }
    }

    #[inline]
    pub fn gc(&mut self) {
        let obj_num = self.obj_num;
        self.mark_all();
        unsafe {
            self.sweep();
        }
        self.max_num = obj_num * 2;
    }
}

fn main() {
    let mut vm: VM<i32> = VM::new(10);
    let a = GCObject::new(&mut vm, Object::new(1));
    vm.push(a);
    let a = GCObject::new(&mut vm, Object::new(2));
    vm.push(a);
    let a = GCObject::new(&mut vm, Object::new(3));
    vm.push(a);
    let a = GCObject::new(&mut vm, Object::new(4));
    vm.push(a);
    vm.gc();

    assert_eq!(vm.obj_num, 4);
//    let a = vm.pop();
//    let a = vm.pop();
    vm.pop();
    vm.pop();

    assert_eq!(vm.obj_num, 2);
}

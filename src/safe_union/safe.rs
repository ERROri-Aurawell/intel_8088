#[derive(Clone, Copy)]
pub union TestUnsafeUnion {
    none: (),
    u8: u8,
    i8: i8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnionType {
    None,
    I8,
    U8,
}

pub trait SafeUnionable<T> {
    const TAG: UnionType;
    unsafe fn read_from(data: &TestUnsafeUnion) -> Self;

    unsafe fn write_to(data: &mut TestUnsafeUnion, content: T);
}

impl SafeUnionable<u8> for u8 {
    const TAG: UnionType = UnionType::U8;
    unsafe fn read_from(data: &TestUnsafeUnion) -> Self {
        unsafe { data.u8 }
    }

    unsafe fn write_to(data: &mut TestUnsafeUnion, content: u8) {
        data.u8 = content;
    }
}

impl SafeUnionable<i8> for i8 {
    const TAG: UnionType = UnionType::I8;
    unsafe fn read_from(data: &TestUnsafeUnion) -> Self {
        unsafe { data.i8 }
    }

    unsafe fn write_to(data: &mut TestUnsafeUnion, content: i8) {
        data.i8 = content;    
    }
}

pub struct SafeUnion {
    union_type: UnionType,
    data: TestUnsafeUnion,
}

impl SafeUnion {
    pub fn new() -> Self {
        SafeUnion {
            union_type: UnionType::None,
            data: TestUnsafeUnion { none: () },
        }
    }

    pub fn set<T: SafeUnionable<T>>(&mut self, val: T) {
        self.union_type = T::TAG;
        
        unsafe {
            T::write_to(&mut self.data, val);
        }
    }

    pub fn get<T: SafeUnionable<T>>(&self) -> Option<T> {
        if self.union_type == T::TAG {
            Some(unsafe { T::read_from(&self.data) })
        } else {
            None
        }
    }
    
}

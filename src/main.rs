trait Page {
    fn set_page(&self, p: i32);
}

trait PerPage {
    fn set_perpage(&self, num: i32){
        println!("Per Page default: {:?}", num);
    }
}

struct MyPaginate{ page: i32 }

impl Page for MyPaginate {
    fn set_page(&self, p: i32){
        println!("Page Default: {:?} page = {:?}", p, self.page);
    }
}
impl PerPage for MyPaginate {}

trait Paginate: Page + PerPage {
    fn set_skip_page(&self, num:i32){
        self.set_page(num);
        self.set_perpage(num);
        println!("Paginate print!");
    }
}

impl <T:Page + PerPage > Paginate for T {}

fn main() {
    let str = "Hello Rust";
    let ptr = str.as_ptr();
    let len = str.len();
    println!("{:p}",ptr);
    println!("{:?}",len);

    let my_paginate = MyPaginate{ page: 100 };
    my_paginate.set_page(401);
    my_paginate.set_perpage(100);
    my_paginate.set_skip_page(5);
}

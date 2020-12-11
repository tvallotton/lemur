


use::std::rc::Rc;

#[derive(Debug)]
enum List <T> {
    Empty,
    Cons {
        head: T,
        tail: Rc<List<T>>
    }
}


impl<T> List<T> {
    fn append(self, value: T) -> List<T> {
        List::Cons {
            head: value,
            tail: Rc::new(self)
        }
    }


}


impl List<i32> {
    fn range(start: i32, end: i32) -> List<i32>{

        let mut out: List<i32> = List::Empty;
        for i in start..end {
            out = out.append(i);
        }
        return out;
    }
}





fn main() {

    let start = 0;
    let end   = 10000;

    //
    // let vector = vec![];

    println!("asd");





}

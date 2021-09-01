pub trait Functor<O, P> {
    type O;
    type P;

    fn transmute(&self, x: Element);
    
    mod _transmute {
        fn _transmute_thing(&self, x: Thing);
        fn _transmute_arrow(&self, x: Arrow);
    }
}

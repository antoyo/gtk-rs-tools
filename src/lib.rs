#[macro_export]
macro_rules! clone {
    ($($n:ident),+ => move |$($p:pat),*| $body:expr) => (
        {
            $( let $n = ::std::rc::Rc::downgrade(&$n); )+
            move |$($p,)*| {
                $(let $n = match $n.upgrade() {
                    Some(val) => val,
                    None => panic!("cannot upgrade weak reference `{}`", stringify!($n)),
                };)+
                $body
            }
        }
    );
}

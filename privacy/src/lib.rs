mod outer {
    pub fn middle() {
        println!("Middle")
    }
    fn middle_private() {
        println!("Middle private")
    }

    pub mod inner {
        pub fn inner() {
            println!("Inner")
        }
        fn inner_private() {
            println!("Inner private")
        }
    }
}

fn try_calling() {
    outer::middle(); // success
                     // outer::middle_private(); failure
    outer::inner::inner(); // success
                           // outer::inner::inner_private(); failure
}

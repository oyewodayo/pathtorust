use std::fs::File;
fn main() {
    let f: Result<File, Error> = File::open("hello.txt");

    let f: File = match f {
        Ok(file)=>file,
        Err(error) => panic!("PRoblem opening the file: {:?}", error)
    };


    let _x = 5 + 90 + 5;
    let _y = 7i64;
    pub mod nested_comments {
        /* In Rust /* we can /* nest comments */ */ */

        // All three types of block comments can contain or be nested inside
        // any other type:

        /*   /* */  /** */  /*! */  */
        /*!  /* */  /** */  /*! */  */
        /**  /* */  /** */  /*! */  */
        pub mod dummy_item {}
    }

}


extern crate libc;

//#! ID Software Queue implementation.
pub mod idqueue {
    use libc::{size_t, c_void};
    use std::intrinsics;
    use std::ptr;
    use std::mem;

    extern {
        pub fn rs_idqueue_add(q: *mut rs_idqueue_t, el: *const c_void) -> c_void;
        pub fn rs_idqueue_get(q: *mut rs_idqueue_t) -> *const c_void;
    }

    //#define idQueue( type, next )idQueueTemplate<type, (int)&(((type*)NULL)->next)>
    /* Original code:
     * template< class type, int nextOffset >
     * class idQueueTemplate {
     * public:
     *   idQueueTemplate( void );
     *
     *   void Add( type *element );
     *   type *Get( void );
     *
     * private:
     *   type *first;
     *   type *last;
     * };
    */
    #[repr(C)]
    pub struct IdQueue<T> {
        first: *mut T,
        last:  *mut T,
        offset: size_t
    }

    #[repr(C)]
    pub type rs_idqueue_t = IdQueue<c_void>;

    impl<T> IdQueue<T> {
        /* Original code:
         * template< class type, int nextOffset >
         * idQueueTemplate<type,nextOffset>::idQueueTemplate( void ) {
         *   first = last = NULL;
         * }
        **/
        pub fn new(offset: size_t) -> IdQueue<T> {
            IdQueue {
                first: ptr::null_mut(),
                last: ptr::null_mut(),
                offset: offset
            }
        }

        //#define QUEUE_NEXT_PTR( element )(*((type**)(((byte*)element)+nextOffset)))
        fn queue_next_ptr<'a>(&self, element: *const T) -> * mut T {
            unsafe { intrinsics::offset(element, self.offset as int) as *mut T}
        }

        /* Original code
         * void idQueueTemplate<type,nextOffset>::Add( type *element ) {
         *   QUEUE_NEXT_PTR(element) = NULL;
         *   if ( last ) {
         *     QUEUE_NEXT_PTR(last) = element;
         *   } else {
         *     first = element;
         *   }
         *     last = element;
         * }
         */
        #[no_mangle]
        pub fn add(&mut self, element: *mut T) {
            let mut next_ptr_el = self.queue_next_ptr(element);
            unsafe { ptr::write(&mut next_ptr_el, ptr::null_mut()) };

            println!("el = {}",  element);
            if self.last.is_not_null() {
                let mut next_ptr_lst = self.queue_next_ptr(self.last);
                println!("pre = {}",  next_ptr_lst);
                unsafe { ptr::write(&mut(next_ptr_lst), element) };
                println!("post = {}",  next_ptr_lst);
            } else {
                println!("self.first pre = {}",  self.first);
                self.first = element;
                println!("self.first post = {}",  self.first);
            }

            println!("self.last pre = {}",  self.last);
            println!("eleme = {}",  element);
            self.last = element;
            println!("self.last post = {}",  self.last);
        }

        pub fn is_empty(&self) -> bool {
            self.last.is_null() && self.first.is_null()
        }

        /* Original code:
         * template< class type, int nextOffset >
         * type *idQueueTemplate<type,nextOffset>::Get( void ) {
         *   type *element;
         *   element = first;
         *   if ( element ) {
         *     first = QUEUE_NEXT_PTR(first);
         *     if ( last == element ) {
         *       last = NULL;
         *     }
         *     QUEUE_NEXT_PTR(element) = NULL;
         *   }
         *   return element;
         * }
        */
        #[no_mangle]
        pub fn get(&mut self) -> *mut T {
            let element = self.first;
            if element.is_not_null() {
              self.first = self.queue_next_ptr(self.first);
              if self.last == element {
                  self.last = ptr::null_mut()
              }
                let mut next_ptr_e = self.queue_next_ptr(element);
                unsafe { ptr::write(&mut next_ptr_e, ptr::null_mut()) };
            }
            element
        }
    }


    #[test]
    fn can_init() {
        struct Point(u8, u8);
        let q: IdQueue<Point> = IdQueue::new(mem::size_of::<Point>() as u64);
        assert!(q.is_empty());
    }

    #[test]
    fn can_add_one_without_segfault() {
        struct Point(u8, u8, *mut Point);
        let mut q: IdQueue<Point> = IdQueue::new((mem::size_of::<u8>() * 2) as u64);
        let ptr: *mut Point = &mut Point(10,20, ptr::null_mut());
        q.add(ptr);
        assert!(q.is_empty() == false);
    }

    #[test]
    fn can_get_nullptr_if_empty() {
        struct Point(u8, u8, *mut Point);
        let mut q: IdQueue<Point> = IdQueue::new((mem::size_of::<u8>() * 2) as u64);
        let res = q.get();
        assert!(res.is_null());
    }

    #[test]
    fn can_get_one() {
        struct Point(u8, u8, *mut Point);
        let mut q: IdQueue<Point> = IdQueue::new((mem::size_of::<u8>() * 2) as u64);
        let ptr: *mut Point = &mut Point(10,20, ptr::null_mut());
        q.add(ptr);
        let res = q.get();
        unsafe { assert!((*res).0 == 10) };
        unsafe { assert!((*res).1 == 20) };
    }

    #[test]
    fn can_push_multiple() {
        struct Point(u8, u8, *mut Point);
        let mut q: IdQueue<Point> = IdQueue::new((mem::size_of::<u8>() * 2) as u64);
        let ptr: *mut Point = &mut Point(10,20, ptr::null_mut());
        let ptr2: *mut Point = &mut Point(8,16, ptr::null_mut());
        q.add(ptr);
        q.add(ptr2);
        unsafe { println!("ptr.2 = {}", (*ptr).2) };
        unsafe { println!("ptr2.2 = {}", (*ptr2).2) };
        assert!(q.first == ptr);
        assert!(q.last == ptr2);
        let res = q.get();
        unsafe { assert!((*res).0 == 10) };
        unsafe { assert!((*res).1 == 20) };
        unsafe { println!("res.2 = {}", (*res).2) };
        unsafe { assert!((*res).2 == ptr2) };
        assert!(!q.is_empty());
        let res2 = q.get();
        unsafe { assert!((*res2).0 == 8) };
        unsafe { assert!((*res2).1 == 16) };
        assert!(q.is_empty());
    }

}

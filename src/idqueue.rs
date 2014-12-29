extern crate libc;

//#! ID Software Queue implementation.
pub mod idqueue {
    use libc::{size_t, c_int, c_void};
    use std::ptr;
    use ptr_math;

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
        fn queue_next_ptr<'a>(&self, element: *const T) -> *mut (* mut T) {
            ptr_math::ptr_add_mut(element as *mut T, self.offset) as *mut (* mut T)
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
        pub fn add(&mut self, element: *mut T) {
            unsafe {
                ptr::write(self.queue_next_ptr(element), ptr::null_mut());
            }

            if self.last.is_not_null() {
                unsafe {
                    ptr::write(self.queue_next_ptr(self.last), element);
                }
            } else {
                self.first = element;
            }

            self.last = element;
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
        pub fn get(&mut self) -> *mut T {
            let element = self.first;
            if element.is_not_null() {
              unsafe { self.first = *(self.queue_next_ptr(self.first)) };
              if self.last == element {
                  self.last = ptr::null_mut()
              }
                unsafe { ptr::write(self.queue_next_ptr(element), ptr::null_mut()) };
            }
            element
        }
    }

    //C layer
    #[no_mangle]
    pub extern fn rs_idqueue_new(sz: size_t) -> rs_idqueue_t {
        IdQueue::new(sz)
    }
    #[no_mangle]
    pub extern "C" fn rs_idqueue_add(q: *mut rs_idqueue_t, el: *mut c_void) -> c_int {
        unsafe {
            (*q).add(el);
            0
        }
    }
    #[no_mangle]
    pub extern fn rs_idqueue_get(q: *mut rs_idqueue_t) -> *const c_void {
        unsafe { (*q).get() }
    }


    #[test]
    fn can_init() {
        struct Point(u8, u8);
        let q: IdQueue<Point> = IdQueue::new(8);
        assert!(q.is_empty());
    }

    #[test]
    fn can_add_one_without_segfault() {
        struct Point(u8, u8, *mut Point);
        let mut q: IdQueue<Point> = IdQueue::new(8);
        let ptr: *mut Point = &mut Point(10,20, ptr::null_mut());
        q.add(ptr);
        assert!(q.is_empty() == false);
    }

    #[test]
    fn can_get_nullptr_if_empty() {
        struct Point(u8, u8, *mut Point);
        let mut q: IdQueue<Point> = IdQueue::new(8);
        let res = q.get();
        assert!(res.is_null());
    }

    #[test]
    fn can_get_one() {
        struct Point(u8, u8, *mut Point);
        let mut q: IdQueue<Point> = IdQueue::new(8);
        let ptr: *mut Point = &mut Point(10,20, ptr::null_mut());
        q.add(ptr);
        let res = q.get();
        unsafe { assert!((*res).0 == 10) };
        unsafe { assert!((*res).1 == 20) };
    }

    #[test]
    #[cfg(target_arch="x86")]
    fn can_push_multiple_x86() {
        struct Point(u8, u8, *mut Point);
        let mut q: IdQueue<Point> = IdQueue::new(4);
        let ptr: *mut Point = &mut Point(10,20, ptr::null_mut());
        let ptr2: *mut Point = &mut Point(8,16, ptr::null_mut());
        q.add(ptr);
        q.add(ptr2);
        assert!(q.first == ptr);
        assert!(q.last == ptr2);
        unsafe { assert!((*ptr).2 == ptr2) };
        let res = q.get();
        unsafe { assert!((*res).0 == 10) };
        unsafe { assert!((*res).1 == 20) };
        assert!(!q.is_empty());
        let res2 = q.get();
        unsafe { assert!((*res2).0 == 8) };
        unsafe { assert!((*res2).1 == 16) };
        assert!(q.is_empty());
    }

    #[test]
    #[cfg(target_arch="x86_64")]
    fn can_push_multiple_x86_64() {
        struct Point(u8, u8, *mut Point);
        let mut q: IdQueue<Point> = IdQueue::new(8);
        let ptr: *mut Point = &mut Point(10,20, ptr::null_mut());
        let ptr2: *mut Point = &mut Point(8,16, ptr::null_mut());
        q.add(ptr);
        q.add(ptr2);
        assert!(q.first == ptr);
        assert!(q.last == ptr2);
        unsafe { assert!((*ptr).2 == ptr2) };
        let res = q.get();
        unsafe { assert!((*res).0 == 10) };
        unsafe { assert!((*res).1 == 20) };
        assert!(!q.is_empty());
        let res2 = q.get();
        unsafe { assert!((*res2).0 == 8) };
        unsafe { assert!((*res2).1 == 16) };
        assert!(q.is_empty());
    }

}

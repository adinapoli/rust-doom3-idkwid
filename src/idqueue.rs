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
        fn queue_next_ptr(&self, element: *const T) -> Box<*mut T> {
            box unsafe {
                mem::transmute(intrinsics::offset(element, self.offset as int))
            }
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
            *self.queue_next_ptr(element) = ptr::null_mut();

            if self.last.is_null() {
                self.first = element;
            } else {
                *self.queue_next_ptr(self.last) = element
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
        #[no_mangle]
        pub fn get(&mut self) -> *mut T {
            let element = self.first;
            if !element.is_null() {
              self.first = *self.queue_next_ptr(self.first);
              if self.last == element {
                  self.last = ptr::null_mut()
              }
              *self.queue_next_ptr(element) = ptr::null_mut();
            }
            element
        }
    }


    #[test]
    fn can_init() {
        struct Point(u8, u8);
        let q: IdQueue<Point> = IdQueue::new(mem::size_of::<Point>() as u64);
        assert!(q.is_empty() == true);
    }

    #[test]
    fn can_add_one_without_segfault() {
        struct Point(u8, u8, *mut Point);
        let mut q: IdQueue<Point> = IdQueue::new((mem::size_of::<u8>() * 2) as u64);
        let ptr: *mut Point = &mut Point(10,20, ptr::null_mut());
        q.add(ptr);
        assert!(q.is_empty() == false);
    }

}

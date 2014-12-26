extern crate libc;

//#! ID Software Queue implementation.
pub mod idqueue {
    use libc::size_t;
    use std::intrinsics;
    use std::ptr;
    use std::mem;
    use std::rc::Rc;

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
        first: Rc<Option<*mut T>>,
        last:  Rc<Option<*mut T>>,
        offset: size_t
    }

    impl<T> IdQueue<T> {
        /* Original code:
         * template< class type, int nextOffset >
         * idQueueTemplate<type,nextOffset>::idQueueTemplate( void ) {
         *   first = last = NULL;
         * }
        **/
        pub extern "C" fn new(offset: size_t) -> IdQueue<T> {
            IdQueue {
                first: Rc::new(None),
                last: Rc::new(None),
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
        pub extern "C" fn add(&mut self, element: *mut T) {
            *self.queue_next_ptr(element) = ptr::null_mut();
            match *self.last {
                Some(ref v) => { *self.queue_next_ptr(*v) = element },
                None => { self.first = Rc::new(Some(element)) }
            }
            self.last = Rc::new(Some(element))
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
        pub extern fn get(&mut self) -> *mut T {
            let element = self.first.clone();
            match *element {
                Some(ref v) => {
                    self.first = Rc::new(Some(*self.queue_next_ptr(*v)));
                    if self.last.is_some() {
                       if (*self.last).unwrap() == *v {
                           self.last = Rc::new(None)
                       }
                    }
                    *self.queue_next_ptr(*v) = ptr::null_mut();
                },
                None => {}
            }
            (*element).unwrap()
        }
    }

}

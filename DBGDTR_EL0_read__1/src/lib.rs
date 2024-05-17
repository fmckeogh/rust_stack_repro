#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use u__IMPDEF_boolean::*;
use common::*;
pub fn DBGDTR_EL0_read__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_29317: (),
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
        gs_29317: (),
    }
    let fn_state = FunctionState {
        gs_29317,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_0_0: const #11568u : u32
        let s_0_0: u32 = 11568;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: write-var r <= s_0_1
        fn_state.r = s_0_1;
        // C s_0_3: const #"read" : str
        let s_0_3: &'static str = "read";
        // S s_0_4: call __IMPDEF_boolean(s_0_3)
        let s_0_4: bool = u__IMPDEF_boolean(state, tracer, s_0_3);
        // N s_0_5: branch s_0_4 b10 b1
        if s_0_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_1_0: const #"read" : str
        let s_1_0: &'static str = "read";
        // S s_1_1: call __IMPDEF_boolean(s_1_0)
        let s_1_1: bool = u__IMPDEF_boolean(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b9 b2
        if s_1_1 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_2_0: read-var r:struct
        let s_2_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_2_1: write-var r <= s_2_0
        fn_state.r = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_3_0: const #"read" : str
        let s_3_0: &'static str = "read";
        // S s_3_1: call __IMPDEF_boolean(s_3_0)
        let s_3_1: bool = u__IMPDEF_boolean(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b8 b4
        if s_3_1 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_4_0: const #"read" : str
        let s_4_0: &'static str = "read";
        // S s_4_1: call __IMPDEF_boolean(s_4_0)
        let s_4_1: bool = u__IMPDEF_boolean(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b7 b5
        if s_4_1 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_5_0: read-var r:struct
        let s_5_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_5_1: write-var r <= s_5_0
        fn_state.r = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_6_0: read-var r:struct
        let s_6_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_7_0: read-var r:struct
        let s_7_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_7_1: write-var r <= s_7_0
        fn_state.r = s_7_0;
        // N s_7_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_8_0: read-var r:struct
        let s_8_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_8_1: write-var r <= s_8_0
        fn_state.r = s_8_0;
        // N s_8_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_9_0: read-var r:struct
        let s_9_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_9_1: write-var r <= s_9_0
        fn_state.r = s_9_0;
        // N s_9_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_10_0: read-var r:struct
        let s_10_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_10_1: write-var r <= s_10_0
        fn_state.r = s_10_0;
        // N s_10_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}

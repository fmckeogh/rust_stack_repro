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
use u_get_MPAM2_EL2_Type_MPAMEN::*;
use common::*;
pub fn u__set_MPAM2_EL2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
        value_name: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #90504u : u32
        let s_0_2: u32 = 90504;
        // D s_0_3: read-reg s_0_2:struct
        let s_0_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // C s_0_4: const #90504u : u32
        let s_0_4: u32 = 90504;
        // N s_0_5: write-reg s_0_4 <= s_0_3
        let s_0_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_4 as isize, s_0_3);
            tracer.write_register(s_0_4 as isize, s_0_3);
        };
        // C s_0_6: const #90504u : u32
        let s_0_6: u32 = 90504;
        // D s_0_7: read-reg s_0_6:struct
        let s_0_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // C s_0_8: const #90504u : u32
        let s_0_8: u32 = 90504;
        // N s_0_9: write-reg s_0_8 <= s_0_7
        let s_0_9: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_8 as isize, s_0_7);
            tracer.write_register(s_0_8 as isize, s_0_7);
        };
        // C s_0_10: const #424u : u32
        let s_0_10: u32 = 424;
        // D s_0_11: read-reg s_0_10:u8
        let s_0_11: u8 = {
            let value = state.read_register::<u8>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // C s_0_12: const #2u : u8
        let s_0_12: u8 = 2;
        // D s_0_13: cmp-lt s_0_11 s_0_12
        let s_0_13: bool = ((s_0_11) < (s_0_12));
        // D s_0_14: not s_0_13
        let s_0_14: bool = !s_0_13;
        // N s_0_15: branch s_0_14 b2 b1
        if s_0_14 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:struct
        let s_2_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_2_1: call _get_MPAM2_EL2_Type_MPAMEN(s_2_0)
        let s_2_1: bool = u_get_MPAM2_EL2_Type_MPAMEN(state, tracer, s_2_0);
        // C s_2_2: const #90504u : u32
        let s_2_2: u32 = 90504;
        // D s_2_3: read-reg s_2_2:struct
        let s_2_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // C s_2_4: const #90504u : u32
        let s_2_4: u32 = 90504;
        // N s_2_5: write-reg s_2_4 <= s_2_3
        let s_2_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_2_4 as isize, s_2_3);
            tracer.write_register(s_2_4 as isize, s_2_3);
        };
        // N s_2_6: return
        return;
    }
}

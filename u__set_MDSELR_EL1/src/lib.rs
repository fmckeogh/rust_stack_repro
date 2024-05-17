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
use u_get_MDSELR_EL1_Type_BANK::*;
use common::*;
pub fn u__set_MDSELR_EL1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
        gs_33384: bool,
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
        // C s_0_2: const #18416u : u32
        let s_0_2: u32 = 18416;
        // D s_0_3: read-reg s_0_2:struct
        let s_0_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // C s_0_4: const #18416u : u32
        let s_0_4: u32 = 18416;
        // N s_0_5: write-reg s_0_4 <= s_0_3
        let s_0_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_4 as isize, s_0_3);
            tracer.write_register(s_0_4 as isize, s_0_3);
        };
        // C s_0_6: const #18416u : u32
        let s_0_6: u32 = 18416;
        // D s_0_7: read-reg s_0_6:struct
        let s_0_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // C s_0_8: const #18416u : u32
        let s_0_8: u32 = 18416;
        // N s_0_9: write-reg s_0_8 <= s_0_7
        let s_0_9: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_8 as isize, s_0_7);
            tracer.write_register(s_0_8 as isize, s_0_7);
        };
        // C s_0_10: const #16s : i
        let s_0_10: i128 = 16;
        // C s_0_11: const #22776u : u32
        let s_0_11: u32 = 22776;
        // D s_0_12: read-reg s_0_11:i
        let s_0_12: i128 = {
            let value = state.read_register::<i128>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: cmp-le s_0_12 s_0_10
        let s_0_13: bool = ((s_0_12) <= (s_0_10));
        // N s_0_14: branch s_0_13 b5 b1
        if s_0_13 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#33384 <= s_1_0
        fn_state.gs_33384 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#33384:u8
        let s_2_0: bool = fn_state.gs_33384;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b4 b3
        if s_2_1 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var r:struct
        let s_4_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_4_1: call _get_MDSELR_EL1_Type_BANK(s_4_0)
        let s_4_1: u8 = u_get_MDSELR_EL1_Type_BANK(state, tracer, s_4_0);
        // C s_4_2: const #18416u : u32
        let s_4_2: u32 = 18416;
        // D s_4_3: read-reg s_4_2:struct
        let s_4_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // C s_4_4: const #18416u : u32
        let s_4_4: u32 = 18416;
        // N s_4_5: write-reg s_4_4 <= s_4_3
        let s_4_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_4_4 as isize, s_4_3);
            tracer.write_register(s_4_4 as isize, s_4_3);
        };
        // N s_4_6: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16s : i
        let s_5_0: i128 = 16;
        // C s_5_1: const #19360u : u32
        let s_5_1: u32 = 19360;
        // D s_5_2: read-reg s_5_1:i
        let s_5_2: i128 = {
            let value = state.read_register::<i128>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: cmp-le s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) <= (s_5_0));
        // D s_5_4: write-var gs#33384 <= s_5_3
        fn_state.gs_33384 = s_5_3;
        // N s_5_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}

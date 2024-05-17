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
use u_get_PMSIDR_EL1_Type_ERnd::*;
use Zeros::*;
use u_get_PMSIRR_EL1_Type_RND::*;
use u_get_PMSIRR_EL1_Type_INTERVAL::*;
use SPEGetRandomInterval::*;
use common::*;
pub fn SPEResetSampleCounter<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_25636: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_25645: bool,
        gs_25636: (),
    }
    let fn_state = FunctionState {
        gs_25636,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #12736u : u32
        let s_0_0: u32 = 12736;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_PMSIRR_EL1_Type_INTERVAL(s_0_1)
        let s_0_2: u32 = u_get_PMSIRR_EL1_Type_INTERVAL(state, tracer, s_0_1);
        // C s_0_3: const #101816u : u32
        let s_0_3: u32 = 101816;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // C s_0_5: const #101816u : u32
        let s_0_5: u32 = 101816;
        // N s_0_6: write-reg s_0_5 <= s_0_4
        let s_0_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_5 as isize, s_0_4);
            tracer.write_register(s_0_5 as isize, s_0_4);
        };
        // C s_0_7: const #12736u : u32
        let s_0_7: u32 = 12736;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_PMSIRR_EL1_Type_RND(s_0_8)
        let s_0_9: bool = u_get_PMSIRR_EL1_Type_RND(state, tracer, s_0_8);
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 1u16);
        // C s_0_11: const #1u : u8
        let s_0_11: bool = true;
        // C s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 1u16);
        // D s_0_13: cmp-eq s_0_10 s_0_12
        let s_0_13: bool = ((s_0_10) == (s_0_12));
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
        // D s_1_1: write-var gs#25645 <= s_1_0
        fn_state.gs_25645 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#25645:u8
        let s_2_0: bool = fn_state.gs_25645;
        // N s_2_1: branch s_2_0 b4 b3
        if s_2_0 {
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
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // S s_3_1: call Zeros(s_3_0)
        let s_3_1: Bits = Zeros(state, tracer, s_3_0);
        // C s_3_2: const #101816u : u32
        let s_3_2: u32 = 101816;
        // D s_3_3: read-reg s_3_2:struct
        let s_3_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // C s_3_4: const #101816u : u32
        let s_3_4: u32 = 101816;
        // N s_3_5: write-reg s_3_4 <= s_3_3
        let s_3_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_3_4 as isize, s_3_3);
            tracer.write_register(s_3_4 as isize, s_3_3);
        };
        // N s_3_6: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call SPEGetRandomInterval(s_4_0)
        let s_4_1: u8 = SPEGetRandomInterval(state, tracer, s_4_0);
        // C s_4_2: const #101816u : u32
        let s_4_2: u32 = 101816;
        // D s_4_3: read-reg s_4_2:struct
        let s_4_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // C s_4_4: const #101816u : u32
        let s_4_4: u32 = 101816;
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
        // C s_5_0: const #14736u : u32
        let s_5_0: u32 = 14736;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call _get_PMSIDR_EL1_Type_ERnd(s_5_1)
        let s_5_2: bool = u_get_PMSIDR_EL1_Type_ERnd(state, tracer, s_5_1);
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // C s_5_4: const #0u : u8
        let s_5_4: bool = false;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 1u16);
        // D s_5_6: cmp-eq s_5_3 s_5_5
        let s_5_6: bool = ((s_5_3) == (s_5_5));
        // D s_5_7: write-var gs#25645 <= s_5_6
        fn_state.gs_25645 = s_5_6;
        // N s_5_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}

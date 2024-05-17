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
use IsZero::*;
use PMUEvent::*;
use u_get_PMBSR_EL1_Type_S::*;
use common::*;
pub fn SPEBufferFilled<T: Tracer>(state: &mut State, tracer: &T, gs_25695: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_25695: (),
    }
    let fn_state = FunctionState {
        gs_25695,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #13704u : u32
        let s_0_0: u32 = 13704;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_PMBSR_EL1_Type_S(s_0_1)
        let s_0_2: bool = u_get_PMBSR_EL1_Type_S(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_4: call IsZero(s_0_3)
        let s_0_4: bool = IsZero(state, tracer, s_0_3);
        // N s_0_5: branch s_0_4 b3 b1
        if s_0_4 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #224u : u32
        let s_2_0: u32 = 224;
        // D s_2_1: read-reg s_2_0:u16
        let s_2_1: u16 = {
            let value = state.read_register::<u16>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call PMUEvent(s_2_1)
        let s_2_2: () = PMUEvent(state, tracer, s_2_1);
        // N s_2_3: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #13704u : u32
        let s_3_0: u32 = 13704;
        // D s_3_1: read-reg s_3_0:struct
        let s_3_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // C s_3_2: const #13704u : u32
        let s_3_2: u32 = 13704;
        // N s_3_3: write-reg s_3_2 <= s_3_1
        let s_3_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_3_2 as isize, s_3_1);
            tracer.write_register(s_3_2 as isize, s_3_1);
        };
        // C s_3_4: const #13704u : u32
        let s_3_4: u32 = 13704;
        // D s_3_5: read-reg s_3_4:struct
        let s_3_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_4 as isize);
            tracer.read_register(s_3_4 as isize, value);
            value
        };
        // C s_3_6: const #13704u : u32
        let s_3_6: u32 = 13704;
        // N s_3_7: write-reg s_3_6 <= s_3_5
        let s_3_7: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_3_6 as isize, s_3_5);
            tracer.write_register(s_3_6 as isize, s_3_5);
        };
        // C s_3_8: const #13704u : u32
        let s_3_8: u32 = 13704;
        // D s_3_9: read-reg s_3_8:struct
        let s_3_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_8 as isize);
            tracer.read_register(s_3_8 as isize, value);
            value
        };
        // C s_3_10: const #13704u : u32
        let s_3_10: u32 = 13704;
        // N s_3_11: write-reg s_3_10 <= s_3_9
        let s_3_11: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_3_10 as isize, s_3_9);
            tracer.write_register(s_3_10 as isize, s_3_9);
        };
        // N s_3_12: jump b2
        return block_2(state, tracer, fn_state);
    }
}

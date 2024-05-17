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
use Halt__1::*;
use u_get_EDESR_Type_RC::*;
use HaltingAllowed::*;
use common::*;
pub fn CheckPendingResetCatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_29548: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_29549: bool,
        gs_29548: (),
    }
    let fn_state = FunctionState {
        gs_29548,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaltingAllowed(s_0_0)
        let s_0_1: bool = HaltingAllowed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b5 b1
        if s_0_1 {
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
        // D s_1_1: write-var gs#29549 <= s_1_0
        fn_state.gs_29549 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#29549:u8
        let s_2_0: bool = fn_state.gs_29549;
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
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // C s_4_1: const #1136u : u32
        let s_4_1: u32 = 1136;
        // D s_4_2: read-reg s_4_1:u8
        let s_4_2: u8 = {
            let value = state.read_register::<u8>(s_4_1 as isize);
            tracer.read_register(s_4_1 as isize, value);
            value
        };
        // D s_4_3: call Halt__1(s_4_2, s_4_0)
        let s_4_3: () = Halt__1(state, tracer, s_4_2, s_4_0);
        // N s_4_4: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #14712u : u32
        let s_5_0: u32 = 14712;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call _get_EDESR_Type_RC(s_5_1)
        let s_5_2: bool = u_get_EDESR_Type_RC(state, tracer, s_5_1);
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // C s_5_4: const #1u : u8
        let s_5_4: bool = true;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 1u16);
        // D s_5_6: cmp-eq s_5_3 s_5_5
        let s_5_6: bool = ((s_5_3) == (s_5_5));
        // D s_5_7: write-var gs#29549 <= s_5_6
        fn_state.gs_29549 = s_5_6;
        // N s_5_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}

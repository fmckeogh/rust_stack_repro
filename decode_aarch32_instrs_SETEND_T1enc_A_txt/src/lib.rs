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
use execute_aarch32_instrs_SETEND_Op_A_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_SETEND_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    E: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        set_bigend: bool,
        E: bool,
    }
    let fn_state = FunctionState {
        E,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var E:u8
        let s_0_0: bool = fn_state.E;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 1u16);
        // C s_0_2: const #1u : u8
        let s_0_2: bool = true;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // D s_0_5: write-var set_bigend <= s_0_4
        fn_state.set_bigend = s_0_4;
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call InITBlock(s_0_6)
        let s_0_7: bool = InITBlock(state, tracer, s_0_6);
        // N s_0_8: branch s_0_7 b2 b1
        if s_0_7 {
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
        // D s_1_0: read-var set_bigend:u8
        let s_1_0: bool = fn_state.set_bigend;
        // D s_1_1: call execute_aarch32_instrs_SETEND_Op_A_txt(s_1_0)
        let s_1_1: () = execute_aarch32_instrs_SETEND_Op_A_txt(state, tracer, s_1_0);
        // N s_1_2: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: panic
        panic!("{:?}", ());
        // N s_2_1: return
        return;
    }
}

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
use execute_aarch64_instrs_integer_flags_rmif::*;
use HaveFlagManipulateExt::*;
use common::*;
pub fn decode_rmif_aarch64_instrs_integer_flags_rmif<T: Tracer>(
    state: &mut State,
    tracer: &T,
    mask: u8,
    Rn: u8,
    imm6: u8,
    sf: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_166791: bool,
        mask: u8,
        Rn: u8,
        imm6: u8,
        sf: bool,
    }
    let fn_state = FunctionState {
        mask,
        Rn,
        imm6,
        sf,
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
        // S s_0_1: call HaveFlagManipulateExt(s_0_0)
        let s_0_1: bool = HaveFlagManipulateExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b5 b1
        if s_0_2 {
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
        // D s_1_0: read-var sf:u8
        let s_1_0: bool = fn_state.sf;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // C s_1_2: const #1u : u8
        let s_1_2: bool = true;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_4: cmp-ne s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) != (s_1_3));
        // D s_1_5: write-var gs#166791 <= s_1_4
        fn_state.gs_166791 = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#166791:u8
        let s_2_0: bool = fn_state.gs_166791;
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
        // D s_3_0: read-var imm6:u8
        let s_3_0: u8 = fn_state.imm6;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 6u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: read-var Rn:u8
        let s_3_4: u8 = fn_state.Rn;
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 5u16);
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (s_3_5.value() as i128);
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: read-var mask:u8
        let s_3_8: u8 = fn_state.mask;
        // D s_3_9: call execute_aarch64_instrs_integer_flags_rmif(s_3_3, s_3_8, s_3_7)
        let s_3_9: () = execute_aarch64_instrs_integer_flags_rmif(
            state,
            tracer,
            s_3_3,
            s_3_8,
            s_3_7,
        );
        // N s_3_10: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var gs#166791 <= s_5_0
        fn_state.gs_166791 = s_5_0;
        // N s_5_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}

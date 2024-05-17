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
use execute_aarch32_instrs_LSL_i_Op_A_txt::*;
use ConditionPassed::*;
use DecodeImmShift::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_LSL_i_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm5: u8,
    Rm: u8,
    Rd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_344898: ProductType396b95aa74979079,
        imm5: u8,
        Rm: u8,
        Rd: u8,
    }
    let fn_state = FunctionState {
        imm5,
        Rm,
        Rd,
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
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
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
        // D s_2_0: read-var imm5:u8
        let s_2_0: u8 = fn_state.imm5;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 5u16);
        // C s_2_2: const #0u : u8
        let s_2_2: u8 = 0;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 5u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b4 b3
        if s_2_4 {
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
        // D s_3_0: read-var Rd:u8
        let s_3_0: u8 = fn_state.Rd;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 3u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: read-var Rm:u8
        let s_3_4: u8 = fn_state.Rm;
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 3u16);
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (s_3_5.value() as i128);
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // C s_3_8: const #() : ()
        let s_3_8: () = ();
        // S s_3_9: call InITBlock(s_3_8)
        let s_3_9: bool = InITBlock(state, tracer, s_3_8);
        // S s_3_10: not s_3_9
        let s_3_10: bool = !s_3_9;
        // C s_3_11: const #0u : u8
        let s_3_11: u8 = 0;
        // D s_3_12: read-var imm5:u8
        let s_3_12: u8 = fn_state.imm5;
        // D s_3_13: call DecodeImmShift(s_3_11, s_3_12)
        let s_3_13: ProductType396b95aa74979079 = DecodeImmShift(
            state,
            tracer,
            s_3_11,
            s_3_12,
        );
        // D s_3_14: write-var ga#344898 <= s_3_13
        fn_state.ga_344898 = s_3_13;
        // D s_3_15: read-var ga#344898.1:struct
        let s_3_15: i128 = fn_state.ga_344898._1;
        // D s_3_16: call execute_aarch32_instrs_LSL_i_Op_A_txt(s_3_3, s_3_7, s_3_10, s_3_15)
        let s_3_16: () = execute_aarch32_instrs_LSL_i_Op_A_txt(
            state,
            tracer,
            s_3_3,
            s_3_7,
            s_3_10,
            s_3_15,
        );
        // N s_3_17: return
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
}

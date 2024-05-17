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
use execute_aarch32_instrs_ASR_i_Op_A_txt::*;
use ConditionPassed::*;
use DecodeImmShift::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_ASR_i_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm5: u8,
    Rm: u8,
    Rd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_343135: ProductType396b95aa74979079,
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
        // D s_2_0: read-var Rd:u8
        let s_2_0: u8 = fn_state.Rd;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 3u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: read-var Rm:u8
        let s_2_4: u8 = fn_state.Rm;
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 3u16);
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (s_2_5.value() as i128);
        // D s_2_7: cast reint s_2_6 -> i64
        let s_2_7: i64 = (s_2_6 as i64);
        // C s_2_8: const #() : ()
        let s_2_8: () = ();
        // S s_2_9: call InITBlock(s_2_8)
        let s_2_9: bool = InITBlock(state, tracer, s_2_8);
        // S s_2_10: not s_2_9
        let s_2_10: bool = !s_2_9;
        // C s_2_11: const #2u : u8
        let s_2_11: u8 = 2;
        // D s_2_12: read-var imm5:u8
        let s_2_12: u8 = fn_state.imm5;
        // D s_2_13: call DecodeImmShift(s_2_11, s_2_12)
        let s_2_13: ProductType396b95aa74979079 = DecodeImmShift(
            state,
            tracer,
            s_2_11,
            s_2_12,
        );
        // D s_2_14: write-var ga#343135 <= s_2_13
        fn_state.ga_343135 = s_2_13;
        // D s_2_15: read-var ga#343135.1:struct
        let s_2_15: i128 = fn_state.ga_343135._1;
        // D s_2_16: call execute_aarch32_instrs_ASR_i_Op_A_txt(s_2_3, s_2_7, s_2_10, s_2_15)
        let s_2_16: () = execute_aarch32_instrs_ASR_i_Op_A_txt(
            state,
            tracer,
            s_2_3,
            s_2_7,
            s_2_10,
            s_2_15,
        );
        // N s_2_17: return
        return;
    }
}

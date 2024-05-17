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
use ConditionPassed::*;
use execute_aarch32_instrs_MOV_r_Op_A_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_MOV_r_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rm: u8,
    Rd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        shift_nshadow_7221: i128,
        shift_t: u32,
        d: i64,
        Rm: u8,
        Rd: u8,
    }
    let fn_state = FunctionState {
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
        // D s_2_4: write-var d <= s_2_3
        fn_state.d = s_2_3;
        // D s_2_5: read-var Rm:u8
        let s_2_5: u8 = fn_state.Rm;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 3u16);
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (s_2_6.value() as i128);
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // D s_2_9: write-var m <= s_2_8
        fn_state.m = s_2_8;
        // C s_2_10: const #0s : i
        let s_2_10: i128 = 0;
        // C s_2_11: const #0u : u32
        let s_2_11: u32 = 0;
        // D s_2_12: create-product struct = ["s_2_11", "s_2_10"]
        let s_2_12: ProductType396b95aa74979079 = ProductType396b95aa74979079 {
            _0: s_2_11,
            _1: s_2_10,
        };
        // D s_2_13: extract-field s_2_12.0
        let s_2_13: u32 = s_2_12._0;
        // C s_2_14: const #0u : u32
        let s_2_14: u32 = 0;
        // D s_2_15: create-product struct = ["s_2_14", "s_2_10"]
        let s_2_15: ProductType396b95aa74979079 = ProductType396b95aa74979079 {
            _0: s_2_14,
            _1: s_2_10,
        };
        // D s_2_16: extract-field s_2_15.1
        let s_2_16: i128 = s_2_15._1;
        // D s_2_17: cast reint s_2_16 -> i64
        let s_2_17: i64 = (s_2_16 as i64);
        // D s_2_18: write-var shift_t <= s_2_13
        fn_state.shift_t = s_2_13;
        // D s_2_19: cast zx s_2_17 -> i
        let s_2_19: i128 = (i128::try_from(s_2_17).unwrap());
        // D s_2_20: write-var shift_nshadow#7221 <= s_2_19
        fn_state.shift_nshadow_7221 = s_2_19;
        // C s_2_21: const #() : ()
        let s_2_21: () = ();
        // S s_2_22: call InITBlock(s_2_21)
        let s_2_22: bool = InITBlock(state, tracer, s_2_21);
        // N s_2_23: branch s_2_22 b4 b3
        if s_2_22 {
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
        // D s_3_0: read-var shift_t:u32
        let s_3_0: u32 = fn_state.shift_t;
        // D s_3_1: read-var shift_nshadow#7221:i
        let s_3_1: i128 = fn_state.shift_nshadow_7221;
        // D s_3_2: read-var d:i64
        let s_3_2: i64 = fn_state.d;
        // D s_3_3: read-var m:i64
        let s_3_3: i64 = fn_state.m;
        // C s_3_4: const #1u : u8
        let s_3_4: bool = true;
        // D s_3_5: call execute_aarch32_instrs_MOV_r_Op_A_txt(s_3_2, s_3_3, s_3_4, s_3_1, s_3_0)
        let s_3_5: () = execute_aarch32_instrs_MOV_r_Op_A_txt(
            state,
            tracer,
            s_3_2,
            s_3_3,
            s_3_4,
            s_3_1,
            s_3_0,
        );
        // N s_3_6: return
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

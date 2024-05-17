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
use DecodeImmShift::*;
use execute_aarch32_instrs_PLI_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_PLI_r_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    Rn: u8,
    imm5: u8,
    stype: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        ga_345494: ProductType396b95aa74979079,
        shift_t: u32,
        shift_nshadow_7246: i128,
        n: i64,
        add: bool,
        U: bool,
        Rn: u8,
        imm5: u8,
        stype: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        U,
        Rn,
        imm5,
        stype,
        Rm,
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
        // D s_2_0: read-var Rn:u8
        let s_2_0: u8 = fn_state.Rn;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var n <= s_2_3
        fn_state.n = s_2_3;
        // D s_2_5: read-var Rm:u8
        let s_2_5: u8 = fn_state.Rm;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 4u16);
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (s_2_6.value() as i128);
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // D s_2_9: write-var m <= s_2_8
        fn_state.m = s_2_8;
        // D s_2_10: read-var U:u8
        let s_2_10: bool = fn_state.U;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 1u16);
        // C s_2_12: const #1u : u8
        let s_2_12: bool = true;
        // C s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 1u16);
        // D s_2_14: cmp-eq s_2_11 s_2_13
        let s_2_14: bool = ((s_2_11) == (s_2_13));
        // D s_2_15: write-var add <= s_2_14
        fn_state.add = s_2_14;
        // D s_2_16: read-var stype:u8
        let s_2_16: u8 = fn_state.stype;
        // D s_2_17: read-var imm5:u8
        let s_2_17: u8 = fn_state.imm5;
        // D s_2_18: call DecodeImmShift(s_2_16, s_2_17)
        let s_2_18: ProductType396b95aa74979079 = DecodeImmShift(
            state,
            tracer,
            s_2_16,
            s_2_17,
        );
        // D s_2_19: write-var ga#345494 <= s_2_18
        fn_state.ga_345494 = s_2_18;
        // D s_2_20: read-var ga#345494.0:struct
        let s_2_20: u32 = fn_state.ga_345494._0;
        // D s_2_21: read-var ga#345494.1:struct
        let s_2_21: i128 = fn_state.ga_345494._1;
        // D s_2_22: write-var shift_t <= s_2_20
        fn_state.shift_t = s_2_20;
        // D s_2_23: write-var shift_nshadow#7246 <= s_2_21
        fn_state.shift_nshadow_7246 = s_2_21;
        // C s_2_24: const #15s : i
        let s_2_24: i128 = 15;
        // D s_2_25: read-var m:i64
        let s_2_25: i64 = fn_state.m;
        // D s_2_26: cast zx s_2_25 -> i
        let s_2_26: i128 = (i128::try_from(s_2_25).unwrap());
        // D s_2_27: cmp-eq s_2_26 s_2_24
        let s_2_27: bool = ((s_2_26) == (s_2_24));
        // N s_2_28: branch s_2_27 b4 b3
        if s_2_27 {
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
        // D s_3_1: read-var shift_nshadow#7246:i
        let s_3_1: i128 = fn_state.shift_nshadow_7246;
        // D s_3_2: read-var add:u8
        let s_3_2: bool = fn_state.add;
        // D s_3_3: read-var m:i64
        let s_3_3: i64 = fn_state.m;
        // D s_3_4: read-var n:i64
        let s_3_4: i64 = fn_state.n;
        // D s_3_5: call execute_aarch32_instrs_PLI_r_Op_A_txt(s_3_2, s_3_3, s_3_4, s_3_1, s_3_0)
        let s_3_5: () = execute_aarch32_instrs_PLI_r_Op_A_txt(
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

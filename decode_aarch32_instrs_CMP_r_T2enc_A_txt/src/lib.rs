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
use execute_aarch32_instrs_CMP_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_CMP_r_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    N: bool,
    Rm: u8,
    Rn: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        shift_nshadow_7141: i128,
        gs_296440: bool,
        shift_t: u32,
        gs_296443: bool,
        n: i64,
        N: bool,
        Rm: u8,
        Rn: u8,
    }
    let fn_state = FunctionState {
        N,
        Rm,
        Rn,
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
        // D s_2_0: read-var N:u8
        let s_2_0: bool = fn_state.N;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // D s_2_2: read-var Rn:u8
        let s_2_2: u8 = fn_state.Rn;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 3u16);
        // D s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // D s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // D s_2_12: cast reint s_2_11 -> u8
        let s_2_12: u8 = (s_2_11.value() as u8);
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 4u16);
        // D s_2_14: cast zx s_2_13 -> i
        let s_2_14: i128 = (s_2_13.value() as i128);
        // D s_2_15: cast reint s_2_14 -> i64
        let s_2_15: i64 = (s_2_14 as i64);
        // D s_2_16: write-var n <= s_2_15
        fn_state.n = s_2_15;
        // D s_2_17: read-var Rm:u8
        let s_2_17: u8 = fn_state.Rm;
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 4u16);
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (s_2_18.value() as i128);
        // D s_2_20: cast reint s_2_19 -> i64
        let s_2_20: i64 = (s_2_19 as i64);
        // D s_2_21: write-var m <= s_2_20
        fn_state.m = s_2_20;
        // C s_2_22: const #0s : i
        let s_2_22: i128 = 0;
        // C s_2_23: const #0u : u32
        let s_2_23: u32 = 0;
        // D s_2_24: create-product struct = ["s_2_23", "s_2_22"]
        let s_2_24: ProductType396b95aa74979079 = ProductType396b95aa74979079 {
            _0: s_2_23,
            _1: s_2_22,
        };
        // D s_2_25: extract-field s_2_24.0
        let s_2_25: u32 = s_2_24._0;
        // C s_2_26: const #0u : u32
        let s_2_26: u32 = 0;
        // D s_2_27: create-product struct = ["s_2_26", "s_2_22"]
        let s_2_27: ProductType396b95aa74979079 = ProductType396b95aa74979079 {
            _0: s_2_26,
            _1: s_2_22,
        };
        // D s_2_28: extract-field s_2_27.1
        let s_2_28: i128 = s_2_27._1;
        // D s_2_29: cast reint s_2_28 -> i64
        let s_2_29: i64 = (s_2_28 as i64);
        // D s_2_30: write-var shift_t <= s_2_25
        fn_state.shift_t = s_2_25;
        // D s_2_31: cast zx s_2_29 -> i
        let s_2_31: i128 = (i128::try_from(s_2_29).unwrap());
        // D s_2_32: write-var shift_nshadow#7141 <= s_2_31
        fn_state.shift_nshadow_7141 = s_2_31;
        // C s_2_33: const #8s : i
        let s_2_33: i128 = 8;
        // D s_2_34: read-var n:i64
        let s_2_34: i64 = fn_state.n;
        // D s_2_35: cast zx s_2_34 -> i
        let s_2_35: i128 = (i128::try_from(s_2_34).unwrap());
        // D s_2_36: cmp-lt s_2_35 s_2_33
        let s_2_36: bool = ((s_2_35) < (s_2_33));
        // N s_2_37: branch s_2_36 b12 b3
        if s_2_36 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#296440 <= s_3_0
        fn_state.gs_296440 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#296440:u8
        let s_4_0: bool = fn_state.gs_296440;
        // N s_4_1: branch s_4_0 b11 b5
        if s_4_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #15s : i
        let s_5_0: i128 = 15;
        // D s_5_1: read-var n:i64
        let s_5_1: i64 = fn_state.n;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // N s_5_4: branch s_5_3 b10 b6
        if s_5_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #15s : i
        let s_6_0: i128 = 15;
        // D s_6_1: read-var m:i64
        let s_6_1: i64 = fn_state.m;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_0
        let s_6_3: bool = ((s_6_2) == (s_6_0));
        // D s_6_4: write-var gs#296443 <= s_6_3
        fn_state.gs_296443 = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#296443:u8
        let s_7_0: bool = fn_state.gs_296443;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var shift_t:u32
        let s_8_0: u32 = fn_state.shift_t;
        // D s_8_1: read-var shift_nshadow#7141:i
        let s_8_1: i128 = fn_state.shift_nshadow_7141;
        // D s_8_2: read-var m:i64
        let s_8_2: i64 = fn_state.m;
        // D s_8_3: read-var n:i64
        let s_8_3: i64 = fn_state.n;
        // D s_8_4: call execute_aarch32_instrs_CMP_r_Op_A_txt(s_8_2, s_8_3, s_8_1, s_8_0)
        let s_8_4: () = execute_aarch32_instrs_CMP_r_Op_A_txt(
            state,
            tracer,
            s_8_2,
            s_8_3,
            s_8_1,
            s_8_0,
        );
        // N s_8_5: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#296443 <= s_10_0
        fn_state.gs_296443 = s_10_0;
        // N s_10_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #8s : i
        let s_12_0: i128 = 8;
        // D s_12_1: read-var m:i64
        let s_12_1: i64 = fn_state.m;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cmp-lt s_12_2 s_12_0
        let s_12_3: bool = ((s_12_2) < (s_12_0));
        // D s_12_4: write-var gs#296440 <= s_12_3
        fn_state.gs_296440 = s_12_3;
        // N s_12_5: jump b4
        return block_4(state, tracer, fn_state);
    }
}

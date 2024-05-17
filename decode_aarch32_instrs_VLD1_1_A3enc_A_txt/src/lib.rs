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
use neq_int::*;
use ConditionPassed::*;
use execute_aarch32_instrs_VLD1_1_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VLD1_1_A3enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Rn: u8,
    Vd: u8,
    size: u8,
    index_align: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_309317: bool,
        gs_309297: bool,
        n: i64,
        index: i64,
        d: i64,
        register_index: bool,
        alignment: i64,
        wback: bool,
        ga_353265: i64,
        D: bool,
        Rn: u8,
        Vd: u8,
        size: u8,
        index_align: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        D,
        Rn,
        Vd,
        size,
        index_align,
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
        // D s_2_0: read-var size:u8
        let s_2_0: u8 = fn_state.size;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #3u : u8
        let s_2_2: u8 = 3;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b19 b3
        if s_2_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var index_align:u8
        let s_3_1: u8 = fn_state.index_align;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 4u16);
        // C s_3_3: const #1u : u64
        let s_3_3: u64 = 1;
        // D s_3_4: bit-extract s_3_2 s_3_0 s_3_3
        let s_3_4: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_3).unwrap(),
        ));
        // D s_3_5: cast reint s_3_4 -> u8
        let s_3_5: bool = ((s_3_4.value()) != 0);
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // C s_3_7: const #0u : u64
        let s_3_7: u64 = 0;
        // D s_3_8: cast zx s_3_5 -> u64
        let s_3_8: u64 = (s_3_5 as u64);
        // C s_3_9: const #1u : u64
        let s_3_9: u64 = 1;
        // D s_3_10: and s_3_8 s_3_9
        let s_3_10: u64 = ((s_3_8) & (s_3_9));
        // D s_3_11: cmp-eq s_3_10 s_3_9
        let s_3_11: bool = ((s_3_10) == (s_3_9));
        // D s_3_12: lsl s_3_8 s_3_6
        let s_3_12: u64 = s_3_8 << s_3_6;
        // D s_3_13: or s_3_7 s_3_12
        let s_3_13: u64 = ((s_3_7) | (s_3_12));
        // D s_3_14: cmpl s_3_12
        let s_3_14: u64 = !s_3_12;
        // D s_3_15: and s_3_7 s_3_14
        let s_3_15: u64 = ((s_3_7) & (s_3_14));
        // D s_3_16: select s_3_11 s_3_13 s_3_15
        let s_3_16: u64 = if s_3_11 { s_3_13 } else { s_3_15 };
        // D s_3_17: cast trunc s_3_16 -> u8
        let s_3_17: bool = ((s_3_16) != 0);
        // D s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 1u16);
        // C s_3_19: const #0u : u8
        let s_3_19: bool = false;
        // C s_3_20: cast zx s_3_19 -> bv
        let s_3_20: Bits = Bits::new(s_3_19 as u128, 1u16);
        // D s_3_21: cmp-ne s_3_18 s_3_20
        let s_3_21: bool = ((s_3_18) != (s_3_20));
        // N s_3_22: branch s_3_21 b18 b4
        if s_3_21 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i
        let s_4_0: i128 = 0;
        // D s_4_1: read-var index_align:u8
        let s_4_1: u8 = fn_state.index_align;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 4u16);
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #1s : i
        let s_4_5: i128 = 1;
        // C s_4_6: add s_4_5 s_4_4
        let s_4_6: i128 = (s_4_5 + s_4_4);
        // D s_4_7: bit-extract s_4_2 s_4_0 s_4_6
        let s_4_7: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_8: cast reint s_4_7 -> u8
        let s_4_8: u8 = (s_4_7.value() as u8);
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 2u16);
        // C s_4_10: const #0u : u8
        let s_4_10: u8 = 0;
        // C s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 2u16);
        // D s_4_12: cmp-ne s_4_9 s_4_11
        let s_4_12: bool = ((s_4_9) != (s_4_11));
        // N s_4_13: branch s_4_12 b17 b5
        if s_4_12 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#309297 <= s_5_0
        fn_state.gs_309297 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#309297:u8
        let s_6_0: bool = fn_state.gs_309297;
        // N s_6_1: branch s_6_0 b16 b7
        if s_6_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #3s : i
        let s_7_0: i128 = 3;
        // D s_7_1: read-var index_align:u8
        let s_7_1: u8 = fn_state.index_align;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 4u16);
        // C s_7_3: const #1u : u64
        let s_7_3: u64 = 1;
        // D s_7_4: bit-extract s_7_2 s_7_0 s_7_3
        let s_7_4: Bits = (Bits::new(
            ((s_7_2) >> (s_7_0)).value(),
            u16::try_from(s_7_3).unwrap(),
        ));
        // D s_7_5: cast reint s_7_4 -> u8
        let s_7_5: bool = ((s_7_4.value()) != 0);
        // C s_7_6: const #0s : i
        let s_7_6: i128 = 0;
        // C s_7_7: const #0u : u64
        let s_7_7: u64 = 0;
        // D s_7_8: cast zx s_7_5 -> u64
        let s_7_8: u64 = (s_7_5 as u64);
        // C s_7_9: const #1u : u64
        let s_7_9: u64 = 1;
        // D s_7_10: and s_7_8 s_7_9
        let s_7_10: u64 = ((s_7_8) & (s_7_9));
        // D s_7_11: cmp-eq s_7_10 s_7_9
        let s_7_11: bool = ((s_7_10) == (s_7_9));
        // D s_7_12: lsl s_7_8 s_7_6
        let s_7_12: u64 = s_7_8 << s_7_6;
        // D s_7_13: or s_7_7 s_7_12
        let s_7_13: u64 = ((s_7_7) | (s_7_12));
        // D s_7_14: cmpl s_7_12
        let s_7_14: u64 = !s_7_12;
        // D s_7_15: and s_7_7 s_7_14
        let s_7_15: u64 = ((s_7_7) & (s_7_14));
        // D s_7_16: select s_7_11 s_7_13 s_7_15
        let s_7_16: u64 = if s_7_11 { s_7_13 } else { s_7_15 };
        // D s_7_17: cast trunc s_7_16 -> u8
        let s_7_17: bool = ((s_7_16) != 0);
        // D s_7_18: cast zx s_7_17 -> bv
        let s_7_18: Bits = Bits::new(s_7_17 as u128, 1u16);
        // D s_7_19: cast zx s_7_18 -> i
        let s_7_19: i128 = (s_7_18.value() as i128);
        // D s_7_20: cast reint s_7_19 -> i64
        let s_7_20: i64 = (s_7_19 as i64);
        // D s_7_21: write-var index <= s_7_20
        fn_state.index = s_7_20;
        // C s_7_22: const #0s : i
        let s_7_22: i128 = 0;
        // D s_7_23: read-var index_align:u8
        let s_7_23: u8 = fn_state.index_align;
        // D s_7_24: cast zx s_7_23 -> bv
        let s_7_24: Bits = Bits::new(s_7_23 as u128, 4u16);
        // C s_7_25: const #1s : i64
        let s_7_25: i64 = 1;
        // C s_7_26: cast zx s_7_25 -> i
        let s_7_26: i128 = (i128::try_from(s_7_25).unwrap());
        // C s_7_27: const #1s : i
        let s_7_27: i128 = 1;
        // C s_7_28: add s_7_27 s_7_26
        let s_7_28: i128 = (s_7_27 + s_7_26);
        // D s_7_29: bit-extract s_7_24 s_7_22 s_7_28
        let s_7_29: Bits = (Bits::new(
            ((s_7_24) >> (s_7_22)).value(),
            u16::try_from(s_7_28).unwrap(),
        ));
        // D s_7_30: cast reint s_7_29 -> u8
        let s_7_30: u8 = (s_7_29.value() as u8);
        // D s_7_31: cast zx s_7_30 -> bv
        let s_7_31: Bits = Bits::new(s_7_30 as u128, 2u16);
        // C s_7_32: const #0u : u8
        let s_7_32: u8 = 0;
        // C s_7_33: cast zx s_7_32 -> bv
        let s_7_33: Bits = Bits::new(s_7_32 as u128, 2u16);
        // D s_7_34: cmp-eq s_7_31 s_7_33
        let s_7_34: bool = ((s_7_31) == (s_7_33));
        // N s_7_35: branch s_7_34 b15 b8
        if s_7_34 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #4s : i64
        let s_8_0: i64 = 4;
        // D s_8_1: write-var ga#353265 <= s_8_0
        fn_state.ga_353265 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#353265:i64
        let s_9_0: i64 = fn_state.ga_353265;
        // D s_9_1: write-var alignment <= s_9_0
        fn_state.alignment = s_9_0;
        // D s_9_2: read-var D:u8
        let s_9_2: bool = fn_state.D;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: read-var Vd:u8
        let s_9_4: u8 = fn_state.Vd;
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 4u16);
        // D s_9_6: cast reint s_9_3 -> u128
        let s_9_6: u128 = (s_9_3.value() as u128);
        // D s_9_7: size-of s_9_3
        let s_9_7: u16 = s_9_3.length();
        // D s_9_8: cast reint s_9_5 -> u128
        let s_9_8: u128 = (s_9_5.value() as u128);
        // D s_9_9: size-of s_9_5
        let s_9_9: u16 = s_9_5.length();
        // D s_9_10: lsl s_9_6 s_9_9
        let s_9_10: u128 = s_9_6 << s_9_9;
        // D s_9_11: or s_9_10 s_9_8
        let s_9_11: u128 = ((s_9_10) | (s_9_8));
        // D s_9_12: add s_9_7 s_9_9
        let s_9_12: u16 = (s_9_7 + s_9_9);
        // D s_9_13: create-bits s_9_11 s_9_12
        let s_9_13: Bits = Bits::new(s_9_11, s_9_12);
        // D s_9_14: cast reint s_9_13 -> u8
        let s_9_14: u8 = (s_9_13.value() as u8);
        // D s_9_15: cast zx s_9_14 -> bv
        let s_9_15: Bits = Bits::new(s_9_14 as u128, 5u16);
        // D s_9_16: cast zx s_9_15 -> i
        let s_9_16: i128 = (s_9_15.value() as i128);
        // D s_9_17: cast reint s_9_16 -> i64
        let s_9_17: i64 = (s_9_16 as i64);
        // D s_9_18: write-var d <= s_9_17
        fn_state.d = s_9_17;
        // D s_9_19: read-var Rn:u8
        let s_9_19: u8 = fn_state.Rn;
        // D s_9_20: cast zx s_9_19 -> bv
        let s_9_20: Bits = Bits::new(s_9_19 as u128, 4u16);
        // D s_9_21: cast zx s_9_20 -> i
        let s_9_21: i128 = (s_9_20.value() as i128);
        // D s_9_22: cast reint s_9_21 -> i64
        let s_9_22: i64 = (s_9_21 as i64);
        // D s_9_23: write-var n <= s_9_22
        fn_state.n = s_9_22;
        // D s_9_24: read-var Rm:u8
        let s_9_24: u8 = fn_state.Rm;
        // D s_9_25: cast zx s_9_24 -> bv
        let s_9_25: Bits = Bits::new(s_9_24 as u128, 4u16);
        // D s_9_26: cast zx s_9_25 -> i
        let s_9_26: i128 = (s_9_25.value() as i128);
        // D s_9_27: cast reint s_9_26 -> i64
        let s_9_27: i64 = (s_9_26 as i64);
        // D s_9_28: write-var m <= s_9_27
        fn_state.m = s_9_27;
        // C s_9_29: const #15s : i
        let s_9_29: i128 = 15;
        // D s_9_30: read-var m:i64
        let s_9_30: i64 = fn_state.m;
        // D s_9_31: cast zx s_9_30 -> i
        let s_9_31: i128 = (i128::try_from(s_9_30).unwrap());
        // D s_9_32: call neq_int(s_9_31, s_9_29)
        let s_9_32: bool = neq_int(state, tracer, s_9_31, s_9_29);
        // D s_9_33: write-var wback <= s_9_32
        fn_state.wback = s_9_32;
        // C s_9_34: const #15s : i
        let s_9_34: i128 = 15;
        // D s_9_35: read-var m:i64
        let s_9_35: i64 = fn_state.m;
        // D s_9_36: cast zx s_9_35 -> i
        let s_9_36: i128 = (i128::try_from(s_9_35).unwrap());
        // D s_9_37: call neq_int(s_9_36, s_9_34)
        let s_9_37: bool = neq_int(state, tracer, s_9_36, s_9_34);
        // N s_9_38: branch s_9_37 b14 b10
        if s_9_37 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#309317 <= s_10_0
        fn_state.gs_309317 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#309317:u8
        let s_11_0: bool = fn_state.gs_309317;
        // D s_11_1: write-var register_index <= s_11_0
        fn_state.register_index = s_11_0;
        // C s_11_2: const #15s : i
        let s_11_2: i128 = 15;
        // D s_11_3: read-var n:i64
        let s_11_3: i64 = fn_state.n;
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: cmp-eq s_11_4 s_11_2
        let s_11_5: bool = ((s_11_4) == (s_11_2));
        // N s_11_6: branch s_11_5 b13 b12
        if s_11_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var alignment:i64
        let s_12_0: i64 = fn_state.alignment;
        // D s_12_1: read-var d:i64
        let s_12_1: i64 = fn_state.d;
        // C s_12_2: const #4s : i64
        let s_12_2: i64 = 4;
        // D s_12_3: read-var index:i64
        let s_12_3: i64 = fn_state.index;
        // D s_12_4: read-var m:i64
        let s_12_4: i64 = fn_state.m;
        // D s_12_5: read-var n:i64
        let s_12_5: i64 = fn_state.n;
        // D s_12_6: read-var register_index:u8
        let s_12_6: bool = fn_state.register_index;
        // D s_12_7: read-var wback:u8
        let s_12_7: bool = fn_state.wback;
        // D s_12_8: call execute_aarch32_instrs_VLD1_1_Op_A_txt(s_12_0, s_12_1, s_12_2, s_12_3, s_12_4, s_12_5, s_12_6, s_12_7)
        let s_12_8: () = execute_aarch32_instrs_VLD1_1_Op_A_txt(
            state,
            tracer,
            s_12_0,
            s_12_1,
            s_12_2,
            s_12_3,
            s_12_4,
            s_12_5,
            s_12_6,
            s_12_7,
        );
        // N s_12_9: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #13s : i
        let s_14_0: i128 = 13;
        // D s_14_1: read-var m:i64
        let s_14_1: i64 = fn_state.m;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: call neq_int(s_14_2, s_14_0)
        let s_14_3: bool = neq_int(state, tracer, s_14_2, s_14_0);
        // D s_14_4: write-var gs#309317 <= s_14_3
        fn_state.gs_309317 = s_14_3;
        // N s_14_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1s : i64
        let s_15_0: i64 = 1;
        // D s_15_1: write-var ga#353265 <= s_15_0
        fn_state.ga_353265 = s_15_0;
        // N s_15_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0s : i
        let s_17_0: i128 = 0;
        // D s_17_1: read-var index_align:u8
        let s_17_1: u8 = fn_state.index_align;
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 4u16);
        // C s_17_3: const #1s : i64
        let s_17_3: i64 = 1;
        // C s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // C s_17_5: const #1s : i
        let s_17_5: i128 = 1;
        // C s_17_6: add s_17_5 s_17_4
        let s_17_6: i128 = (s_17_5 + s_17_4);
        // D s_17_7: bit-extract s_17_2 s_17_0 s_17_6
        let s_17_7: Bits = (Bits::new(
            ((s_17_2) >> (s_17_0)).value(),
            u16::try_from(s_17_6).unwrap(),
        ));
        // D s_17_8: cast reint s_17_7 -> u8
        let s_17_8: u8 = (s_17_7.value() as u8);
        // D s_17_9: cast zx s_17_8 -> bv
        let s_17_9: Bits = Bits::new(s_17_8 as u128, 2u16);
        // C s_17_10: const #3u : u8
        let s_17_10: u8 = 3;
        // C s_17_11: cast zx s_17_10 -> bv
        let s_17_11: Bits = Bits::new(s_17_10 as u128, 2u16);
        // D s_17_12: cmp-ne s_17_9 s_17_11
        let s_17_12: bool = ((s_17_9) != (s_17_11));
        // D s_17_13: write-var gs#309297 <= s_17_12
        fn_state.gs_309297 = s_17_12;
        // N s_17_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
}

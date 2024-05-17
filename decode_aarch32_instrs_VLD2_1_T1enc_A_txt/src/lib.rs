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
use execute_aarch32_instrs_VLD2_1_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VLD2_1_T1enc_A_txt<T: Tracer>(
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
        gs_310212: bool,
        d2: i64,
        n: i64,
        index: i64,
        d: i64,
        register_index: bool,
        alignment: i64,
        wback: bool,
        ga_353955: i64,
        gs_310209: bool,
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
        // N s_2_5: branch s_2_4 b15 b3
        if s_2_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1s : i
        let s_3_0: i128 = 1;
        // D s_3_1: read-var index_align:u8
        let s_3_1: u8 = fn_state.index_align;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 4u16);
        // C s_3_3: const #1s : i64
        let s_3_3: i64 = 1;
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: const #2s : i
        let s_3_5: i128 = 2;
        // C s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: bit-extract s_3_2 s_3_0 s_3_6
        let s_3_7: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_6).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u8
        let s_3_8: u8 = (s_3_7.value() as u8);
        // D s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 3u16);
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (s_3_9.value() as i128);
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: write-var index <= s_3_11
        fn_state.index = s_3_11;
        // C s_3_13: const #0s : i
        let s_3_13: i128 = 0;
        // D s_3_14: read-var index_align:u8
        let s_3_14: u8 = fn_state.index_align;
        // D s_3_15: cast zx s_3_14 -> bv
        let s_3_15: Bits = Bits::new(s_3_14 as u128, 4u16);
        // C s_3_16: const #1u : u64
        let s_3_16: u64 = 1;
        // D s_3_17: bit-extract s_3_15 s_3_13 s_3_16
        let s_3_17: Bits = (Bits::new(
            ((s_3_15) >> (s_3_13)).value(),
            u16::try_from(s_3_16).unwrap(),
        ));
        // D s_3_18: cast reint s_3_17 -> u8
        let s_3_18: bool = ((s_3_17.value()) != 0);
        // C s_3_19: const #0s : i
        let s_3_19: i128 = 0;
        // C s_3_20: const #0u : u64
        let s_3_20: u64 = 0;
        // D s_3_21: cast zx s_3_18 -> u64
        let s_3_21: u64 = (s_3_18 as u64);
        // C s_3_22: const #1u : u64
        let s_3_22: u64 = 1;
        // D s_3_23: and s_3_21 s_3_22
        let s_3_23: u64 = ((s_3_21) & (s_3_22));
        // D s_3_24: cmp-eq s_3_23 s_3_22
        let s_3_24: bool = ((s_3_23) == (s_3_22));
        // D s_3_25: lsl s_3_21 s_3_19
        let s_3_25: u64 = s_3_21 << s_3_19;
        // D s_3_26: or s_3_20 s_3_25
        let s_3_26: u64 = ((s_3_20) | (s_3_25));
        // D s_3_27: cmpl s_3_25
        let s_3_27: u64 = !s_3_25;
        // D s_3_28: and s_3_20 s_3_27
        let s_3_28: u64 = ((s_3_20) & (s_3_27));
        // D s_3_29: select s_3_24 s_3_26 s_3_28
        let s_3_29: u64 = if s_3_24 { s_3_26 } else { s_3_28 };
        // D s_3_30: cast trunc s_3_29 -> u8
        let s_3_30: bool = ((s_3_29) != 0);
        // D s_3_31: cast zx s_3_30 -> bv
        let s_3_31: Bits = Bits::new(s_3_30 as u128, 1u16);
        // C s_3_32: const #0u : u8
        let s_3_32: bool = false;
        // C s_3_33: cast zx s_3_32 -> bv
        let s_3_33: Bits = Bits::new(s_3_32 as u128, 1u16);
        // D s_3_34: cmp-eq s_3_31 s_3_33
        let s_3_34: bool = ((s_3_31) == (s_3_33));
        // N s_3_35: branch s_3_34 b14 b4
        if s_3_34 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2s : i64
        let s_4_0: i64 = 2;
        // D s_4_1: write-var ga#353955 <= s_4_0
        fn_state.ga_353955 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#353955:i64
        let s_5_0: i64 = fn_state.ga_353955;
        // D s_5_1: write-var alignment <= s_5_0
        fn_state.alignment = s_5_0;
        // D s_5_2: read-var D:u8
        let s_5_2: bool = fn_state.D;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: read-var Vd:u8
        let s_5_4: u8 = fn_state.Vd;
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 4u16);
        // D s_5_6: cast reint s_5_3 -> u128
        let s_5_6: u128 = (s_5_3.value() as u128);
        // D s_5_7: size-of s_5_3
        let s_5_7: u16 = s_5_3.length();
        // D s_5_8: cast reint s_5_5 -> u128
        let s_5_8: u128 = (s_5_5.value() as u128);
        // D s_5_9: size-of s_5_5
        let s_5_9: u16 = s_5_5.length();
        // D s_5_10: lsl s_5_6 s_5_9
        let s_5_10: u128 = s_5_6 << s_5_9;
        // D s_5_11: or s_5_10 s_5_8
        let s_5_11: u128 = ((s_5_10) | (s_5_8));
        // D s_5_12: add s_5_7 s_5_9
        let s_5_12: u16 = (s_5_7 + s_5_9);
        // D s_5_13: create-bits s_5_11 s_5_12
        let s_5_13: Bits = Bits::new(s_5_11, s_5_12);
        // D s_5_14: cast reint s_5_13 -> u8
        let s_5_14: u8 = (s_5_13.value() as u8);
        // D s_5_15: cast zx s_5_14 -> bv
        let s_5_15: Bits = Bits::new(s_5_14 as u128, 5u16);
        // D s_5_16: cast zx s_5_15 -> i
        let s_5_16: i128 = (s_5_15.value() as i128);
        // D s_5_17: cast reint s_5_16 -> i64
        let s_5_17: i64 = (s_5_16 as i64);
        // D s_5_18: write-var d <= s_5_17
        fn_state.d = s_5_17;
        // D s_5_19: read-var d:i64
        let s_5_19: i64 = fn_state.d;
        // D s_5_20: cast zx s_5_19 -> i
        let s_5_20: i128 = (i128::try_from(s_5_19).unwrap());
        // C s_5_21: const #1s : i64
        let s_5_21: i64 = 1;
        // C s_5_22: cast zx s_5_21 -> i
        let s_5_22: i128 = (i128::try_from(s_5_21).unwrap());
        // D s_5_23: add s_5_20 s_5_22
        let s_5_23: i128 = (s_5_20 + s_5_22);
        // D s_5_24: cast reint s_5_23 -> i64
        let s_5_24: i64 = (s_5_23 as i64);
        // D s_5_25: write-var d2 <= s_5_24
        fn_state.d2 = s_5_24;
        // D s_5_26: read-var Rn:u8
        let s_5_26: u8 = fn_state.Rn;
        // D s_5_27: cast zx s_5_26 -> bv
        let s_5_27: Bits = Bits::new(s_5_26 as u128, 4u16);
        // D s_5_28: cast zx s_5_27 -> i
        let s_5_28: i128 = (s_5_27.value() as i128);
        // D s_5_29: cast reint s_5_28 -> i64
        let s_5_29: i64 = (s_5_28 as i64);
        // D s_5_30: write-var n <= s_5_29
        fn_state.n = s_5_29;
        // D s_5_31: read-var Rm:u8
        let s_5_31: u8 = fn_state.Rm;
        // D s_5_32: cast zx s_5_31 -> bv
        let s_5_32: Bits = Bits::new(s_5_31 as u128, 4u16);
        // D s_5_33: cast zx s_5_32 -> i
        let s_5_33: i128 = (s_5_32.value() as i128);
        // D s_5_34: cast reint s_5_33 -> i64
        let s_5_34: i64 = (s_5_33 as i64);
        // D s_5_35: write-var m <= s_5_34
        fn_state.m = s_5_34;
        // C s_5_36: const #15s : i
        let s_5_36: i128 = 15;
        // D s_5_37: read-var m:i64
        let s_5_37: i64 = fn_state.m;
        // D s_5_38: cast zx s_5_37 -> i
        let s_5_38: i128 = (i128::try_from(s_5_37).unwrap());
        // D s_5_39: call neq_int(s_5_38, s_5_36)
        let s_5_39: bool = neq_int(state, tracer, s_5_38, s_5_36);
        // D s_5_40: write-var wback <= s_5_39
        fn_state.wback = s_5_39;
        // C s_5_41: const #15s : i
        let s_5_41: i128 = 15;
        // D s_5_42: read-var m:i64
        let s_5_42: i64 = fn_state.m;
        // D s_5_43: cast zx s_5_42 -> i
        let s_5_43: i128 = (i128::try_from(s_5_42).unwrap());
        // D s_5_44: call neq_int(s_5_43, s_5_41)
        let s_5_44: bool = neq_int(state, tracer, s_5_43, s_5_41);
        // N s_5_45: branch s_5_44 b13 b6
        if s_5_44 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#310209 <= s_6_0
        fn_state.gs_310209 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#310209:u8
        let s_7_0: bool = fn_state.gs_310209;
        // D s_7_1: write-var register_index <= s_7_0
        fn_state.register_index = s_7_0;
        // C s_7_2: const #15s : i
        let s_7_2: i128 = 15;
        // D s_7_3: read-var n:i64
        let s_7_3: i64 = fn_state.n;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cmp-eq s_7_4 s_7_2
        let s_7_5: bool = ((s_7_4) == (s_7_2));
        // N s_7_6: branch s_7_5 b12 b8
        if s_7_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #31s : i
        let s_8_0: i128 = 31;
        // D s_8_1: read-var d2:i64
        let s_8_1: i64 = fn_state.d2;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cmp-gt s_8_2 s_8_0
        let s_8_3: bool = ((s_8_2) > (s_8_0));
        // D s_8_4: write-var gs#310212 <= s_8_3
        fn_state.gs_310212 = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#310212:u8
        let s_9_0: bool = fn_state.gs_310212;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var d2:i64
        let s_10_0: i64 = fn_state.d2;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var alignment:i64
        let s_10_2: i64 = fn_state.alignment;
        // D s_10_3: read-var d:i64
        let s_10_3: i64 = fn_state.d;
        // C s_10_4: const #1s : i64
        let s_10_4: i64 = 1;
        // D s_10_5: read-var index:i64
        let s_10_5: i64 = fn_state.index;
        // D s_10_6: read-var m:i64
        let s_10_6: i64 = fn_state.m;
        // D s_10_7: read-var n:i64
        let s_10_7: i64 = fn_state.n;
        // D s_10_8: read-var register_index:u8
        let s_10_8: bool = fn_state.register_index;
        // D s_10_9: read-var wback:u8
        let s_10_9: bool = fn_state.wback;
        // D s_10_10: call execute_aarch32_instrs_VLD2_1_Op_A_txt(s_10_2, s_10_3, s_10_1, s_10_4, s_10_5, s_10_6, s_10_7, s_10_8, s_10_9)
        let s_10_10: () = execute_aarch32_instrs_VLD2_1_Op_A_txt(
            state,
            tracer,
            s_10_2,
            s_10_3,
            s_10_1,
            s_10_4,
            s_10_5,
            s_10_6,
            s_10_7,
            s_10_8,
            s_10_9,
        );
        // N s_10_11: return
        return;
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
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#310212 <= s_12_0
        fn_state.gs_310212 = s_12_0;
        // N s_12_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #13s : i
        let s_13_0: i128 = 13;
        // D s_13_1: read-var m:i64
        let s_13_1: i64 = fn_state.m;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: call neq_int(s_13_2, s_13_0)
        let s_13_3: bool = neq_int(state, tracer, s_13_2, s_13_0);
        // D s_13_4: write-var gs#310209 <= s_13_3
        fn_state.gs_310209 = s_13_3;
        // N s_13_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1s : i64
        let s_14_0: i64 = 1;
        // D s_14_1: write-var ga#353955 <= s_14_0
        fn_state.ga_353955 = s_14_0;
        // N s_14_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
}

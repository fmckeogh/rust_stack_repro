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
pub fn decode_aarch32_instrs_VLD1_1_A2enc_A_txt<T: Tracer>(
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
        gs_309281: bool,
        index: i64,
        n: i64,
        ga_353240: i64,
        d: i64,
        register_index: bool,
        alignment: i64,
        wback: bool,
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
        // N s_2_5: branch s_2_4 b14 b3
        if s_2_4 {
            return block_14(state, tracer, fn_state);
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
        // N s_3_22: branch s_3_21 b13 b4
        if s_3_21 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2s : i
        let s_4_0: i128 = 2;
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
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (s_4_9.value() as i128);
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // D s_4_12: write-var index <= s_4_11
        fn_state.index = s_4_11;
        // C s_4_13: const #0s : i
        let s_4_13: i128 = 0;
        // D s_4_14: read-var index_align:u8
        let s_4_14: u8 = fn_state.index_align;
        // D s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 4u16);
        // C s_4_16: const #1u : u64
        let s_4_16: u64 = 1;
        // D s_4_17: bit-extract s_4_15 s_4_13 s_4_16
        let s_4_17: Bits = (Bits::new(
            ((s_4_15) >> (s_4_13)).value(),
            u16::try_from(s_4_16).unwrap(),
        ));
        // D s_4_18: cast reint s_4_17 -> u8
        let s_4_18: bool = ((s_4_17.value()) != 0);
        // C s_4_19: const #0s : i
        let s_4_19: i128 = 0;
        // C s_4_20: const #0u : u64
        let s_4_20: u64 = 0;
        // D s_4_21: cast zx s_4_18 -> u64
        let s_4_21: u64 = (s_4_18 as u64);
        // C s_4_22: const #1u : u64
        let s_4_22: u64 = 1;
        // D s_4_23: and s_4_21 s_4_22
        let s_4_23: u64 = ((s_4_21) & (s_4_22));
        // D s_4_24: cmp-eq s_4_23 s_4_22
        let s_4_24: bool = ((s_4_23) == (s_4_22));
        // D s_4_25: lsl s_4_21 s_4_19
        let s_4_25: u64 = s_4_21 << s_4_19;
        // D s_4_26: or s_4_20 s_4_25
        let s_4_26: u64 = ((s_4_20) | (s_4_25));
        // D s_4_27: cmpl s_4_25
        let s_4_27: u64 = !s_4_25;
        // D s_4_28: and s_4_20 s_4_27
        let s_4_28: u64 = ((s_4_20) & (s_4_27));
        // D s_4_29: select s_4_24 s_4_26 s_4_28
        let s_4_29: u64 = if s_4_24 { s_4_26 } else { s_4_28 };
        // D s_4_30: cast trunc s_4_29 -> u8
        let s_4_30: bool = ((s_4_29) != 0);
        // D s_4_31: cast zx s_4_30 -> bv
        let s_4_31: Bits = Bits::new(s_4_30 as u128, 1u16);
        // C s_4_32: const #0u : u8
        let s_4_32: bool = false;
        // C s_4_33: cast zx s_4_32 -> bv
        let s_4_33: Bits = Bits::new(s_4_32 as u128, 1u16);
        // D s_4_34: cmp-eq s_4_31 s_4_33
        let s_4_34: bool = ((s_4_31) == (s_4_33));
        // N s_4_35: branch s_4_34 b12 b5
        if s_4_34 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #2s : i64
        let s_5_0: i64 = 2;
        // D s_5_1: write-var ga#353240 <= s_5_0
        fn_state.ga_353240 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#353240:i64
        let s_6_0: i64 = fn_state.ga_353240;
        // D s_6_1: write-var alignment <= s_6_0
        fn_state.alignment = s_6_0;
        // D s_6_2: read-var D:u8
        let s_6_2: bool = fn_state.D;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: read-var Vd:u8
        let s_6_4: u8 = fn_state.Vd;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 4u16);
        // D s_6_6: cast reint s_6_3 -> u128
        let s_6_6: u128 = (s_6_3.value() as u128);
        // D s_6_7: size-of s_6_3
        let s_6_7: u16 = s_6_3.length();
        // D s_6_8: cast reint s_6_5 -> u128
        let s_6_8: u128 = (s_6_5.value() as u128);
        // D s_6_9: size-of s_6_5
        let s_6_9: u16 = s_6_5.length();
        // D s_6_10: lsl s_6_6 s_6_9
        let s_6_10: u128 = s_6_6 << s_6_9;
        // D s_6_11: or s_6_10 s_6_8
        let s_6_11: u128 = ((s_6_10) | (s_6_8));
        // D s_6_12: add s_6_7 s_6_9
        let s_6_12: u16 = (s_6_7 + s_6_9);
        // D s_6_13: create-bits s_6_11 s_6_12
        let s_6_13: Bits = Bits::new(s_6_11, s_6_12);
        // D s_6_14: cast reint s_6_13 -> u8
        let s_6_14: u8 = (s_6_13.value() as u8);
        // D s_6_15: cast zx s_6_14 -> bv
        let s_6_15: Bits = Bits::new(s_6_14 as u128, 5u16);
        // D s_6_16: cast zx s_6_15 -> i
        let s_6_16: i128 = (s_6_15.value() as i128);
        // D s_6_17: cast reint s_6_16 -> i64
        let s_6_17: i64 = (s_6_16 as i64);
        // D s_6_18: write-var d <= s_6_17
        fn_state.d = s_6_17;
        // D s_6_19: read-var Rn:u8
        let s_6_19: u8 = fn_state.Rn;
        // D s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 4u16);
        // D s_6_21: cast zx s_6_20 -> i
        let s_6_21: i128 = (s_6_20.value() as i128);
        // D s_6_22: cast reint s_6_21 -> i64
        let s_6_22: i64 = (s_6_21 as i64);
        // D s_6_23: write-var n <= s_6_22
        fn_state.n = s_6_22;
        // D s_6_24: read-var Rm:u8
        let s_6_24: u8 = fn_state.Rm;
        // D s_6_25: cast zx s_6_24 -> bv
        let s_6_25: Bits = Bits::new(s_6_24 as u128, 4u16);
        // D s_6_26: cast zx s_6_25 -> i
        let s_6_26: i128 = (s_6_25.value() as i128);
        // D s_6_27: cast reint s_6_26 -> i64
        let s_6_27: i64 = (s_6_26 as i64);
        // D s_6_28: write-var m <= s_6_27
        fn_state.m = s_6_27;
        // C s_6_29: const #15s : i
        let s_6_29: i128 = 15;
        // D s_6_30: read-var m:i64
        let s_6_30: i64 = fn_state.m;
        // D s_6_31: cast zx s_6_30 -> i
        let s_6_31: i128 = (i128::try_from(s_6_30).unwrap());
        // D s_6_32: call neq_int(s_6_31, s_6_29)
        let s_6_32: bool = neq_int(state, tracer, s_6_31, s_6_29);
        // D s_6_33: write-var wback <= s_6_32
        fn_state.wback = s_6_32;
        // C s_6_34: const #15s : i
        let s_6_34: i128 = 15;
        // D s_6_35: read-var m:i64
        let s_6_35: i64 = fn_state.m;
        // D s_6_36: cast zx s_6_35 -> i
        let s_6_36: i128 = (i128::try_from(s_6_35).unwrap());
        // D s_6_37: call neq_int(s_6_36, s_6_34)
        let s_6_37: bool = neq_int(state, tracer, s_6_36, s_6_34);
        // N s_6_38: branch s_6_37 b11 b7
        if s_6_37 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#309281 <= s_7_0
        fn_state.gs_309281 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#309281:u8
        let s_8_0: bool = fn_state.gs_309281;
        // D s_8_1: write-var register_index <= s_8_0
        fn_state.register_index = s_8_0;
        // C s_8_2: const #15s : i
        let s_8_2: i128 = 15;
        // D s_8_3: read-var n:i64
        let s_8_3: i64 = fn_state.n;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: cmp-eq s_8_4 s_8_2
        let s_8_5: bool = ((s_8_4) == (s_8_2));
        // N s_8_6: branch s_8_5 b10 b9
        if s_8_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var alignment:i64
        let s_9_0: i64 = fn_state.alignment;
        // D s_9_1: read-var d:i64
        let s_9_1: i64 = fn_state.d;
        // C s_9_2: const #2s : i64
        let s_9_2: i64 = 2;
        // D s_9_3: read-var index:i64
        let s_9_3: i64 = fn_state.index;
        // D s_9_4: read-var m:i64
        let s_9_4: i64 = fn_state.m;
        // D s_9_5: read-var n:i64
        let s_9_5: i64 = fn_state.n;
        // D s_9_6: read-var register_index:u8
        let s_9_6: bool = fn_state.register_index;
        // D s_9_7: read-var wback:u8
        let s_9_7: bool = fn_state.wback;
        // D s_9_8: call execute_aarch32_instrs_VLD1_1_Op_A_txt(s_9_0, s_9_1, s_9_2, s_9_3, s_9_4, s_9_5, s_9_6, s_9_7)
        let s_9_8: () = execute_aarch32_instrs_VLD1_1_Op_A_txt(
            state,
            tracer,
            s_9_0,
            s_9_1,
            s_9_2,
            s_9_3,
            s_9_4,
            s_9_5,
            s_9_6,
            s_9_7,
        );
        // N s_9_9: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #13s : i
        let s_11_0: i128 = 13;
        // D s_11_1: read-var m:i64
        let s_11_1: i64 = fn_state.m;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: call neq_int(s_11_2, s_11_0)
        let s_11_3: bool = neq_int(state, tracer, s_11_2, s_11_0);
        // D s_11_4: write-var gs#309281 <= s_11_3
        fn_state.gs_309281 = s_11_3;
        // N s_11_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1s : i64
        let s_12_0: i64 = 1;
        // D s_12_1: write-var ga#353240 <= s_12_0
        fn_state.ga_353240 = s_12_0;
        // N s_12_2: jump b6
        return block_6(state, tracer, fn_state);
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
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
}

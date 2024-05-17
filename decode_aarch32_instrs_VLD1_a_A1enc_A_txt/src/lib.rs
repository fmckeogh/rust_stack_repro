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
use execute_aarch32_instrs_VLD1_a_Op_A_txt::*;
use ConditionPassed::*;
use common::*;
pub fn decode_aarch32_instrs_VLD1_a_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Rn: u8,
    Vd: u8,
    size: u8,
    T: bool,
    a: bool,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_309454: bool,
        ebytes: i64,
        ga_353355: i64,
        regs: i64,
        gs_309436: bool,
        n: i64,
        d: i64,
        register_index: bool,
        gs_309451: bool,
        alignment: i64,
        wback: bool,
        ga_353353: i64,
        gs_309435: bool,
        D: bool,
        Rn: u8,
        Vd: u8,
        size: u8,
        T: bool,
        a: bool,
        Rm: u8,
    }
    let fn_state = FunctionState {
        D,
        Rn,
        Vd,
        size,
        T,
        a,
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
        // N s_2_5: branch s_2_4 b24 b3
        if s_2_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var size:u8
        let s_3_0: u8 = fn_state.size;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #0u : u8
        let s_3_2: u8 = 0;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b23 b4
        if s_3_4 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#309435 <= s_4_0
        fn_state.gs_309435 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#309435:u8
        let s_5_0: bool = fn_state.gs_309435;
        // D s_5_1: write-var gs#309436 <= s_5_0
        fn_state.gs_309436 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#309436:u8
        let s_6_0: bool = fn_state.gs_309436;
        // N s_6_1: branch s_6_0 b22 b7
        if s_6_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var size:u8
        let s_7_0: u8 = fn_state.size;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (s_7_1.value() as i128);
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: const #1s : i64
        let s_7_4: i64 = 1;
        // D s_7_5: lsl s_7_4 s_7_3
        let s_7_5: i64 = s_7_4 << s_7_3;
        // D s_7_6: write-var ebytes <= s_7_5
        fn_state.ebytes = s_7_5;
        // D s_7_7: read-var T:u8
        let s_7_7: bool = fn_state.T;
        // D s_7_8: cast zx s_7_7 -> bv
        let s_7_8: Bits = Bits::new(s_7_7 as u128, 1u16);
        // C s_7_9: const #0u : u8
        let s_7_9: bool = false;
        // C s_7_10: cast zx s_7_9 -> bv
        let s_7_10: Bits = Bits::new(s_7_9 as u128, 1u16);
        // D s_7_11: cmp-eq s_7_8 s_7_10
        let s_7_11: bool = ((s_7_8) == (s_7_10));
        // N s_7_12: branch s_7_11 b21 b8
        if s_7_11 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #2s : i64
        let s_8_0: i64 = 2;
        // D s_8_1: write-var ga#353353 <= s_8_0
        fn_state.ga_353353 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#353353:i64
        let s_9_0: i64 = fn_state.ga_353353;
        // D s_9_1: write-var regs <= s_9_0
        fn_state.regs = s_9_0;
        // D s_9_2: read-var a:u8
        let s_9_2: bool = fn_state.a;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // C s_9_4: const #0u : u8
        let s_9_4: bool = false;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // D s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // N s_9_7: branch s_9_6 b20 b10
        if s_9_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ebytes:i64
        let s_10_0: i64 = fn_state.ebytes;
        // D s_10_1: write-var ga#353355 <= s_10_0
        fn_state.ga_353355 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#353355:i64
        let s_11_0: i64 = fn_state.ga_353355;
        // D s_11_1: write-var alignment <= s_11_0
        fn_state.alignment = s_11_0;
        // D s_11_2: read-var D:u8
        let s_11_2: bool = fn_state.D;
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: read-var Vd:u8
        let s_11_4: u8 = fn_state.Vd;
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 4u16);
        // D s_11_6: cast reint s_11_3 -> u128
        let s_11_6: u128 = (s_11_3.value() as u128);
        // D s_11_7: size-of s_11_3
        let s_11_7: u16 = s_11_3.length();
        // D s_11_8: cast reint s_11_5 -> u128
        let s_11_8: u128 = (s_11_5.value() as u128);
        // D s_11_9: size-of s_11_5
        let s_11_9: u16 = s_11_5.length();
        // D s_11_10: lsl s_11_6 s_11_9
        let s_11_10: u128 = s_11_6 << s_11_9;
        // D s_11_11: or s_11_10 s_11_8
        let s_11_11: u128 = ((s_11_10) | (s_11_8));
        // D s_11_12: add s_11_7 s_11_9
        let s_11_12: u16 = (s_11_7 + s_11_9);
        // D s_11_13: create-bits s_11_11 s_11_12
        let s_11_13: Bits = Bits::new(s_11_11, s_11_12);
        // D s_11_14: cast reint s_11_13 -> u8
        let s_11_14: u8 = (s_11_13.value() as u8);
        // D s_11_15: cast zx s_11_14 -> bv
        let s_11_15: Bits = Bits::new(s_11_14 as u128, 5u16);
        // D s_11_16: cast zx s_11_15 -> i
        let s_11_16: i128 = (s_11_15.value() as i128);
        // D s_11_17: cast reint s_11_16 -> i64
        let s_11_17: i64 = (s_11_16 as i64);
        // D s_11_18: write-var d <= s_11_17
        fn_state.d = s_11_17;
        // D s_11_19: read-var Rn:u8
        let s_11_19: u8 = fn_state.Rn;
        // D s_11_20: cast zx s_11_19 -> bv
        let s_11_20: Bits = Bits::new(s_11_19 as u128, 4u16);
        // D s_11_21: cast zx s_11_20 -> i
        let s_11_21: i128 = (s_11_20.value() as i128);
        // D s_11_22: cast reint s_11_21 -> i64
        let s_11_22: i64 = (s_11_21 as i64);
        // D s_11_23: write-var n <= s_11_22
        fn_state.n = s_11_22;
        // D s_11_24: read-var Rm:u8
        let s_11_24: u8 = fn_state.Rm;
        // D s_11_25: cast zx s_11_24 -> bv
        let s_11_25: Bits = Bits::new(s_11_24 as u128, 4u16);
        // D s_11_26: cast zx s_11_25 -> i
        let s_11_26: i128 = (s_11_25.value() as i128);
        // D s_11_27: cast reint s_11_26 -> i64
        let s_11_27: i64 = (s_11_26 as i64);
        // D s_11_28: write-var m <= s_11_27
        fn_state.m = s_11_27;
        // C s_11_29: const #15s : i
        let s_11_29: i128 = 15;
        // D s_11_30: read-var m:i64
        let s_11_30: i64 = fn_state.m;
        // D s_11_31: cast zx s_11_30 -> i
        let s_11_31: i128 = (i128::try_from(s_11_30).unwrap());
        // D s_11_32: call neq_int(s_11_31, s_11_29)
        let s_11_32: bool = neq_int(state, tracer, s_11_31, s_11_29);
        // D s_11_33: write-var wback <= s_11_32
        fn_state.wback = s_11_32;
        // C s_11_34: const #15s : i
        let s_11_34: i128 = 15;
        // D s_11_35: read-var m:i64
        let s_11_35: i64 = fn_state.m;
        // D s_11_36: cast zx s_11_35 -> i
        let s_11_36: i128 = (i128::try_from(s_11_35).unwrap());
        // D s_11_37: call neq_int(s_11_36, s_11_34)
        let s_11_37: bool = neq_int(state, tracer, s_11_36, s_11_34);
        // N s_11_38: branch s_11_37 b19 b12
        if s_11_37 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#309451 <= s_12_0
        fn_state.gs_309451 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#309451:u8
        let s_13_0: bool = fn_state.gs_309451;
        // D s_13_1: write-var register_index <= s_13_0
        fn_state.register_index = s_13_0;
        // C s_13_2: const #15s : i
        let s_13_2: i128 = 15;
        // D s_13_3: read-var n:i64
        let s_13_3: i64 = fn_state.n;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: cmp-eq s_13_4 s_13_2
        let s_13_5: bool = ((s_13_4) == (s_13_2));
        // N s_13_6: branch s_13_5 b18 b14
        if s_13_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var d:i64
        let s_14_0: i64 = fn_state.d;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: read-var regs:i64
        let s_14_2: i64 = fn_state.regs;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: add s_14_1 s_14_3
        let s_14_4: i128 = (s_14_1 + s_14_3);
        // D s_14_5: cast reint s_14_4 -> i64
        let s_14_5: i64 = (s_14_4 as i64);
        // C s_14_6: const #32s : i
        let s_14_6: i128 = 32;
        // D s_14_7: cast zx s_14_5 -> i
        let s_14_7: i128 = (i128::try_from(s_14_5).unwrap());
        // D s_14_8: cmp-gt s_14_7 s_14_6
        let s_14_8: bool = ((s_14_7) > (s_14_6));
        // D s_14_9: write-var gs#309454 <= s_14_8
        fn_state.gs_309454 = s_14_8;
        // N s_14_10: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#309454:u8
        let s_15_0: bool = fn_state.gs_309454;
        // N s_15_1: branch s_15_0 b17 b16
        if s_15_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var alignment:i64
        let s_16_0: i64 = fn_state.alignment;
        // D s_16_1: read-var d:i64
        let s_16_1: i64 = fn_state.d;
        // D s_16_2: read-var ebytes:i64
        let s_16_2: i64 = fn_state.ebytes;
        // D s_16_3: read-var m:i64
        let s_16_3: i64 = fn_state.m;
        // D s_16_4: read-var n:i64
        let s_16_4: i64 = fn_state.n;
        // D s_16_5: read-var register_index:u8
        let s_16_5: bool = fn_state.register_index;
        // D s_16_6: read-var regs:i64
        let s_16_6: i64 = fn_state.regs;
        // D s_16_7: read-var wback:u8
        let s_16_7: bool = fn_state.wback;
        // D s_16_8: call execute_aarch32_instrs_VLD1_a_Op_A_txt(s_16_0, s_16_1, s_16_2, s_16_3, s_16_4, s_16_5, s_16_6, s_16_7)
        let s_16_8: () = execute_aarch32_instrs_VLD1_a_Op_A_txt(
            state,
            tracer,
            s_16_0,
            s_16_1,
            s_16_2,
            s_16_3,
            s_16_4,
            s_16_5,
            s_16_6,
            s_16_7,
        );
        // N s_16_9: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#309454 <= s_18_0
        fn_state.gs_309454 = s_18_0;
        // N s_18_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #13s : i
        let s_19_0: i128 = 13;
        // D s_19_1: read-var m:i64
        let s_19_1: i64 = fn_state.m;
        // D s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (i128::try_from(s_19_1).unwrap());
        // D s_19_3: call neq_int(s_19_2, s_19_0)
        let s_19_3: bool = neq_int(state, tracer, s_19_2, s_19_0);
        // D s_19_4: write-var gs#309451 <= s_19_3
        fn_state.gs_309451 = s_19_3;
        // N s_19_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1s : i64
        let s_20_0: i64 = 1;
        // D s_20_1: write-var ga#353355 <= s_20_0
        fn_state.ga_353355 = s_20_0;
        // N s_20_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1s : i64
        let s_21_0: i64 = 1;
        // D s_21_1: write-var ga#353353 <= s_21_0
        fn_state.ga_353353 = s_21_0;
        // N s_21_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var a:u8
        let s_23_0: bool = fn_state.a;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 1u16);
        // C s_23_2: const #1u : u8
        let s_23_2: bool = true;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: write-var gs#309435 <= s_23_4
        fn_state.gs_309435 = s_23_4;
        // N s_23_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#309436 <= s_24_0
        fn_state.gs_309436 = s_24_0;
        // N s_24_2: jump b6
        return block_6(state, tracer, fn_state);
    }
}

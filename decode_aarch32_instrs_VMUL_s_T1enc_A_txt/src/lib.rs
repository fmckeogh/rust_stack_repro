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
use execute_aarch32_instrs_VMUL_s_Op_A_txt::*;
use ConditionPassed::*;
use u__id::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_VMUL_s_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Q: bool,
    D: bool,
    size: u8,
    Vn: u8,
    Vd: u8,
    F: bool,
    N: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_357043: i64,
        gs_314048: bool,
        gs_314072: bool,
        gs_314073: bool,
        gs_314070: bool,
        gs_314062: bool,
        gs_314074: bool,
        gs_314069: bool,
        elements: i64,
        gs_314065: bool,
        mshadow_7598: i128,
        gs_314071: bool,
        gs_314075: bool,
        floating_point: bool,
        m: i128,
        gs_314068: bool,
        gs_314059: bool,
        esize: i64,
        n: i64,
        index: i128,
        gs_314014: bool,
        gs_314006: bool,
        indexshadow_7599: i128,
        gs_314055: bool,
        gs_314076: bool,
        esizeshadow_7600: i64,
        regs: i64,
        gs_314007: bool,
        gs_314008: bool,
        gs_314013: bool,
        elementsshadow_7601: i64,
        gs_314057: bool,
        d: i64,
        Q: bool,
        D: bool,
        size: u8,
        Vn: u8,
        Vd: u8,
        F: bool,
        N: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        Q,
        D,
        size,
        Vn,
        Vd,
        F,
        N,
        M,
        Vm,
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
        // N s_2_5: branch s_2_4 b79 b3
        if s_2_4 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var F:u8
        let s_3_0: bool = fn_state.F;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b78 b4
        if s_3_4 {
            return block_78(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#314006 <= s_4_0
        fn_state.gs_314006 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#314006:u8
        let s_5_0: bool = fn_state.gs_314006;
        // N s_5_1: branch s_5_0 b77 b6
        if s_5_0 {
            return block_77(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#314007 <= s_6_0
        fn_state.gs_314007 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#314007:u8
        let s_7_0: bool = fn_state.gs_314007;
        // N s_7_1: branch s_7_0 b76 b8
        if s_7_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var size:u8
        let s_8_0: u8 = fn_state.size;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #0u : u8
        let s_8_2: u8 = 0;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b75 b9
        if s_8_4 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#314008 <= s_9_0
        fn_state.gs_314008 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#314008:u8
        let s_10_0: bool = fn_state.gs_314008;
        // N s_10_1: branch s_10_0 b74 b11
        if s_10_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var Q:u8
        let s_11_0: bool = fn_state.Q;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #1u : u8
        let s_11_2: bool = true;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // N s_11_5: branch s_11_4 b70 b12
        if s_11_4 {
            return block_70(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#314014 <= s_12_0
        fn_state.gs_314014 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#314014:u8
        let s_13_0: bool = fn_state.gs_314014;
        // N s_13_1: branch s_13_0 b69 b14
        if s_13_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var F:u8
        let s_14_0: bool = fn_state.F;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: write-var floating_point <= s_14_4
        fn_state.floating_point = s_14_4;
        // D s_14_6: read-var D:u8
        let s_14_6: bool = fn_state.D;
        // D s_14_7: cast zx s_14_6 -> bv
        let s_14_7: Bits = Bits::new(s_14_6 as u128, 1u16);
        // D s_14_8: read-var Vd:u8
        let s_14_8: u8 = fn_state.Vd;
        // D s_14_9: cast zx s_14_8 -> bv
        let s_14_9: Bits = Bits::new(s_14_8 as u128, 4u16);
        // D s_14_10: cast reint s_14_7 -> u128
        let s_14_10: u128 = (s_14_7.value() as u128);
        // D s_14_11: size-of s_14_7
        let s_14_11: u16 = s_14_7.length();
        // D s_14_12: cast reint s_14_9 -> u128
        let s_14_12: u128 = (s_14_9.value() as u128);
        // D s_14_13: size-of s_14_9
        let s_14_13: u16 = s_14_9.length();
        // D s_14_14: lsl s_14_10 s_14_13
        let s_14_14: u128 = s_14_10 << s_14_13;
        // D s_14_15: or s_14_14 s_14_12
        let s_14_15: u128 = ((s_14_14) | (s_14_12));
        // D s_14_16: add s_14_11 s_14_13
        let s_14_16: u16 = (s_14_11 + s_14_13);
        // D s_14_17: create-bits s_14_15 s_14_16
        let s_14_17: Bits = Bits::new(s_14_15, s_14_16);
        // D s_14_18: cast reint s_14_17 -> u8
        let s_14_18: u8 = (s_14_17.value() as u8);
        // D s_14_19: cast zx s_14_18 -> bv
        let s_14_19: Bits = Bits::new(s_14_18 as u128, 5u16);
        // D s_14_20: cast zx s_14_19 -> i
        let s_14_20: i128 = (s_14_19.value() as i128);
        // D s_14_21: cast reint s_14_20 -> i64
        let s_14_21: i64 = (s_14_20 as i64);
        // D s_14_22: write-var d <= s_14_21
        fn_state.d = s_14_21;
        // D s_14_23: read-var N:u8
        let s_14_23: bool = fn_state.N;
        // D s_14_24: cast zx s_14_23 -> bv
        let s_14_24: Bits = Bits::new(s_14_23 as u128, 1u16);
        // D s_14_25: read-var Vn:u8
        let s_14_25: u8 = fn_state.Vn;
        // D s_14_26: cast zx s_14_25 -> bv
        let s_14_26: Bits = Bits::new(s_14_25 as u128, 4u16);
        // D s_14_27: cast reint s_14_24 -> u128
        let s_14_27: u128 = (s_14_24.value() as u128);
        // D s_14_28: size-of s_14_24
        let s_14_28: u16 = s_14_24.length();
        // D s_14_29: cast reint s_14_26 -> u128
        let s_14_29: u128 = (s_14_26.value() as u128);
        // D s_14_30: size-of s_14_26
        let s_14_30: u16 = s_14_26.length();
        // D s_14_31: lsl s_14_27 s_14_30
        let s_14_31: u128 = s_14_27 << s_14_30;
        // D s_14_32: or s_14_31 s_14_29
        let s_14_32: u128 = ((s_14_31) | (s_14_29));
        // D s_14_33: add s_14_28 s_14_30
        let s_14_33: u16 = (s_14_28 + s_14_30);
        // D s_14_34: create-bits s_14_32 s_14_33
        let s_14_34: Bits = Bits::new(s_14_32, s_14_33);
        // D s_14_35: cast reint s_14_34 -> u8
        let s_14_35: u8 = (s_14_34.value() as u8);
        // D s_14_36: cast zx s_14_35 -> bv
        let s_14_36: Bits = Bits::new(s_14_35 as u128, 5u16);
        // D s_14_37: cast zx s_14_36 -> i
        let s_14_37: i128 = (s_14_36.value() as i128);
        // D s_14_38: cast reint s_14_37 -> i64
        let s_14_38: i64 = (s_14_37 as i64);
        // D s_14_39: write-var n <= s_14_38
        fn_state.n = s_14_38;
        // D s_14_40: read-var Q:u8
        let s_14_40: bool = fn_state.Q;
        // D s_14_41: cast zx s_14_40 -> bv
        let s_14_41: Bits = Bits::new(s_14_40 as u128, 1u16);
        // C s_14_42: const #0u : u8
        let s_14_42: bool = false;
        // C s_14_43: cast zx s_14_42 -> bv
        let s_14_43: Bits = Bits::new(s_14_42 as u128, 1u16);
        // D s_14_44: cmp-eq s_14_41 s_14_43
        let s_14_44: bool = ((s_14_41) == (s_14_43));
        // N s_14_45: branch s_14_44 b68 b15
        if s_14_44 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #2s : i64
        let s_15_0: i64 = 2;
        // D s_15_1: write-var ga#357043 <= s_15_0
        fn_state.ga_357043 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var ga#357043:i64
        let s_16_0: i64 = fn_state.ga_357043;
        // D s_16_1: write-var regs <= s_16_0
        fn_state.regs = s_16_0;
        // C s_16_2: const #16s : i64
        let s_16_2: i64 = 16;
        // D s_16_3: write-var esize <= s_16_2
        fn_state.esize = s_16_2;
        // C s_16_4: const #2s : i64
        let s_16_4: i64 = 2;
        // D s_16_5: write-var elements <= s_16_4
        fn_state.elements = s_16_4;
        // D s_16_6: read-var size:u8
        let s_16_6: u8 = fn_state.size;
        // D s_16_7: cast zx s_16_6 -> bv
        let s_16_7: Bits = Bits::new(s_16_6 as u128, 2u16);
        // C s_16_8: const #1u : u8
        let s_16_8: u8 = 1;
        // C s_16_9: cast zx s_16_8 -> bv
        let s_16_9: Bits = Bits::new(s_16_8 as u128, 2u16);
        // D s_16_10: cmp-eq s_16_7 s_16_9
        let s_16_10: bool = ((s_16_7) == (s_16_9));
        // N s_16_11: branch s_16_10 b67 b17
        if s_16_10 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var size:u8
        let s_18_0: u8 = fn_state.size;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 2u16);
        // C s_18_2: const #2u : u8
        let s_18_2: u8 = 2;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 2u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // N s_18_5: branch s_18_4 b66 b19
        if s_18_4 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var m:i
        let s_20_0: i128 = fn_state.m;
        // D s_20_1: write-var mshadow#7598 <= s_20_0
        fn_state.mshadow_7598 = s_20_0;
        // D s_20_2: read-var index:i
        let s_20_2: i128 = fn_state.index;
        // D s_20_3: write-var indexshadow#7599 <= s_20_2
        fn_state.indexshadow_7599 = s_20_2;
        // D s_20_4: read-var esize:i64
        let s_20_4: i64 = fn_state.esize;
        // D s_20_5: write-var esizeshadow#7600 <= s_20_4
        fn_state.esizeshadow_7600 = s_20_4;
        // D s_20_6: read-var elements:i64
        let s_20_6: i64 = fn_state.elements;
        // D s_20_7: write-var elementsshadow#7601 <= s_20_6
        fn_state.elementsshadow_7601 = s_20_6;
        // D s_20_8: read-var regs:i64
        let s_20_8: i64 = fn_state.regs;
        // D s_20_9: cast zx s_20_8 -> i
        let s_20_9: i128 = (i128::try_from(s_20_8).unwrap());
        // D s_20_10: call __id(s_20_9)
        let s_20_10: i128 = u__id(state, tracer, s_20_9);
        // D s_20_11: cast reint s_20_10 -> i64
        let s_20_11: i64 = (s_20_10 as i64);
        // C s_20_12: const #1s : i
        let s_20_12: i128 = 1;
        // D s_20_13: cast zx s_20_11 -> i
        let s_20_13: i128 = (i128::try_from(s_20_11).unwrap());
        // D s_20_14: cmp-eq s_20_13 s_20_12
        let s_20_14: bool = ((s_20_13) == (s_20_12));
        // N s_20_15: branch s_20_14 b65 b21
        if s_20_14 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var regs:i64
        let s_21_0: i64 = fn_state.regs;
        // D s_21_1: cast zx s_21_0 -> i
        let s_21_1: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_2: call __id(s_21_1)
        let s_21_2: i128 = u__id(state, tracer, s_21_1);
        // D s_21_3: cast reint s_21_2 -> i64
        let s_21_3: i64 = (s_21_2 as i64);
        // C s_21_4: const #2s : i
        let s_21_4: i128 = 2;
        // D s_21_5: cast zx s_21_3 -> i
        let s_21_5: i128 = (i128::try_from(s_21_3).unwrap());
        // D s_21_6: cmp-eq s_21_5 s_21_4
        let s_21_6: bool = ((s_21_5) == (s_21_4));
        // D s_21_7: write-var gs#314048 <= s_21_6
        fn_state.gs_314048 = s_21_6;
        // N s_21_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#314048:u8
        let s_22_0: bool = fn_state.gs_314048;
        // N s_22_1: branch s_22_0 b25 b23
        if s_22_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#314076 <= s_23_0
        fn_state.gs_314076 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#314076:u8
        let s_24_0: bool = fn_state.gs_314076;
        // N s_24_1: assert s_24_0
        let s_24_1: () = assert!(s_24_0);
        // D s_24_2: read-var esizeshadow#7600:i64
        let s_24_2: i64 = fn_state.esizeshadow_7600;
        // D s_24_3: cast zx s_24_2 -> i
        let s_24_3: i128 = (i128::try_from(s_24_2).unwrap());
        // D s_24_4: cast reint s_24_3 -> i64
        let s_24_4: i64 = (s_24_3 as i64);
        // D s_24_5: read-var indexshadow#7599:i
        let s_24_5: i128 = fn_state.indexshadow_7599;
        // D s_24_6: cast reint s_24_5 -> i64
        let s_24_6: i64 = (s_24_5 as i64);
        // D s_24_7: read-var mshadow#7598:i
        let s_24_7: i128 = fn_state.mshadow_7598;
        // D s_24_8: cast reint s_24_7 -> i64
        let s_24_8: i64 = (s_24_7 as i64);
        // D s_24_9: read-var d:i64
        let s_24_9: i64 = fn_state.d;
        // D s_24_10: read-var elementsshadow#7601:i64
        let s_24_10: i64 = fn_state.elementsshadow_7601;
        // D s_24_11: read-var floating_point:u8
        let s_24_11: bool = fn_state.floating_point;
        // C s_24_12: const #0u : u8
        let s_24_12: bool = false;
        // D s_24_13: read-var n:i64
        let s_24_13: i64 = fn_state.n;
        // D s_24_14: read-var regs:i64
        let s_24_14: i64 = fn_state.regs;
        // C s_24_15: const #0u : u8
        let s_24_15: bool = false;
        // D s_24_16: call execute_aarch32_instrs_VMUL_s_Op_A_txt(s_24_9, s_24_10, s_24_4, s_24_11, s_24_6, s_24_12, s_24_8, s_24_13, s_24_14, s_24_15)
        let s_24_16: () = execute_aarch32_instrs_VMUL_s_Op_A_txt(
            state,
            tracer,
            s_24_9,
            s_24_10,
            s_24_4,
            s_24_11,
            s_24_6,
            s_24_12,
            s_24_8,
            s_24_13,
            s_24_14,
            s_24_15,
        );
        // N s_24_17: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var n:i64
        let s_25_0: i64 = fn_state.n;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: call __id(s_25_1)
        let s_25_2: i128 = u__id(state, tracer, s_25_1);
        // D s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: const #0s : i
        let s_25_4: i128 = 0;
        // D s_25_5: cast zx s_25_3 -> i
        let s_25_5: i128 = (i128::try_from(s_25_3).unwrap());
        // D s_25_6: cmp-le s_25_4 s_25_5
        let s_25_6: bool = ((s_25_4) <= (s_25_5));
        // N s_25_7: branch s_25_6 b28 b26
        if s_25_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#314075 <= s_26_0
        fn_state.gs_314075 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#314075:u8
        let s_27_0: bool = fn_state.gs_314075;
        // D s_27_1: write-var gs#314076 <= s_27_0
        fn_state.gs_314076 = s_27_0;
        // N s_27_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var n:i64
        let s_28_0: i64 = fn_state.n;
        // D s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_2: call __id(s_28_1)
        let s_28_2: i128 = u__id(state, tracer, s_28_1);
        // D s_28_3: cast reint s_28_2 -> i64
        let s_28_3: i64 = (s_28_2 as i64);
        // C s_28_4: const #31s : i
        let s_28_4: i128 = 31;
        // D s_28_5: cast zx s_28_3 -> i
        let s_28_5: i128 = (i128::try_from(s_28_3).unwrap());
        // D s_28_6: cmp-le s_28_5 s_28_4
        let s_28_6: bool = ((s_28_5) <= (s_28_4));
        // N s_28_7: branch s_28_6 b31 b29
        if s_28_6 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#314074 <= s_29_0
        fn_state.gs_314074 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#314074:u8
        let s_30_0: bool = fn_state.gs_314074;
        // D s_30_1: write-var gs#314075 <= s_30_0
        fn_state.gs_314075 = s_30_0;
        // N s_30_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var mshadow#7598:i
        let s_31_0: i128 = fn_state.mshadow_7598;
        // D s_31_1: call __id(s_31_0)
        let s_31_1: i128 = u__id(state, tracer, s_31_0);
        // C s_31_2: const #0s : i
        let s_31_2: i128 = 0;
        // D s_31_3: cmp-le s_31_2 s_31_1
        let s_31_3: bool = ((s_31_2) <= (s_31_1));
        // N s_31_4: branch s_31_3 b34 b32
        if s_31_3 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#314073 <= s_32_0
        fn_state.gs_314073 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#314073:u8
        let s_33_0: bool = fn_state.gs_314073;
        // D s_33_1: write-var gs#314074 <= s_33_0
        fn_state.gs_314074 = s_33_0;
        // N s_33_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var mshadow#7598:i
        let s_34_0: i128 = fn_state.mshadow_7598;
        // D s_34_1: call __id(s_34_0)
        let s_34_1: i128 = u__id(state, tracer, s_34_0);
        // C s_34_2: const #15s : i
        let s_34_2: i128 = 15;
        // D s_34_3: cmp-le s_34_1 s_34_2
        let s_34_3: bool = ((s_34_1) <= (s_34_2));
        // N s_34_4: branch s_34_3 b37 b35
        if s_34_3 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#314072 <= s_35_0
        fn_state.gs_314072 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#314072:u8
        let s_36_0: bool = fn_state.gs_314072;
        // D s_36_1: write-var gs#314073 <= s_36_0
        fn_state.gs_314073 = s_36_0;
        // N s_36_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var indexshadow#7599:i
        let s_37_0: i128 = fn_state.indexshadow_7599;
        // D s_37_1: call __id(s_37_0)
        let s_37_1: i128 = u__id(state, tracer, s_37_0);
        // C s_37_2: const #0s : i
        let s_37_2: i128 = 0;
        // D s_37_3: cmp-eq s_37_1 s_37_2
        let s_37_3: bool = ((s_37_1) == (s_37_2));
        // N s_37_4: branch s_37_3 b64 b38
        if s_37_3 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var indexshadow#7599:i
        let s_38_0: i128 = fn_state.indexshadow_7599;
        // D s_38_1: call __id(s_38_0)
        let s_38_1: i128 = u__id(state, tracer, s_38_0);
        // C s_38_2: const #1s : i
        let s_38_2: i128 = 1;
        // D s_38_3: cmp-eq s_38_1 s_38_2
        let s_38_3: bool = ((s_38_1) == (s_38_2));
        // D s_38_4: write-var gs#314055 <= s_38_3
        fn_state.gs_314055 = s_38_3;
        // N s_38_5: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#314055:u8
        let s_39_0: bool = fn_state.gs_314055;
        // N s_39_1: branch s_39_0 b63 b40
        if s_39_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var indexshadow#7599:i
        let s_40_0: i128 = fn_state.indexshadow_7599;
        // D s_40_1: call __id(s_40_0)
        let s_40_1: i128 = u__id(state, tracer, s_40_0);
        // C s_40_2: const #2s : i
        let s_40_2: i128 = 2;
        // D s_40_3: cmp-eq s_40_1 s_40_2
        let s_40_3: bool = ((s_40_1) == (s_40_2));
        // D s_40_4: write-var gs#314057 <= s_40_3
        fn_state.gs_314057 = s_40_3;
        // N s_40_5: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#314057:u8
        let s_41_0: bool = fn_state.gs_314057;
        // N s_41_1: branch s_41_0 b62 b42
        if s_41_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var indexshadow#7599:i
        let s_42_0: i128 = fn_state.indexshadow_7599;
        // D s_42_1: call __id(s_42_0)
        let s_42_1: i128 = u__id(state, tracer, s_42_0);
        // C s_42_2: const #3s : i
        let s_42_2: i128 = 3;
        // D s_42_3: cmp-eq s_42_1 s_42_2
        let s_42_3: bool = ((s_42_1) == (s_42_2));
        // D s_42_4: write-var gs#314059 <= s_42_3
        fn_state.gs_314059 = s_42_3;
        // N s_42_5: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#314059:u8
        let s_43_0: bool = fn_state.gs_314059;
        // N s_43_1: branch s_43_0 b46 b44
        if s_43_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#314071 <= s_44_0
        fn_state.gs_314071 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#314071:u8
        let s_45_0: bool = fn_state.gs_314071;
        // D s_45_1: write-var gs#314072 <= s_45_0
        fn_state.gs_314072 = s_45_0;
        // N s_45_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var esizeshadow#7600:i64
        let s_46_0: i64 = fn_state.esizeshadow_7600;
        // D s_46_1: cast zx s_46_0 -> i
        let s_46_1: i128 = (i128::try_from(s_46_0).unwrap());
        // D s_46_2: call __id(s_46_1)
        let s_46_2: i128 = u__id(state, tracer, s_46_1);
        // D s_46_3: cast reint s_46_2 -> i64
        let s_46_3: i64 = (s_46_2 as i64);
        // C s_46_4: const #16s : i
        let s_46_4: i128 = 16;
        // D s_46_5: cast zx s_46_3 -> i
        let s_46_5: i128 = (i128::try_from(s_46_3).unwrap());
        // D s_46_6: cmp-eq s_46_5 s_46_4
        let s_46_6: bool = ((s_46_5) == (s_46_4));
        // N s_46_7: branch s_46_6 b61 b47
        if s_46_6 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var esizeshadow#7600:i64
        let s_47_0: i64 = fn_state.esizeshadow_7600;
        // D s_47_1: cast zx s_47_0 -> i
        let s_47_1: i128 = (i128::try_from(s_47_0).unwrap());
        // D s_47_2: call __id(s_47_1)
        let s_47_2: i128 = u__id(state, tracer, s_47_1);
        // D s_47_3: cast reint s_47_2 -> i64
        let s_47_3: i64 = (s_47_2 as i64);
        // C s_47_4: const #32s : i
        let s_47_4: i128 = 32;
        // D s_47_5: cast zx s_47_3 -> i
        let s_47_5: i128 = (i128::try_from(s_47_3).unwrap());
        // D s_47_6: cmp-eq s_47_5 s_47_4
        let s_47_6: bool = ((s_47_5) == (s_47_4));
        // D s_47_7: write-var gs#314062 <= s_47_6
        fn_state.gs_314062 = s_47_6;
        // N s_47_8: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#314062:u8
        let s_48_0: bool = fn_state.gs_314062;
        // N s_48_1: branch s_48_0 b51 b49
        if s_48_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#314070 <= s_49_0
        fn_state.gs_314070 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#314070:u8
        let s_50_0: bool = fn_state.gs_314070;
        // D s_50_1: write-var gs#314071 <= s_50_0
        fn_state.gs_314071 = s_50_0;
        // N s_50_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var elementsshadow#7601:i64
        let s_51_0: i64 = fn_state.elementsshadow_7601;
        // D s_51_1: cast zx s_51_0 -> i
        let s_51_1: i128 = (i128::try_from(s_51_0).unwrap());
        // D s_51_2: call __id(s_51_1)
        let s_51_2: i128 = u__id(state, tracer, s_51_1);
        // D s_51_3: cast reint s_51_2 -> i64
        let s_51_3: i64 = (s_51_2 as i64);
        // C s_51_4: const #2s : i
        let s_51_4: i128 = 2;
        // D s_51_5: cast zx s_51_3 -> i
        let s_51_5: i128 = (i128::try_from(s_51_3).unwrap());
        // D s_51_6: cmp-eq s_51_5 s_51_4
        let s_51_6: bool = ((s_51_5) == (s_51_4));
        // N s_51_7: branch s_51_6 b60 b52
        if s_51_6 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var elementsshadow#7601:i64
        let s_52_0: i64 = fn_state.elementsshadow_7601;
        // D s_52_1: cast zx s_52_0 -> i
        let s_52_1: i128 = (i128::try_from(s_52_0).unwrap());
        // D s_52_2: call __id(s_52_1)
        let s_52_2: i128 = u__id(state, tracer, s_52_1);
        // D s_52_3: cast reint s_52_2 -> i64
        let s_52_3: i64 = (s_52_2 as i64);
        // C s_52_4: const #4s : i
        let s_52_4: i128 = 4;
        // D s_52_5: cast zx s_52_3 -> i
        let s_52_5: i128 = (i128::try_from(s_52_3).unwrap());
        // D s_52_6: cmp-eq s_52_5 s_52_4
        let s_52_6: bool = ((s_52_5) == (s_52_4));
        // D s_52_7: write-var gs#314065 <= s_52_6
        fn_state.gs_314065 = s_52_6;
        // N s_52_8: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#314065:u8
        let s_53_0: bool = fn_state.gs_314065;
        // N s_53_1: branch s_53_0 b56 b54
        if s_53_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#314069 <= s_54_0
        fn_state.gs_314069 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#314069:u8
        let s_55_0: bool = fn_state.gs_314069;
        // D s_55_1: write-var gs#314070 <= s_55_0
        fn_state.gs_314070 = s_55_0;
        // N s_55_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var d:i64
        let s_56_0: i64 = fn_state.d;
        // D s_56_1: cast zx s_56_0 -> i
        let s_56_1: i128 = (i128::try_from(s_56_0).unwrap());
        // D s_56_2: call __id(s_56_1)
        let s_56_2: i128 = u__id(state, tracer, s_56_1);
        // D s_56_3: cast reint s_56_2 -> i64
        let s_56_3: i64 = (s_56_2 as i64);
        // C s_56_4: const #0s : i
        let s_56_4: i128 = 0;
        // D s_56_5: cast zx s_56_3 -> i
        let s_56_5: i128 = (i128::try_from(s_56_3).unwrap());
        // D s_56_6: cmp-le s_56_4 s_56_5
        let s_56_6: bool = ((s_56_4) <= (s_56_5));
        // N s_56_7: branch s_56_6 b59 b57
        if s_56_6 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#314068 <= s_57_0
        fn_state.gs_314068 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#314068:u8
        let s_58_0: bool = fn_state.gs_314068;
        // D s_58_1: write-var gs#314069 <= s_58_0
        fn_state.gs_314069 = s_58_0;
        // N s_58_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var d:i64
        let s_59_0: i64 = fn_state.d;
        // D s_59_1: cast zx s_59_0 -> i
        let s_59_1: i128 = (i128::try_from(s_59_0).unwrap());
        // D s_59_2: call __id(s_59_1)
        let s_59_2: i128 = u__id(state, tracer, s_59_1);
        // D s_59_3: cast reint s_59_2 -> i64
        let s_59_3: i64 = (s_59_2 as i64);
        // C s_59_4: const #31s : i
        let s_59_4: i128 = 31;
        // D s_59_5: cast zx s_59_3 -> i
        let s_59_5: i128 = (i128::try_from(s_59_3).unwrap());
        // D s_59_6: cmp-le s_59_5 s_59_4
        let s_59_6: bool = ((s_59_5) <= (s_59_4));
        // D s_59_7: write-var gs#314068 <= s_59_6
        fn_state.gs_314068 = s_59_6;
        // N s_59_8: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#314065 <= s_60_0
        fn_state.gs_314065 = s_60_0;
        // N s_60_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#314062 <= s_61_0
        fn_state.gs_314062 = s_61_0;
        // N s_61_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #1u : u8
        let s_62_0: bool = true;
        // D s_62_1: write-var gs#314059 <= s_62_0
        fn_state.gs_314059 = s_62_0;
        // N s_62_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #1u : u8
        let s_63_0: bool = true;
        // D s_63_1: write-var gs#314057 <= s_63_0
        fn_state.gs_314057 = s_63_0;
        // N s_63_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #1u : u8
        let s_64_0: bool = true;
        // D s_64_1: write-var gs#314055 <= s_64_0
        fn_state.gs_314055 = s_64_0;
        // N s_64_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#314048 <= s_65_0
        fn_state.gs_314048 = s_65_0;
        // N s_65_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #32s : i64
        let s_66_0: i64 = 32;
        // D s_66_1: write-var esize <= s_66_0
        fn_state.esize = s_66_0;
        // C s_66_2: const #2s : i64
        let s_66_2: i64 = 2;
        // D s_66_3: write-var elements <= s_66_2
        fn_state.elements = s_66_2;
        // D s_66_4: read-var Vm:u8
        let s_66_4: u8 = fn_state.Vm;
        // D s_66_5: cast zx s_66_4 -> bv
        let s_66_5: Bits = Bits::new(s_66_4 as u128, 4u16);
        // D s_66_6: cast zx s_66_5 -> i
        let s_66_6: i128 = (s_66_5.value() as i128);
        // D s_66_7: write-var m <= s_66_6
        fn_state.m = s_66_6;
        // D s_66_8: read-var M:u8
        let s_66_8: bool = fn_state.M;
        // D s_66_9: cast zx s_66_8 -> bv
        let s_66_9: Bits = Bits::new(s_66_8 as u128, 1u16);
        // D s_66_10: cast zx s_66_9 -> i
        let s_66_10: i128 = (s_66_9.value() as i128);
        // D s_66_11: write-var index <= s_66_10
        fn_state.index = s_66_10;
        // N s_66_12: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #16s : i64
        let s_67_0: i64 = 16;
        // D s_67_1: write-var esize <= s_67_0
        fn_state.esize = s_67_0;
        // C s_67_2: const #4s : i64
        let s_67_2: i64 = 4;
        // D s_67_3: write-var elements <= s_67_2
        fn_state.elements = s_67_2;
        // C s_67_4: const #0s : i
        let s_67_4: i128 = 0;
        // D s_67_5: read-var Vm:u8
        let s_67_5: u8 = fn_state.Vm;
        // D s_67_6: cast zx s_67_5 -> bv
        let s_67_6: Bits = Bits::new(s_67_5 as u128, 4u16);
        // C s_67_7: const #1s : i64
        let s_67_7: i64 = 1;
        // C s_67_8: cast zx s_67_7 -> i
        let s_67_8: i128 = (i128::try_from(s_67_7).unwrap());
        // C s_67_9: const #2s : i
        let s_67_9: i128 = 2;
        // C s_67_10: add s_67_9 s_67_8
        let s_67_10: i128 = (s_67_9 + s_67_8);
        // D s_67_11: bit-extract s_67_6 s_67_4 s_67_10
        let s_67_11: Bits = (Bits::new(
            ((s_67_6) >> (s_67_4)).value(),
            u16::try_from(s_67_10).unwrap(),
        ));
        // D s_67_12: cast reint s_67_11 -> u8
        let s_67_12: u8 = (s_67_11.value() as u8);
        // D s_67_13: cast zx s_67_12 -> bv
        let s_67_13: Bits = Bits::new(s_67_12 as u128, 3u16);
        // D s_67_14: cast zx s_67_13 -> i
        let s_67_14: i128 = (s_67_13.value() as i128);
        // D s_67_15: write-var m <= s_67_14
        fn_state.m = s_67_14;
        // C s_67_16: const #3s : i
        let s_67_16: i128 = 3;
        // D s_67_17: read-var Vm:u8
        let s_67_17: u8 = fn_state.Vm;
        // D s_67_18: cast zx s_67_17 -> bv
        let s_67_18: Bits = Bits::new(s_67_17 as u128, 4u16);
        // C s_67_19: const #1u : u64
        let s_67_19: u64 = 1;
        // D s_67_20: bit-extract s_67_18 s_67_16 s_67_19
        let s_67_20: Bits = (Bits::new(
            ((s_67_18) >> (s_67_16)).value(),
            u16::try_from(s_67_19).unwrap(),
        ));
        // D s_67_21: cast reint s_67_20 -> u8
        let s_67_21: bool = ((s_67_20.value()) != 0);
        // C s_67_22: const #0s : i
        let s_67_22: i128 = 0;
        // C s_67_23: const #0u : u64
        let s_67_23: u64 = 0;
        // D s_67_24: cast zx s_67_21 -> u64
        let s_67_24: u64 = (s_67_21 as u64);
        // C s_67_25: const #1u : u64
        let s_67_25: u64 = 1;
        // D s_67_26: and s_67_24 s_67_25
        let s_67_26: u64 = ((s_67_24) & (s_67_25));
        // D s_67_27: cmp-eq s_67_26 s_67_25
        let s_67_27: bool = ((s_67_26) == (s_67_25));
        // D s_67_28: lsl s_67_24 s_67_22
        let s_67_28: u64 = s_67_24 << s_67_22;
        // D s_67_29: or s_67_23 s_67_28
        let s_67_29: u64 = ((s_67_23) | (s_67_28));
        // D s_67_30: cmpl s_67_28
        let s_67_30: u64 = !s_67_28;
        // D s_67_31: and s_67_23 s_67_30
        let s_67_31: u64 = ((s_67_23) & (s_67_30));
        // D s_67_32: select s_67_27 s_67_29 s_67_31
        let s_67_32: u64 = if s_67_27 { s_67_29 } else { s_67_31 };
        // D s_67_33: cast trunc s_67_32 -> u8
        let s_67_33: bool = ((s_67_32) != 0);
        // D s_67_34: read-var M:u8
        let s_67_34: bool = fn_state.M;
        // D s_67_35: cast zx s_67_34 -> bv
        let s_67_35: Bits = Bits::new(s_67_34 as u128, 1u16);
        // D s_67_36: cast zx s_67_33 -> bv
        let s_67_36: Bits = Bits::new(s_67_33 as u128, 1u16);
        // D s_67_37: cast reint s_67_35 -> u128
        let s_67_37: u128 = (s_67_35.value() as u128);
        // D s_67_38: size-of s_67_35
        let s_67_38: u16 = s_67_35.length();
        // D s_67_39: cast reint s_67_36 -> u128
        let s_67_39: u128 = (s_67_36.value() as u128);
        // D s_67_40: size-of s_67_36
        let s_67_40: u16 = s_67_36.length();
        // D s_67_41: lsl s_67_37 s_67_40
        let s_67_41: u128 = s_67_37 << s_67_40;
        // D s_67_42: or s_67_41 s_67_39
        let s_67_42: u128 = ((s_67_41) | (s_67_39));
        // D s_67_43: add s_67_38 s_67_40
        let s_67_43: u16 = (s_67_38 + s_67_40);
        // D s_67_44: create-bits s_67_42 s_67_43
        let s_67_44: Bits = Bits::new(s_67_42, s_67_43);
        // D s_67_45: cast reint s_67_44 -> u8
        let s_67_45: u8 = (s_67_44.value() as u8);
        // D s_67_46: cast zx s_67_45 -> bv
        let s_67_46: Bits = Bits::new(s_67_45 as u128, 2u16);
        // D s_67_47: cast zx s_67_46 -> i
        let s_67_47: i128 = (s_67_46.value() as i128);
        // D s_67_48: write-var index <= s_67_47
        fn_state.index = s_67_47;
        // N s_67_49: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #1s : i64
        let s_68_0: i64 = 1;
        // D s_68_1: write-var ga#357043 <= s_68_0
        fn_state.ga_357043 = s_68_0;
        // N s_68_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_69_0: panic
        panic!("{:?}", ());
        // N s_69_1: return
        return;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #0s : i
        let s_70_0: i128 = 0;
        // D s_70_1: read-var Vd:u8
        let s_70_1: u8 = fn_state.Vd;
        // D s_70_2: cast zx s_70_1 -> bv
        let s_70_2: Bits = Bits::new(s_70_1 as u128, 4u16);
        // C s_70_3: const #1u : u64
        let s_70_3: u64 = 1;
        // D s_70_4: bit-extract s_70_2 s_70_0 s_70_3
        let s_70_4: Bits = (Bits::new(
            ((s_70_2) >> (s_70_0)).value(),
            u16::try_from(s_70_3).unwrap(),
        ));
        // D s_70_5: cast reint s_70_4 -> u8
        let s_70_5: bool = ((s_70_4.value()) != 0);
        // C s_70_6: const #0s : i
        let s_70_6: i128 = 0;
        // C s_70_7: const #0u : u64
        let s_70_7: u64 = 0;
        // D s_70_8: cast zx s_70_5 -> u64
        let s_70_8: u64 = (s_70_5 as u64);
        // C s_70_9: const #1u : u64
        let s_70_9: u64 = 1;
        // D s_70_10: and s_70_8 s_70_9
        let s_70_10: u64 = ((s_70_8) & (s_70_9));
        // D s_70_11: cmp-eq s_70_10 s_70_9
        let s_70_11: bool = ((s_70_10) == (s_70_9));
        // D s_70_12: lsl s_70_8 s_70_6
        let s_70_12: u64 = s_70_8 << s_70_6;
        // D s_70_13: or s_70_7 s_70_12
        let s_70_13: u64 = ((s_70_7) | (s_70_12));
        // D s_70_14: cmpl s_70_12
        let s_70_14: u64 = !s_70_12;
        // D s_70_15: and s_70_7 s_70_14
        let s_70_15: u64 = ((s_70_7) & (s_70_14));
        // D s_70_16: select s_70_11 s_70_13 s_70_15
        let s_70_16: u64 = if s_70_11 { s_70_13 } else { s_70_15 };
        // D s_70_17: cast trunc s_70_16 -> u8
        let s_70_17: bool = ((s_70_16) != 0);
        // D s_70_18: cast zx s_70_17 -> bv
        let s_70_18: Bits = Bits::new(s_70_17 as u128, 1u16);
        // C s_70_19: const #1u : u8
        let s_70_19: bool = true;
        // C s_70_20: cast zx s_70_19 -> bv
        let s_70_20: Bits = Bits::new(s_70_19 as u128, 1u16);
        // D s_70_21: cmp-eq s_70_18 s_70_20
        let s_70_21: bool = ((s_70_18) == (s_70_20));
        // N s_70_22: branch s_70_21 b73 b71
        if s_70_21 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #0s : i
        let s_71_0: i128 = 0;
        // D s_71_1: read-var Vn:u8
        let s_71_1: u8 = fn_state.Vn;
        // D s_71_2: cast zx s_71_1 -> bv
        let s_71_2: Bits = Bits::new(s_71_1 as u128, 4u16);
        // C s_71_3: const #1u : u64
        let s_71_3: u64 = 1;
        // D s_71_4: bit-extract s_71_2 s_71_0 s_71_3
        let s_71_4: Bits = (Bits::new(
            ((s_71_2) >> (s_71_0)).value(),
            u16::try_from(s_71_3).unwrap(),
        ));
        // D s_71_5: cast reint s_71_4 -> u8
        let s_71_5: bool = ((s_71_4.value()) != 0);
        // C s_71_6: const #0s : i
        let s_71_6: i128 = 0;
        // C s_71_7: const #0u : u64
        let s_71_7: u64 = 0;
        // D s_71_8: cast zx s_71_5 -> u64
        let s_71_8: u64 = (s_71_5 as u64);
        // C s_71_9: const #1u : u64
        let s_71_9: u64 = 1;
        // D s_71_10: and s_71_8 s_71_9
        let s_71_10: u64 = ((s_71_8) & (s_71_9));
        // D s_71_11: cmp-eq s_71_10 s_71_9
        let s_71_11: bool = ((s_71_10) == (s_71_9));
        // D s_71_12: lsl s_71_8 s_71_6
        let s_71_12: u64 = s_71_8 << s_71_6;
        // D s_71_13: or s_71_7 s_71_12
        let s_71_13: u64 = ((s_71_7) | (s_71_12));
        // D s_71_14: cmpl s_71_12
        let s_71_14: u64 = !s_71_12;
        // D s_71_15: and s_71_7 s_71_14
        let s_71_15: u64 = ((s_71_7) & (s_71_14));
        // D s_71_16: select s_71_11 s_71_13 s_71_15
        let s_71_16: u64 = if s_71_11 { s_71_13 } else { s_71_15 };
        // D s_71_17: cast trunc s_71_16 -> u8
        let s_71_17: bool = ((s_71_16) != 0);
        // D s_71_18: cast zx s_71_17 -> bv
        let s_71_18: Bits = Bits::new(s_71_17 as u128, 1u16);
        // C s_71_19: const #1u : u8
        let s_71_19: bool = true;
        // C s_71_20: cast zx s_71_19 -> bv
        let s_71_20: Bits = Bits::new(s_71_19 as u128, 1u16);
        // D s_71_21: cmp-eq s_71_18 s_71_20
        let s_71_21: bool = ((s_71_18) == (s_71_20));
        // D s_71_22: write-var gs#314013 <= s_71_21
        fn_state.gs_314013 = s_71_21;
        // N s_71_23: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#314013:u8
        let s_72_0: bool = fn_state.gs_314013;
        // D s_72_1: write-var gs#314014 <= s_72_0
        fn_state.gs_314014 = s_72_0;
        // N s_72_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #1u : u8
        let s_73_0: bool = true;
        // D s_73_1: write-var gs#314013 <= s_73_0
        fn_state.gs_314013 = s_73_0;
        // N s_73_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_74_0: panic
        panic!("{:?}", ());
        // N s_74_1: return
        return;
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#314008 <= s_75_0
        fn_state.gs_314008 = s_75_0;
        // N s_75_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_76_0: panic
        panic!("{:?}", ());
        // N s_76_1: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #() : ()
        let s_77_0: () = ();
        // S s_77_1: call InITBlock(s_77_0)
        let s_77_1: bool = InITBlock(state, tracer, s_77_0);
        // D s_77_2: write-var gs#314007 <= s_77_1
        fn_state.gs_314007 = s_77_1;
        // N s_77_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var size:u8
        let s_78_0: u8 = fn_state.size;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 2u16);
        // C s_78_2: const #1u : u8
        let s_78_2: u8 = 1;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 2u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: write-var gs#314006 <= s_78_4
        fn_state.gs_314006 = s_78_4;
        // N s_78_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_79_0: panic
        panic!("{:?}", ());
        // N s_79_1: return
        return;
    }
}

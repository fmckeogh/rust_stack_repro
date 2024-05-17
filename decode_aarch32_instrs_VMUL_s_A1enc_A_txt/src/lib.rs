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
use common::*;
pub fn decode_aarch32_instrs_VMUL_s_A1enc_A_txt<T: Tracer>(
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
        floating_point: bool,
        m: i128,
        gs_313863: bool,
        gs_313927: bool,
        gs_313911: bool,
        gs_313902: bool,
        gs_313919: bool,
        gs_313916: bool,
        esize: i64,
        elementsshadow_7593: i64,
        n: i64,
        index: i128,
        gs_313923: bool,
        gs_313928: bool,
        gs_313930: bool,
        ga_356933: i64,
        gs_313924: bool,
        gs_313922: bool,
        esizeshadow_7592: i64,
        gs_313868: bool,
        mshadow_7590: i128,
        gs_313929: bool,
        regs: i64,
        gs_313913: bool,
        d: i64,
        elements: i64,
        indexshadow_7591: i128,
        gs_313925: bool,
        gs_313869: bool,
        gs_313926: bool,
        gs_313909: bool,
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
        // N s_2_5: branch s_2_4 b71 b3
        if s_2_4 {
            return block_71(state, tracer, fn_state);
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
        // N s_3_5: branch s_3_4 b70 b4
        if s_3_4 {
            return block_70(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#313863 <= s_4_0
        fn_state.gs_313863 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#313863:u8
        let s_5_0: bool = fn_state.gs_313863;
        // N s_5_1: branch s_5_0 b69 b6
        if s_5_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var Q:u8
        let s_6_0: bool = fn_state.Q;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b65 b7
        if s_6_4 {
            return block_65(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#313869 <= s_7_0
        fn_state.gs_313869 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#313869:u8
        let s_8_0: bool = fn_state.gs_313869;
        // N s_8_1: branch s_8_0 b64 b9
        if s_8_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var F:u8
        let s_9_0: bool = fn_state.F;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // C s_9_2: const #1u : u8
        let s_9_2: bool = true;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: write-var floating_point <= s_9_4
        fn_state.floating_point = s_9_4;
        // D s_9_6: read-var D:u8
        let s_9_6: bool = fn_state.D;
        // D s_9_7: cast zx s_9_6 -> bv
        let s_9_7: Bits = Bits::new(s_9_6 as u128, 1u16);
        // D s_9_8: read-var Vd:u8
        let s_9_8: u8 = fn_state.Vd;
        // D s_9_9: cast zx s_9_8 -> bv
        let s_9_9: Bits = Bits::new(s_9_8 as u128, 4u16);
        // D s_9_10: cast reint s_9_7 -> u128
        let s_9_10: u128 = (s_9_7.value() as u128);
        // D s_9_11: size-of s_9_7
        let s_9_11: u16 = s_9_7.length();
        // D s_9_12: cast reint s_9_9 -> u128
        let s_9_12: u128 = (s_9_9.value() as u128);
        // D s_9_13: size-of s_9_9
        let s_9_13: u16 = s_9_9.length();
        // D s_9_14: lsl s_9_10 s_9_13
        let s_9_14: u128 = s_9_10 << s_9_13;
        // D s_9_15: or s_9_14 s_9_12
        let s_9_15: u128 = ((s_9_14) | (s_9_12));
        // D s_9_16: add s_9_11 s_9_13
        let s_9_16: u16 = (s_9_11 + s_9_13);
        // D s_9_17: create-bits s_9_15 s_9_16
        let s_9_17: Bits = Bits::new(s_9_15, s_9_16);
        // D s_9_18: cast reint s_9_17 -> u8
        let s_9_18: u8 = (s_9_17.value() as u8);
        // D s_9_19: cast zx s_9_18 -> bv
        let s_9_19: Bits = Bits::new(s_9_18 as u128, 5u16);
        // D s_9_20: cast zx s_9_19 -> i
        let s_9_20: i128 = (s_9_19.value() as i128);
        // D s_9_21: cast reint s_9_20 -> i64
        let s_9_21: i64 = (s_9_20 as i64);
        // D s_9_22: write-var d <= s_9_21
        fn_state.d = s_9_21;
        // D s_9_23: read-var N:u8
        let s_9_23: bool = fn_state.N;
        // D s_9_24: cast zx s_9_23 -> bv
        let s_9_24: Bits = Bits::new(s_9_23 as u128, 1u16);
        // D s_9_25: read-var Vn:u8
        let s_9_25: u8 = fn_state.Vn;
        // D s_9_26: cast zx s_9_25 -> bv
        let s_9_26: Bits = Bits::new(s_9_25 as u128, 4u16);
        // D s_9_27: cast reint s_9_24 -> u128
        let s_9_27: u128 = (s_9_24.value() as u128);
        // D s_9_28: size-of s_9_24
        let s_9_28: u16 = s_9_24.length();
        // D s_9_29: cast reint s_9_26 -> u128
        let s_9_29: u128 = (s_9_26.value() as u128);
        // D s_9_30: size-of s_9_26
        let s_9_30: u16 = s_9_26.length();
        // D s_9_31: lsl s_9_27 s_9_30
        let s_9_31: u128 = s_9_27 << s_9_30;
        // D s_9_32: or s_9_31 s_9_29
        let s_9_32: u128 = ((s_9_31) | (s_9_29));
        // D s_9_33: add s_9_28 s_9_30
        let s_9_33: u16 = (s_9_28 + s_9_30);
        // D s_9_34: create-bits s_9_32 s_9_33
        let s_9_34: Bits = Bits::new(s_9_32, s_9_33);
        // D s_9_35: cast reint s_9_34 -> u8
        let s_9_35: u8 = (s_9_34.value() as u8);
        // D s_9_36: cast zx s_9_35 -> bv
        let s_9_36: Bits = Bits::new(s_9_35 as u128, 5u16);
        // D s_9_37: cast zx s_9_36 -> i
        let s_9_37: i128 = (s_9_36.value() as i128);
        // D s_9_38: cast reint s_9_37 -> i64
        let s_9_38: i64 = (s_9_37 as i64);
        // D s_9_39: write-var n <= s_9_38
        fn_state.n = s_9_38;
        // D s_9_40: read-var Q:u8
        let s_9_40: bool = fn_state.Q;
        // D s_9_41: cast zx s_9_40 -> bv
        let s_9_41: Bits = Bits::new(s_9_40 as u128, 1u16);
        // C s_9_42: const #0u : u8
        let s_9_42: bool = false;
        // C s_9_43: cast zx s_9_42 -> bv
        let s_9_43: Bits = Bits::new(s_9_42 as u128, 1u16);
        // D s_9_44: cmp-eq s_9_41 s_9_43
        let s_9_44: bool = ((s_9_41) == (s_9_43));
        // N s_9_45: branch s_9_44 b63 b10
        if s_9_44 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #2s : i64
        let s_10_0: i64 = 2;
        // D s_10_1: write-var ga#356933 <= s_10_0
        fn_state.ga_356933 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#356933:i64
        let s_11_0: i64 = fn_state.ga_356933;
        // D s_11_1: write-var regs <= s_11_0
        fn_state.regs = s_11_0;
        // C s_11_2: const #16s : i64
        let s_11_2: i64 = 16;
        // D s_11_3: write-var esize <= s_11_2
        fn_state.esize = s_11_2;
        // C s_11_4: const #2s : i64
        let s_11_4: i64 = 2;
        // D s_11_5: write-var elements <= s_11_4
        fn_state.elements = s_11_4;
        // D s_11_6: read-var size:u8
        let s_11_6: u8 = fn_state.size;
        // D s_11_7: cast zx s_11_6 -> bv
        let s_11_7: Bits = Bits::new(s_11_6 as u128, 2u16);
        // C s_11_8: const #1u : u8
        let s_11_8: u8 = 1;
        // C s_11_9: cast zx s_11_8 -> bv
        let s_11_9: Bits = Bits::new(s_11_8 as u128, 2u16);
        // D s_11_10: cmp-eq s_11_7 s_11_9
        let s_11_10: bool = ((s_11_7) == (s_11_9));
        // N s_11_11: branch s_11_10 b62 b12
        if s_11_10 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var size:u8
        let s_13_0: u8 = fn_state.size;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // N s_13_5: branch s_13_4 b61 b14
        if s_13_4 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var m:i
        let s_15_0: i128 = fn_state.m;
        // D s_15_1: write-var mshadow#7590 <= s_15_0
        fn_state.mshadow_7590 = s_15_0;
        // D s_15_2: read-var index:i
        let s_15_2: i128 = fn_state.index;
        // D s_15_3: write-var indexshadow#7591 <= s_15_2
        fn_state.indexshadow_7591 = s_15_2;
        // D s_15_4: read-var esize:i64
        let s_15_4: i64 = fn_state.esize;
        // D s_15_5: write-var esizeshadow#7592 <= s_15_4
        fn_state.esizeshadow_7592 = s_15_4;
        // D s_15_6: read-var elements:i64
        let s_15_6: i64 = fn_state.elements;
        // D s_15_7: write-var elementsshadow#7593 <= s_15_6
        fn_state.elementsshadow_7593 = s_15_6;
        // D s_15_8: read-var regs:i64
        let s_15_8: i64 = fn_state.regs;
        // D s_15_9: cast zx s_15_8 -> i
        let s_15_9: i128 = (i128::try_from(s_15_8).unwrap());
        // D s_15_10: call __id(s_15_9)
        let s_15_10: i128 = u__id(state, tracer, s_15_9);
        // D s_15_11: cast reint s_15_10 -> i64
        let s_15_11: i64 = (s_15_10 as i64);
        // C s_15_12: const #1s : i
        let s_15_12: i128 = 1;
        // D s_15_13: cast zx s_15_11 -> i
        let s_15_13: i128 = (i128::try_from(s_15_11).unwrap());
        // D s_15_14: cmp-eq s_15_13 s_15_12
        let s_15_14: bool = ((s_15_13) == (s_15_12));
        // N s_15_15: branch s_15_14 b60 b16
        if s_15_14 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var regs:i64
        let s_16_0: i64 = fn_state.regs;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: call __id(s_16_1)
        let s_16_2: i128 = u__id(state, tracer, s_16_1);
        // D s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // C s_16_4: const #2s : i
        let s_16_4: i128 = 2;
        // D s_16_5: cast zx s_16_3 -> i
        let s_16_5: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_6: cmp-eq s_16_5 s_16_4
        let s_16_6: bool = ((s_16_5) == (s_16_4));
        // D s_16_7: write-var gs#313902 <= s_16_6
        fn_state.gs_313902 = s_16_6;
        // N s_16_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#313902:u8
        let s_17_0: bool = fn_state.gs_313902;
        // N s_17_1: branch s_17_0 b20 b18
        if s_17_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#313930 <= s_18_0
        fn_state.gs_313930 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#313930:u8
        let s_19_0: bool = fn_state.gs_313930;
        // N s_19_1: assert s_19_0
        let s_19_1: () = assert!(s_19_0);
        // D s_19_2: read-var esizeshadow#7592:i64
        let s_19_2: i64 = fn_state.esizeshadow_7592;
        // D s_19_3: cast zx s_19_2 -> i
        let s_19_3: i128 = (i128::try_from(s_19_2).unwrap());
        // D s_19_4: cast reint s_19_3 -> i64
        let s_19_4: i64 = (s_19_3 as i64);
        // D s_19_5: read-var indexshadow#7591:i
        let s_19_5: i128 = fn_state.indexshadow_7591;
        // D s_19_6: cast reint s_19_5 -> i64
        let s_19_6: i64 = (s_19_5 as i64);
        // D s_19_7: read-var mshadow#7590:i
        let s_19_7: i128 = fn_state.mshadow_7590;
        // D s_19_8: cast reint s_19_7 -> i64
        let s_19_8: i64 = (s_19_7 as i64);
        // D s_19_9: read-var d:i64
        let s_19_9: i64 = fn_state.d;
        // D s_19_10: read-var elementsshadow#7593:i64
        let s_19_10: i64 = fn_state.elementsshadow_7593;
        // D s_19_11: read-var floating_point:u8
        let s_19_11: bool = fn_state.floating_point;
        // C s_19_12: const #0u : u8
        let s_19_12: bool = false;
        // D s_19_13: read-var n:i64
        let s_19_13: i64 = fn_state.n;
        // D s_19_14: read-var regs:i64
        let s_19_14: i64 = fn_state.regs;
        // C s_19_15: const #0u : u8
        let s_19_15: bool = false;
        // D s_19_16: call execute_aarch32_instrs_VMUL_s_Op_A_txt(s_19_9, s_19_10, s_19_4, s_19_11, s_19_6, s_19_12, s_19_8, s_19_13, s_19_14, s_19_15)
        let s_19_16: () = execute_aarch32_instrs_VMUL_s_Op_A_txt(
            state,
            tracer,
            s_19_9,
            s_19_10,
            s_19_4,
            s_19_11,
            s_19_6,
            s_19_12,
            s_19_8,
            s_19_13,
            s_19_14,
            s_19_15,
        );
        // N s_19_17: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var n:i64
        let s_20_0: i64 = fn_state.n;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: call __id(s_20_1)
        let s_20_2: i128 = u__id(state, tracer, s_20_1);
        // D s_20_3: cast reint s_20_2 -> i64
        let s_20_3: i64 = (s_20_2 as i64);
        // C s_20_4: const #0s : i
        let s_20_4: i128 = 0;
        // D s_20_5: cast zx s_20_3 -> i
        let s_20_5: i128 = (i128::try_from(s_20_3).unwrap());
        // D s_20_6: cmp-le s_20_4 s_20_5
        let s_20_6: bool = ((s_20_4) <= (s_20_5));
        // N s_20_7: branch s_20_6 b23 b21
        if s_20_6 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#313929 <= s_21_0
        fn_state.gs_313929 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#313929:u8
        let s_22_0: bool = fn_state.gs_313929;
        // D s_22_1: write-var gs#313930 <= s_22_0
        fn_state.gs_313930 = s_22_0;
        // N s_22_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var n:i64
        let s_23_0: i64 = fn_state.n;
        // D s_23_1: cast zx s_23_0 -> i
        let s_23_1: i128 = (i128::try_from(s_23_0).unwrap());
        // D s_23_2: call __id(s_23_1)
        let s_23_2: i128 = u__id(state, tracer, s_23_1);
        // D s_23_3: cast reint s_23_2 -> i64
        let s_23_3: i64 = (s_23_2 as i64);
        // C s_23_4: const #31s : i
        let s_23_4: i128 = 31;
        // D s_23_5: cast zx s_23_3 -> i
        let s_23_5: i128 = (i128::try_from(s_23_3).unwrap());
        // D s_23_6: cmp-le s_23_5 s_23_4
        let s_23_6: bool = ((s_23_5) <= (s_23_4));
        // N s_23_7: branch s_23_6 b26 b24
        if s_23_6 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#313928 <= s_24_0
        fn_state.gs_313928 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#313928:u8
        let s_25_0: bool = fn_state.gs_313928;
        // D s_25_1: write-var gs#313929 <= s_25_0
        fn_state.gs_313929 = s_25_0;
        // N s_25_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var mshadow#7590:i
        let s_26_0: i128 = fn_state.mshadow_7590;
        // D s_26_1: call __id(s_26_0)
        let s_26_1: i128 = u__id(state, tracer, s_26_0);
        // C s_26_2: const #0s : i
        let s_26_2: i128 = 0;
        // D s_26_3: cmp-le s_26_2 s_26_1
        let s_26_3: bool = ((s_26_2) <= (s_26_1));
        // N s_26_4: branch s_26_3 b29 b27
        if s_26_3 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#313927 <= s_27_0
        fn_state.gs_313927 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#313927:u8
        let s_28_0: bool = fn_state.gs_313927;
        // D s_28_1: write-var gs#313928 <= s_28_0
        fn_state.gs_313928 = s_28_0;
        // N s_28_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var mshadow#7590:i
        let s_29_0: i128 = fn_state.mshadow_7590;
        // D s_29_1: call __id(s_29_0)
        let s_29_1: i128 = u__id(state, tracer, s_29_0);
        // C s_29_2: const #15s : i
        let s_29_2: i128 = 15;
        // D s_29_3: cmp-le s_29_1 s_29_2
        let s_29_3: bool = ((s_29_1) <= (s_29_2));
        // N s_29_4: branch s_29_3 b32 b30
        if s_29_3 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#313926 <= s_30_0
        fn_state.gs_313926 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#313926:u8
        let s_31_0: bool = fn_state.gs_313926;
        // D s_31_1: write-var gs#313927 <= s_31_0
        fn_state.gs_313927 = s_31_0;
        // N s_31_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var indexshadow#7591:i
        let s_32_0: i128 = fn_state.indexshadow_7591;
        // D s_32_1: call __id(s_32_0)
        let s_32_1: i128 = u__id(state, tracer, s_32_0);
        // C s_32_2: const #0s : i
        let s_32_2: i128 = 0;
        // D s_32_3: cmp-eq s_32_1 s_32_2
        let s_32_3: bool = ((s_32_1) == (s_32_2));
        // N s_32_4: branch s_32_3 b59 b33
        if s_32_3 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var indexshadow#7591:i
        let s_33_0: i128 = fn_state.indexshadow_7591;
        // D s_33_1: call __id(s_33_0)
        let s_33_1: i128 = u__id(state, tracer, s_33_0);
        // C s_33_2: const #1s : i
        let s_33_2: i128 = 1;
        // D s_33_3: cmp-eq s_33_1 s_33_2
        let s_33_3: bool = ((s_33_1) == (s_33_2));
        // D s_33_4: write-var gs#313909 <= s_33_3
        fn_state.gs_313909 = s_33_3;
        // N s_33_5: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#313909:u8
        let s_34_0: bool = fn_state.gs_313909;
        // N s_34_1: branch s_34_0 b58 b35
        if s_34_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var indexshadow#7591:i
        let s_35_0: i128 = fn_state.indexshadow_7591;
        // D s_35_1: call __id(s_35_0)
        let s_35_1: i128 = u__id(state, tracer, s_35_0);
        // C s_35_2: const #2s : i
        let s_35_2: i128 = 2;
        // D s_35_3: cmp-eq s_35_1 s_35_2
        let s_35_3: bool = ((s_35_1) == (s_35_2));
        // D s_35_4: write-var gs#313911 <= s_35_3
        fn_state.gs_313911 = s_35_3;
        // N s_35_5: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#313911:u8
        let s_36_0: bool = fn_state.gs_313911;
        // N s_36_1: branch s_36_0 b57 b37
        if s_36_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var indexshadow#7591:i
        let s_37_0: i128 = fn_state.indexshadow_7591;
        // D s_37_1: call __id(s_37_0)
        let s_37_1: i128 = u__id(state, tracer, s_37_0);
        // C s_37_2: const #3s : i
        let s_37_2: i128 = 3;
        // D s_37_3: cmp-eq s_37_1 s_37_2
        let s_37_3: bool = ((s_37_1) == (s_37_2));
        // D s_37_4: write-var gs#313913 <= s_37_3
        fn_state.gs_313913 = s_37_3;
        // N s_37_5: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#313913:u8
        let s_38_0: bool = fn_state.gs_313913;
        // N s_38_1: branch s_38_0 b41 b39
        if s_38_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#313925 <= s_39_0
        fn_state.gs_313925 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#313925:u8
        let s_40_0: bool = fn_state.gs_313925;
        // D s_40_1: write-var gs#313926 <= s_40_0
        fn_state.gs_313926 = s_40_0;
        // N s_40_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var esizeshadow#7592:i64
        let s_41_0: i64 = fn_state.esizeshadow_7592;
        // D s_41_1: cast zx s_41_0 -> i
        let s_41_1: i128 = (i128::try_from(s_41_0).unwrap());
        // D s_41_2: call __id(s_41_1)
        let s_41_2: i128 = u__id(state, tracer, s_41_1);
        // D s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: const #16s : i
        let s_41_4: i128 = 16;
        // D s_41_5: cast zx s_41_3 -> i
        let s_41_5: i128 = (i128::try_from(s_41_3).unwrap());
        // D s_41_6: cmp-eq s_41_5 s_41_4
        let s_41_6: bool = ((s_41_5) == (s_41_4));
        // N s_41_7: branch s_41_6 b56 b42
        if s_41_6 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var esizeshadow#7592:i64
        let s_42_0: i64 = fn_state.esizeshadow_7592;
        // D s_42_1: cast zx s_42_0 -> i
        let s_42_1: i128 = (i128::try_from(s_42_0).unwrap());
        // D s_42_2: call __id(s_42_1)
        let s_42_2: i128 = u__id(state, tracer, s_42_1);
        // D s_42_3: cast reint s_42_2 -> i64
        let s_42_3: i64 = (s_42_2 as i64);
        // C s_42_4: const #32s : i
        let s_42_4: i128 = 32;
        // D s_42_5: cast zx s_42_3 -> i
        let s_42_5: i128 = (i128::try_from(s_42_3).unwrap());
        // D s_42_6: cmp-eq s_42_5 s_42_4
        let s_42_6: bool = ((s_42_5) == (s_42_4));
        // D s_42_7: write-var gs#313916 <= s_42_6
        fn_state.gs_313916 = s_42_6;
        // N s_42_8: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#313916:u8
        let s_43_0: bool = fn_state.gs_313916;
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
        // D s_44_1: write-var gs#313924 <= s_44_0
        fn_state.gs_313924 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#313924:u8
        let s_45_0: bool = fn_state.gs_313924;
        // D s_45_1: write-var gs#313925 <= s_45_0
        fn_state.gs_313925 = s_45_0;
        // N s_45_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var elementsshadow#7593:i64
        let s_46_0: i64 = fn_state.elementsshadow_7593;
        // D s_46_1: cast zx s_46_0 -> i
        let s_46_1: i128 = (i128::try_from(s_46_0).unwrap());
        // D s_46_2: call __id(s_46_1)
        let s_46_2: i128 = u__id(state, tracer, s_46_1);
        // D s_46_3: cast reint s_46_2 -> i64
        let s_46_3: i64 = (s_46_2 as i64);
        // C s_46_4: const #2s : i
        let s_46_4: i128 = 2;
        // D s_46_5: cast zx s_46_3 -> i
        let s_46_5: i128 = (i128::try_from(s_46_3).unwrap());
        // D s_46_6: cmp-eq s_46_5 s_46_4
        let s_46_6: bool = ((s_46_5) == (s_46_4));
        // N s_46_7: branch s_46_6 b55 b47
        if s_46_6 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var elementsshadow#7593:i64
        let s_47_0: i64 = fn_state.elementsshadow_7593;
        // D s_47_1: cast zx s_47_0 -> i
        let s_47_1: i128 = (i128::try_from(s_47_0).unwrap());
        // D s_47_2: call __id(s_47_1)
        let s_47_2: i128 = u__id(state, tracer, s_47_1);
        // D s_47_3: cast reint s_47_2 -> i64
        let s_47_3: i64 = (s_47_2 as i64);
        // C s_47_4: const #4s : i
        let s_47_4: i128 = 4;
        // D s_47_5: cast zx s_47_3 -> i
        let s_47_5: i128 = (i128::try_from(s_47_3).unwrap());
        // D s_47_6: cmp-eq s_47_5 s_47_4
        let s_47_6: bool = ((s_47_5) == (s_47_4));
        // D s_47_7: write-var gs#313919 <= s_47_6
        fn_state.gs_313919 = s_47_6;
        // N s_47_8: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#313919:u8
        let s_48_0: bool = fn_state.gs_313919;
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
        // D s_49_1: write-var gs#313923 <= s_49_0
        fn_state.gs_313923 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#313923:u8
        let s_50_0: bool = fn_state.gs_313923;
        // D s_50_1: write-var gs#313924 <= s_50_0
        fn_state.gs_313924 = s_50_0;
        // N s_50_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var d:i64
        let s_51_0: i64 = fn_state.d;
        // D s_51_1: cast zx s_51_0 -> i
        let s_51_1: i128 = (i128::try_from(s_51_0).unwrap());
        // D s_51_2: call __id(s_51_1)
        let s_51_2: i128 = u__id(state, tracer, s_51_1);
        // D s_51_3: cast reint s_51_2 -> i64
        let s_51_3: i64 = (s_51_2 as i64);
        // C s_51_4: const #0s : i
        let s_51_4: i128 = 0;
        // D s_51_5: cast zx s_51_3 -> i
        let s_51_5: i128 = (i128::try_from(s_51_3).unwrap());
        // D s_51_6: cmp-le s_51_4 s_51_5
        let s_51_6: bool = ((s_51_4) <= (s_51_5));
        // N s_51_7: branch s_51_6 b54 b52
        if s_51_6 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#313922 <= s_52_0
        fn_state.gs_313922 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#313922:u8
        let s_53_0: bool = fn_state.gs_313922;
        // D s_53_1: write-var gs#313923 <= s_53_0
        fn_state.gs_313923 = s_53_0;
        // N s_53_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var d:i64
        let s_54_0: i64 = fn_state.d;
        // D s_54_1: cast zx s_54_0 -> i
        let s_54_1: i128 = (i128::try_from(s_54_0).unwrap());
        // D s_54_2: call __id(s_54_1)
        let s_54_2: i128 = u__id(state, tracer, s_54_1);
        // D s_54_3: cast reint s_54_2 -> i64
        let s_54_3: i64 = (s_54_2 as i64);
        // C s_54_4: const #31s : i
        let s_54_4: i128 = 31;
        // D s_54_5: cast zx s_54_3 -> i
        let s_54_5: i128 = (i128::try_from(s_54_3).unwrap());
        // D s_54_6: cmp-le s_54_5 s_54_4
        let s_54_6: bool = ((s_54_5) <= (s_54_4));
        // D s_54_7: write-var gs#313922 <= s_54_6
        fn_state.gs_313922 = s_54_6;
        // N s_54_8: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#313919 <= s_55_0
        fn_state.gs_313919 = s_55_0;
        // N s_55_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var gs#313916 <= s_56_0
        fn_state.gs_313916 = s_56_0;
        // N s_56_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var gs#313913 <= s_57_0
        fn_state.gs_313913 = s_57_0;
        // N s_57_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var gs#313911 <= s_58_0
        fn_state.gs_313911 = s_58_0;
        // N s_58_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #1u : u8
        let s_59_0: bool = true;
        // D s_59_1: write-var gs#313909 <= s_59_0
        fn_state.gs_313909 = s_59_0;
        // N s_59_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#313902 <= s_60_0
        fn_state.gs_313902 = s_60_0;
        // N s_60_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #32s : i64
        let s_61_0: i64 = 32;
        // D s_61_1: write-var esize <= s_61_0
        fn_state.esize = s_61_0;
        // C s_61_2: const #2s : i64
        let s_61_2: i64 = 2;
        // D s_61_3: write-var elements <= s_61_2
        fn_state.elements = s_61_2;
        // D s_61_4: read-var Vm:u8
        let s_61_4: u8 = fn_state.Vm;
        // D s_61_5: cast zx s_61_4 -> bv
        let s_61_5: Bits = Bits::new(s_61_4 as u128, 4u16);
        // D s_61_6: cast zx s_61_5 -> i
        let s_61_6: i128 = (s_61_5.value() as i128);
        // D s_61_7: write-var m <= s_61_6
        fn_state.m = s_61_6;
        // D s_61_8: read-var M:u8
        let s_61_8: bool = fn_state.M;
        // D s_61_9: cast zx s_61_8 -> bv
        let s_61_9: Bits = Bits::new(s_61_8 as u128, 1u16);
        // D s_61_10: cast zx s_61_9 -> i
        let s_61_10: i128 = (s_61_9.value() as i128);
        // D s_61_11: write-var index <= s_61_10
        fn_state.index = s_61_10;
        // N s_61_12: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #16s : i64
        let s_62_0: i64 = 16;
        // D s_62_1: write-var esize <= s_62_0
        fn_state.esize = s_62_0;
        // C s_62_2: const #4s : i64
        let s_62_2: i64 = 4;
        // D s_62_3: write-var elements <= s_62_2
        fn_state.elements = s_62_2;
        // C s_62_4: const #0s : i
        let s_62_4: i128 = 0;
        // D s_62_5: read-var Vm:u8
        let s_62_5: u8 = fn_state.Vm;
        // D s_62_6: cast zx s_62_5 -> bv
        let s_62_6: Bits = Bits::new(s_62_5 as u128, 4u16);
        // C s_62_7: const #1s : i64
        let s_62_7: i64 = 1;
        // C s_62_8: cast zx s_62_7 -> i
        let s_62_8: i128 = (i128::try_from(s_62_7).unwrap());
        // C s_62_9: const #2s : i
        let s_62_9: i128 = 2;
        // C s_62_10: add s_62_9 s_62_8
        let s_62_10: i128 = (s_62_9 + s_62_8);
        // D s_62_11: bit-extract s_62_6 s_62_4 s_62_10
        let s_62_11: Bits = (Bits::new(
            ((s_62_6) >> (s_62_4)).value(),
            u16::try_from(s_62_10).unwrap(),
        ));
        // D s_62_12: cast reint s_62_11 -> u8
        let s_62_12: u8 = (s_62_11.value() as u8);
        // D s_62_13: cast zx s_62_12 -> bv
        let s_62_13: Bits = Bits::new(s_62_12 as u128, 3u16);
        // D s_62_14: cast zx s_62_13 -> i
        let s_62_14: i128 = (s_62_13.value() as i128);
        // D s_62_15: write-var m <= s_62_14
        fn_state.m = s_62_14;
        // C s_62_16: const #3s : i
        let s_62_16: i128 = 3;
        // D s_62_17: read-var Vm:u8
        let s_62_17: u8 = fn_state.Vm;
        // D s_62_18: cast zx s_62_17 -> bv
        let s_62_18: Bits = Bits::new(s_62_17 as u128, 4u16);
        // C s_62_19: const #1u : u64
        let s_62_19: u64 = 1;
        // D s_62_20: bit-extract s_62_18 s_62_16 s_62_19
        let s_62_20: Bits = (Bits::new(
            ((s_62_18) >> (s_62_16)).value(),
            u16::try_from(s_62_19).unwrap(),
        ));
        // D s_62_21: cast reint s_62_20 -> u8
        let s_62_21: bool = ((s_62_20.value()) != 0);
        // C s_62_22: const #0s : i
        let s_62_22: i128 = 0;
        // C s_62_23: const #0u : u64
        let s_62_23: u64 = 0;
        // D s_62_24: cast zx s_62_21 -> u64
        let s_62_24: u64 = (s_62_21 as u64);
        // C s_62_25: const #1u : u64
        let s_62_25: u64 = 1;
        // D s_62_26: and s_62_24 s_62_25
        let s_62_26: u64 = ((s_62_24) & (s_62_25));
        // D s_62_27: cmp-eq s_62_26 s_62_25
        let s_62_27: bool = ((s_62_26) == (s_62_25));
        // D s_62_28: lsl s_62_24 s_62_22
        let s_62_28: u64 = s_62_24 << s_62_22;
        // D s_62_29: or s_62_23 s_62_28
        let s_62_29: u64 = ((s_62_23) | (s_62_28));
        // D s_62_30: cmpl s_62_28
        let s_62_30: u64 = !s_62_28;
        // D s_62_31: and s_62_23 s_62_30
        let s_62_31: u64 = ((s_62_23) & (s_62_30));
        // D s_62_32: select s_62_27 s_62_29 s_62_31
        let s_62_32: u64 = if s_62_27 { s_62_29 } else { s_62_31 };
        // D s_62_33: cast trunc s_62_32 -> u8
        let s_62_33: bool = ((s_62_32) != 0);
        // D s_62_34: read-var M:u8
        let s_62_34: bool = fn_state.M;
        // D s_62_35: cast zx s_62_34 -> bv
        let s_62_35: Bits = Bits::new(s_62_34 as u128, 1u16);
        // D s_62_36: cast zx s_62_33 -> bv
        let s_62_36: Bits = Bits::new(s_62_33 as u128, 1u16);
        // D s_62_37: cast reint s_62_35 -> u128
        let s_62_37: u128 = (s_62_35.value() as u128);
        // D s_62_38: size-of s_62_35
        let s_62_38: u16 = s_62_35.length();
        // D s_62_39: cast reint s_62_36 -> u128
        let s_62_39: u128 = (s_62_36.value() as u128);
        // D s_62_40: size-of s_62_36
        let s_62_40: u16 = s_62_36.length();
        // D s_62_41: lsl s_62_37 s_62_40
        let s_62_41: u128 = s_62_37 << s_62_40;
        // D s_62_42: or s_62_41 s_62_39
        let s_62_42: u128 = ((s_62_41) | (s_62_39));
        // D s_62_43: add s_62_38 s_62_40
        let s_62_43: u16 = (s_62_38 + s_62_40);
        // D s_62_44: create-bits s_62_42 s_62_43
        let s_62_44: Bits = Bits::new(s_62_42, s_62_43);
        // D s_62_45: cast reint s_62_44 -> u8
        let s_62_45: u8 = (s_62_44.value() as u8);
        // D s_62_46: cast zx s_62_45 -> bv
        let s_62_46: Bits = Bits::new(s_62_45 as u128, 2u16);
        // D s_62_47: cast zx s_62_46 -> i
        let s_62_47: i128 = (s_62_46.value() as i128);
        // D s_62_48: write-var index <= s_62_47
        fn_state.index = s_62_47;
        // N s_62_49: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #1s : i64
        let s_63_0: i64 = 1;
        // D s_63_1: write-var ga#356933 <= s_63_0
        fn_state.ga_356933 = s_63_0;
        // N s_63_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_64_0: panic
        panic!("{:?}", ());
        // N s_64_1: return
        return;
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #0s : i
        let s_65_0: i128 = 0;
        // D s_65_1: read-var Vd:u8
        let s_65_1: u8 = fn_state.Vd;
        // D s_65_2: cast zx s_65_1 -> bv
        let s_65_2: Bits = Bits::new(s_65_1 as u128, 4u16);
        // C s_65_3: const #1u : u64
        let s_65_3: u64 = 1;
        // D s_65_4: bit-extract s_65_2 s_65_0 s_65_3
        let s_65_4: Bits = (Bits::new(
            ((s_65_2) >> (s_65_0)).value(),
            u16::try_from(s_65_3).unwrap(),
        ));
        // D s_65_5: cast reint s_65_4 -> u8
        let s_65_5: bool = ((s_65_4.value()) != 0);
        // C s_65_6: const #0s : i
        let s_65_6: i128 = 0;
        // C s_65_7: const #0u : u64
        let s_65_7: u64 = 0;
        // D s_65_8: cast zx s_65_5 -> u64
        let s_65_8: u64 = (s_65_5 as u64);
        // C s_65_9: const #1u : u64
        let s_65_9: u64 = 1;
        // D s_65_10: and s_65_8 s_65_9
        let s_65_10: u64 = ((s_65_8) & (s_65_9));
        // D s_65_11: cmp-eq s_65_10 s_65_9
        let s_65_11: bool = ((s_65_10) == (s_65_9));
        // D s_65_12: lsl s_65_8 s_65_6
        let s_65_12: u64 = s_65_8 << s_65_6;
        // D s_65_13: or s_65_7 s_65_12
        let s_65_13: u64 = ((s_65_7) | (s_65_12));
        // D s_65_14: cmpl s_65_12
        let s_65_14: u64 = !s_65_12;
        // D s_65_15: and s_65_7 s_65_14
        let s_65_15: u64 = ((s_65_7) & (s_65_14));
        // D s_65_16: select s_65_11 s_65_13 s_65_15
        let s_65_16: u64 = if s_65_11 { s_65_13 } else { s_65_15 };
        // D s_65_17: cast trunc s_65_16 -> u8
        let s_65_17: bool = ((s_65_16) != 0);
        // D s_65_18: cast zx s_65_17 -> bv
        let s_65_18: Bits = Bits::new(s_65_17 as u128, 1u16);
        // C s_65_19: const #1u : u8
        let s_65_19: bool = true;
        // C s_65_20: cast zx s_65_19 -> bv
        let s_65_20: Bits = Bits::new(s_65_19 as u128, 1u16);
        // D s_65_21: cmp-eq s_65_18 s_65_20
        let s_65_21: bool = ((s_65_18) == (s_65_20));
        // N s_65_22: branch s_65_21 b68 b66
        if s_65_21 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #0s : i
        let s_66_0: i128 = 0;
        // D s_66_1: read-var Vn:u8
        let s_66_1: u8 = fn_state.Vn;
        // D s_66_2: cast zx s_66_1 -> bv
        let s_66_2: Bits = Bits::new(s_66_1 as u128, 4u16);
        // C s_66_3: const #1u : u64
        let s_66_3: u64 = 1;
        // D s_66_4: bit-extract s_66_2 s_66_0 s_66_3
        let s_66_4: Bits = (Bits::new(
            ((s_66_2) >> (s_66_0)).value(),
            u16::try_from(s_66_3).unwrap(),
        ));
        // D s_66_5: cast reint s_66_4 -> u8
        let s_66_5: bool = ((s_66_4.value()) != 0);
        // C s_66_6: const #0s : i
        let s_66_6: i128 = 0;
        // C s_66_7: const #0u : u64
        let s_66_7: u64 = 0;
        // D s_66_8: cast zx s_66_5 -> u64
        let s_66_8: u64 = (s_66_5 as u64);
        // C s_66_9: const #1u : u64
        let s_66_9: u64 = 1;
        // D s_66_10: and s_66_8 s_66_9
        let s_66_10: u64 = ((s_66_8) & (s_66_9));
        // D s_66_11: cmp-eq s_66_10 s_66_9
        let s_66_11: bool = ((s_66_10) == (s_66_9));
        // D s_66_12: lsl s_66_8 s_66_6
        let s_66_12: u64 = s_66_8 << s_66_6;
        // D s_66_13: or s_66_7 s_66_12
        let s_66_13: u64 = ((s_66_7) | (s_66_12));
        // D s_66_14: cmpl s_66_12
        let s_66_14: u64 = !s_66_12;
        // D s_66_15: and s_66_7 s_66_14
        let s_66_15: u64 = ((s_66_7) & (s_66_14));
        // D s_66_16: select s_66_11 s_66_13 s_66_15
        let s_66_16: u64 = if s_66_11 { s_66_13 } else { s_66_15 };
        // D s_66_17: cast trunc s_66_16 -> u8
        let s_66_17: bool = ((s_66_16) != 0);
        // D s_66_18: cast zx s_66_17 -> bv
        let s_66_18: Bits = Bits::new(s_66_17 as u128, 1u16);
        // C s_66_19: const #1u : u8
        let s_66_19: bool = true;
        // C s_66_20: cast zx s_66_19 -> bv
        let s_66_20: Bits = Bits::new(s_66_19 as u128, 1u16);
        // D s_66_21: cmp-eq s_66_18 s_66_20
        let s_66_21: bool = ((s_66_18) == (s_66_20));
        // D s_66_22: write-var gs#313868 <= s_66_21
        fn_state.gs_313868 = s_66_21;
        // N s_66_23: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#313868:u8
        let s_67_0: bool = fn_state.gs_313868;
        // D s_67_1: write-var gs#313869 <= s_67_0
        fn_state.gs_313869 = s_67_0;
        // N s_67_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #1u : u8
        let s_68_0: bool = true;
        // D s_68_1: write-var gs#313868 <= s_68_0
        fn_state.gs_313868 = s_68_0;
        // N s_68_2: jump b67
        return block_67(state, tracer, fn_state);
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
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: write-var gs#313863 <= s_70_0
        fn_state.gs_313863 = s_70_0;
        // N s_70_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_71_0: panic
        panic!("{:?}", ());
        // N s_71_1: return
        return;
    }
}

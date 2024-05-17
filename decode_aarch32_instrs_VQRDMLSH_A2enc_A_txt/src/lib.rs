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
use execute_aarch32_instrs_VQRDMLSH_Op_A_txt::*;
use u__id::*;
use HaveQRDMLAHExt::*;
use common::*;
pub fn decode_aarch32_instrs_VQRDMLSH_A2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Q: bool,
    D: bool,
    size: u8,
    Vn: u8,
    Vd: u8,
    N: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i128,
        gs_316682: bool,
        esize: i64,
        n: i64,
        index: i128,
        mshadow_7707: i128,
        gs_316681: bool,
        regs: i64,
        indexshadow_7708: i128,
        elementsshadow_7710: i128,
        esizeshadow_7709: i64,
        ga_359350: i64,
        gs_316715: bool,
        d: i64,
        elements: i128,
        Q: bool,
        D: bool,
        size: u8,
        Vn: u8,
        Vd: u8,
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
        // S s_0_1: call HaveQRDMLAHExt(s_0_0)
        let s_0_1: bool = HaveQRDMLAHExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b26 b1
        if s_0_2 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var size:u8
        let s_1_0: u8 = fn_state.size;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #3u : u8
        let s_1_2: u8 = 3;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b25 b2
        if s_1_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
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
        // C s_2_2: const #0u : u8
        let s_2_2: u8 = 0;
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
        // D s_3_0: read-var Q:u8
        let s_3_0: bool = fn_state.Q;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b20 b4
        if s_3_4 {
            return block_20(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#316682 <= s_4_0
        fn_state.gs_316682 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#316682:u8
        let s_5_0: bool = fn_state.gs_316682;
        // N s_5_1: branch s_5_0 b19 b6
        if s_5_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var D:u8
        let s_6_0: bool = fn_state.D;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // D s_6_2: read-var Vd:u8
        let s_6_2: u8 = fn_state.Vd;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 4u16);
        // D s_6_4: cast reint s_6_1 -> u128
        let s_6_4: u128 = (s_6_1.value() as u128);
        // D s_6_5: size-of s_6_1
        let s_6_5: u16 = s_6_1.length();
        // D s_6_6: cast reint s_6_3 -> u128
        let s_6_6: u128 = (s_6_3.value() as u128);
        // D s_6_7: size-of s_6_3
        let s_6_7: u16 = s_6_3.length();
        // D s_6_8: lsl s_6_4 s_6_7
        let s_6_8: u128 = s_6_4 << s_6_7;
        // D s_6_9: or s_6_8 s_6_6
        let s_6_9: u128 = ((s_6_8) | (s_6_6));
        // D s_6_10: add s_6_5 s_6_7
        let s_6_10: u16 = (s_6_5 + s_6_7);
        // D s_6_11: create-bits s_6_9 s_6_10
        let s_6_11: Bits = Bits::new(s_6_9, s_6_10);
        // D s_6_12: cast reint s_6_11 -> u8
        let s_6_12: u8 = (s_6_11.value() as u8);
        // D s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 5u16);
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (s_6_13.value() as i128);
        // D s_6_15: cast reint s_6_14 -> i64
        let s_6_15: i64 = (s_6_14 as i64);
        // D s_6_16: write-var d <= s_6_15
        fn_state.d = s_6_15;
        // D s_6_17: read-var N:u8
        let s_6_17: bool = fn_state.N;
        // D s_6_18: cast zx s_6_17 -> bv
        let s_6_18: Bits = Bits::new(s_6_17 as u128, 1u16);
        // D s_6_19: read-var Vn:u8
        let s_6_19: u8 = fn_state.Vn;
        // D s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 4u16);
        // D s_6_21: cast reint s_6_18 -> u128
        let s_6_21: u128 = (s_6_18.value() as u128);
        // D s_6_22: size-of s_6_18
        let s_6_22: u16 = s_6_18.length();
        // D s_6_23: cast reint s_6_20 -> u128
        let s_6_23: u128 = (s_6_20.value() as u128);
        // D s_6_24: size-of s_6_20
        let s_6_24: u16 = s_6_20.length();
        // D s_6_25: lsl s_6_21 s_6_24
        let s_6_25: u128 = s_6_21 << s_6_24;
        // D s_6_26: or s_6_25 s_6_23
        let s_6_26: u128 = ((s_6_25) | (s_6_23));
        // D s_6_27: add s_6_22 s_6_24
        let s_6_27: u16 = (s_6_22 + s_6_24);
        // D s_6_28: create-bits s_6_26 s_6_27
        let s_6_28: Bits = Bits::new(s_6_26, s_6_27);
        // D s_6_29: cast reint s_6_28 -> u8
        let s_6_29: u8 = (s_6_28.value() as u8);
        // D s_6_30: cast zx s_6_29 -> bv
        let s_6_30: Bits = Bits::new(s_6_29 as u128, 5u16);
        // D s_6_31: cast zx s_6_30 -> i
        let s_6_31: i128 = (s_6_30.value() as i128);
        // D s_6_32: cast reint s_6_31 -> i64
        let s_6_32: i64 = (s_6_31 as i64);
        // D s_6_33: write-var n <= s_6_32
        fn_state.n = s_6_32;
        // D s_6_34: read-var Q:u8
        let s_6_34: bool = fn_state.Q;
        // D s_6_35: cast zx s_6_34 -> bv
        let s_6_35: Bits = Bits::new(s_6_34 as u128, 1u16);
        // C s_6_36: const #0u : u8
        let s_6_36: bool = false;
        // C s_6_37: cast zx s_6_36 -> bv
        let s_6_37: Bits = Bits::new(s_6_36 as u128, 1u16);
        // D s_6_38: cmp-eq s_6_35 s_6_37
        let s_6_38: bool = ((s_6_35) == (s_6_37));
        // N s_6_39: branch s_6_38 b18 b7
        if s_6_38 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2s : i64
        let s_7_0: i64 = 2;
        // D s_7_1: write-var ga#359350 <= s_7_0
        fn_state.ga_359350 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#359350:i64
        let s_8_0: i64 = fn_state.ga_359350;
        // D s_8_1: write-var regs <= s_8_0
        fn_state.regs = s_8_0;
        // C s_8_2: const #8s : i64
        let s_8_2: i64 = 8;
        // D s_8_3: write-var esize <= s_8_2
        fn_state.esize = s_8_2;
        // D s_8_4: read-var size:u8
        let s_8_4: u8 = fn_state.size;
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 2u16);
        // C s_8_6: const #1u : u8
        let s_8_6: u8 = 1;
        // C s_8_7: cast zx s_8_6 -> bv
        let s_8_7: Bits = Bits::new(s_8_6 as u128, 2u16);
        // D s_8_8: cmp-eq s_8_5 s_8_7
        let s_8_8: bool = ((s_8_5) == (s_8_7));
        // N s_8_9: branch s_8_8 b17 b9
        if s_8_8 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var size:u8
        let s_10_0: u8 = fn_state.size;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #2u : u8
        let s_10_2: u8 = 2;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b16 b11
        if s_10_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var m:i
        let s_12_0: i128 = fn_state.m;
        // D s_12_1: write-var mshadow#7707 <= s_12_0
        fn_state.mshadow_7707 = s_12_0;
        // D s_12_2: read-var index:i
        let s_12_2: i128 = fn_state.index;
        // D s_12_3: write-var indexshadow#7708 <= s_12_2
        fn_state.indexshadow_7708 = s_12_2;
        // D s_12_4: read-var esize:i64
        let s_12_4: i64 = fn_state.esize;
        // D s_12_5: write-var esizeshadow#7709 <= s_12_4
        fn_state.esizeshadow_7709 = s_12_4;
        // D s_12_6: read-var elements:i
        let s_12_6: i128 = fn_state.elements;
        // D s_12_7: write-var elementsshadow#7710 <= s_12_6
        fn_state.elementsshadow_7710 = s_12_6;
        // D s_12_8: read-var mshadow#7707:i
        let s_12_8: i128 = fn_state.mshadow_7707;
        // D s_12_9: call __id(s_12_8)
        let s_12_9: i128 = u__id(state, tracer, s_12_8);
        // C s_12_10: const #0s : i
        let s_12_10: i128 = 0;
        // D s_12_11: cmp-le s_12_10 s_12_9
        let s_12_11: bool = ((s_12_10) <= (s_12_9));
        // N s_12_12: branch s_12_11 b15 b13
        if s_12_11 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#316715 <= s_13_0
        fn_state.gs_316715 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#316715:u8
        let s_14_0: bool = fn_state.gs_316715;
        // N s_14_1: assert s_14_0
        let s_14_1: () = assert!(s_14_0);
        // D s_14_2: read-var esizeshadow#7709:i64
        let s_14_2: i64 = fn_state.esizeshadow_7709;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: cast reint s_14_3 -> i64
        let s_14_4: i64 = (s_14_3 as i64);
        // D s_14_5: read-var mshadow#7707:i
        let s_14_5: i128 = fn_state.mshadow_7707;
        // D s_14_6: cast reint s_14_5 -> i64
        let s_14_6: i64 = (s_14_5 as i64);
        // D s_14_7: read-var d:i64
        let s_14_7: i64 = fn_state.d;
        // D s_14_8: read-var elementsshadow#7710:i
        let s_14_8: i128 = fn_state.elementsshadow_7710;
        // D s_14_9: read-var indexshadow#7708:i
        let s_14_9: i128 = fn_state.indexshadow_7708;
        // D s_14_10: read-var n:i64
        let s_14_10: i64 = fn_state.n;
        // D s_14_11: read-var regs:i64
        let s_14_11: i64 = fn_state.regs;
        // C s_14_12: const #1u : u8
        let s_14_12: bool = true;
        // D s_14_13: call execute_aarch32_instrs_VQRDMLSH_Op_A_txt(s_14_7, s_14_8, s_14_4, s_14_9, s_14_6, s_14_10, s_14_11, s_14_12)
        let s_14_13: () = execute_aarch32_instrs_VQRDMLSH_Op_A_txt(
            state,
            tracer,
            s_14_7,
            s_14_8,
            s_14_4,
            s_14_9,
            s_14_6,
            s_14_10,
            s_14_11,
            s_14_12,
        );
        // N s_14_14: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var mshadow#7707:i
        let s_15_0: i128 = fn_state.mshadow_7707;
        // D s_15_1: call __id(s_15_0)
        let s_15_1: i128 = u__id(state, tracer, s_15_0);
        // C s_15_2: const #31s : i
        let s_15_2: i128 = 31;
        // D s_15_3: cmp-le s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) <= (s_15_2));
        // D s_15_4: write-var gs#316715 <= s_15_3
        fn_state.gs_316715 = s_15_3;
        // N s_15_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #32s : i64
        let s_16_0: i64 = 32;
        // D s_16_1: write-var esize <= s_16_0
        fn_state.esize = s_16_0;
        // C s_16_2: const #2s : i
        let s_16_2: i128 = 2;
        // D s_16_3: write-var elements <= s_16_2
        fn_state.elements = s_16_2;
        // D s_16_4: read-var Vm:u8
        let s_16_4: u8 = fn_state.Vm;
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 4u16);
        // D s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (s_16_5.value() as i128);
        // D s_16_7: write-var m <= s_16_6
        fn_state.m = s_16_6;
        // D s_16_8: read-var M:u8
        let s_16_8: bool = fn_state.M;
        // D s_16_9: cast zx s_16_8 -> bv
        let s_16_9: Bits = Bits::new(s_16_8 as u128, 1u16);
        // D s_16_10: cast zx s_16_9 -> i
        let s_16_10: i128 = (s_16_9.value() as i128);
        // D s_16_11: write-var index <= s_16_10
        fn_state.index = s_16_10;
        // N s_16_12: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #16s : i64
        let s_17_0: i64 = 16;
        // D s_17_1: write-var esize <= s_17_0
        fn_state.esize = s_17_0;
        // C s_17_2: const #4s : i
        let s_17_2: i128 = 4;
        // D s_17_3: write-var elements <= s_17_2
        fn_state.elements = s_17_2;
        // C s_17_4: const #0s : i
        let s_17_4: i128 = 0;
        // D s_17_5: read-var Vm:u8
        let s_17_5: u8 = fn_state.Vm;
        // D s_17_6: cast zx s_17_5 -> bv
        let s_17_6: Bits = Bits::new(s_17_5 as u128, 4u16);
        // C s_17_7: const #1s : i64
        let s_17_7: i64 = 1;
        // C s_17_8: cast zx s_17_7 -> i
        let s_17_8: i128 = (i128::try_from(s_17_7).unwrap());
        // C s_17_9: const #2s : i
        let s_17_9: i128 = 2;
        // C s_17_10: add s_17_9 s_17_8
        let s_17_10: i128 = (s_17_9 + s_17_8);
        // D s_17_11: bit-extract s_17_6 s_17_4 s_17_10
        let s_17_11: Bits = (Bits::new(
            ((s_17_6) >> (s_17_4)).value(),
            u16::try_from(s_17_10).unwrap(),
        ));
        // D s_17_12: cast reint s_17_11 -> u8
        let s_17_12: u8 = (s_17_11.value() as u8);
        // D s_17_13: cast zx s_17_12 -> bv
        let s_17_13: Bits = Bits::new(s_17_12 as u128, 3u16);
        // D s_17_14: cast zx s_17_13 -> i
        let s_17_14: i128 = (s_17_13.value() as i128);
        // D s_17_15: write-var m <= s_17_14
        fn_state.m = s_17_14;
        // C s_17_16: const #3s : i
        let s_17_16: i128 = 3;
        // D s_17_17: read-var Vm:u8
        let s_17_17: u8 = fn_state.Vm;
        // D s_17_18: cast zx s_17_17 -> bv
        let s_17_18: Bits = Bits::new(s_17_17 as u128, 4u16);
        // C s_17_19: const #1u : u64
        let s_17_19: u64 = 1;
        // D s_17_20: bit-extract s_17_18 s_17_16 s_17_19
        let s_17_20: Bits = (Bits::new(
            ((s_17_18) >> (s_17_16)).value(),
            u16::try_from(s_17_19).unwrap(),
        ));
        // D s_17_21: cast reint s_17_20 -> u8
        let s_17_21: bool = ((s_17_20.value()) != 0);
        // C s_17_22: const #0s : i
        let s_17_22: i128 = 0;
        // C s_17_23: const #0u : u64
        let s_17_23: u64 = 0;
        // D s_17_24: cast zx s_17_21 -> u64
        let s_17_24: u64 = (s_17_21 as u64);
        // C s_17_25: const #1u : u64
        let s_17_25: u64 = 1;
        // D s_17_26: and s_17_24 s_17_25
        let s_17_26: u64 = ((s_17_24) & (s_17_25));
        // D s_17_27: cmp-eq s_17_26 s_17_25
        let s_17_27: bool = ((s_17_26) == (s_17_25));
        // D s_17_28: lsl s_17_24 s_17_22
        let s_17_28: u64 = s_17_24 << s_17_22;
        // D s_17_29: or s_17_23 s_17_28
        let s_17_29: u64 = ((s_17_23) | (s_17_28));
        // D s_17_30: cmpl s_17_28
        let s_17_30: u64 = !s_17_28;
        // D s_17_31: and s_17_23 s_17_30
        let s_17_31: u64 = ((s_17_23) & (s_17_30));
        // D s_17_32: select s_17_27 s_17_29 s_17_31
        let s_17_32: u64 = if s_17_27 { s_17_29 } else { s_17_31 };
        // D s_17_33: cast trunc s_17_32 -> u8
        let s_17_33: bool = ((s_17_32) != 0);
        // D s_17_34: read-var M:u8
        let s_17_34: bool = fn_state.M;
        // D s_17_35: cast zx s_17_34 -> bv
        let s_17_35: Bits = Bits::new(s_17_34 as u128, 1u16);
        // D s_17_36: cast zx s_17_33 -> bv
        let s_17_36: Bits = Bits::new(s_17_33 as u128, 1u16);
        // D s_17_37: cast reint s_17_35 -> u128
        let s_17_37: u128 = (s_17_35.value() as u128);
        // D s_17_38: size-of s_17_35
        let s_17_38: u16 = s_17_35.length();
        // D s_17_39: cast reint s_17_36 -> u128
        let s_17_39: u128 = (s_17_36.value() as u128);
        // D s_17_40: size-of s_17_36
        let s_17_40: u16 = s_17_36.length();
        // D s_17_41: lsl s_17_37 s_17_40
        let s_17_41: u128 = s_17_37 << s_17_40;
        // D s_17_42: or s_17_41 s_17_39
        let s_17_42: u128 = ((s_17_41) | (s_17_39));
        // D s_17_43: add s_17_38 s_17_40
        let s_17_43: u16 = (s_17_38 + s_17_40);
        // D s_17_44: create-bits s_17_42 s_17_43
        let s_17_44: Bits = Bits::new(s_17_42, s_17_43);
        // D s_17_45: cast reint s_17_44 -> u8
        let s_17_45: u8 = (s_17_44.value() as u8);
        // D s_17_46: cast zx s_17_45 -> bv
        let s_17_46: Bits = Bits::new(s_17_45 as u128, 2u16);
        // D s_17_47: cast zx s_17_46 -> i
        let s_17_47: i128 = (s_17_46.value() as i128);
        // D s_17_48: write-var index <= s_17_47
        fn_state.index = s_17_47;
        // N s_17_49: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1s : i64
        let s_18_0: i64 = 1;
        // D s_18_1: write-var ga#359350 <= s_18_0
        fn_state.ga_359350 = s_18_0;
        // N s_18_2: jump b8
        return block_8(state, tracer, fn_state);
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
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0s : i
        let s_20_0: i128 = 0;
        // D s_20_1: read-var Vd:u8
        let s_20_1: u8 = fn_state.Vd;
        // D s_20_2: cast zx s_20_1 -> bv
        let s_20_2: Bits = Bits::new(s_20_1 as u128, 4u16);
        // C s_20_3: const #1u : u64
        let s_20_3: u64 = 1;
        // D s_20_4: bit-extract s_20_2 s_20_0 s_20_3
        let s_20_4: Bits = (Bits::new(
            ((s_20_2) >> (s_20_0)).value(),
            u16::try_from(s_20_3).unwrap(),
        ));
        // D s_20_5: cast reint s_20_4 -> u8
        let s_20_5: bool = ((s_20_4.value()) != 0);
        // C s_20_6: const #0s : i
        let s_20_6: i128 = 0;
        // C s_20_7: const #0u : u64
        let s_20_7: u64 = 0;
        // D s_20_8: cast zx s_20_5 -> u64
        let s_20_8: u64 = (s_20_5 as u64);
        // C s_20_9: const #1u : u64
        let s_20_9: u64 = 1;
        // D s_20_10: and s_20_8 s_20_9
        let s_20_10: u64 = ((s_20_8) & (s_20_9));
        // D s_20_11: cmp-eq s_20_10 s_20_9
        let s_20_11: bool = ((s_20_10) == (s_20_9));
        // D s_20_12: lsl s_20_8 s_20_6
        let s_20_12: u64 = s_20_8 << s_20_6;
        // D s_20_13: or s_20_7 s_20_12
        let s_20_13: u64 = ((s_20_7) | (s_20_12));
        // D s_20_14: cmpl s_20_12
        let s_20_14: u64 = !s_20_12;
        // D s_20_15: and s_20_7 s_20_14
        let s_20_15: u64 = ((s_20_7) & (s_20_14));
        // D s_20_16: select s_20_11 s_20_13 s_20_15
        let s_20_16: u64 = if s_20_11 { s_20_13 } else { s_20_15 };
        // D s_20_17: cast trunc s_20_16 -> u8
        let s_20_17: bool = ((s_20_16) != 0);
        // D s_20_18: cast zx s_20_17 -> bv
        let s_20_18: Bits = Bits::new(s_20_17 as u128, 1u16);
        // C s_20_19: const #1u : u8
        let s_20_19: bool = true;
        // C s_20_20: cast zx s_20_19 -> bv
        let s_20_20: Bits = Bits::new(s_20_19 as u128, 1u16);
        // D s_20_21: cmp-eq s_20_18 s_20_20
        let s_20_21: bool = ((s_20_18) == (s_20_20));
        // N s_20_22: branch s_20_21 b23 b21
        if s_20_21 {
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
        // C s_21_0: const #0s : i
        let s_21_0: i128 = 0;
        // D s_21_1: read-var Vn:u8
        let s_21_1: u8 = fn_state.Vn;
        // D s_21_2: cast zx s_21_1 -> bv
        let s_21_2: Bits = Bits::new(s_21_1 as u128, 4u16);
        // C s_21_3: const #1u : u64
        let s_21_3: u64 = 1;
        // D s_21_4: bit-extract s_21_2 s_21_0 s_21_3
        let s_21_4: Bits = (Bits::new(
            ((s_21_2) >> (s_21_0)).value(),
            u16::try_from(s_21_3).unwrap(),
        ));
        // D s_21_5: cast reint s_21_4 -> u8
        let s_21_5: bool = ((s_21_4.value()) != 0);
        // C s_21_6: const #0s : i
        let s_21_6: i128 = 0;
        // C s_21_7: const #0u : u64
        let s_21_7: u64 = 0;
        // D s_21_8: cast zx s_21_5 -> u64
        let s_21_8: u64 = (s_21_5 as u64);
        // C s_21_9: const #1u : u64
        let s_21_9: u64 = 1;
        // D s_21_10: and s_21_8 s_21_9
        let s_21_10: u64 = ((s_21_8) & (s_21_9));
        // D s_21_11: cmp-eq s_21_10 s_21_9
        let s_21_11: bool = ((s_21_10) == (s_21_9));
        // D s_21_12: lsl s_21_8 s_21_6
        let s_21_12: u64 = s_21_8 << s_21_6;
        // D s_21_13: or s_21_7 s_21_12
        let s_21_13: u64 = ((s_21_7) | (s_21_12));
        // D s_21_14: cmpl s_21_12
        let s_21_14: u64 = !s_21_12;
        // D s_21_15: and s_21_7 s_21_14
        let s_21_15: u64 = ((s_21_7) & (s_21_14));
        // D s_21_16: select s_21_11 s_21_13 s_21_15
        let s_21_16: u64 = if s_21_11 { s_21_13 } else { s_21_15 };
        // D s_21_17: cast trunc s_21_16 -> u8
        let s_21_17: bool = ((s_21_16) != 0);
        // D s_21_18: cast zx s_21_17 -> bv
        let s_21_18: Bits = Bits::new(s_21_17 as u128, 1u16);
        // C s_21_19: const #1u : u8
        let s_21_19: bool = true;
        // C s_21_20: cast zx s_21_19 -> bv
        let s_21_20: Bits = Bits::new(s_21_19 as u128, 1u16);
        // D s_21_21: cmp-eq s_21_18 s_21_20
        let s_21_21: bool = ((s_21_18) == (s_21_20));
        // D s_21_22: write-var gs#316681 <= s_21_21
        fn_state.gs_316681 = s_21_21;
        // N s_21_23: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#316681:u8
        let s_22_0: bool = fn_state.gs_316681;
        // D s_22_1: write-var gs#316682 <= s_22_0
        fn_state.gs_316682 = s_22_0;
        // N s_22_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#316681 <= s_23_0
        fn_state.gs_316681 = s_23_0;
        // N s_23_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
}

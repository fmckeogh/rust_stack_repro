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
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_VQRDMLSH_T2enc_A_txt<T: Tracer>(
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
        gs_316758: bool,
        m: i128,
        indexshadow_7712: i128,
        esize: i64,
        n: i64,
        index: i128,
        ga_359416: i64,
        esizeshadow_7713: i64,
        regs: i64,
        gs_316759: bool,
        gs_316793: bool,
        mshadow_7711: i128,
        d: i64,
        elements: i128,
        elementsshadow_7714: i128,
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
        // N s_0_3: branch s_0_2 b28 b1
        if s_0_2 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call InITBlock(s_1_0)
        let s_1_1: bool = InITBlock(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b27 b2
        if s_1_1 {
            return block_27(state, tracer, fn_state);
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
        // C s_2_2: const #3u : u8
        let s_2_2: u8 = 3;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b26 b3
        if s_2_4 {
            return block_26(state, tracer, fn_state);
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
        // N s_3_5: branch s_3_4 b25 b4
        if s_3_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var Q:u8
        let s_4_0: bool = fn_state.Q;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b21 b5
        if s_4_4 {
            return block_21(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#316759 <= s_5_0
        fn_state.gs_316759 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#316759:u8
        let s_6_0: bool = fn_state.gs_316759;
        // N s_6_1: branch s_6_0 b20 b7
        if s_6_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var D:u8
        let s_7_0: bool = fn_state.D;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // D s_7_2: read-var Vd:u8
        let s_7_2: u8 = fn_state.Vd;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 4u16);
        // D s_7_4: cast reint s_7_1 -> u128
        let s_7_4: u128 = (s_7_1.value() as u128);
        // D s_7_5: size-of s_7_1
        let s_7_5: u16 = s_7_1.length();
        // D s_7_6: cast reint s_7_3 -> u128
        let s_7_6: u128 = (s_7_3.value() as u128);
        // D s_7_7: size-of s_7_3
        let s_7_7: u16 = s_7_3.length();
        // D s_7_8: lsl s_7_4 s_7_7
        let s_7_8: u128 = s_7_4 << s_7_7;
        // D s_7_9: or s_7_8 s_7_6
        let s_7_9: u128 = ((s_7_8) | (s_7_6));
        // D s_7_10: add s_7_5 s_7_7
        let s_7_10: u16 = (s_7_5 + s_7_7);
        // D s_7_11: create-bits s_7_9 s_7_10
        let s_7_11: Bits = Bits::new(s_7_9, s_7_10);
        // D s_7_12: cast reint s_7_11 -> u8
        let s_7_12: u8 = (s_7_11.value() as u8);
        // D s_7_13: cast zx s_7_12 -> bv
        let s_7_13: Bits = Bits::new(s_7_12 as u128, 5u16);
        // D s_7_14: cast zx s_7_13 -> i
        let s_7_14: i128 = (s_7_13.value() as i128);
        // D s_7_15: cast reint s_7_14 -> i64
        let s_7_15: i64 = (s_7_14 as i64);
        // D s_7_16: write-var d <= s_7_15
        fn_state.d = s_7_15;
        // D s_7_17: read-var N:u8
        let s_7_17: bool = fn_state.N;
        // D s_7_18: cast zx s_7_17 -> bv
        let s_7_18: Bits = Bits::new(s_7_17 as u128, 1u16);
        // D s_7_19: read-var Vn:u8
        let s_7_19: u8 = fn_state.Vn;
        // D s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 4u16);
        // D s_7_21: cast reint s_7_18 -> u128
        let s_7_21: u128 = (s_7_18.value() as u128);
        // D s_7_22: size-of s_7_18
        let s_7_22: u16 = s_7_18.length();
        // D s_7_23: cast reint s_7_20 -> u128
        let s_7_23: u128 = (s_7_20.value() as u128);
        // D s_7_24: size-of s_7_20
        let s_7_24: u16 = s_7_20.length();
        // D s_7_25: lsl s_7_21 s_7_24
        let s_7_25: u128 = s_7_21 << s_7_24;
        // D s_7_26: or s_7_25 s_7_23
        let s_7_26: u128 = ((s_7_25) | (s_7_23));
        // D s_7_27: add s_7_22 s_7_24
        let s_7_27: u16 = (s_7_22 + s_7_24);
        // D s_7_28: create-bits s_7_26 s_7_27
        let s_7_28: Bits = Bits::new(s_7_26, s_7_27);
        // D s_7_29: cast reint s_7_28 -> u8
        let s_7_29: u8 = (s_7_28.value() as u8);
        // D s_7_30: cast zx s_7_29 -> bv
        let s_7_30: Bits = Bits::new(s_7_29 as u128, 5u16);
        // D s_7_31: cast zx s_7_30 -> i
        let s_7_31: i128 = (s_7_30.value() as i128);
        // D s_7_32: cast reint s_7_31 -> i64
        let s_7_32: i64 = (s_7_31 as i64);
        // D s_7_33: write-var n <= s_7_32
        fn_state.n = s_7_32;
        // D s_7_34: read-var Q:u8
        let s_7_34: bool = fn_state.Q;
        // D s_7_35: cast zx s_7_34 -> bv
        let s_7_35: Bits = Bits::new(s_7_34 as u128, 1u16);
        // C s_7_36: const #0u : u8
        let s_7_36: bool = false;
        // C s_7_37: cast zx s_7_36 -> bv
        let s_7_37: Bits = Bits::new(s_7_36 as u128, 1u16);
        // D s_7_38: cmp-eq s_7_35 s_7_37
        let s_7_38: bool = ((s_7_35) == (s_7_37));
        // N s_7_39: branch s_7_38 b19 b8
        if s_7_38 {
            return block_19(state, tracer, fn_state);
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
        // D s_8_1: write-var ga#359416 <= s_8_0
        fn_state.ga_359416 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#359416:i64
        let s_9_0: i64 = fn_state.ga_359416;
        // D s_9_1: write-var regs <= s_9_0
        fn_state.regs = s_9_0;
        // C s_9_2: const #8s : i64
        let s_9_2: i64 = 8;
        // D s_9_3: write-var esize <= s_9_2
        fn_state.esize = s_9_2;
        // D s_9_4: read-var size:u8
        let s_9_4: u8 = fn_state.size;
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 2u16);
        // C s_9_6: const #1u : u8
        let s_9_6: u8 = 1;
        // C s_9_7: cast zx s_9_6 -> bv
        let s_9_7: Bits = Bits::new(s_9_6 as u128, 2u16);
        // D s_9_8: cmp-eq s_9_5 s_9_7
        let s_9_8: bool = ((s_9_5) == (s_9_7));
        // N s_9_9: branch s_9_8 b18 b10
        if s_9_8 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var size:u8
        let s_11_0: u8 = fn_state.size;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 2u16);
        // C s_11_2: const #2u : u8
        let s_11_2: u8 = 2;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 2u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // N s_11_5: branch s_11_4 b17 b12
        if s_11_4 {
            return block_17(state, tracer, fn_state);
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
        // D s_13_0: read-var m:i
        let s_13_0: i128 = fn_state.m;
        // D s_13_1: write-var mshadow#7711 <= s_13_0
        fn_state.mshadow_7711 = s_13_0;
        // D s_13_2: read-var index:i
        let s_13_2: i128 = fn_state.index;
        // D s_13_3: write-var indexshadow#7712 <= s_13_2
        fn_state.indexshadow_7712 = s_13_2;
        // D s_13_4: read-var esize:i64
        let s_13_4: i64 = fn_state.esize;
        // D s_13_5: write-var esizeshadow#7713 <= s_13_4
        fn_state.esizeshadow_7713 = s_13_4;
        // D s_13_6: read-var elements:i
        let s_13_6: i128 = fn_state.elements;
        // D s_13_7: write-var elementsshadow#7714 <= s_13_6
        fn_state.elementsshadow_7714 = s_13_6;
        // D s_13_8: read-var mshadow#7711:i
        let s_13_8: i128 = fn_state.mshadow_7711;
        // D s_13_9: call __id(s_13_8)
        let s_13_9: i128 = u__id(state, tracer, s_13_8);
        // C s_13_10: const #0s : i
        let s_13_10: i128 = 0;
        // D s_13_11: cmp-le s_13_10 s_13_9
        let s_13_11: bool = ((s_13_10) <= (s_13_9));
        // N s_13_12: branch s_13_11 b16 b14
        if s_13_11 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#316793 <= s_14_0
        fn_state.gs_316793 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#316793:u8
        let s_15_0: bool = fn_state.gs_316793;
        // N s_15_1: assert s_15_0
        let s_15_1: () = assert!(s_15_0);
        // D s_15_2: read-var esizeshadow#7713:i64
        let s_15_2: i64 = fn_state.esizeshadow_7713;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_4: cast reint s_15_3 -> i64
        let s_15_4: i64 = (s_15_3 as i64);
        // D s_15_5: read-var mshadow#7711:i
        let s_15_5: i128 = fn_state.mshadow_7711;
        // D s_15_6: cast reint s_15_5 -> i64
        let s_15_6: i64 = (s_15_5 as i64);
        // D s_15_7: read-var d:i64
        let s_15_7: i64 = fn_state.d;
        // D s_15_8: read-var elementsshadow#7714:i
        let s_15_8: i128 = fn_state.elementsshadow_7714;
        // D s_15_9: read-var indexshadow#7712:i
        let s_15_9: i128 = fn_state.indexshadow_7712;
        // D s_15_10: read-var n:i64
        let s_15_10: i64 = fn_state.n;
        // D s_15_11: read-var regs:i64
        let s_15_11: i64 = fn_state.regs;
        // C s_15_12: const #1u : u8
        let s_15_12: bool = true;
        // D s_15_13: call execute_aarch32_instrs_VQRDMLSH_Op_A_txt(s_15_7, s_15_8, s_15_4, s_15_9, s_15_6, s_15_10, s_15_11, s_15_12)
        let s_15_13: () = execute_aarch32_instrs_VQRDMLSH_Op_A_txt(
            state,
            tracer,
            s_15_7,
            s_15_8,
            s_15_4,
            s_15_9,
            s_15_6,
            s_15_10,
            s_15_11,
            s_15_12,
        );
        // N s_15_14: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var mshadow#7711:i
        let s_16_0: i128 = fn_state.mshadow_7711;
        // D s_16_1: call __id(s_16_0)
        let s_16_1: i128 = u__id(state, tracer, s_16_0);
        // C s_16_2: const #31s : i
        let s_16_2: i128 = 31;
        // D s_16_3: cmp-le s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) <= (s_16_2));
        // D s_16_4: write-var gs#316793 <= s_16_3
        fn_state.gs_316793 = s_16_3;
        // N s_16_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #32s : i64
        let s_17_0: i64 = 32;
        // D s_17_1: write-var esize <= s_17_0
        fn_state.esize = s_17_0;
        // C s_17_2: const #2s : i
        let s_17_2: i128 = 2;
        // D s_17_3: write-var elements <= s_17_2
        fn_state.elements = s_17_2;
        // D s_17_4: read-var Vm:u8
        let s_17_4: u8 = fn_state.Vm;
        // D s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 4u16);
        // D s_17_6: cast zx s_17_5 -> i
        let s_17_6: i128 = (s_17_5.value() as i128);
        // D s_17_7: write-var m <= s_17_6
        fn_state.m = s_17_6;
        // D s_17_8: read-var M:u8
        let s_17_8: bool = fn_state.M;
        // D s_17_9: cast zx s_17_8 -> bv
        let s_17_9: Bits = Bits::new(s_17_8 as u128, 1u16);
        // D s_17_10: cast zx s_17_9 -> i
        let s_17_10: i128 = (s_17_9.value() as i128);
        // D s_17_11: write-var index <= s_17_10
        fn_state.index = s_17_10;
        // N s_17_12: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #16s : i64
        let s_18_0: i64 = 16;
        // D s_18_1: write-var esize <= s_18_0
        fn_state.esize = s_18_0;
        // C s_18_2: const #4s : i
        let s_18_2: i128 = 4;
        // D s_18_3: write-var elements <= s_18_2
        fn_state.elements = s_18_2;
        // C s_18_4: const #0s : i
        let s_18_4: i128 = 0;
        // D s_18_5: read-var Vm:u8
        let s_18_5: u8 = fn_state.Vm;
        // D s_18_6: cast zx s_18_5 -> bv
        let s_18_6: Bits = Bits::new(s_18_5 as u128, 4u16);
        // C s_18_7: const #1s : i64
        let s_18_7: i64 = 1;
        // C s_18_8: cast zx s_18_7 -> i
        let s_18_8: i128 = (i128::try_from(s_18_7).unwrap());
        // C s_18_9: const #2s : i
        let s_18_9: i128 = 2;
        // C s_18_10: add s_18_9 s_18_8
        let s_18_10: i128 = (s_18_9 + s_18_8);
        // D s_18_11: bit-extract s_18_6 s_18_4 s_18_10
        let s_18_11: Bits = (Bits::new(
            ((s_18_6) >> (s_18_4)).value(),
            u16::try_from(s_18_10).unwrap(),
        ));
        // D s_18_12: cast reint s_18_11 -> u8
        let s_18_12: u8 = (s_18_11.value() as u8);
        // D s_18_13: cast zx s_18_12 -> bv
        let s_18_13: Bits = Bits::new(s_18_12 as u128, 3u16);
        // D s_18_14: cast zx s_18_13 -> i
        let s_18_14: i128 = (s_18_13.value() as i128);
        // D s_18_15: write-var m <= s_18_14
        fn_state.m = s_18_14;
        // C s_18_16: const #3s : i
        let s_18_16: i128 = 3;
        // D s_18_17: read-var Vm:u8
        let s_18_17: u8 = fn_state.Vm;
        // D s_18_18: cast zx s_18_17 -> bv
        let s_18_18: Bits = Bits::new(s_18_17 as u128, 4u16);
        // C s_18_19: const #1u : u64
        let s_18_19: u64 = 1;
        // D s_18_20: bit-extract s_18_18 s_18_16 s_18_19
        let s_18_20: Bits = (Bits::new(
            ((s_18_18) >> (s_18_16)).value(),
            u16::try_from(s_18_19).unwrap(),
        ));
        // D s_18_21: cast reint s_18_20 -> u8
        let s_18_21: bool = ((s_18_20.value()) != 0);
        // C s_18_22: const #0s : i
        let s_18_22: i128 = 0;
        // C s_18_23: const #0u : u64
        let s_18_23: u64 = 0;
        // D s_18_24: cast zx s_18_21 -> u64
        let s_18_24: u64 = (s_18_21 as u64);
        // C s_18_25: const #1u : u64
        let s_18_25: u64 = 1;
        // D s_18_26: and s_18_24 s_18_25
        let s_18_26: u64 = ((s_18_24) & (s_18_25));
        // D s_18_27: cmp-eq s_18_26 s_18_25
        let s_18_27: bool = ((s_18_26) == (s_18_25));
        // D s_18_28: lsl s_18_24 s_18_22
        let s_18_28: u64 = s_18_24 << s_18_22;
        // D s_18_29: or s_18_23 s_18_28
        let s_18_29: u64 = ((s_18_23) | (s_18_28));
        // D s_18_30: cmpl s_18_28
        let s_18_30: u64 = !s_18_28;
        // D s_18_31: and s_18_23 s_18_30
        let s_18_31: u64 = ((s_18_23) & (s_18_30));
        // D s_18_32: select s_18_27 s_18_29 s_18_31
        let s_18_32: u64 = if s_18_27 { s_18_29 } else { s_18_31 };
        // D s_18_33: cast trunc s_18_32 -> u8
        let s_18_33: bool = ((s_18_32) != 0);
        // D s_18_34: read-var M:u8
        let s_18_34: bool = fn_state.M;
        // D s_18_35: cast zx s_18_34 -> bv
        let s_18_35: Bits = Bits::new(s_18_34 as u128, 1u16);
        // D s_18_36: cast zx s_18_33 -> bv
        let s_18_36: Bits = Bits::new(s_18_33 as u128, 1u16);
        // D s_18_37: cast reint s_18_35 -> u128
        let s_18_37: u128 = (s_18_35.value() as u128);
        // D s_18_38: size-of s_18_35
        let s_18_38: u16 = s_18_35.length();
        // D s_18_39: cast reint s_18_36 -> u128
        let s_18_39: u128 = (s_18_36.value() as u128);
        // D s_18_40: size-of s_18_36
        let s_18_40: u16 = s_18_36.length();
        // D s_18_41: lsl s_18_37 s_18_40
        let s_18_41: u128 = s_18_37 << s_18_40;
        // D s_18_42: or s_18_41 s_18_39
        let s_18_42: u128 = ((s_18_41) | (s_18_39));
        // D s_18_43: add s_18_38 s_18_40
        let s_18_43: u16 = (s_18_38 + s_18_40);
        // D s_18_44: create-bits s_18_42 s_18_43
        let s_18_44: Bits = Bits::new(s_18_42, s_18_43);
        // D s_18_45: cast reint s_18_44 -> u8
        let s_18_45: u8 = (s_18_44.value() as u8);
        // D s_18_46: cast zx s_18_45 -> bv
        let s_18_46: Bits = Bits::new(s_18_45 as u128, 2u16);
        // D s_18_47: cast zx s_18_46 -> i
        let s_18_47: i128 = (s_18_46.value() as i128);
        // D s_18_48: write-var index <= s_18_47
        fn_state.index = s_18_47;
        // N s_18_49: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1s : i64
        let s_19_0: i64 = 1;
        // D s_19_1: write-var ga#359416 <= s_19_0
        fn_state.ga_359416 = s_19_0;
        // N s_19_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: panic
        panic!("{:?}", ());
        // N s_20_1: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0s : i
        let s_21_0: i128 = 0;
        // D s_21_1: read-var Vd:u8
        let s_21_1: u8 = fn_state.Vd;
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
        // N s_21_22: branch s_21_21 b24 b22
        if s_21_21 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0s : i
        let s_22_0: i128 = 0;
        // D s_22_1: read-var Vn:u8
        let s_22_1: u8 = fn_state.Vn;
        // D s_22_2: cast zx s_22_1 -> bv
        let s_22_2: Bits = Bits::new(s_22_1 as u128, 4u16);
        // C s_22_3: const #1u : u64
        let s_22_3: u64 = 1;
        // D s_22_4: bit-extract s_22_2 s_22_0 s_22_3
        let s_22_4: Bits = (Bits::new(
            ((s_22_2) >> (s_22_0)).value(),
            u16::try_from(s_22_3).unwrap(),
        ));
        // D s_22_5: cast reint s_22_4 -> u8
        let s_22_5: bool = ((s_22_4.value()) != 0);
        // C s_22_6: const #0s : i
        let s_22_6: i128 = 0;
        // C s_22_7: const #0u : u64
        let s_22_7: u64 = 0;
        // D s_22_8: cast zx s_22_5 -> u64
        let s_22_8: u64 = (s_22_5 as u64);
        // C s_22_9: const #1u : u64
        let s_22_9: u64 = 1;
        // D s_22_10: and s_22_8 s_22_9
        let s_22_10: u64 = ((s_22_8) & (s_22_9));
        // D s_22_11: cmp-eq s_22_10 s_22_9
        let s_22_11: bool = ((s_22_10) == (s_22_9));
        // D s_22_12: lsl s_22_8 s_22_6
        let s_22_12: u64 = s_22_8 << s_22_6;
        // D s_22_13: or s_22_7 s_22_12
        let s_22_13: u64 = ((s_22_7) | (s_22_12));
        // D s_22_14: cmpl s_22_12
        let s_22_14: u64 = !s_22_12;
        // D s_22_15: and s_22_7 s_22_14
        let s_22_15: u64 = ((s_22_7) & (s_22_14));
        // D s_22_16: select s_22_11 s_22_13 s_22_15
        let s_22_16: u64 = if s_22_11 { s_22_13 } else { s_22_15 };
        // D s_22_17: cast trunc s_22_16 -> u8
        let s_22_17: bool = ((s_22_16) != 0);
        // D s_22_18: cast zx s_22_17 -> bv
        let s_22_18: Bits = Bits::new(s_22_17 as u128, 1u16);
        // C s_22_19: const #1u : u8
        let s_22_19: bool = true;
        // C s_22_20: cast zx s_22_19 -> bv
        let s_22_20: Bits = Bits::new(s_22_19 as u128, 1u16);
        // D s_22_21: cmp-eq s_22_18 s_22_20
        let s_22_21: bool = ((s_22_18) == (s_22_20));
        // D s_22_22: write-var gs#316758 <= s_22_21
        fn_state.gs_316758 = s_22_21;
        // N s_22_23: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#316758:u8
        let s_23_0: bool = fn_state.gs_316758;
        // D s_23_1: write-var gs#316759 <= s_23_0
        fn_state.gs_316759 = s_23_0;
        // N s_23_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#316758 <= s_24_0
        fn_state.gs_316758 = s_24_0;
        // N s_24_2: jump b23
        return block_23(state, tracer, fn_state);
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
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: panic
        panic!("{:?}", ());
        // N s_27_1: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
    }
}

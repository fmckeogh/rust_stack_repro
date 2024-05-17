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
use u__id::*;
use ConditionPassed::*;
use execute_aarch32_instrs_VQRDMULH_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VQRDMULH_T2enc_A_txt<T: Tracer>(
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
        gs_316423: bool,
        esize: i64,
        n: i64,
        index: i128,
        gs_316376: bool,
        indexshadow_7694: i128,
        gs_316418: bool,
        gs_316427: bool,
        ga_359080: i64,
        gs_316416: bool,
        gs_316424: bool,
        esizeshadow_7695: i64,
        gs_316377: bool,
        gs_316420: bool,
        elementsshadow_7696: i128,
        mshadow_7693: i128,
        gs_316409: bool,
        regs: i64,
        gs_316428: bool,
        gs_316429: bool,
        gs_316426: bool,
        gs_316425: bool,
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
        // N s_2_5: branch s_2_4 b56 b3
        if s_2_4 {
            return block_56(state, tracer, fn_state);
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
        // N s_3_5: branch s_3_4 b55 b4
        if s_3_4 {
            return block_55(state, tracer, fn_state);
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
        // N s_4_5: branch s_4_4 b51 b5
        if s_4_4 {
            return block_51(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#316377 <= s_5_0
        fn_state.gs_316377 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#316377:u8
        let s_6_0: bool = fn_state.gs_316377;
        // N s_6_1: branch s_6_0 b50 b7
        if s_6_0 {
            return block_50(state, tracer, fn_state);
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
        // N s_7_39: branch s_7_38 b49 b8
        if s_7_38 {
            return block_49(state, tracer, fn_state);
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
        // D s_8_1: write-var ga#359080 <= s_8_0
        fn_state.ga_359080 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#359080:i64
        let s_9_0: i64 = fn_state.ga_359080;
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
        // N s_9_9: branch s_9_8 b48 b10
        if s_9_8 {
            return block_48(state, tracer, fn_state);
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
        // N s_11_5: branch s_11_4 b47 b12
        if s_11_4 {
            return block_47(state, tracer, fn_state);
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
        // D s_13_1: write-var mshadow#7693 <= s_13_0
        fn_state.mshadow_7693 = s_13_0;
        // D s_13_2: read-var index:i
        let s_13_2: i128 = fn_state.index;
        // D s_13_3: write-var indexshadow#7694 <= s_13_2
        fn_state.indexshadow_7694 = s_13_2;
        // D s_13_4: read-var esize:i64
        let s_13_4: i64 = fn_state.esize;
        // D s_13_5: write-var esizeshadow#7695 <= s_13_4
        fn_state.esizeshadow_7695 = s_13_4;
        // D s_13_6: read-var elements:i
        let s_13_6: i128 = fn_state.elements;
        // D s_13_7: write-var elementsshadow#7696 <= s_13_6
        fn_state.elementsshadow_7696 = s_13_6;
        // D s_13_8: read-var regs:i64
        let s_13_8: i64 = fn_state.regs;
        // D s_13_9: cast zx s_13_8 -> i
        let s_13_9: i128 = (i128::try_from(s_13_8).unwrap());
        // D s_13_10: call __id(s_13_9)
        let s_13_10: i128 = u__id(state, tracer, s_13_9);
        // D s_13_11: cast reint s_13_10 -> i64
        let s_13_11: i64 = (s_13_10 as i64);
        // C s_13_12: const #1s : i
        let s_13_12: i128 = 1;
        // D s_13_13: cast zx s_13_11 -> i
        let s_13_13: i128 = (i128::try_from(s_13_11).unwrap());
        // D s_13_14: cmp-eq s_13_13 s_13_12
        let s_13_14: bool = ((s_13_13) == (s_13_12));
        // N s_13_15: branch s_13_14 b46 b14
        if s_13_14 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var regs:i64
        let s_14_0: i64 = fn_state.regs;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: call __id(s_14_1)
        let s_14_2: i128 = u__id(state, tracer, s_14_1);
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // C s_14_4: const #2s : i
        let s_14_4: i128 = 2;
        // D s_14_5: cast zx s_14_3 -> i
        let s_14_5: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_6: cmp-eq s_14_5 s_14_4
        let s_14_6: bool = ((s_14_5) == (s_14_4));
        // D s_14_7: write-var gs#316409 <= s_14_6
        fn_state.gs_316409 = s_14_6;
        // N s_14_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#316409:u8
        let s_15_0: bool = fn_state.gs_316409;
        // N s_15_1: branch s_15_0 b18 b16
        if s_15_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#316429 <= s_16_0
        fn_state.gs_316429 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#316429:u8
        let s_17_0: bool = fn_state.gs_316429;
        // N s_17_1: assert s_17_0
        let s_17_1: () = assert!(s_17_0);
        // D s_17_2: read-var esizeshadow#7695:i64
        let s_17_2: i64 = fn_state.esizeshadow_7695;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: cast reint s_17_3 -> i64
        let s_17_4: i64 = (s_17_3 as i64);
        // D s_17_5: read-var mshadow#7693:i
        let s_17_5: i128 = fn_state.mshadow_7693;
        // D s_17_6: cast reint s_17_5 -> i64
        let s_17_6: i64 = (s_17_5 as i64);
        // D s_17_7: read-var d:i64
        let s_17_7: i64 = fn_state.d;
        // D s_17_8: read-var elementsshadow#7696:i
        let s_17_8: i128 = fn_state.elementsshadow_7696;
        // D s_17_9: read-var indexshadow#7694:i
        let s_17_9: i128 = fn_state.indexshadow_7694;
        // D s_17_10: read-var n:i64
        let s_17_10: i64 = fn_state.n;
        // D s_17_11: read-var regs:i64
        let s_17_11: i64 = fn_state.regs;
        // C s_17_12: const #1u : u8
        let s_17_12: bool = true;
        // D s_17_13: call execute_aarch32_instrs_VQRDMULH_Op_A_txt(s_17_7, s_17_8, s_17_4, s_17_9, s_17_6, s_17_10, s_17_11, s_17_12)
        let s_17_13: () = execute_aarch32_instrs_VQRDMULH_Op_A_txt(
            state,
            tracer,
            s_17_7,
            s_17_8,
            s_17_4,
            s_17_9,
            s_17_6,
            s_17_10,
            s_17_11,
            s_17_12,
        );
        // N s_17_14: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var n:i64
        let s_18_0: i64 = fn_state.n;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: call __id(s_18_1)
        let s_18_2: i128 = u__id(state, tracer, s_18_1);
        // D s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: const #0s : i
        let s_18_4: i128 = 0;
        // D s_18_5: cast zx s_18_3 -> i
        let s_18_5: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_6: cmp-le s_18_4 s_18_5
        let s_18_6: bool = ((s_18_4) <= (s_18_5));
        // N s_18_7: branch s_18_6 b21 b19
        if s_18_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#316428 <= s_19_0
        fn_state.gs_316428 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#316428:u8
        let s_20_0: bool = fn_state.gs_316428;
        // D s_20_1: write-var gs#316429 <= s_20_0
        fn_state.gs_316429 = s_20_0;
        // N s_20_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var n:i64
        let s_21_0: i64 = fn_state.n;
        // D s_21_1: cast zx s_21_0 -> i
        let s_21_1: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_2: call __id(s_21_1)
        let s_21_2: i128 = u__id(state, tracer, s_21_1);
        // D s_21_3: cast reint s_21_2 -> i64
        let s_21_3: i64 = (s_21_2 as i64);
        // C s_21_4: const #31s : i
        let s_21_4: i128 = 31;
        // D s_21_5: cast zx s_21_3 -> i
        let s_21_5: i128 = (i128::try_from(s_21_3).unwrap());
        // D s_21_6: cmp-le s_21_5 s_21_4
        let s_21_6: bool = ((s_21_5) <= (s_21_4));
        // N s_21_7: branch s_21_6 b24 b22
        if s_21_6 {
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
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#316427 <= s_22_0
        fn_state.gs_316427 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#316427:u8
        let s_23_0: bool = fn_state.gs_316427;
        // D s_23_1: write-var gs#316428 <= s_23_0
        fn_state.gs_316428 = s_23_0;
        // N s_23_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var mshadow#7693:i
        let s_24_0: i128 = fn_state.mshadow_7693;
        // D s_24_1: call __id(s_24_0)
        let s_24_1: i128 = u__id(state, tracer, s_24_0);
        // C s_24_2: const #0s : i
        let s_24_2: i128 = 0;
        // D s_24_3: cmp-le s_24_2 s_24_1
        let s_24_3: bool = ((s_24_2) <= (s_24_1));
        // N s_24_4: branch s_24_3 b27 b25
        if s_24_3 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#316426 <= s_25_0
        fn_state.gs_316426 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#316426:u8
        let s_26_0: bool = fn_state.gs_316426;
        // D s_26_1: write-var gs#316427 <= s_26_0
        fn_state.gs_316427 = s_26_0;
        // N s_26_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var mshadow#7693:i
        let s_27_0: i128 = fn_state.mshadow_7693;
        // D s_27_1: call __id(s_27_0)
        let s_27_1: i128 = u__id(state, tracer, s_27_0);
        // C s_27_2: const #31s : i
        let s_27_2: i128 = 31;
        // D s_27_3: cmp-le s_27_1 s_27_2
        let s_27_3: bool = ((s_27_1) <= (s_27_2));
        // N s_27_4: branch s_27_3 b30 b28
        if s_27_3 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#316425 <= s_28_0
        fn_state.gs_316425 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#316425:u8
        let s_29_0: bool = fn_state.gs_316425;
        // D s_29_1: write-var gs#316426 <= s_29_0
        fn_state.gs_316426 = s_29_0;
        // N s_29_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var esizeshadow#7695:i64
        let s_30_0: i64 = fn_state.esizeshadow_7695;
        // D s_30_1: cast zx s_30_0 -> i
        let s_30_1: i128 = (i128::try_from(s_30_0).unwrap());
        // D s_30_2: call __id(s_30_1)
        let s_30_2: i128 = u__id(state, tracer, s_30_1);
        // D s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: const #8s : i
        let s_30_4: i128 = 8;
        // D s_30_5: cast zx s_30_3 -> i
        let s_30_5: i128 = (i128::try_from(s_30_3).unwrap());
        // D s_30_6: cmp-eq s_30_5 s_30_4
        let s_30_6: bool = ((s_30_5) == (s_30_4));
        // N s_30_7: branch s_30_6 b45 b31
        if s_30_6 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var esizeshadow#7695:i64
        let s_31_0: i64 = fn_state.esizeshadow_7695;
        // D s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_2: call __id(s_31_1)
        let s_31_2: i128 = u__id(state, tracer, s_31_1);
        // D s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: const #16s : i
        let s_31_4: i128 = 16;
        // D s_31_5: cast zx s_31_3 -> i
        let s_31_5: i128 = (i128::try_from(s_31_3).unwrap());
        // D s_31_6: cmp-eq s_31_5 s_31_4
        let s_31_6: bool = ((s_31_5) == (s_31_4));
        // D s_31_7: write-var gs#316416 <= s_31_6
        fn_state.gs_316416 = s_31_6;
        // N s_31_8: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#316416:u8
        let s_32_0: bool = fn_state.gs_316416;
        // N s_32_1: branch s_32_0 b44 b33
        if s_32_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var esizeshadow#7695:i64
        let s_33_0: i64 = fn_state.esizeshadow_7695;
        // D s_33_1: cast zx s_33_0 -> i
        let s_33_1: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_2: call __id(s_33_1)
        let s_33_2: i128 = u__id(state, tracer, s_33_1);
        // D s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: const #32s : i
        let s_33_4: i128 = 32;
        // D s_33_5: cast zx s_33_3 -> i
        let s_33_5: i128 = (i128::try_from(s_33_3).unwrap());
        // D s_33_6: cmp-eq s_33_5 s_33_4
        let s_33_6: bool = ((s_33_5) == (s_33_4));
        // D s_33_7: write-var gs#316418 <= s_33_6
        fn_state.gs_316418 = s_33_6;
        // N s_33_8: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#316418:u8
        let s_34_0: bool = fn_state.gs_316418;
        // N s_34_1: branch s_34_0 b43 b35
        if s_34_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var esizeshadow#7695:i64
        let s_35_0: i64 = fn_state.esizeshadow_7695;
        // D s_35_1: cast zx s_35_0 -> i
        let s_35_1: i128 = (i128::try_from(s_35_0).unwrap());
        // D s_35_2: call __id(s_35_1)
        let s_35_2: i128 = u__id(state, tracer, s_35_1);
        // D s_35_3: cast reint s_35_2 -> i64
        let s_35_3: i64 = (s_35_2 as i64);
        // C s_35_4: const #64s : i
        let s_35_4: i128 = 64;
        // D s_35_5: cast zx s_35_3 -> i
        let s_35_5: i128 = (i128::try_from(s_35_3).unwrap());
        // D s_35_6: cmp-eq s_35_5 s_35_4
        let s_35_6: bool = ((s_35_5) == (s_35_4));
        // D s_35_7: write-var gs#316420 <= s_35_6
        fn_state.gs_316420 = s_35_6;
        // N s_35_8: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#316420:u8
        let s_36_0: bool = fn_state.gs_316420;
        // N s_36_1: branch s_36_0 b39 b37
        if s_36_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#316424 <= s_37_0
        fn_state.gs_316424 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#316424:u8
        let s_38_0: bool = fn_state.gs_316424;
        // D s_38_1: write-var gs#316425 <= s_38_0
        fn_state.gs_316425 = s_38_0;
        // N s_38_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var d:i64
        let s_39_0: i64 = fn_state.d;
        // D s_39_1: cast zx s_39_0 -> i
        let s_39_1: i128 = (i128::try_from(s_39_0).unwrap());
        // D s_39_2: call __id(s_39_1)
        let s_39_2: i128 = u__id(state, tracer, s_39_1);
        // D s_39_3: cast reint s_39_2 -> i64
        let s_39_3: i64 = (s_39_2 as i64);
        // C s_39_4: const #0s : i
        let s_39_4: i128 = 0;
        // D s_39_5: cast zx s_39_3 -> i
        let s_39_5: i128 = (i128::try_from(s_39_3).unwrap());
        // D s_39_6: cmp-le s_39_4 s_39_5
        let s_39_6: bool = ((s_39_4) <= (s_39_5));
        // N s_39_7: branch s_39_6 b42 b40
        if s_39_6 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#316423 <= s_40_0
        fn_state.gs_316423 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#316423:u8
        let s_41_0: bool = fn_state.gs_316423;
        // D s_41_1: write-var gs#316424 <= s_41_0
        fn_state.gs_316424 = s_41_0;
        // N s_41_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var d:i64
        let s_42_0: i64 = fn_state.d;
        // D s_42_1: cast zx s_42_0 -> i
        let s_42_1: i128 = (i128::try_from(s_42_0).unwrap());
        // D s_42_2: call __id(s_42_1)
        let s_42_2: i128 = u__id(state, tracer, s_42_1);
        // D s_42_3: cast reint s_42_2 -> i64
        let s_42_3: i64 = (s_42_2 as i64);
        // C s_42_4: const #31s : i
        let s_42_4: i128 = 31;
        // D s_42_5: cast zx s_42_3 -> i
        let s_42_5: i128 = (i128::try_from(s_42_3).unwrap());
        // D s_42_6: cmp-le s_42_5 s_42_4
        let s_42_6: bool = ((s_42_5) <= (s_42_4));
        // D s_42_7: write-var gs#316423 <= s_42_6
        fn_state.gs_316423 = s_42_6;
        // N s_42_8: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#316420 <= s_43_0
        fn_state.gs_316420 = s_43_0;
        // N s_43_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#316418 <= s_44_0
        fn_state.gs_316418 = s_44_0;
        // N s_44_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#316416 <= s_45_0
        fn_state.gs_316416 = s_45_0;
        // N s_45_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var gs#316409 <= s_46_0
        fn_state.gs_316409 = s_46_0;
        // N s_46_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #32s : i64
        let s_47_0: i64 = 32;
        // D s_47_1: write-var esize <= s_47_0
        fn_state.esize = s_47_0;
        // C s_47_2: const #2s : i
        let s_47_2: i128 = 2;
        // D s_47_3: write-var elements <= s_47_2
        fn_state.elements = s_47_2;
        // D s_47_4: read-var Vm:u8
        let s_47_4: u8 = fn_state.Vm;
        // D s_47_5: cast zx s_47_4 -> bv
        let s_47_5: Bits = Bits::new(s_47_4 as u128, 4u16);
        // D s_47_6: cast zx s_47_5 -> i
        let s_47_6: i128 = (s_47_5.value() as i128);
        // D s_47_7: write-var m <= s_47_6
        fn_state.m = s_47_6;
        // D s_47_8: read-var M:u8
        let s_47_8: bool = fn_state.M;
        // D s_47_9: cast zx s_47_8 -> bv
        let s_47_9: Bits = Bits::new(s_47_8 as u128, 1u16);
        // D s_47_10: cast zx s_47_9 -> i
        let s_47_10: i128 = (s_47_9.value() as i128);
        // D s_47_11: write-var index <= s_47_10
        fn_state.index = s_47_10;
        // N s_47_12: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #16s : i64
        let s_48_0: i64 = 16;
        // D s_48_1: write-var esize <= s_48_0
        fn_state.esize = s_48_0;
        // C s_48_2: const #4s : i
        let s_48_2: i128 = 4;
        // D s_48_3: write-var elements <= s_48_2
        fn_state.elements = s_48_2;
        // C s_48_4: const #0s : i
        let s_48_4: i128 = 0;
        // D s_48_5: read-var Vm:u8
        let s_48_5: u8 = fn_state.Vm;
        // D s_48_6: cast zx s_48_5 -> bv
        let s_48_6: Bits = Bits::new(s_48_5 as u128, 4u16);
        // C s_48_7: const #1s : i64
        let s_48_7: i64 = 1;
        // C s_48_8: cast zx s_48_7 -> i
        let s_48_8: i128 = (i128::try_from(s_48_7).unwrap());
        // C s_48_9: const #2s : i
        let s_48_9: i128 = 2;
        // C s_48_10: add s_48_9 s_48_8
        let s_48_10: i128 = (s_48_9 + s_48_8);
        // D s_48_11: bit-extract s_48_6 s_48_4 s_48_10
        let s_48_11: Bits = (Bits::new(
            ((s_48_6) >> (s_48_4)).value(),
            u16::try_from(s_48_10).unwrap(),
        ));
        // D s_48_12: cast reint s_48_11 -> u8
        let s_48_12: u8 = (s_48_11.value() as u8);
        // D s_48_13: cast zx s_48_12 -> bv
        let s_48_13: Bits = Bits::new(s_48_12 as u128, 3u16);
        // D s_48_14: cast zx s_48_13 -> i
        let s_48_14: i128 = (s_48_13.value() as i128);
        // D s_48_15: write-var m <= s_48_14
        fn_state.m = s_48_14;
        // C s_48_16: const #3s : i
        let s_48_16: i128 = 3;
        // D s_48_17: read-var Vm:u8
        let s_48_17: u8 = fn_state.Vm;
        // D s_48_18: cast zx s_48_17 -> bv
        let s_48_18: Bits = Bits::new(s_48_17 as u128, 4u16);
        // C s_48_19: const #1u : u64
        let s_48_19: u64 = 1;
        // D s_48_20: bit-extract s_48_18 s_48_16 s_48_19
        let s_48_20: Bits = (Bits::new(
            ((s_48_18) >> (s_48_16)).value(),
            u16::try_from(s_48_19).unwrap(),
        ));
        // D s_48_21: cast reint s_48_20 -> u8
        let s_48_21: bool = ((s_48_20.value()) != 0);
        // C s_48_22: const #0s : i
        let s_48_22: i128 = 0;
        // C s_48_23: const #0u : u64
        let s_48_23: u64 = 0;
        // D s_48_24: cast zx s_48_21 -> u64
        let s_48_24: u64 = (s_48_21 as u64);
        // C s_48_25: const #1u : u64
        let s_48_25: u64 = 1;
        // D s_48_26: and s_48_24 s_48_25
        let s_48_26: u64 = ((s_48_24) & (s_48_25));
        // D s_48_27: cmp-eq s_48_26 s_48_25
        let s_48_27: bool = ((s_48_26) == (s_48_25));
        // D s_48_28: lsl s_48_24 s_48_22
        let s_48_28: u64 = s_48_24 << s_48_22;
        // D s_48_29: or s_48_23 s_48_28
        let s_48_29: u64 = ((s_48_23) | (s_48_28));
        // D s_48_30: cmpl s_48_28
        let s_48_30: u64 = !s_48_28;
        // D s_48_31: and s_48_23 s_48_30
        let s_48_31: u64 = ((s_48_23) & (s_48_30));
        // D s_48_32: select s_48_27 s_48_29 s_48_31
        let s_48_32: u64 = if s_48_27 { s_48_29 } else { s_48_31 };
        // D s_48_33: cast trunc s_48_32 -> u8
        let s_48_33: bool = ((s_48_32) != 0);
        // D s_48_34: read-var M:u8
        let s_48_34: bool = fn_state.M;
        // D s_48_35: cast zx s_48_34 -> bv
        let s_48_35: Bits = Bits::new(s_48_34 as u128, 1u16);
        // D s_48_36: cast zx s_48_33 -> bv
        let s_48_36: Bits = Bits::new(s_48_33 as u128, 1u16);
        // D s_48_37: cast reint s_48_35 -> u128
        let s_48_37: u128 = (s_48_35.value() as u128);
        // D s_48_38: size-of s_48_35
        let s_48_38: u16 = s_48_35.length();
        // D s_48_39: cast reint s_48_36 -> u128
        let s_48_39: u128 = (s_48_36.value() as u128);
        // D s_48_40: size-of s_48_36
        let s_48_40: u16 = s_48_36.length();
        // D s_48_41: lsl s_48_37 s_48_40
        let s_48_41: u128 = s_48_37 << s_48_40;
        // D s_48_42: or s_48_41 s_48_39
        let s_48_42: u128 = ((s_48_41) | (s_48_39));
        // D s_48_43: add s_48_38 s_48_40
        let s_48_43: u16 = (s_48_38 + s_48_40);
        // D s_48_44: create-bits s_48_42 s_48_43
        let s_48_44: Bits = Bits::new(s_48_42, s_48_43);
        // D s_48_45: cast reint s_48_44 -> u8
        let s_48_45: u8 = (s_48_44.value() as u8);
        // D s_48_46: cast zx s_48_45 -> bv
        let s_48_46: Bits = Bits::new(s_48_45 as u128, 2u16);
        // D s_48_47: cast zx s_48_46 -> i
        let s_48_47: i128 = (s_48_46.value() as i128);
        // D s_48_48: write-var index <= s_48_47
        fn_state.index = s_48_47;
        // N s_48_49: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1s : i64
        let s_49_0: i64 = 1;
        // D s_49_1: write-var ga#359080 <= s_49_0
        fn_state.ga_359080 = s_49_0;
        // N s_49_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: panic
        panic!("{:?}", ());
        // N s_50_1: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #0s : i
        let s_51_0: i128 = 0;
        // D s_51_1: read-var Vd:u8
        let s_51_1: u8 = fn_state.Vd;
        // D s_51_2: cast zx s_51_1 -> bv
        let s_51_2: Bits = Bits::new(s_51_1 as u128, 4u16);
        // C s_51_3: const #1u : u64
        let s_51_3: u64 = 1;
        // D s_51_4: bit-extract s_51_2 s_51_0 s_51_3
        let s_51_4: Bits = (Bits::new(
            ((s_51_2) >> (s_51_0)).value(),
            u16::try_from(s_51_3).unwrap(),
        ));
        // D s_51_5: cast reint s_51_4 -> u8
        let s_51_5: bool = ((s_51_4.value()) != 0);
        // C s_51_6: const #0s : i
        let s_51_6: i128 = 0;
        // C s_51_7: const #0u : u64
        let s_51_7: u64 = 0;
        // D s_51_8: cast zx s_51_5 -> u64
        let s_51_8: u64 = (s_51_5 as u64);
        // C s_51_9: const #1u : u64
        let s_51_9: u64 = 1;
        // D s_51_10: and s_51_8 s_51_9
        let s_51_10: u64 = ((s_51_8) & (s_51_9));
        // D s_51_11: cmp-eq s_51_10 s_51_9
        let s_51_11: bool = ((s_51_10) == (s_51_9));
        // D s_51_12: lsl s_51_8 s_51_6
        let s_51_12: u64 = s_51_8 << s_51_6;
        // D s_51_13: or s_51_7 s_51_12
        let s_51_13: u64 = ((s_51_7) | (s_51_12));
        // D s_51_14: cmpl s_51_12
        let s_51_14: u64 = !s_51_12;
        // D s_51_15: and s_51_7 s_51_14
        let s_51_15: u64 = ((s_51_7) & (s_51_14));
        // D s_51_16: select s_51_11 s_51_13 s_51_15
        let s_51_16: u64 = if s_51_11 { s_51_13 } else { s_51_15 };
        // D s_51_17: cast trunc s_51_16 -> u8
        let s_51_17: bool = ((s_51_16) != 0);
        // D s_51_18: cast zx s_51_17 -> bv
        let s_51_18: Bits = Bits::new(s_51_17 as u128, 1u16);
        // C s_51_19: const #1u : u8
        let s_51_19: bool = true;
        // C s_51_20: cast zx s_51_19 -> bv
        let s_51_20: Bits = Bits::new(s_51_19 as u128, 1u16);
        // D s_51_21: cmp-eq s_51_18 s_51_20
        let s_51_21: bool = ((s_51_18) == (s_51_20));
        // N s_51_22: branch s_51_21 b54 b52
        if s_51_21 {
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
        // C s_52_0: const #0s : i
        let s_52_0: i128 = 0;
        // D s_52_1: read-var Vn:u8
        let s_52_1: u8 = fn_state.Vn;
        // D s_52_2: cast zx s_52_1 -> bv
        let s_52_2: Bits = Bits::new(s_52_1 as u128, 4u16);
        // C s_52_3: const #1u : u64
        let s_52_3: u64 = 1;
        // D s_52_4: bit-extract s_52_2 s_52_0 s_52_3
        let s_52_4: Bits = (Bits::new(
            ((s_52_2) >> (s_52_0)).value(),
            u16::try_from(s_52_3).unwrap(),
        ));
        // D s_52_5: cast reint s_52_4 -> u8
        let s_52_5: bool = ((s_52_4.value()) != 0);
        // C s_52_6: const #0s : i
        let s_52_6: i128 = 0;
        // C s_52_7: const #0u : u64
        let s_52_7: u64 = 0;
        // D s_52_8: cast zx s_52_5 -> u64
        let s_52_8: u64 = (s_52_5 as u64);
        // C s_52_9: const #1u : u64
        let s_52_9: u64 = 1;
        // D s_52_10: and s_52_8 s_52_9
        let s_52_10: u64 = ((s_52_8) & (s_52_9));
        // D s_52_11: cmp-eq s_52_10 s_52_9
        let s_52_11: bool = ((s_52_10) == (s_52_9));
        // D s_52_12: lsl s_52_8 s_52_6
        let s_52_12: u64 = s_52_8 << s_52_6;
        // D s_52_13: or s_52_7 s_52_12
        let s_52_13: u64 = ((s_52_7) | (s_52_12));
        // D s_52_14: cmpl s_52_12
        let s_52_14: u64 = !s_52_12;
        // D s_52_15: and s_52_7 s_52_14
        let s_52_15: u64 = ((s_52_7) & (s_52_14));
        // D s_52_16: select s_52_11 s_52_13 s_52_15
        let s_52_16: u64 = if s_52_11 { s_52_13 } else { s_52_15 };
        // D s_52_17: cast trunc s_52_16 -> u8
        let s_52_17: bool = ((s_52_16) != 0);
        // D s_52_18: cast zx s_52_17 -> bv
        let s_52_18: Bits = Bits::new(s_52_17 as u128, 1u16);
        // C s_52_19: const #1u : u8
        let s_52_19: bool = true;
        // C s_52_20: cast zx s_52_19 -> bv
        let s_52_20: Bits = Bits::new(s_52_19 as u128, 1u16);
        // D s_52_21: cmp-eq s_52_18 s_52_20
        let s_52_21: bool = ((s_52_18) == (s_52_20));
        // D s_52_22: write-var gs#316376 <= s_52_21
        fn_state.gs_316376 = s_52_21;
        // N s_52_23: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#316376:u8
        let s_53_0: bool = fn_state.gs_316376;
        // D s_53_1: write-var gs#316377 <= s_53_0
        fn_state.gs_316377 = s_53_0;
        // N s_53_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#316376 <= s_54_0
        fn_state.gs_316376 = s_54_0;
        // N s_54_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_55_0: panic
        panic!("{:?}", ());
        // N s_55_1: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_56_0: panic
        panic!("{:?}", ());
        // N s_56_1: return
        return;
    }
}

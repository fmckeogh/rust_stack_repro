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
use execute_aarch32_instrs_VCMLA_idx_Op_A_txt::*;
use u__id::*;
use HaveFCADDExt::*;
use fdiv_int::*;
use common::*;
pub fn decode_aarch32_instrs_VCMLA_idx_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    S: bool,
    D: bool,
    rot: u8,
    Vn: u8,
    Vd: u8,
    N: bool,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_366630: i128,
        m: i128,
        ga_366635: i64,
        regs: i64,
        esize: i128,
        gs_326435: bool,
        n: i64,
        index: i64,
        gs_326434: bool,
        d: i64,
        elements: i128,
        ga_366637: i64,
        gs_326412: bool,
        gs_326413: bool,
        gs_326433: bool,
        S: bool,
        D: bool,
        rot: u8,
        Vn: u8,
        Vd: u8,
        N: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        S,
        D,
        rot,
        Vn,
        Vd,
        N,
        Q,
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
        // S s_0_1: call HaveFCADDExt(s_0_0)
        let s_0_1: bool = HaveFCADDExt(state, tracer, s_0_0);
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
        // D s_1_0: read-var Q:u8
        let s_1_0: bool = fn_state.Q;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // C s_1_2: const #1u : u8
        let s_1_2: bool = true;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b24 b2
        if s_1_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#326413 <= s_2_0
        fn_state.gs_326413 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#326413:u8
        let s_3_0: bool = fn_state.gs_326413;
        // N s_3_1: branch s_3_0 b23 b4
        if s_3_0 {
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
        // D s_4_0: read-var D:u8
        let s_4_0: bool = fn_state.D;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // D s_4_2: read-var Vd:u8
        let s_4_2: u8 = fn_state.Vd;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 4u16);
        // D s_4_4: cast reint s_4_1 -> u128
        let s_4_4: u128 = (s_4_1.value() as u128);
        // D s_4_5: size-of s_4_1
        let s_4_5: u16 = s_4_1.length();
        // D s_4_6: cast reint s_4_3 -> u128
        let s_4_6: u128 = (s_4_3.value() as u128);
        // D s_4_7: size-of s_4_3
        let s_4_7: u16 = s_4_3.length();
        // D s_4_8: lsl s_4_4 s_4_7
        let s_4_8: u128 = s_4_4 << s_4_7;
        // D s_4_9: or s_4_8 s_4_6
        let s_4_9: u128 = ((s_4_8) | (s_4_6));
        // D s_4_10: add s_4_5 s_4_7
        let s_4_10: u16 = (s_4_5 + s_4_7);
        // D s_4_11: create-bits s_4_9 s_4_10
        let s_4_11: Bits = Bits::new(s_4_9, s_4_10);
        // D s_4_12: cast reint s_4_11 -> u8
        let s_4_12: u8 = (s_4_11.value() as u8);
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 5u16);
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (s_4_13.value() as i128);
        // D s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: write-var d <= s_4_15
        fn_state.d = s_4_15;
        // D s_4_17: read-var N:u8
        let s_4_17: bool = fn_state.N;
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 1u16);
        // D s_4_19: read-var Vn:u8
        let s_4_19: u8 = fn_state.Vn;
        // D s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 4u16);
        // D s_4_21: cast reint s_4_18 -> u128
        let s_4_21: u128 = (s_4_18.value() as u128);
        // D s_4_22: size-of s_4_18
        let s_4_22: u16 = s_4_18.length();
        // D s_4_23: cast reint s_4_20 -> u128
        let s_4_23: u128 = (s_4_20.value() as u128);
        // D s_4_24: size-of s_4_20
        let s_4_24: u16 = s_4_20.length();
        // D s_4_25: lsl s_4_21 s_4_24
        let s_4_25: u128 = s_4_21 << s_4_24;
        // D s_4_26: or s_4_25 s_4_23
        let s_4_26: u128 = ((s_4_25) | (s_4_23));
        // D s_4_27: add s_4_22 s_4_24
        let s_4_27: u16 = (s_4_22 + s_4_24);
        // D s_4_28: create-bits s_4_26 s_4_27
        let s_4_28: Bits = Bits::new(s_4_26, s_4_27);
        // D s_4_29: cast reint s_4_28 -> u8
        let s_4_29: u8 = (s_4_28.value() as u8);
        // D s_4_30: cast zx s_4_29 -> bv
        let s_4_30: Bits = Bits::new(s_4_29 as u128, 5u16);
        // D s_4_31: cast zx s_4_30 -> i
        let s_4_31: i128 = (s_4_30.value() as i128);
        // D s_4_32: cast reint s_4_31 -> i64
        let s_4_32: i64 = (s_4_31 as i64);
        // D s_4_33: write-var n <= s_4_32
        fn_state.n = s_4_32;
        // D s_4_34: read-var S:u8
        let s_4_34: bool = fn_state.S;
        // D s_4_35: cast zx s_4_34 -> bv
        let s_4_35: Bits = Bits::new(s_4_34 as u128, 1u16);
        // C s_4_36: const #1u : u8
        let s_4_36: bool = true;
        // C s_4_37: cast zx s_4_36 -> bv
        let s_4_37: Bits = Bits::new(s_4_36 as u128, 1u16);
        // D s_4_38: cmp-eq s_4_35 s_4_37
        let s_4_38: bool = ((s_4_35) == (s_4_37));
        // N s_4_39: branch s_4_38 b22 b5
        if s_4_38 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Vm:u8
        let s_5_0: u8 = fn_state.Vm;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: write-var ga#366630 <= s_5_2
        fn_state.ga_366630 = s_5_2;
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#366630:i
        let s_6_0: i128 = fn_state.ga_366630;
        // D s_6_1: write-var m <= s_6_0
        fn_state.m = s_6_0;
        // D s_6_2: read-var S:u8
        let s_6_2: bool = fn_state.S;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (s_6_3.value() as i128);
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // C s_6_6: const #16s : i
        let s_6_6: i128 = 16;
        // D s_6_7: cast zx s_6_5 -> i
        let s_6_7: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_8: lsl s_6_6 s_6_7
        let s_6_8: i128 = s_6_6 << s_6_7;
        // D s_6_9: write-var esize <= s_6_8
        fn_state.esize = s_6_8;
        // C s_6_10: const #64s : i
        let s_6_10: i128 = 64;
        // D s_6_11: read-var esize:i
        let s_6_11: i128 = fn_state.esize;
        // D s_6_12: call fdiv_int(s_6_10, s_6_11)
        let s_6_12: i128 = fdiv_int(state, tracer, s_6_10, s_6_11);
        // D s_6_13: write-var elements <= s_6_12
        fn_state.elements = s_6_12;
        // D s_6_14: read-var Q:u8
        let s_6_14: bool = fn_state.Q;
        // D s_6_15: cast zx s_6_14 -> bv
        let s_6_15: Bits = Bits::new(s_6_14 as u128, 1u16);
        // C s_6_16: const #0u : u8
        let s_6_16: bool = false;
        // C s_6_17: cast zx s_6_16 -> bv
        let s_6_17: Bits = Bits::new(s_6_16 as u128, 1u16);
        // D s_6_18: cmp-eq s_6_15 s_6_17
        let s_6_18: bool = ((s_6_15) == (s_6_17));
        // N s_6_19: branch s_6_18 b21 b7
        if s_6_18 {
            return block_21(state, tracer, fn_state);
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
        // D s_7_1: write-var ga#366635 <= s_7_0
        fn_state.ga_366635 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#366635:i64
        let s_8_0: i64 = fn_state.ga_366635;
        // D s_8_1: write-var regs <= s_8_0
        fn_state.regs = s_8_0;
        // D s_8_2: read-var S:u8
        let s_8_2: bool = fn_state.S;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // C s_8_4: const #1u : u8
        let s_8_4: bool = true;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // D s_8_6: cmp-eq s_8_3 s_8_5
        let s_8_6: bool = ((s_8_3) == (s_8_5));
        // N s_8_7: branch s_8_6 b20 b9
        if s_8_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var M:u8
        let s_9_0: bool = fn_state.M;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (s_9_1.value() as i128);
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // D s_9_4: write-var ga#366637 <= s_9_3
        fn_state.ga_366637 = s_9_3;
        // N s_9_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#366637:i64
        let s_10_0: i64 = fn_state.ga_366637;
        // D s_10_1: write-var index <= s_10_0
        fn_state.index = s_10_0;
        // D s_10_2: read-var m:i
        let s_10_2: i128 = fn_state.m;
        // D s_10_3: call __id(s_10_2)
        let s_10_3: i128 = u__id(state, tracer, s_10_2);
        // C s_10_4: const #0s : i
        let s_10_4: i128 = 0;
        // D s_10_5: cmp-le s_10_4 s_10_3
        let s_10_5: bool = ((s_10_4) <= (s_10_3));
        // N s_10_6: branch s_10_5 b13 b11
        if s_10_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#326435 <= s_11_0
        fn_state.gs_326435 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#326435:u8
        let s_12_0: bool = fn_state.gs_326435;
        // N s_12_1: assert s_12_0
        let s_12_1: () = assert!(s_12_0);
        // D s_12_2: read-var esize:i
        let s_12_2: i128 = fn_state.esize;
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // D s_12_4: read-var m:i
        let s_12_4: i128 = fn_state.m;
        // D s_12_5: cast reint s_12_4 -> i64
        let s_12_5: i64 = (s_12_4 as i64);
        // D s_12_6: read-var d:i64
        let s_12_6: i64 = fn_state.d;
        // D s_12_7: read-var elements:i
        let s_12_7: i128 = fn_state.elements;
        // D s_12_8: read-var index:i64
        let s_12_8: i64 = fn_state.index;
        // D s_12_9: read-var n:i64
        let s_12_9: i64 = fn_state.n;
        // D s_12_10: read-var regs:i64
        let s_12_10: i64 = fn_state.regs;
        // D s_12_11: read-var rot:u8
        let s_12_11: u8 = fn_state.rot;
        // D s_12_12: call execute_aarch32_instrs_VCMLA_idx_Op_A_txt(s_12_6, s_12_7, s_12_3, s_12_8, s_12_5, s_12_9, s_12_10, s_12_11)
        let s_12_12: () = execute_aarch32_instrs_VCMLA_idx_Op_A_txt(
            state,
            tracer,
            s_12_6,
            s_12_7,
            s_12_3,
            s_12_8,
            s_12_5,
            s_12_9,
            s_12_10,
            s_12_11,
        );
        // N s_12_13: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var m:i
        let s_13_0: i128 = fn_state.m;
        // D s_13_1: call __id(s_13_0)
        let s_13_1: i128 = u__id(state, tracer, s_13_0);
        // C s_13_2: const #31s : i
        let s_13_2: i128 = 31;
        // D s_13_3: cmp-le s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) <= (s_13_2));
        // N s_13_4: branch s_13_3 b16 b14
        if s_13_3 {
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
        // D s_14_1: write-var gs#326434 <= s_14_0
        fn_state.gs_326434 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#326434:u8
        let s_15_0: bool = fn_state.gs_326434;
        // D s_15_1: write-var gs#326435 <= s_15_0
        fn_state.gs_326435 = s_15_0;
        // N s_15_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var esize:i
        let s_16_0: i128 = fn_state.esize;
        // D s_16_1: call __id(s_16_0)
        let s_16_1: i128 = u__id(state, tracer, s_16_0);
        // C s_16_2: const #16s : i
        let s_16_2: i128 = 16;
        // D s_16_3: cmp-eq s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) == (s_16_2));
        // N s_16_4: branch s_16_3 b19 b17
        if s_16_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var esize:i
        let s_17_0: i128 = fn_state.esize;
        // D s_17_1: call __id(s_17_0)
        let s_17_1: i128 = u__id(state, tracer, s_17_0);
        // C s_17_2: const #32s : i
        let s_17_2: i128 = 32;
        // D s_17_3: cmp-eq s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) == (s_17_2));
        // D s_17_4: write-var gs#326433 <= s_17_3
        fn_state.gs_326433 = s_17_3;
        // N s_17_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#326433:u8
        let s_18_0: bool = fn_state.gs_326433;
        // D s_18_1: write-var gs#326434 <= s_18_0
        fn_state.gs_326434 = s_18_0;
        // N s_18_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#326433 <= s_19_0
        fn_state.gs_326433 = s_19_0;
        // N s_19_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0s : i64
        let s_20_0: i64 = 0;
        // D s_20_1: write-var ga#366637 <= s_20_0
        fn_state.ga_366637 = s_20_0;
        // N s_20_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1s : i64
        let s_21_0: i64 = 1;
        // D s_21_1: write-var ga#366635 <= s_21_0
        fn_state.ga_366635 = s_21_0;
        // N s_21_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var M:u8
        let s_22_0: bool = fn_state.M;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // D s_22_2: read-var Vm:u8
        let s_22_2: u8 = fn_state.Vm;
        // D s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 4u16);
        // D s_22_4: cast reint s_22_1 -> u128
        let s_22_4: u128 = (s_22_1.value() as u128);
        // D s_22_5: size-of s_22_1
        let s_22_5: u16 = s_22_1.length();
        // D s_22_6: cast reint s_22_3 -> u128
        let s_22_6: u128 = (s_22_3.value() as u128);
        // D s_22_7: size-of s_22_3
        let s_22_7: u16 = s_22_3.length();
        // D s_22_8: lsl s_22_4 s_22_7
        let s_22_8: u128 = s_22_4 << s_22_7;
        // D s_22_9: or s_22_8 s_22_6
        let s_22_9: u128 = ((s_22_8) | (s_22_6));
        // D s_22_10: add s_22_5 s_22_7
        let s_22_10: u16 = (s_22_5 + s_22_7);
        // D s_22_11: create-bits s_22_9 s_22_10
        let s_22_11: Bits = Bits::new(s_22_9, s_22_10);
        // D s_22_12: cast reint s_22_11 -> u8
        let s_22_12: u8 = (s_22_11.value() as u8);
        // D s_22_13: cast zx s_22_12 -> bv
        let s_22_13: Bits = Bits::new(s_22_12 as u128, 5u16);
        // D s_22_14: cast zx s_22_13 -> i
        let s_22_14: i128 = (s_22_13.value() as i128);
        // D s_22_15: write-var ga#366630 <= s_22_14
        fn_state.ga_366630 = s_22_14;
        // N s_22_16: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0s : i
        let s_24_0: i128 = 0;
        // D s_24_1: read-var Vd:u8
        let s_24_1: u8 = fn_state.Vd;
        // D s_24_2: cast zx s_24_1 -> bv
        let s_24_2: Bits = Bits::new(s_24_1 as u128, 4u16);
        // C s_24_3: const #1u : u64
        let s_24_3: u64 = 1;
        // D s_24_4: bit-extract s_24_2 s_24_0 s_24_3
        let s_24_4: Bits = (Bits::new(
            ((s_24_2) >> (s_24_0)).value(),
            u16::try_from(s_24_3).unwrap(),
        ));
        // D s_24_5: cast reint s_24_4 -> u8
        let s_24_5: bool = ((s_24_4.value()) != 0);
        // C s_24_6: const #0s : i
        let s_24_6: i128 = 0;
        // C s_24_7: const #0u : u64
        let s_24_7: u64 = 0;
        // D s_24_8: cast zx s_24_5 -> u64
        let s_24_8: u64 = (s_24_5 as u64);
        // C s_24_9: const #1u : u64
        let s_24_9: u64 = 1;
        // D s_24_10: and s_24_8 s_24_9
        let s_24_10: u64 = ((s_24_8) & (s_24_9));
        // D s_24_11: cmp-eq s_24_10 s_24_9
        let s_24_11: bool = ((s_24_10) == (s_24_9));
        // D s_24_12: lsl s_24_8 s_24_6
        let s_24_12: u64 = s_24_8 << s_24_6;
        // D s_24_13: or s_24_7 s_24_12
        let s_24_13: u64 = ((s_24_7) | (s_24_12));
        // D s_24_14: cmpl s_24_12
        let s_24_14: u64 = !s_24_12;
        // D s_24_15: and s_24_7 s_24_14
        let s_24_15: u64 = ((s_24_7) & (s_24_14));
        // D s_24_16: select s_24_11 s_24_13 s_24_15
        let s_24_16: u64 = if s_24_11 { s_24_13 } else { s_24_15 };
        // D s_24_17: cast trunc s_24_16 -> u8
        let s_24_17: bool = ((s_24_16) != 0);
        // D s_24_18: cast zx s_24_17 -> bv
        let s_24_18: Bits = Bits::new(s_24_17 as u128, 1u16);
        // C s_24_19: const #1u : u8
        let s_24_19: bool = true;
        // C s_24_20: cast zx s_24_19 -> bv
        let s_24_20: Bits = Bits::new(s_24_19 as u128, 1u16);
        // D s_24_21: cmp-eq s_24_18 s_24_20
        let s_24_21: bool = ((s_24_18) == (s_24_20));
        // N s_24_22: branch s_24_21 b27 b25
        if s_24_21 {
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
        // C s_25_0: const #0s : i
        let s_25_0: i128 = 0;
        // D s_25_1: read-var Vn:u8
        let s_25_1: u8 = fn_state.Vn;
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 4u16);
        // C s_25_3: const #1u : u64
        let s_25_3: u64 = 1;
        // D s_25_4: bit-extract s_25_2 s_25_0 s_25_3
        let s_25_4: Bits = (Bits::new(
            ((s_25_2) >> (s_25_0)).value(),
            u16::try_from(s_25_3).unwrap(),
        ));
        // D s_25_5: cast reint s_25_4 -> u8
        let s_25_5: bool = ((s_25_4.value()) != 0);
        // C s_25_6: const #0s : i
        let s_25_6: i128 = 0;
        // C s_25_7: const #0u : u64
        let s_25_7: u64 = 0;
        // D s_25_8: cast zx s_25_5 -> u64
        let s_25_8: u64 = (s_25_5 as u64);
        // C s_25_9: const #1u : u64
        let s_25_9: u64 = 1;
        // D s_25_10: and s_25_8 s_25_9
        let s_25_10: u64 = ((s_25_8) & (s_25_9));
        // D s_25_11: cmp-eq s_25_10 s_25_9
        let s_25_11: bool = ((s_25_10) == (s_25_9));
        // D s_25_12: lsl s_25_8 s_25_6
        let s_25_12: u64 = s_25_8 << s_25_6;
        // D s_25_13: or s_25_7 s_25_12
        let s_25_13: u64 = ((s_25_7) | (s_25_12));
        // D s_25_14: cmpl s_25_12
        let s_25_14: u64 = !s_25_12;
        // D s_25_15: and s_25_7 s_25_14
        let s_25_15: u64 = ((s_25_7) & (s_25_14));
        // D s_25_16: select s_25_11 s_25_13 s_25_15
        let s_25_16: u64 = if s_25_11 { s_25_13 } else { s_25_15 };
        // D s_25_17: cast trunc s_25_16 -> u8
        let s_25_17: bool = ((s_25_16) != 0);
        // D s_25_18: cast zx s_25_17 -> bv
        let s_25_18: Bits = Bits::new(s_25_17 as u128, 1u16);
        // C s_25_19: const #1u : u8
        let s_25_19: bool = true;
        // C s_25_20: cast zx s_25_19 -> bv
        let s_25_20: Bits = Bits::new(s_25_19 as u128, 1u16);
        // D s_25_21: cmp-eq s_25_18 s_25_20
        let s_25_21: bool = ((s_25_18) == (s_25_20));
        // D s_25_22: write-var gs#326412 <= s_25_21
        fn_state.gs_326412 = s_25_21;
        // N s_25_23: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#326412:u8
        let s_26_0: bool = fn_state.gs_326412;
        // D s_26_1: write-var gs#326413 <= s_26_0
        fn_state.gs_326413 = s_26_0;
        // N s_26_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#326412 <= s_27_0
        fn_state.gs_326412 = s_27_0;
        // N s_27_2: jump b26
        return block_26(state, tracer, fn_state);
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

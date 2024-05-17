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
use HaveFCADDExt::*;
use execute_aarch32_instrs_VCMLA_idx_Op_A_txt::*;
use u__id::*;
use InITBlock::*;
use fdiv_int::*;
use common::*;
pub fn decode_aarch32_instrs_VCMLA_idx_T1enc_A_txt<T: Tracer>(
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
        m: i128,
        gs_326471: bool,
        ga_366673: i64,
        ga_366666: i128,
        regs: i64,
        esize: i128,
        gs_326470: bool,
        gs_326449: bool,
        n: i64,
        index: i64,
        gs_326448: bool,
        d: i64,
        gs_326472: bool,
        elements: i128,
        ga_366671: i64,
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
        // S s_0_1: call InITBlock(s_0_0)
        let s_0_1: bool = InITBlock(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b30 b1
        if s_0_1 {
            return block_30(state, tracer, fn_state);
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
        // S s_1_1: call HaveFCADDExt(s_1_0)
        let s_1_1: bool = HaveFCADDExt(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // N s_1_3: branch s_1_2 b29 b2
        if s_1_2 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var Q:u8
        let s_2_0: bool = fn_state.Q;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b25 b3
        if s_2_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#326449 <= s_3_0
        fn_state.gs_326449 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#326449:u8
        let s_4_0: bool = fn_state.gs_326449;
        // N s_4_1: branch s_4_0 b24 b5
        if s_4_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var D:u8
        let s_5_0: bool = fn_state.D;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // D s_5_2: read-var Vd:u8
        let s_5_2: u8 = fn_state.Vd;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
        // D s_5_4: cast reint s_5_1 -> u128
        let s_5_4: u128 = (s_5_1.value() as u128);
        // D s_5_5: size-of s_5_1
        let s_5_5: u16 = s_5_1.length();
        // D s_5_6: cast reint s_5_3 -> u128
        let s_5_6: u128 = (s_5_3.value() as u128);
        // D s_5_7: size-of s_5_3
        let s_5_7: u16 = s_5_3.length();
        // D s_5_8: lsl s_5_4 s_5_7
        let s_5_8: u128 = s_5_4 << s_5_7;
        // D s_5_9: or s_5_8 s_5_6
        let s_5_9: u128 = ((s_5_8) | (s_5_6));
        // D s_5_10: add s_5_5 s_5_7
        let s_5_10: u16 = (s_5_5 + s_5_7);
        // D s_5_11: create-bits s_5_9 s_5_10
        let s_5_11: Bits = Bits::new(s_5_9, s_5_10);
        // D s_5_12: cast reint s_5_11 -> u8
        let s_5_12: u8 = (s_5_11.value() as u8);
        // D s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 5u16);
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (s_5_13.value() as i128);
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // D s_5_16: write-var d <= s_5_15
        fn_state.d = s_5_15;
        // D s_5_17: read-var N:u8
        let s_5_17: bool = fn_state.N;
        // D s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 1u16);
        // D s_5_19: read-var Vn:u8
        let s_5_19: u8 = fn_state.Vn;
        // D s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 4u16);
        // D s_5_21: cast reint s_5_18 -> u128
        let s_5_21: u128 = (s_5_18.value() as u128);
        // D s_5_22: size-of s_5_18
        let s_5_22: u16 = s_5_18.length();
        // D s_5_23: cast reint s_5_20 -> u128
        let s_5_23: u128 = (s_5_20.value() as u128);
        // D s_5_24: size-of s_5_20
        let s_5_24: u16 = s_5_20.length();
        // D s_5_25: lsl s_5_21 s_5_24
        let s_5_25: u128 = s_5_21 << s_5_24;
        // D s_5_26: or s_5_25 s_5_23
        let s_5_26: u128 = ((s_5_25) | (s_5_23));
        // D s_5_27: add s_5_22 s_5_24
        let s_5_27: u16 = (s_5_22 + s_5_24);
        // D s_5_28: create-bits s_5_26 s_5_27
        let s_5_28: Bits = Bits::new(s_5_26, s_5_27);
        // D s_5_29: cast reint s_5_28 -> u8
        let s_5_29: u8 = (s_5_28.value() as u8);
        // D s_5_30: cast zx s_5_29 -> bv
        let s_5_30: Bits = Bits::new(s_5_29 as u128, 5u16);
        // D s_5_31: cast zx s_5_30 -> i
        let s_5_31: i128 = (s_5_30.value() as i128);
        // D s_5_32: cast reint s_5_31 -> i64
        let s_5_32: i64 = (s_5_31 as i64);
        // D s_5_33: write-var n <= s_5_32
        fn_state.n = s_5_32;
        // D s_5_34: read-var S:u8
        let s_5_34: bool = fn_state.S;
        // D s_5_35: cast zx s_5_34 -> bv
        let s_5_35: Bits = Bits::new(s_5_34 as u128, 1u16);
        // C s_5_36: const #1u : u8
        let s_5_36: bool = true;
        // C s_5_37: cast zx s_5_36 -> bv
        let s_5_37: Bits = Bits::new(s_5_36 as u128, 1u16);
        // D s_5_38: cmp-eq s_5_35 s_5_37
        let s_5_38: bool = ((s_5_35) == (s_5_37));
        // N s_5_39: branch s_5_38 b23 b6
        if s_5_38 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var Vm:u8
        let s_6_0: u8 = fn_state.Vm;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 4u16);
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (s_6_1.value() as i128);
        // D s_6_3: write-var ga#366666 <= s_6_2
        fn_state.ga_366666 = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#366666:i
        let s_7_0: i128 = fn_state.ga_366666;
        // D s_7_1: write-var m <= s_7_0
        fn_state.m = s_7_0;
        // D s_7_2: read-var S:u8
        let s_7_2: bool = fn_state.S;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (s_7_3.value() as i128);
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // C s_7_6: const #16s : i
        let s_7_6: i128 = 16;
        // D s_7_7: cast zx s_7_5 -> i
        let s_7_7: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_8: lsl s_7_6 s_7_7
        let s_7_8: i128 = s_7_6 << s_7_7;
        // D s_7_9: write-var esize <= s_7_8
        fn_state.esize = s_7_8;
        // C s_7_10: const #64s : i
        let s_7_10: i128 = 64;
        // D s_7_11: read-var esize:i
        let s_7_11: i128 = fn_state.esize;
        // D s_7_12: call fdiv_int(s_7_10, s_7_11)
        let s_7_12: i128 = fdiv_int(state, tracer, s_7_10, s_7_11);
        // D s_7_13: write-var elements <= s_7_12
        fn_state.elements = s_7_12;
        // D s_7_14: read-var Q:u8
        let s_7_14: bool = fn_state.Q;
        // D s_7_15: cast zx s_7_14 -> bv
        let s_7_15: Bits = Bits::new(s_7_14 as u128, 1u16);
        // C s_7_16: const #0u : u8
        let s_7_16: bool = false;
        // C s_7_17: cast zx s_7_16 -> bv
        let s_7_17: Bits = Bits::new(s_7_16 as u128, 1u16);
        // D s_7_18: cmp-eq s_7_15 s_7_17
        let s_7_18: bool = ((s_7_15) == (s_7_17));
        // N s_7_19: branch s_7_18 b22 b8
        if s_7_18 {
            return block_22(state, tracer, fn_state);
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
        // D s_8_1: write-var ga#366671 <= s_8_0
        fn_state.ga_366671 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#366671:i64
        let s_9_0: i64 = fn_state.ga_366671;
        // D s_9_1: write-var regs <= s_9_0
        fn_state.regs = s_9_0;
        // D s_9_2: read-var S:u8
        let s_9_2: bool = fn_state.S;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // C s_9_4: const #1u : u8
        let s_9_4: bool = true;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // D s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // N s_9_7: branch s_9_6 b21 b10
        if s_9_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var M:u8
        let s_10_0: bool = fn_state.M;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (s_10_1.value() as i128);
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // D s_10_4: write-var ga#366673 <= s_10_3
        fn_state.ga_366673 = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#366673:i64
        let s_11_0: i64 = fn_state.ga_366673;
        // D s_11_1: write-var index <= s_11_0
        fn_state.index = s_11_0;
        // D s_11_2: read-var m:i
        let s_11_2: i128 = fn_state.m;
        // D s_11_3: call __id(s_11_2)
        let s_11_3: i128 = u__id(state, tracer, s_11_2);
        // C s_11_4: const #0s : i
        let s_11_4: i128 = 0;
        // D s_11_5: cmp-le s_11_4 s_11_3
        let s_11_5: bool = ((s_11_4) <= (s_11_3));
        // N s_11_6: branch s_11_5 b14 b12
        if s_11_5 {
            return block_14(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#326472 <= s_12_0
        fn_state.gs_326472 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#326472:u8
        let s_13_0: bool = fn_state.gs_326472;
        // N s_13_1: assert s_13_0
        let s_13_1: () = assert!(s_13_0);
        // D s_13_2: read-var esize:i
        let s_13_2: i128 = fn_state.esize;
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // D s_13_4: read-var m:i
        let s_13_4: i128 = fn_state.m;
        // D s_13_5: cast reint s_13_4 -> i64
        let s_13_5: i64 = (s_13_4 as i64);
        // D s_13_6: read-var d:i64
        let s_13_6: i64 = fn_state.d;
        // D s_13_7: read-var elements:i
        let s_13_7: i128 = fn_state.elements;
        // D s_13_8: read-var index:i64
        let s_13_8: i64 = fn_state.index;
        // D s_13_9: read-var n:i64
        let s_13_9: i64 = fn_state.n;
        // D s_13_10: read-var regs:i64
        let s_13_10: i64 = fn_state.regs;
        // D s_13_11: read-var rot:u8
        let s_13_11: u8 = fn_state.rot;
        // D s_13_12: call execute_aarch32_instrs_VCMLA_idx_Op_A_txt(s_13_6, s_13_7, s_13_3, s_13_8, s_13_5, s_13_9, s_13_10, s_13_11)
        let s_13_12: () = execute_aarch32_instrs_VCMLA_idx_Op_A_txt(
            state,
            tracer,
            s_13_6,
            s_13_7,
            s_13_3,
            s_13_8,
            s_13_5,
            s_13_9,
            s_13_10,
            s_13_11,
        );
        // N s_13_13: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var m:i
        let s_14_0: i128 = fn_state.m;
        // D s_14_1: call __id(s_14_0)
        let s_14_1: i128 = u__id(state, tracer, s_14_0);
        // C s_14_2: const #31s : i
        let s_14_2: i128 = 31;
        // D s_14_3: cmp-le s_14_1 s_14_2
        let s_14_3: bool = ((s_14_1) <= (s_14_2));
        // N s_14_4: branch s_14_3 b17 b15
        if s_14_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#326471 <= s_15_0
        fn_state.gs_326471 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#326471:u8
        let s_16_0: bool = fn_state.gs_326471;
        // D s_16_1: write-var gs#326472 <= s_16_0
        fn_state.gs_326472 = s_16_0;
        // N s_16_2: jump b13
        return block_13(state, tracer, fn_state);
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
        // C s_17_2: const #16s : i
        let s_17_2: i128 = 16;
        // D s_17_3: cmp-eq s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) == (s_17_2));
        // N s_17_4: branch s_17_3 b20 b18
        if s_17_3 {
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
        // D s_18_0: read-var esize:i
        let s_18_0: i128 = fn_state.esize;
        // D s_18_1: call __id(s_18_0)
        let s_18_1: i128 = u__id(state, tracer, s_18_0);
        // C s_18_2: const #32s : i
        let s_18_2: i128 = 32;
        // D s_18_3: cmp-eq s_18_1 s_18_2
        let s_18_3: bool = ((s_18_1) == (s_18_2));
        // D s_18_4: write-var gs#326470 <= s_18_3
        fn_state.gs_326470 = s_18_3;
        // N s_18_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#326470:u8
        let s_19_0: bool = fn_state.gs_326470;
        // D s_19_1: write-var gs#326471 <= s_19_0
        fn_state.gs_326471 = s_19_0;
        // N s_19_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#326470 <= s_20_0
        fn_state.gs_326470 = s_20_0;
        // N s_20_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0s : i64
        let s_21_0: i64 = 0;
        // D s_21_1: write-var ga#366673 <= s_21_0
        fn_state.ga_366673 = s_21_0;
        // N s_21_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1s : i64
        let s_22_0: i64 = 1;
        // D s_22_1: write-var ga#366671 <= s_22_0
        fn_state.ga_366671 = s_22_0;
        // N s_22_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var M:u8
        let s_23_0: bool = fn_state.M;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 1u16);
        // D s_23_2: read-var Vm:u8
        let s_23_2: u8 = fn_state.Vm;
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 4u16);
        // D s_23_4: cast reint s_23_1 -> u128
        let s_23_4: u128 = (s_23_1.value() as u128);
        // D s_23_5: size-of s_23_1
        let s_23_5: u16 = s_23_1.length();
        // D s_23_6: cast reint s_23_3 -> u128
        let s_23_6: u128 = (s_23_3.value() as u128);
        // D s_23_7: size-of s_23_3
        let s_23_7: u16 = s_23_3.length();
        // D s_23_8: lsl s_23_4 s_23_7
        let s_23_8: u128 = s_23_4 << s_23_7;
        // D s_23_9: or s_23_8 s_23_6
        let s_23_9: u128 = ((s_23_8) | (s_23_6));
        // D s_23_10: add s_23_5 s_23_7
        let s_23_10: u16 = (s_23_5 + s_23_7);
        // D s_23_11: create-bits s_23_9 s_23_10
        let s_23_11: Bits = Bits::new(s_23_9, s_23_10);
        // D s_23_12: cast reint s_23_11 -> u8
        let s_23_12: u8 = (s_23_11.value() as u8);
        // D s_23_13: cast zx s_23_12 -> bv
        let s_23_13: Bits = Bits::new(s_23_12 as u128, 5u16);
        // D s_23_14: cast zx s_23_13 -> i
        let s_23_14: i128 = (s_23_13.value() as i128);
        // D s_23_15: write-var ga#366666 <= s_23_14
        fn_state.ga_366666 = s_23_14;
        // N s_23_16: jump b7
        return block_7(state, tracer, fn_state);
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
        // C s_25_0: const #0s : i
        let s_25_0: i128 = 0;
        // D s_25_1: read-var Vd:u8
        let s_25_1: u8 = fn_state.Vd;
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
        // N s_25_22: branch s_25_21 b28 b26
        if s_25_21 {
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
        // C s_26_0: const #0s : i
        let s_26_0: i128 = 0;
        // D s_26_1: read-var Vn:u8
        let s_26_1: u8 = fn_state.Vn;
        // D s_26_2: cast zx s_26_1 -> bv
        let s_26_2: Bits = Bits::new(s_26_1 as u128, 4u16);
        // C s_26_3: const #1u : u64
        let s_26_3: u64 = 1;
        // D s_26_4: bit-extract s_26_2 s_26_0 s_26_3
        let s_26_4: Bits = (Bits::new(
            ((s_26_2) >> (s_26_0)).value(),
            u16::try_from(s_26_3).unwrap(),
        ));
        // D s_26_5: cast reint s_26_4 -> u8
        let s_26_5: bool = ((s_26_4.value()) != 0);
        // C s_26_6: const #0s : i
        let s_26_6: i128 = 0;
        // C s_26_7: const #0u : u64
        let s_26_7: u64 = 0;
        // D s_26_8: cast zx s_26_5 -> u64
        let s_26_8: u64 = (s_26_5 as u64);
        // C s_26_9: const #1u : u64
        let s_26_9: u64 = 1;
        // D s_26_10: and s_26_8 s_26_9
        let s_26_10: u64 = ((s_26_8) & (s_26_9));
        // D s_26_11: cmp-eq s_26_10 s_26_9
        let s_26_11: bool = ((s_26_10) == (s_26_9));
        // D s_26_12: lsl s_26_8 s_26_6
        let s_26_12: u64 = s_26_8 << s_26_6;
        // D s_26_13: or s_26_7 s_26_12
        let s_26_13: u64 = ((s_26_7) | (s_26_12));
        // D s_26_14: cmpl s_26_12
        let s_26_14: u64 = !s_26_12;
        // D s_26_15: and s_26_7 s_26_14
        let s_26_15: u64 = ((s_26_7) & (s_26_14));
        // D s_26_16: select s_26_11 s_26_13 s_26_15
        let s_26_16: u64 = if s_26_11 { s_26_13 } else { s_26_15 };
        // D s_26_17: cast trunc s_26_16 -> u8
        let s_26_17: bool = ((s_26_16) != 0);
        // D s_26_18: cast zx s_26_17 -> bv
        let s_26_18: Bits = Bits::new(s_26_17 as u128, 1u16);
        // C s_26_19: const #1u : u8
        let s_26_19: bool = true;
        // C s_26_20: cast zx s_26_19 -> bv
        let s_26_20: Bits = Bits::new(s_26_19 as u128, 1u16);
        // D s_26_21: cmp-eq s_26_18 s_26_20
        let s_26_21: bool = ((s_26_18) == (s_26_20));
        // D s_26_22: write-var gs#326448 <= s_26_21
        fn_state.gs_326448 = s_26_21;
        // N s_26_23: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#326448:u8
        let s_27_0: bool = fn_state.gs_326448;
        // D s_27_1: write-var gs#326449 <= s_27_0
        fn_state.gs_326449 = s_27_0;
        // N s_27_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#326448 <= s_28_0
        fn_state.gs_326448 = s_28_0;
        // N s_28_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: panic
        panic!("{:?}", ());
        // N s_29_1: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: panic
        panic!("{:?}", ());
        // N s_30_1: return
        return;
    }
}

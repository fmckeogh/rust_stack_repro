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
use u__id::*;
use execute_aarch32_instrs_VCADD_Op_A_txt::*;
use InITBlock::*;
use fdiv_int::*;
use common::*;
pub fn decode_aarch32_instrs_VCADD_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    rot: bool,
    D: bool,
    S: bool,
    Vn: u8,
    Vd: u8,
    N: bool,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_326577: bool,
        gs_326559: bool,
        regs: i64,
        esize: i128,
        gs_326556: bool,
        n: i64,
        d: i64,
        elements: i128,
        ga_366776: i64,
        gs_326560: bool,
        rot: bool,
        D: bool,
        S: bool,
        Vn: u8,
        Vd: u8,
        N: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        rot,
        D,
        S,
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
        // N s_0_2: branch s_0_1 b21 b1
        if s_0_1 {
            return block_21(state, tracer, fn_state);
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
        // N s_1_3: branch s_1_2 b20 b2
        if s_1_2 {
            return block_20(state, tracer, fn_state);
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
        // N s_2_5: branch s_2_4 b13 b3
        if s_2_4 {
            return block_13(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#326560 <= s_3_0
        fn_state.gs_326560 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#326560:u8
        let s_4_0: bool = fn_state.gs_326560;
        // N s_4_1: branch s_4_0 b12 b5
        if s_4_0 {
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
        // D s_5_34: read-var M:u8
        let s_5_34: bool = fn_state.M;
        // D s_5_35: cast zx s_5_34 -> bv
        let s_5_35: Bits = Bits::new(s_5_34 as u128, 1u16);
        // D s_5_36: read-var Vm:u8
        let s_5_36: u8 = fn_state.Vm;
        // D s_5_37: cast zx s_5_36 -> bv
        let s_5_37: Bits = Bits::new(s_5_36 as u128, 4u16);
        // D s_5_38: cast reint s_5_35 -> u128
        let s_5_38: u128 = (s_5_35.value() as u128);
        // D s_5_39: size-of s_5_35
        let s_5_39: u16 = s_5_35.length();
        // D s_5_40: cast reint s_5_37 -> u128
        let s_5_40: u128 = (s_5_37.value() as u128);
        // D s_5_41: size-of s_5_37
        let s_5_41: u16 = s_5_37.length();
        // D s_5_42: lsl s_5_38 s_5_41
        let s_5_42: u128 = s_5_38 << s_5_41;
        // D s_5_43: or s_5_42 s_5_40
        let s_5_43: u128 = ((s_5_42) | (s_5_40));
        // D s_5_44: add s_5_39 s_5_41
        let s_5_44: u16 = (s_5_39 + s_5_41);
        // D s_5_45: create-bits s_5_43 s_5_44
        let s_5_45: Bits = Bits::new(s_5_43, s_5_44);
        // D s_5_46: cast reint s_5_45 -> u8
        let s_5_46: u8 = (s_5_45.value() as u8);
        // D s_5_47: cast zx s_5_46 -> bv
        let s_5_47: Bits = Bits::new(s_5_46 as u128, 5u16);
        // D s_5_48: cast zx s_5_47 -> i
        let s_5_48: i128 = (s_5_47.value() as i128);
        // D s_5_49: cast reint s_5_48 -> i64
        let s_5_49: i64 = (s_5_48 as i64);
        // D s_5_50: write-var m <= s_5_49
        fn_state.m = s_5_49;
        // D s_5_51: read-var S:u8
        let s_5_51: bool = fn_state.S;
        // D s_5_52: cast zx s_5_51 -> bv
        let s_5_52: Bits = Bits::new(s_5_51 as u128, 1u16);
        // D s_5_53: cast zx s_5_52 -> i
        let s_5_53: i128 = (s_5_52.value() as i128);
        // D s_5_54: cast reint s_5_53 -> i64
        let s_5_54: i64 = (s_5_53 as i64);
        // C s_5_55: const #16s : i
        let s_5_55: i128 = 16;
        // D s_5_56: cast zx s_5_54 -> i
        let s_5_56: i128 = (i128::try_from(s_5_54).unwrap());
        // D s_5_57: lsl s_5_55 s_5_56
        let s_5_57: i128 = s_5_55 << s_5_56;
        // D s_5_58: write-var esize <= s_5_57
        fn_state.esize = s_5_57;
        // C s_5_59: const #64s : i
        let s_5_59: i128 = 64;
        // D s_5_60: read-var esize:i
        let s_5_60: i128 = fn_state.esize;
        // D s_5_61: call fdiv_int(s_5_59, s_5_60)
        let s_5_61: i128 = fdiv_int(state, tracer, s_5_59, s_5_60);
        // D s_5_62: write-var elements <= s_5_61
        fn_state.elements = s_5_61;
        // D s_5_63: read-var Q:u8
        let s_5_63: bool = fn_state.Q;
        // D s_5_64: cast zx s_5_63 -> bv
        let s_5_64: Bits = Bits::new(s_5_63 as u128, 1u16);
        // C s_5_65: const #0u : u8
        let s_5_65: bool = false;
        // C s_5_66: cast zx s_5_65 -> bv
        let s_5_66: Bits = Bits::new(s_5_65 as u128, 1u16);
        // D s_5_67: cmp-eq s_5_64 s_5_66
        let s_5_67: bool = ((s_5_64) == (s_5_66));
        // N s_5_68: branch s_5_67 b11 b6
        if s_5_67 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #2s : i64
        let s_6_0: i64 = 2;
        // D s_6_1: write-var ga#366776 <= s_6_0
        fn_state.ga_366776 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#366776:i64
        let s_7_0: i64 = fn_state.ga_366776;
        // D s_7_1: write-var regs <= s_7_0
        fn_state.regs = s_7_0;
        // D s_7_2: read-var esize:i
        let s_7_2: i128 = fn_state.esize;
        // D s_7_3: call __id(s_7_2)
        let s_7_3: i128 = u__id(state, tracer, s_7_2);
        // C s_7_4: const #16s : i
        let s_7_4: i128 = 16;
        // D s_7_5: cmp-eq s_7_3 s_7_4
        let s_7_5: bool = ((s_7_3) == (s_7_4));
        // N s_7_6: branch s_7_5 b10 b8
        if s_7_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esize:i
        let s_8_0: i128 = fn_state.esize;
        // D s_8_1: call __id(s_8_0)
        let s_8_1: i128 = u__id(state, tracer, s_8_0);
        // C s_8_2: const #32s : i
        let s_8_2: i128 = 32;
        // D s_8_3: cmp-eq s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) == (s_8_2));
        // D s_8_4: write-var gs#326577 <= s_8_3
        fn_state.gs_326577 = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#326577:u8
        let s_9_0: bool = fn_state.gs_326577;
        // N s_9_1: assert s_9_0
        let s_9_1: () = assert!(s_9_0);
        // D s_9_2: read-var esize:i
        let s_9_2: i128 = fn_state.esize;
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // D s_9_4: read-var d:i64
        let s_9_4: i64 = fn_state.d;
        // D s_9_5: read-var elements:i
        let s_9_5: i128 = fn_state.elements;
        // D s_9_6: read-var m:i64
        let s_9_6: i64 = fn_state.m;
        // D s_9_7: read-var n:i64
        let s_9_7: i64 = fn_state.n;
        // D s_9_8: read-var regs:i64
        let s_9_8: i64 = fn_state.regs;
        // D s_9_9: read-var rot:u8
        let s_9_9: bool = fn_state.rot;
        // D s_9_10: call execute_aarch32_instrs_VCADD_Op_A_txt(s_9_4, s_9_5, s_9_3, s_9_6, s_9_7, s_9_8, s_9_9)
        let s_9_10: () = execute_aarch32_instrs_VCADD_Op_A_txt(
            state,
            tracer,
            s_9_4,
            s_9_5,
            s_9_3,
            s_9_6,
            s_9_7,
            s_9_8,
            s_9_9,
        );
        // N s_9_11: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#326577 <= s_10_0
        fn_state.gs_326577 = s_10_0;
        // N s_10_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1s : i64
        let s_11_0: i64 = 1;
        // D s_11_1: write-var ga#366776 <= s_11_0
        fn_state.ga_366776 = s_11_0;
        // N s_11_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var Vd:u8
        let s_13_1: u8 = fn_state.Vd;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 4u16);
        // C s_13_3: const #1u : u64
        let s_13_3: u64 = 1;
        // D s_13_4: bit-extract s_13_2 s_13_0 s_13_3
        let s_13_4: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_3).unwrap(),
        ));
        // D s_13_5: cast reint s_13_4 -> u8
        let s_13_5: bool = ((s_13_4.value()) != 0);
        // C s_13_6: const #0s : i
        let s_13_6: i128 = 0;
        // C s_13_7: const #0u : u64
        let s_13_7: u64 = 0;
        // D s_13_8: cast zx s_13_5 -> u64
        let s_13_8: u64 = (s_13_5 as u64);
        // C s_13_9: const #1u : u64
        let s_13_9: u64 = 1;
        // D s_13_10: and s_13_8 s_13_9
        let s_13_10: u64 = ((s_13_8) & (s_13_9));
        // D s_13_11: cmp-eq s_13_10 s_13_9
        let s_13_11: bool = ((s_13_10) == (s_13_9));
        // D s_13_12: lsl s_13_8 s_13_6
        let s_13_12: u64 = s_13_8 << s_13_6;
        // D s_13_13: or s_13_7 s_13_12
        let s_13_13: u64 = ((s_13_7) | (s_13_12));
        // D s_13_14: cmpl s_13_12
        let s_13_14: u64 = !s_13_12;
        // D s_13_15: and s_13_7 s_13_14
        let s_13_15: u64 = ((s_13_7) & (s_13_14));
        // D s_13_16: select s_13_11 s_13_13 s_13_15
        let s_13_16: u64 = if s_13_11 { s_13_13 } else { s_13_15 };
        // D s_13_17: cast trunc s_13_16 -> u8
        let s_13_17: bool = ((s_13_16) != 0);
        // D s_13_18: cast zx s_13_17 -> bv
        let s_13_18: Bits = Bits::new(s_13_17 as u128, 1u16);
        // C s_13_19: const #1u : u8
        let s_13_19: bool = true;
        // C s_13_20: cast zx s_13_19 -> bv
        let s_13_20: Bits = Bits::new(s_13_19 as u128, 1u16);
        // D s_13_21: cmp-eq s_13_18 s_13_20
        let s_13_21: bool = ((s_13_18) == (s_13_20));
        // N s_13_22: branch s_13_21 b19 b14
        if s_13_21 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0s : i
        let s_14_0: i128 = 0;
        // D s_14_1: read-var Vn:u8
        let s_14_1: u8 = fn_state.Vn;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 4u16);
        // C s_14_3: const #1u : u64
        let s_14_3: u64 = 1;
        // D s_14_4: bit-extract s_14_2 s_14_0 s_14_3
        let s_14_4: Bits = (Bits::new(
            ((s_14_2) >> (s_14_0)).value(),
            u16::try_from(s_14_3).unwrap(),
        ));
        // D s_14_5: cast reint s_14_4 -> u8
        let s_14_5: bool = ((s_14_4.value()) != 0);
        // C s_14_6: const #0s : i
        let s_14_6: i128 = 0;
        // C s_14_7: const #0u : u64
        let s_14_7: u64 = 0;
        // D s_14_8: cast zx s_14_5 -> u64
        let s_14_8: u64 = (s_14_5 as u64);
        // C s_14_9: const #1u : u64
        let s_14_9: u64 = 1;
        // D s_14_10: and s_14_8 s_14_9
        let s_14_10: u64 = ((s_14_8) & (s_14_9));
        // D s_14_11: cmp-eq s_14_10 s_14_9
        let s_14_11: bool = ((s_14_10) == (s_14_9));
        // D s_14_12: lsl s_14_8 s_14_6
        let s_14_12: u64 = s_14_8 << s_14_6;
        // D s_14_13: or s_14_7 s_14_12
        let s_14_13: u64 = ((s_14_7) | (s_14_12));
        // D s_14_14: cmpl s_14_12
        let s_14_14: u64 = !s_14_12;
        // D s_14_15: and s_14_7 s_14_14
        let s_14_15: u64 = ((s_14_7) & (s_14_14));
        // D s_14_16: select s_14_11 s_14_13 s_14_15
        let s_14_16: u64 = if s_14_11 { s_14_13 } else { s_14_15 };
        // D s_14_17: cast trunc s_14_16 -> u8
        let s_14_17: bool = ((s_14_16) != 0);
        // D s_14_18: cast zx s_14_17 -> bv
        let s_14_18: Bits = Bits::new(s_14_17 as u128, 1u16);
        // C s_14_19: const #1u : u8
        let s_14_19: bool = true;
        // C s_14_20: cast zx s_14_19 -> bv
        let s_14_20: Bits = Bits::new(s_14_19 as u128, 1u16);
        // D s_14_21: cmp-eq s_14_18 s_14_20
        let s_14_21: bool = ((s_14_18) == (s_14_20));
        // D s_14_22: write-var gs#326556 <= s_14_21
        fn_state.gs_326556 = s_14_21;
        // N s_14_23: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#326556:u8
        let s_15_0: bool = fn_state.gs_326556;
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
        // C s_16_0: const #0s : i
        let s_16_0: i128 = 0;
        // D s_16_1: read-var Vm:u8
        let s_16_1: u8 = fn_state.Vm;
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 4u16);
        // C s_16_3: const #1u : u64
        let s_16_3: u64 = 1;
        // D s_16_4: bit-extract s_16_2 s_16_0 s_16_3
        let s_16_4: Bits = (Bits::new(
            ((s_16_2) >> (s_16_0)).value(),
            u16::try_from(s_16_3).unwrap(),
        ));
        // D s_16_5: cast reint s_16_4 -> u8
        let s_16_5: bool = ((s_16_4.value()) != 0);
        // C s_16_6: const #0s : i
        let s_16_6: i128 = 0;
        // C s_16_7: const #0u : u64
        let s_16_7: u64 = 0;
        // D s_16_8: cast zx s_16_5 -> u64
        let s_16_8: u64 = (s_16_5 as u64);
        // C s_16_9: const #1u : u64
        let s_16_9: u64 = 1;
        // D s_16_10: and s_16_8 s_16_9
        let s_16_10: u64 = ((s_16_8) & (s_16_9));
        // D s_16_11: cmp-eq s_16_10 s_16_9
        let s_16_11: bool = ((s_16_10) == (s_16_9));
        // D s_16_12: lsl s_16_8 s_16_6
        let s_16_12: u64 = s_16_8 << s_16_6;
        // D s_16_13: or s_16_7 s_16_12
        let s_16_13: u64 = ((s_16_7) | (s_16_12));
        // D s_16_14: cmpl s_16_12
        let s_16_14: u64 = !s_16_12;
        // D s_16_15: and s_16_7 s_16_14
        let s_16_15: u64 = ((s_16_7) & (s_16_14));
        // D s_16_16: select s_16_11 s_16_13 s_16_15
        let s_16_16: u64 = if s_16_11 { s_16_13 } else { s_16_15 };
        // D s_16_17: cast trunc s_16_16 -> u8
        let s_16_17: bool = ((s_16_16) != 0);
        // D s_16_18: cast zx s_16_17 -> bv
        let s_16_18: Bits = Bits::new(s_16_17 as u128, 1u16);
        // C s_16_19: const #1u : u8
        let s_16_19: bool = true;
        // C s_16_20: cast zx s_16_19 -> bv
        let s_16_20: Bits = Bits::new(s_16_19 as u128, 1u16);
        // D s_16_21: cmp-eq s_16_18 s_16_20
        let s_16_21: bool = ((s_16_18) == (s_16_20));
        // D s_16_22: write-var gs#326559 <= s_16_21
        fn_state.gs_326559 = s_16_21;
        // N s_16_23: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#326559:u8
        let s_17_0: bool = fn_state.gs_326559;
        // D s_17_1: write-var gs#326560 <= s_17_0
        fn_state.gs_326560 = s_17_0;
        // N s_17_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#326559 <= s_18_0
        fn_state.gs_326559 = s_18_0;
        // N s_18_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#326556 <= s_19_0
        fn_state.gs_326556 = s_19_0;
        // N s_19_2: jump b15
        return block_15(state, tracer, fn_state);
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
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: return
        return;
    }
}

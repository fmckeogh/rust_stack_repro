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
use execute_aarch32_instrs_VCMLA_Op_A_txt::*;
use u__id::*;
use HaveFCADDExt::*;
use fdiv_int::*;
use common::*;
pub fn decode_aarch32_instrs_VCMLA_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    rot: u8,
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
        gs_326297: bool,
        gs_326281: bool,
        gs_326277: bool,
        esize: i128,
        regs: i64,
        n: i64,
        d: i64,
        ga_366506: i64,
        elements: i128,
        gs_326280: bool,
        rot: u8,
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
        // S s_0_1: call HaveFCADDExt(s_0_0)
        let s_0_1: bool = HaveFCADDExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b19 b1
        if s_0_2 {
            return block_19(state, tracer, fn_state);
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
        // N s_1_5: branch s_1_4 b12 b2
        if s_1_4 {
            return block_12(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#326281 <= s_2_0
        fn_state.gs_326281 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#326281:u8
        let s_3_0: bool = fn_state.gs_326281;
        // N s_3_1: branch s_3_0 b11 b4
        if s_3_0 {
            return block_11(state, tracer, fn_state);
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
        // D s_4_34: read-var M:u8
        let s_4_34: bool = fn_state.M;
        // D s_4_35: cast zx s_4_34 -> bv
        let s_4_35: Bits = Bits::new(s_4_34 as u128, 1u16);
        // D s_4_36: read-var Vm:u8
        let s_4_36: u8 = fn_state.Vm;
        // D s_4_37: cast zx s_4_36 -> bv
        let s_4_37: Bits = Bits::new(s_4_36 as u128, 4u16);
        // D s_4_38: cast reint s_4_35 -> u128
        let s_4_38: u128 = (s_4_35.value() as u128);
        // D s_4_39: size-of s_4_35
        let s_4_39: u16 = s_4_35.length();
        // D s_4_40: cast reint s_4_37 -> u128
        let s_4_40: u128 = (s_4_37.value() as u128);
        // D s_4_41: size-of s_4_37
        let s_4_41: u16 = s_4_37.length();
        // D s_4_42: lsl s_4_38 s_4_41
        let s_4_42: u128 = s_4_38 << s_4_41;
        // D s_4_43: or s_4_42 s_4_40
        let s_4_43: u128 = ((s_4_42) | (s_4_40));
        // D s_4_44: add s_4_39 s_4_41
        let s_4_44: u16 = (s_4_39 + s_4_41);
        // D s_4_45: create-bits s_4_43 s_4_44
        let s_4_45: Bits = Bits::new(s_4_43, s_4_44);
        // D s_4_46: cast reint s_4_45 -> u8
        let s_4_46: u8 = (s_4_45.value() as u8);
        // D s_4_47: cast zx s_4_46 -> bv
        let s_4_47: Bits = Bits::new(s_4_46 as u128, 5u16);
        // D s_4_48: cast zx s_4_47 -> i
        let s_4_48: i128 = (s_4_47.value() as i128);
        // D s_4_49: cast reint s_4_48 -> i64
        let s_4_49: i64 = (s_4_48 as i64);
        // D s_4_50: write-var m <= s_4_49
        fn_state.m = s_4_49;
        // D s_4_51: read-var S:u8
        let s_4_51: bool = fn_state.S;
        // D s_4_52: cast zx s_4_51 -> bv
        let s_4_52: Bits = Bits::new(s_4_51 as u128, 1u16);
        // D s_4_53: cast zx s_4_52 -> i
        let s_4_53: i128 = (s_4_52.value() as i128);
        // D s_4_54: cast reint s_4_53 -> i64
        let s_4_54: i64 = (s_4_53 as i64);
        // C s_4_55: const #16s : i
        let s_4_55: i128 = 16;
        // D s_4_56: cast zx s_4_54 -> i
        let s_4_56: i128 = (i128::try_from(s_4_54).unwrap());
        // D s_4_57: lsl s_4_55 s_4_56
        let s_4_57: i128 = s_4_55 << s_4_56;
        // D s_4_58: write-var esize <= s_4_57
        fn_state.esize = s_4_57;
        // C s_4_59: const #64s : i
        let s_4_59: i128 = 64;
        // D s_4_60: read-var esize:i
        let s_4_60: i128 = fn_state.esize;
        // D s_4_61: call fdiv_int(s_4_59, s_4_60)
        let s_4_61: i128 = fdiv_int(state, tracer, s_4_59, s_4_60);
        // D s_4_62: write-var elements <= s_4_61
        fn_state.elements = s_4_61;
        // D s_4_63: read-var Q:u8
        let s_4_63: bool = fn_state.Q;
        // D s_4_64: cast zx s_4_63 -> bv
        let s_4_64: Bits = Bits::new(s_4_63 as u128, 1u16);
        // C s_4_65: const #0u : u8
        let s_4_65: bool = false;
        // C s_4_66: cast zx s_4_65 -> bv
        let s_4_66: Bits = Bits::new(s_4_65 as u128, 1u16);
        // D s_4_67: cmp-eq s_4_64 s_4_66
        let s_4_67: bool = ((s_4_64) == (s_4_66));
        // N s_4_68: branch s_4_67 b10 b5
        if s_4_67 {
            return block_10(state, tracer, fn_state);
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
        // D s_5_1: write-var ga#366506 <= s_5_0
        fn_state.ga_366506 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#366506:i64
        let s_6_0: i64 = fn_state.ga_366506;
        // D s_6_1: write-var regs <= s_6_0
        fn_state.regs = s_6_0;
        // D s_6_2: read-var esize:i
        let s_6_2: i128 = fn_state.esize;
        // D s_6_3: call __id(s_6_2)
        let s_6_3: i128 = u__id(state, tracer, s_6_2);
        // C s_6_4: const #16s : i
        let s_6_4: i128 = 16;
        // D s_6_5: cmp-eq s_6_3 s_6_4
        let s_6_5: bool = ((s_6_3) == (s_6_4));
        // N s_6_6: branch s_6_5 b9 b7
        if s_6_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esize:i
        let s_7_0: i128 = fn_state.esize;
        // D s_7_1: call __id(s_7_0)
        let s_7_1: i128 = u__id(state, tracer, s_7_0);
        // C s_7_2: const #32s : i
        let s_7_2: i128 = 32;
        // D s_7_3: cmp-eq s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) == (s_7_2));
        // D s_7_4: write-var gs#326297 <= s_7_3
        fn_state.gs_326297 = s_7_3;
        // N s_7_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#326297:u8
        let s_8_0: bool = fn_state.gs_326297;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // D s_8_2: read-var esize:i
        let s_8_2: i128 = fn_state.esize;
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // D s_8_4: read-var d:i64
        let s_8_4: i64 = fn_state.d;
        // D s_8_5: read-var elements:i
        let s_8_5: i128 = fn_state.elements;
        // D s_8_6: read-var m:i64
        let s_8_6: i64 = fn_state.m;
        // D s_8_7: read-var n:i64
        let s_8_7: i64 = fn_state.n;
        // D s_8_8: read-var regs:i64
        let s_8_8: i64 = fn_state.regs;
        // D s_8_9: read-var rot:u8
        let s_8_9: u8 = fn_state.rot;
        // D s_8_10: call execute_aarch32_instrs_VCMLA_Op_A_txt(s_8_4, s_8_5, s_8_3, s_8_6, s_8_7, s_8_8, s_8_9)
        let s_8_10: () = execute_aarch32_instrs_VCMLA_Op_A_txt(
            state,
            tracer,
            s_8_4,
            s_8_5,
            s_8_3,
            s_8_6,
            s_8_7,
            s_8_8,
            s_8_9,
        );
        // N s_8_11: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#326297 <= s_9_0
        fn_state.gs_326297 = s_9_0;
        // N s_9_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1s : i64
        let s_10_0: i64 = 1;
        // D s_10_1: write-var ga#366506 <= s_10_0
        fn_state.ga_366506 = s_10_0;
        // N s_10_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0s : i
        let s_12_0: i128 = 0;
        // D s_12_1: read-var Vd:u8
        let s_12_1: u8 = fn_state.Vd;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 4u16);
        // C s_12_3: const #1u : u64
        let s_12_3: u64 = 1;
        // D s_12_4: bit-extract s_12_2 s_12_0 s_12_3
        let s_12_4: Bits = (Bits::new(
            ((s_12_2) >> (s_12_0)).value(),
            u16::try_from(s_12_3).unwrap(),
        ));
        // D s_12_5: cast reint s_12_4 -> u8
        let s_12_5: bool = ((s_12_4.value()) != 0);
        // C s_12_6: const #0s : i
        let s_12_6: i128 = 0;
        // C s_12_7: const #0u : u64
        let s_12_7: u64 = 0;
        // D s_12_8: cast zx s_12_5 -> u64
        let s_12_8: u64 = (s_12_5 as u64);
        // C s_12_9: const #1u : u64
        let s_12_9: u64 = 1;
        // D s_12_10: and s_12_8 s_12_9
        let s_12_10: u64 = ((s_12_8) & (s_12_9));
        // D s_12_11: cmp-eq s_12_10 s_12_9
        let s_12_11: bool = ((s_12_10) == (s_12_9));
        // D s_12_12: lsl s_12_8 s_12_6
        let s_12_12: u64 = s_12_8 << s_12_6;
        // D s_12_13: or s_12_7 s_12_12
        let s_12_13: u64 = ((s_12_7) | (s_12_12));
        // D s_12_14: cmpl s_12_12
        let s_12_14: u64 = !s_12_12;
        // D s_12_15: and s_12_7 s_12_14
        let s_12_15: u64 = ((s_12_7) & (s_12_14));
        // D s_12_16: select s_12_11 s_12_13 s_12_15
        let s_12_16: u64 = if s_12_11 { s_12_13 } else { s_12_15 };
        // D s_12_17: cast trunc s_12_16 -> u8
        let s_12_17: bool = ((s_12_16) != 0);
        // D s_12_18: cast zx s_12_17 -> bv
        let s_12_18: Bits = Bits::new(s_12_17 as u128, 1u16);
        // C s_12_19: const #1u : u8
        let s_12_19: bool = true;
        // C s_12_20: cast zx s_12_19 -> bv
        let s_12_20: Bits = Bits::new(s_12_19 as u128, 1u16);
        // D s_12_21: cmp-eq s_12_18 s_12_20
        let s_12_21: bool = ((s_12_18) == (s_12_20));
        // N s_12_22: branch s_12_21 b18 b13
        if s_12_21 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var Vn:u8
        let s_13_1: u8 = fn_state.Vn;
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
        // D s_13_22: write-var gs#326277 <= s_13_21
        fn_state.gs_326277 = s_13_21;
        // N s_13_23: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#326277:u8
        let s_14_0: bool = fn_state.gs_326277;
        // N s_14_1: branch s_14_0 b17 b15
        if s_14_0 {
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
        // C s_15_0: const #0s : i
        let s_15_0: i128 = 0;
        // D s_15_1: read-var Vm:u8
        let s_15_1: u8 = fn_state.Vm;
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 4u16);
        // C s_15_3: const #1u : u64
        let s_15_3: u64 = 1;
        // D s_15_4: bit-extract s_15_2 s_15_0 s_15_3
        let s_15_4: Bits = (Bits::new(
            ((s_15_2) >> (s_15_0)).value(),
            u16::try_from(s_15_3).unwrap(),
        ));
        // D s_15_5: cast reint s_15_4 -> u8
        let s_15_5: bool = ((s_15_4.value()) != 0);
        // C s_15_6: const #0s : i
        let s_15_6: i128 = 0;
        // C s_15_7: const #0u : u64
        let s_15_7: u64 = 0;
        // D s_15_8: cast zx s_15_5 -> u64
        let s_15_8: u64 = (s_15_5 as u64);
        // C s_15_9: const #1u : u64
        let s_15_9: u64 = 1;
        // D s_15_10: and s_15_8 s_15_9
        let s_15_10: u64 = ((s_15_8) & (s_15_9));
        // D s_15_11: cmp-eq s_15_10 s_15_9
        let s_15_11: bool = ((s_15_10) == (s_15_9));
        // D s_15_12: lsl s_15_8 s_15_6
        let s_15_12: u64 = s_15_8 << s_15_6;
        // D s_15_13: or s_15_7 s_15_12
        let s_15_13: u64 = ((s_15_7) | (s_15_12));
        // D s_15_14: cmpl s_15_12
        let s_15_14: u64 = !s_15_12;
        // D s_15_15: and s_15_7 s_15_14
        let s_15_15: u64 = ((s_15_7) & (s_15_14));
        // D s_15_16: select s_15_11 s_15_13 s_15_15
        let s_15_16: u64 = if s_15_11 { s_15_13 } else { s_15_15 };
        // D s_15_17: cast trunc s_15_16 -> u8
        let s_15_17: bool = ((s_15_16) != 0);
        // D s_15_18: cast zx s_15_17 -> bv
        let s_15_18: Bits = Bits::new(s_15_17 as u128, 1u16);
        // C s_15_19: const #1u : u8
        let s_15_19: bool = true;
        // C s_15_20: cast zx s_15_19 -> bv
        let s_15_20: Bits = Bits::new(s_15_19 as u128, 1u16);
        // D s_15_21: cmp-eq s_15_18 s_15_20
        let s_15_21: bool = ((s_15_18) == (s_15_20));
        // D s_15_22: write-var gs#326280 <= s_15_21
        fn_state.gs_326280 = s_15_21;
        // N s_15_23: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#326280:u8
        let s_16_0: bool = fn_state.gs_326280;
        // D s_16_1: write-var gs#326281 <= s_16_0
        fn_state.gs_326281 = s_16_0;
        // N s_16_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#326280 <= s_17_0
        fn_state.gs_326280 = s_17_0;
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#326277 <= s_18_0
        fn_state.gs_326277 = s_18_0;
        // N s_18_2: jump b14
        return block_14(state, tracer, fn_state);
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
}

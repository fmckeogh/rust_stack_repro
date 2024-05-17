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
use execute_aarch32_instrs_VDOT_s_Op_A_txt::*;
use HaveDOTPExt::*;
use common::*;
pub fn decode_aarch32_instrs_VDOT_s_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op1: bool,
    D: bool,
    op2: u8,
    Vn: u8,
    Vd: u8,
    N: bool,
    Q: bool,
    M: bool,
    U: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        n: i64,
        index: i64,
        d: i64,
        gs_326134: bool,
        gs_326135: bool,
        ga_366362: i64,
        is_signed: bool,
        op1: bool,
        D: bool,
        op2: u8,
        Vn: u8,
        Vd: u8,
        N: bool,
        Q: bool,
        M: bool,
        U: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        op1,
        D,
        op2,
        Vn,
        Vd,
        N,
        Q,
        M,
        U,
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
        // S s_0_1: call HaveDOTPExt(s_0_0)
        let s_0_1: bool = HaveDOTPExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b13 b1
        if s_0_2 {
            return block_13(state, tracer, fn_state);
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
        // N s_1_5: branch s_1_4 b9 b2
        if s_1_4 {
            return block_9(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#326135 <= s_2_0
        fn_state.gs_326135 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#326135:u8
        let s_3_0: bool = fn_state.gs_326135;
        // N s_3_1: branch s_3_0 b8 b4
        if s_3_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var U:u8
        let s_4_0: bool = fn_state.U;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #0u : u8
        let s_4_2: bool = false;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: write-var is_signed <= s_4_4
        fn_state.is_signed = s_4_4;
        // D s_4_6: read-var D:u8
        let s_4_6: bool = fn_state.D;
        // D s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 1u16);
        // D s_4_8: read-var Vd:u8
        let s_4_8: u8 = fn_state.Vd;
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 4u16);
        // D s_4_10: cast reint s_4_7 -> u128
        let s_4_10: u128 = (s_4_7.value() as u128);
        // D s_4_11: size-of s_4_7
        let s_4_11: u16 = s_4_7.length();
        // D s_4_12: cast reint s_4_9 -> u128
        let s_4_12: u128 = (s_4_9.value() as u128);
        // D s_4_13: size-of s_4_9
        let s_4_13: u16 = s_4_9.length();
        // D s_4_14: lsl s_4_10 s_4_13
        let s_4_14: u128 = s_4_10 << s_4_13;
        // D s_4_15: or s_4_14 s_4_12
        let s_4_15: u128 = ((s_4_14) | (s_4_12));
        // D s_4_16: add s_4_11 s_4_13
        let s_4_16: u16 = (s_4_11 + s_4_13);
        // D s_4_17: create-bits s_4_15 s_4_16
        let s_4_17: Bits = Bits::new(s_4_15, s_4_16);
        // D s_4_18: cast reint s_4_17 -> u8
        let s_4_18: u8 = (s_4_17.value() as u8);
        // D s_4_19: cast zx s_4_18 -> bv
        let s_4_19: Bits = Bits::new(s_4_18 as u128, 5u16);
        // D s_4_20: cast zx s_4_19 -> i
        let s_4_20: i128 = (s_4_19.value() as i128);
        // D s_4_21: cast reint s_4_20 -> i64
        let s_4_21: i64 = (s_4_20 as i64);
        // D s_4_22: write-var d <= s_4_21
        fn_state.d = s_4_21;
        // D s_4_23: read-var N:u8
        let s_4_23: bool = fn_state.N;
        // D s_4_24: cast zx s_4_23 -> bv
        let s_4_24: Bits = Bits::new(s_4_23 as u128, 1u16);
        // D s_4_25: read-var Vn:u8
        let s_4_25: u8 = fn_state.Vn;
        // D s_4_26: cast zx s_4_25 -> bv
        let s_4_26: Bits = Bits::new(s_4_25 as u128, 4u16);
        // D s_4_27: cast reint s_4_24 -> u128
        let s_4_27: u128 = (s_4_24.value() as u128);
        // D s_4_28: size-of s_4_24
        let s_4_28: u16 = s_4_24.length();
        // D s_4_29: cast reint s_4_26 -> u128
        let s_4_29: u128 = (s_4_26.value() as u128);
        // D s_4_30: size-of s_4_26
        let s_4_30: u16 = s_4_26.length();
        // D s_4_31: lsl s_4_27 s_4_30
        let s_4_31: u128 = s_4_27 << s_4_30;
        // D s_4_32: or s_4_31 s_4_29
        let s_4_32: u128 = ((s_4_31) | (s_4_29));
        // D s_4_33: add s_4_28 s_4_30
        let s_4_33: u16 = (s_4_28 + s_4_30);
        // D s_4_34: create-bits s_4_32 s_4_33
        let s_4_34: Bits = Bits::new(s_4_32, s_4_33);
        // D s_4_35: cast reint s_4_34 -> u8
        let s_4_35: u8 = (s_4_34.value() as u8);
        // D s_4_36: cast zx s_4_35 -> bv
        let s_4_36: Bits = Bits::new(s_4_35 as u128, 5u16);
        // D s_4_37: cast zx s_4_36 -> i
        let s_4_37: i128 = (s_4_36.value() as i128);
        // D s_4_38: cast reint s_4_37 -> i64
        let s_4_38: i64 = (s_4_37 as i64);
        // D s_4_39: write-var n <= s_4_38
        fn_state.n = s_4_38;
        // C s_4_40: const #0s : i
        let s_4_40: i128 = 0;
        // D s_4_41: read-var Vm:u8
        let s_4_41: u8 = fn_state.Vm;
        // D s_4_42: cast zx s_4_41 -> bv
        let s_4_42: Bits = Bits::new(s_4_41 as u128, 4u16);
        // C s_4_43: const #1s : i64
        let s_4_43: i64 = 1;
        // C s_4_44: cast zx s_4_43 -> i
        let s_4_44: i128 = (i128::try_from(s_4_43).unwrap());
        // C s_4_45: const #3s : i
        let s_4_45: i128 = 3;
        // C s_4_46: add s_4_45 s_4_44
        let s_4_46: i128 = (s_4_45 + s_4_44);
        // D s_4_47: bit-extract s_4_42 s_4_40 s_4_46
        let s_4_47: Bits = (Bits::new(
            ((s_4_42) >> (s_4_40)).value(),
            u16::try_from(s_4_46).unwrap(),
        ));
        // D s_4_48: cast reint s_4_47 -> u8
        let s_4_48: u8 = (s_4_47.value() as u8);
        // D s_4_49: cast zx s_4_48 -> bv
        let s_4_49: Bits = Bits::new(s_4_48 as u128, 4u16);
        // D s_4_50: cast zx s_4_49 -> i
        let s_4_50: i128 = (s_4_49.value() as i128);
        // D s_4_51: cast reint s_4_50 -> i64
        let s_4_51: i64 = (s_4_50 as i64);
        // D s_4_52: write-var m <= s_4_51
        fn_state.m = s_4_51;
        // D s_4_53: read-var M:u8
        let s_4_53: bool = fn_state.M;
        // D s_4_54: cast zx s_4_53 -> bv
        let s_4_54: Bits = Bits::new(s_4_53 as u128, 1u16);
        // D s_4_55: cast zx s_4_54 -> i
        let s_4_55: i128 = (s_4_54.value() as i128);
        // D s_4_56: cast reint s_4_55 -> i64
        let s_4_56: i64 = (s_4_55 as i64);
        // D s_4_57: write-var index <= s_4_56
        fn_state.index = s_4_56;
        // D s_4_58: read-var Q:u8
        let s_4_58: bool = fn_state.Q;
        // D s_4_59: cast zx s_4_58 -> bv
        let s_4_59: Bits = Bits::new(s_4_58 as u128, 1u16);
        // C s_4_60: const #1u : u8
        let s_4_60: bool = true;
        // C s_4_61: cast zx s_4_60 -> bv
        let s_4_61: Bits = Bits::new(s_4_60 as u128, 1u16);
        // D s_4_62: cmp-eq s_4_59 s_4_61
        let s_4_62: bool = ((s_4_59) == (s_4_61));
        // N s_4_63: branch s_4_62 b7 b5
        if s_4_62 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1s : i64
        let s_5_0: i64 = 1;
        // D s_5_1: write-var ga#366362 <= s_5_0
        fn_state.ga_366362 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#366362:i64
        let s_6_0: i64 = fn_state.ga_366362;
        // D s_6_1: read-var d:i64
        let s_6_1: i64 = fn_state.d;
        // C s_6_2: const #32s : i64
        let s_6_2: i64 = 32;
        // D s_6_3: read-var index:i64
        let s_6_3: i64 = fn_state.index;
        // D s_6_4: read-var m:i64
        let s_6_4: i64 = fn_state.m;
        // D s_6_5: read-var n:i64
        let s_6_5: i64 = fn_state.n;
        // D s_6_6: read-var is_signed:u8
        let s_6_6: bool = fn_state.is_signed;
        // D s_6_7: call execute_aarch32_instrs_VDOT_s_Op_A_txt(s_6_1, s_6_2, s_6_3, s_6_4, s_6_5, s_6_0, s_6_6)
        let s_6_7: () = execute_aarch32_instrs_VDOT_s_Op_A_txt(
            state,
            tracer,
            s_6_1,
            s_6_2,
            s_6_3,
            s_6_4,
            s_6_5,
            s_6_0,
            s_6_6,
        );
        // N s_6_8: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2s : i64
        let s_7_0: i64 = 2;
        // D s_7_1: write-var ga#366362 <= s_7_0
        fn_state.ga_366362 = s_7_0;
        // N s_7_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0s : i
        let s_9_0: i128 = 0;
        // D s_9_1: read-var Vd:u8
        let s_9_1: u8 = fn_state.Vd;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 4u16);
        // C s_9_3: const #1u : u64
        let s_9_3: u64 = 1;
        // D s_9_4: bit-extract s_9_2 s_9_0 s_9_3
        let s_9_4: Bits = (Bits::new(
            ((s_9_2) >> (s_9_0)).value(),
            u16::try_from(s_9_3).unwrap(),
        ));
        // D s_9_5: cast reint s_9_4 -> u8
        let s_9_5: bool = ((s_9_4.value()) != 0);
        // C s_9_6: const #0s : i
        let s_9_6: i128 = 0;
        // C s_9_7: const #0u : u64
        let s_9_7: u64 = 0;
        // D s_9_8: cast zx s_9_5 -> u64
        let s_9_8: u64 = (s_9_5 as u64);
        // C s_9_9: const #1u : u64
        let s_9_9: u64 = 1;
        // D s_9_10: and s_9_8 s_9_9
        let s_9_10: u64 = ((s_9_8) & (s_9_9));
        // D s_9_11: cmp-eq s_9_10 s_9_9
        let s_9_11: bool = ((s_9_10) == (s_9_9));
        // D s_9_12: lsl s_9_8 s_9_6
        let s_9_12: u64 = s_9_8 << s_9_6;
        // D s_9_13: or s_9_7 s_9_12
        let s_9_13: u64 = ((s_9_7) | (s_9_12));
        // D s_9_14: cmpl s_9_12
        let s_9_14: u64 = !s_9_12;
        // D s_9_15: and s_9_7 s_9_14
        let s_9_15: u64 = ((s_9_7) & (s_9_14));
        // D s_9_16: select s_9_11 s_9_13 s_9_15
        let s_9_16: u64 = if s_9_11 { s_9_13 } else { s_9_15 };
        // D s_9_17: cast trunc s_9_16 -> u8
        let s_9_17: bool = ((s_9_16) != 0);
        // D s_9_18: cast zx s_9_17 -> bv
        let s_9_18: Bits = Bits::new(s_9_17 as u128, 1u16);
        // C s_9_19: const #1u : u8
        let s_9_19: bool = true;
        // C s_9_20: cast zx s_9_19 -> bv
        let s_9_20: Bits = Bits::new(s_9_19 as u128, 1u16);
        // D s_9_21: cmp-eq s_9_18 s_9_20
        let s_9_21: bool = ((s_9_18) == (s_9_20));
        // N s_9_22: branch s_9_21 b12 b10
        if s_9_21 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0s : i
        let s_10_0: i128 = 0;
        // D s_10_1: read-var Vn:u8
        let s_10_1: u8 = fn_state.Vn;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 4u16);
        // C s_10_3: const #1u : u64
        let s_10_3: u64 = 1;
        // D s_10_4: bit-extract s_10_2 s_10_0 s_10_3
        let s_10_4: Bits = (Bits::new(
            ((s_10_2) >> (s_10_0)).value(),
            u16::try_from(s_10_3).unwrap(),
        ));
        // D s_10_5: cast reint s_10_4 -> u8
        let s_10_5: bool = ((s_10_4.value()) != 0);
        // C s_10_6: const #0s : i
        let s_10_6: i128 = 0;
        // C s_10_7: const #0u : u64
        let s_10_7: u64 = 0;
        // D s_10_8: cast zx s_10_5 -> u64
        let s_10_8: u64 = (s_10_5 as u64);
        // C s_10_9: const #1u : u64
        let s_10_9: u64 = 1;
        // D s_10_10: and s_10_8 s_10_9
        let s_10_10: u64 = ((s_10_8) & (s_10_9));
        // D s_10_11: cmp-eq s_10_10 s_10_9
        let s_10_11: bool = ((s_10_10) == (s_10_9));
        // D s_10_12: lsl s_10_8 s_10_6
        let s_10_12: u64 = s_10_8 << s_10_6;
        // D s_10_13: or s_10_7 s_10_12
        let s_10_13: u64 = ((s_10_7) | (s_10_12));
        // D s_10_14: cmpl s_10_12
        let s_10_14: u64 = !s_10_12;
        // D s_10_15: and s_10_7 s_10_14
        let s_10_15: u64 = ((s_10_7) & (s_10_14));
        // D s_10_16: select s_10_11 s_10_13 s_10_15
        let s_10_16: u64 = if s_10_11 { s_10_13 } else { s_10_15 };
        // D s_10_17: cast trunc s_10_16 -> u8
        let s_10_17: bool = ((s_10_16) != 0);
        // D s_10_18: cast zx s_10_17 -> bv
        let s_10_18: Bits = Bits::new(s_10_17 as u128, 1u16);
        // C s_10_19: const #1u : u8
        let s_10_19: bool = true;
        // C s_10_20: cast zx s_10_19 -> bv
        let s_10_20: Bits = Bits::new(s_10_19 as u128, 1u16);
        // D s_10_21: cmp-eq s_10_18 s_10_20
        let s_10_21: bool = ((s_10_18) == (s_10_20));
        // D s_10_22: write-var gs#326134 <= s_10_21
        fn_state.gs_326134 = s_10_21;
        // N s_10_23: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#326134:u8
        let s_11_0: bool = fn_state.gs_326134;
        // D s_11_1: write-var gs#326135 <= s_11_0
        fn_state.gs_326135 = s_11_0;
        // N s_11_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#326134 <= s_12_0
        fn_state.gs_326134 = s_12_0;
        // N s_12_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
}

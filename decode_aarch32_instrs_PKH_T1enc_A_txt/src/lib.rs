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
use ConditionPassed::*;
use DecodeImmShift::*;
use execute_aarch32_instrs_PKH_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_PKH_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    S: bool,
    Rn: u8,
    imm3: u8,
    Rd: u8,
    imm2: u8,
    tb: bool,
    T: bool,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_345422: ProductType396b95aa74979079,
        m: i64,
        n: i64,
        d: i64,
        gs_299152: bool,
        gs_299150: bool,
        gs_299138: bool,
        shift_nshadow_7241: i128,
        shift_t: u32,
        tbform: bool,
        S: bool,
        Rn: u8,
        imm3: u8,
        Rd: u8,
        imm2: u8,
        tb: bool,
        T: bool,
        Rm: u8,
    }
    let fn_state = FunctionState {
        S,
        Rn,
        imm3,
        Rd,
        imm2,
        tb,
        T,
        Rm,
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
        // D s_2_0: read-var S:u8
        let s_2_0: bool = fn_state.S;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b15 b3
        if s_2_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var T:u8
        let s_3_0: bool = fn_state.T;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: write-var gs#299138 <= s_3_4
        fn_state.gs_299138 = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#299138:u8
        let s_4_0: bool = fn_state.gs_299138;
        // N s_4_1: branch s_4_0 b14 b5
        if s_4_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Rd:u8
        let s_5_0: u8 = fn_state.Rd;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: write-var d <= s_5_3
        fn_state.d = s_5_3;
        // D s_5_5: read-var Rn:u8
        let s_5_5: u8 = fn_state.Rn;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 4u16);
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (s_5_6.value() as i128);
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // D s_5_9: write-var n <= s_5_8
        fn_state.n = s_5_8;
        // D s_5_10: read-var Rm:u8
        let s_5_10: u8 = fn_state.Rm;
        // D s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 4u16);
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (s_5_11.value() as i128);
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: write-var m <= s_5_13
        fn_state.m = s_5_13;
        // D s_5_15: read-var tb:u8
        let s_5_15: bool = fn_state.tb;
        // D s_5_16: cast zx s_5_15 -> bv
        let s_5_16: Bits = Bits::new(s_5_15 as u128, 1u16);
        // C s_5_17: const #1u : u8
        let s_5_17: bool = true;
        // C s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 1u16);
        // D s_5_19: cmp-eq s_5_16 s_5_18
        let s_5_19: bool = ((s_5_16) == (s_5_18));
        // D s_5_20: write-var tbform <= s_5_19
        fn_state.tbform = s_5_19;
        // D s_5_21: read-var tb:u8
        let s_5_21: bool = fn_state.tb;
        // D s_5_22: cast zx s_5_21 -> bv
        let s_5_22: Bits = Bits::new(s_5_21 as u128, 1u16);
        // C s_5_23: const #0u : u8
        let s_5_23: bool = false;
        // C s_5_24: cast zx s_5_23 -> bv
        let s_5_24: Bits = Bits::new(s_5_23 as u128, 1u16);
        // D s_5_25: cast reint s_5_22 -> u128
        let s_5_25: u128 = (s_5_22.value() as u128);
        // D s_5_26: size-of s_5_22
        let s_5_26: u16 = s_5_22.length();
        // C s_5_27: cast reint s_5_24 -> u128
        let s_5_27: u128 = (s_5_24.value() as u128);
        // D s_5_28: size-of s_5_24
        let s_5_28: u16 = s_5_24.length();
        // D s_5_29: lsl s_5_25 s_5_28
        let s_5_29: u128 = s_5_25 << s_5_28;
        // D s_5_30: or s_5_29 s_5_27
        let s_5_30: u128 = ((s_5_29) | (s_5_27));
        // D s_5_31: add s_5_26 s_5_28
        let s_5_31: u16 = (s_5_26 + s_5_28);
        // D s_5_32: create-bits s_5_30 s_5_31
        let s_5_32: Bits = Bits::new(s_5_30, s_5_31);
        // D s_5_33: cast reint s_5_32 -> u8
        let s_5_33: u8 = (s_5_32.value() as u8);
        // D s_5_34: read-var imm3:u8
        let s_5_34: u8 = fn_state.imm3;
        // D s_5_35: cast zx s_5_34 -> bv
        let s_5_35: Bits = Bits::new(s_5_34 as u128, 3u16);
        // D s_5_36: read-var imm2:u8
        let s_5_36: u8 = fn_state.imm2;
        // D s_5_37: cast zx s_5_36 -> bv
        let s_5_37: Bits = Bits::new(s_5_36 as u128, 2u16);
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
        // D s_5_47: call DecodeImmShift(s_5_33, s_5_46)
        let s_5_47: ProductType396b95aa74979079 = DecodeImmShift(
            state,
            tracer,
            s_5_33,
            s_5_46,
        );
        // D s_5_48: write-var ga#345422 <= s_5_47
        fn_state.ga_345422 = s_5_47;
        // D s_5_49: read-var ga#345422.0:struct
        let s_5_49: u32 = fn_state.ga_345422._0;
        // D s_5_50: read-var ga#345422.1:struct
        let s_5_50: i128 = fn_state.ga_345422._1;
        // D s_5_51: write-var shift_t <= s_5_49
        fn_state.shift_t = s_5_49;
        // D s_5_52: write-var shift_nshadow#7241 <= s_5_50
        fn_state.shift_nshadow_7241 = s_5_50;
        // C s_5_53: const #15s : i
        let s_5_53: i128 = 15;
        // D s_5_54: read-var d:i64
        let s_5_54: i64 = fn_state.d;
        // D s_5_55: cast zx s_5_54 -> i
        let s_5_55: i128 = (i128::try_from(s_5_54).unwrap());
        // D s_5_56: cmp-eq s_5_55 s_5_53
        let s_5_56: bool = ((s_5_55) == (s_5_53));
        // N s_5_57: branch s_5_56 b13 b6
        if s_5_56 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #15s : i
        let s_6_0: i128 = 15;
        // D s_6_1: read-var n:i64
        let s_6_1: i64 = fn_state.n;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_0
        let s_6_3: bool = ((s_6_2) == (s_6_0));
        // D s_6_4: write-var gs#299150 <= s_6_3
        fn_state.gs_299150 = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#299150:u8
        let s_7_0: bool = fn_state.gs_299150;
        // N s_7_1: branch s_7_0 b12 b8
        if s_7_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #15s : i
        let s_8_0: i128 = 15;
        // D s_8_1: read-var m:i64
        let s_8_1: i64 = fn_state.m;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_0
        let s_8_3: bool = ((s_8_2) == (s_8_0));
        // D s_8_4: write-var gs#299152 <= s_8_3
        fn_state.gs_299152 = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#299152:u8
        let s_9_0: bool = fn_state.gs_299152;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var d:i64
        let s_10_0: i64 = fn_state.d;
        // D s_10_1: read-var m:i64
        let s_10_1: i64 = fn_state.m;
        // D s_10_2: read-var n:i64
        let s_10_2: i64 = fn_state.n;
        // D s_10_3: read-var shift_nshadow#7241:i
        let s_10_3: i128 = fn_state.shift_nshadow_7241;
        // D s_10_4: read-var shift_t:u32
        let s_10_4: u32 = fn_state.shift_t;
        // D s_10_5: read-var tbform:u8
        let s_10_5: bool = fn_state.tbform;
        // D s_10_6: call execute_aarch32_instrs_PKH_Op_A_txt(s_10_0, s_10_1, s_10_2, s_10_3, s_10_4, s_10_5)
        let s_10_6: () = execute_aarch32_instrs_PKH_Op_A_txt(
            state,
            tracer,
            s_10_0,
            s_10_1,
            s_10_2,
            s_10_3,
            s_10_4,
            s_10_5,
        );
        // N s_10_7: return
        return;
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
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#299152 <= s_12_0
        fn_state.gs_299152 = s_12_0;
        // N s_12_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#299150 <= s_13_0
        fn_state.gs_299150 = s_13_0;
        // N s_13_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#299138 <= s_15_0
        fn_state.gs_299138 = s_15_0;
        // N s_15_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}

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
use execute_aarch32_instrs_USAT_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_USAT_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    sh: bool,
    Rn: u8,
    imm3: u8,
    Rd: u8,
    imm2: u8,
    sat_imm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        shift_nshadow_7329: i128,
        gs_305089: bool,
        saturate_to: i64,
        ga_349870: ProductType396b95aa74979079,
        n: i64,
        d: i64,
        gs_305101: bool,
        shift_t: u32,
        sh: bool,
        Rn: u8,
        imm3: u8,
        Rd: u8,
        imm2: u8,
        sat_imm: u8,
    }
    let fn_state = FunctionState {
        sh,
        Rn,
        imm3,
        Rd,
        imm2,
        sat_imm,
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
        // D s_2_0: read-var sh:u8
        let s_2_0: bool = fn_state.sh;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b12 b3
        if s_2_4 {
            return block_12(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#305089 <= s_3_0
        fn_state.gs_305089 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#305089:u8
        let s_4_0: bool = fn_state.gs_305089;
        // N s_4_1: branch s_4_0 b11 b5
        if s_4_0 {
            return block_11(state, tracer, fn_state);
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
        // D s_5_10: read-var sat_imm:u8
        let s_5_10: u8 = fn_state.sat_imm;
        // D s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 5u16);
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (s_5_11.value() as i128);
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: write-var saturate_to <= s_5_13
        fn_state.saturate_to = s_5_13;
        // D s_5_15: read-var sh:u8
        let s_5_15: bool = fn_state.sh;
        // D s_5_16: cast zx s_5_15 -> bv
        let s_5_16: Bits = Bits::new(s_5_15 as u128, 1u16);
        // C s_5_17: const #0u : u8
        let s_5_17: bool = false;
        // C s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 1u16);
        // D s_5_19: cast reint s_5_16 -> u128
        let s_5_19: u128 = (s_5_16.value() as u128);
        // D s_5_20: size-of s_5_16
        let s_5_20: u16 = s_5_16.length();
        // C s_5_21: cast reint s_5_18 -> u128
        let s_5_21: u128 = (s_5_18.value() as u128);
        // D s_5_22: size-of s_5_18
        let s_5_22: u16 = s_5_18.length();
        // D s_5_23: lsl s_5_19 s_5_22
        let s_5_23: u128 = s_5_19 << s_5_22;
        // D s_5_24: or s_5_23 s_5_21
        let s_5_24: u128 = ((s_5_23) | (s_5_21));
        // D s_5_25: add s_5_20 s_5_22
        let s_5_25: u16 = (s_5_20 + s_5_22);
        // D s_5_26: create-bits s_5_24 s_5_25
        let s_5_26: Bits = Bits::new(s_5_24, s_5_25);
        // D s_5_27: cast reint s_5_26 -> u8
        let s_5_27: u8 = (s_5_26.value() as u8);
        // D s_5_28: read-var imm3:u8
        let s_5_28: u8 = fn_state.imm3;
        // D s_5_29: cast zx s_5_28 -> bv
        let s_5_29: Bits = Bits::new(s_5_28 as u128, 3u16);
        // D s_5_30: read-var imm2:u8
        let s_5_30: u8 = fn_state.imm2;
        // D s_5_31: cast zx s_5_30 -> bv
        let s_5_31: Bits = Bits::new(s_5_30 as u128, 2u16);
        // D s_5_32: cast reint s_5_29 -> u128
        let s_5_32: u128 = (s_5_29.value() as u128);
        // D s_5_33: size-of s_5_29
        let s_5_33: u16 = s_5_29.length();
        // D s_5_34: cast reint s_5_31 -> u128
        let s_5_34: u128 = (s_5_31.value() as u128);
        // D s_5_35: size-of s_5_31
        let s_5_35: u16 = s_5_31.length();
        // D s_5_36: lsl s_5_32 s_5_35
        let s_5_36: u128 = s_5_32 << s_5_35;
        // D s_5_37: or s_5_36 s_5_34
        let s_5_37: u128 = ((s_5_36) | (s_5_34));
        // D s_5_38: add s_5_33 s_5_35
        let s_5_38: u16 = (s_5_33 + s_5_35);
        // D s_5_39: create-bits s_5_37 s_5_38
        let s_5_39: Bits = Bits::new(s_5_37, s_5_38);
        // D s_5_40: cast reint s_5_39 -> u8
        let s_5_40: u8 = (s_5_39.value() as u8);
        // D s_5_41: call DecodeImmShift(s_5_27, s_5_40)
        let s_5_41: ProductType396b95aa74979079 = DecodeImmShift(
            state,
            tracer,
            s_5_27,
            s_5_40,
        );
        // D s_5_42: write-var ga#349870 <= s_5_41
        fn_state.ga_349870 = s_5_41;
        // D s_5_43: read-var ga#349870.0:struct
        let s_5_43: u32 = fn_state.ga_349870._0;
        // D s_5_44: read-var ga#349870.1:struct
        let s_5_44: i128 = fn_state.ga_349870._1;
        // D s_5_45: write-var shift_t <= s_5_43
        fn_state.shift_t = s_5_43;
        // D s_5_46: write-var shift_nshadow#7329 <= s_5_44
        fn_state.shift_nshadow_7329 = s_5_44;
        // C s_5_47: const #15s : i
        let s_5_47: i128 = 15;
        // D s_5_48: read-var d:i64
        let s_5_48: i64 = fn_state.d;
        // D s_5_49: cast zx s_5_48 -> i
        let s_5_49: i128 = (i128::try_from(s_5_48).unwrap());
        // D s_5_50: cmp-eq s_5_49 s_5_47
        let s_5_50: bool = ((s_5_49) == (s_5_47));
        // N s_5_51: branch s_5_50 b10 b6
        if s_5_50 {
            return block_10(state, tracer, fn_state);
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
        // D s_6_4: write-var gs#305101 <= s_6_3
        fn_state.gs_305101 = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#305101:u8
        let s_7_0: bool = fn_state.gs_305101;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var d:i64
        let s_8_0: i64 = fn_state.d;
        // D s_8_1: read-var n:i64
        let s_8_1: i64 = fn_state.n;
        // D s_8_2: read-var saturate_to:i64
        let s_8_2: i64 = fn_state.saturate_to;
        // D s_8_3: read-var shift_nshadow#7329:i
        let s_8_3: i128 = fn_state.shift_nshadow_7329;
        // D s_8_4: read-var shift_t:u32
        let s_8_4: u32 = fn_state.shift_t;
        // D s_8_5: call execute_aarch32_instrs_USAT_Op_A_txt(s_8_0, s_8_1, s_8_2, s_8_3, s_8_4)
        let s_8_5: () = execute_aarch32_instrs_USAT_Op_A_txt(
            state,
            tracer,
            s_8_0,
            s_8_1,
            s_8_2,
            s_8_3,
            s_8_4,
        );
        // N s_8_6: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#305101 <= s_10_0
        fn_state.gs_305101 = s_10_0;
        // N s_10_2: jump b7
        return block_7(state, tracer, fn_state);
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
        // D s_12_0: read-var imm3:u8
        let s_12_0: u8 = fn_state.imm3;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 3u16);
        // D s_12_2: read-var imm2:u8
        let s_12_2: u8 = fn_state.imm2;
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 2u16);
        // D s_12_4: cast reint s_12_1 -> u128
        let s_12_4: u128 = (s_12_1.value() as u128);
        // D s_12_5: size-of s_12_1
        let s_12_5: u16 = s_12_1.length();
        // D s_12_6: cast reint s_12_3 -> u128
        let s_12_6: u128 = (s_12_3.value() as u128);
        // D s_12_7: size-of s_12_3
        let s_12_7: u16 = s_12_3.length();
        // D s_12_8: lsl s_12_4 s_12_7
        let s_12_8: u128 = s_12_4 << s_12_7;
        // D s_12_9: or s_12_8 s_12_6
        let s_12_9: u128 = ((s_12_8) | (s_12_6));
        // D s_12_10: add s_12_5 s_12_7
        let s_12_10: u16 = (s_12_5 + s_12_7);
        // D s_12_11: create-bits s_12_9 s_12_10
        let s_12_11: Bits = Bits::new(s_12_9, s_12_10);
        // D s_12_12: cast reint s_12_11 -> u8
        let s_12_12: u8 = (s_12_11.value() as u8);
        // D s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 5u16);
        // C s_12_14: const #0u : u8
        let s_12_14: u8 = 0;
        // C s_12_15: cast zx s_12_14 -> bv
        let s_12_15: Bits = Bits::new(s_12_14 as u128, 5u16);
        // D s_12_16: cmp-eq s_12_13 s_12_15
        let s_12_16: bool = ((s_12_13) == (s_12_15));
        // D s_12_17: write-var gs#305089 <= s_12_16
        fn_state.gs_305089 = s_12_16;
        // N s_12_18: jump b4
        return block_4(state, tracer, fn_state);
    }
}

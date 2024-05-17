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
use execute_aarch32_instrs_ORN_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_ORN_r_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    S: bool,
    Rn: u8,
    imm3: u8,
    Rd: u8,
    imm2: u8,
    stype: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_298979: bool,
        setflags: bool,
        shift_nshadow_7234: i128,
        shift_t: u32,
        ga_345320: ProductType396b95aa74979079,
        n: i64,
        d: i64,
        S: bool,
        Rn: u8,
        imm3: u8,
        Rd: u8,
        imm2: u8,
        stype: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        S,
        Rn,
        imm3,
        Rd,
        imm2,
        stype,
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
        // D s_2_0: read-var Rn:u8
        let s_2_0: u8 = fn_state.Rn;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b9 b3
        if s_2_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rd:u8
        let s_3_0: u8 = fn_state.Rd;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 4u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: write-var d <= s_3_3
        fn_state.d = s_3_3;
        // D s_3_5: read-var Rn:u8
        let s_3_5: u8 = fn_state.Rn;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 4u16);
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (s_3_6.value() as i128);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: write-var n <= s_3_8
        fn_state.n = s_3_8;
        // D s_3_10: read-var Rm:u8
        let s_3_10: u8 = fn_state.Rm;
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 4u16);
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (s_3_11.value() as i128);
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: write-var m <= s_3_13
        fn_state.m = s_3_13;
        // D s_3_15: read-var S:u8
        let s_3_15: bool = fn_state.S;
        // D s_3_16: cast zx s_3_15 -> bv
        let s_3_16: Bits = Bits::new(s_3_15 as u128, 1u16);
        // C s_3_17: const #1u : u8
        let s_3_17: bool = true;
        // C s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 1u16);
        // D s_3_19: cmp-eq s_3_16 s_3_18
        let s_3_19: bool = ((s_3_16) == (s_3_18));
        // D s_3_20: write-var setflags <= s_3_19
        fn_state.setflags = s_3_19;
        // D s_3_21: read-var imm3:u8
        let s_3_21: u8 = fn_state.imm3;
        // D s_3_22: cast zx s_3_21 -> bv
        let s_3_22: Bits = Bits::new(s_3_21 as u128, 3u16);
        // D s_3_23: read-var imm2:u8
        let s_3_23: u8 = fn_state.imm2;
        // D s_3_24: cast zx s_3_23 -> bv
        let s_3_24: Bits = Bits::new(s_3_23 as u128, 2u16);
        // D s_3_25: cast reint s_3_22 -> u128
        let s_3_25: u128 = (s_3_22.value() as u128);
        // D s_3_26: size-of s_3_22
        let s_3_26: u16 = s_3_22.length();
        // D s_3_27: cast reint s_3_24 -> u128
        let s_3_27: u128 = (s_3_24.value() as u128);
        // D s_3_28: size-of s_3_24
        let s_3_28: u16 = s_3_24.length();
        // D s_3_29: lsl s_3_25 s_3_28
        let s_3_29: u128 = s_3_25 << s_3_28;
        // D s_3_30: or s_3_29 s_3_27
        let s_3_30: u128 = ((s_3_29) | (s_3_27));
        // D s_3_31: add s_3_26 s_3_28
        let s_3_31: u16 = (s_3_26 + s_3_28);
        // D s_3_32: create-bits s_3_30 s_3_31
        let s_3_32: Bits = Bits::new(s_3_30, s_3_31);
        // D s_3_33: cast reint s_3_32 -> u8
        let s_3_33: u8 = (s_3_32.value() as u8);
        // D s_3_34: read-var stype:u8
        let s_3_34: u8 = fn_state.stype;
        // D s_3_35: call DecodeImmShift(s_3_34, s_3_33)
        let s_3_35: ProductType396b95aa74979079 = DecodeImmShift(
            state,
            tracer,
            s_3_34,
            s_3_33,
        );
        // D s_3_36: write-var ga#345320 <= s_3_35
        fn_state.ga_345320 = s_3_35;
        // D s_3_37: read-var ga#345320.0:struct
        let s_3_37: u32 = fn_state.ga_345320._0;
        // D s_3_38: read-var ga#345320.1:struct
        let s_3_38: i128 = fn_state.ga_345320._1;
        // D s_3_39: write-var shift_t <= s_3_37
        fn_state.shift_t = s_3_37;
        // D s_3_40: write-var shift_nshadow#7234 <= s_3_38
        fn_state.shift_nshadow_7234 = s_3_38;
        // C s_3_41: const #15s : i
        let s_3_41: i128 = 15;
        // D s_3_42: read-var d:i64
        let s_3_42: i64 = fn_state.d;
        // D s_3_43: cast zx s_3_42 -> i
        let s_3_43: i128 = (i128::try_from(s_3_42).unwrap());
        // D s_3_44: cmp-eq s_3_43 s_3_41
        let s_3_44: bool = ((s_3_43) == (s_3_41));
        // N s_3_45: branch s_3_44 b8 b4
        if s_3_44 {
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
        // C s_4_0: const #15s : i
        let s_4_0: i128 = 15;
        // D s_4_1: read-var m:i64
        let s_4_1: i64 = fn_state.m;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_0
        let s_4_3: bool = ((s_4_2) == (s_4_0));
        // D s_4_4: write-var gs#298979 <= s_4_3
        fn_state.gs_298979 = s_4_3;
        // N s_4_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#298979:u8
        let s_5_0: bool = fn_state.gs_298979;
        // N s_5_1: branch s_5_0 b7 b6
        if s_5_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var d:i64
        let s_6_0: i64 = fn_state.d;
        // D s_6_1: read-var m:i64
        let s_6_1: i64 = fn_state.m;
        // D s_6_2: read-var n:i64
        let s_6_2: i64 = fn_state.n;
        // D s_6_3: read-var setflags:u8
        let s_6_3: bool = fn_state.setflags;
        // D s_6_4: read-var shift_nshadow#7234:i
        let s_6_4: i128 = fn_state.shift_nshadow_7234;
        // D s_6_5: read-var shift_t:u32
        let s_6_5: u32 = fn_state.shift_t;
        // D s_6_6: call execute_aarch32_instrs_ORN_r_Op_A_txt(s_6_0, s_6_1, s_6_2, s_6_3, s_6_4, s_6_5)
        let s_6_6: () = execute_aarch32_instrs_ORN_r_Op_A_txt(
            state,
            tracer,
            s_6_0,
            s_6_1,
            s_6_2,
            s_6_3,
            s_6_4,
            s_6_5,
        );
        // N s_6_7: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var gs#298979 <= s_8_0
        fn_state.gs_298979 = s_8_0;
        // N s_8_2: jump b5
        return block_5(state, tracer, fn_state);
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
}

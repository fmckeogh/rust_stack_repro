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
use execute_aarch32_instrs_RSB_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_RSB_r_T1enc_A_txt<T: Tracer>(
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
        d: i64,
        m: i64,
        shift_nshadow_7251: i128,
        gs_300076: bool,
        setflags: bool,
        ga_346099: ProductType396b95aa74979079,
        gs_300074: bool,
        shift_t: u32,
        n: i64,
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
        // D s_2_0: read-var Rd:u8
        let s_2_0: u8 = fn_state.Rd;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var d <= s_2_3
        fn_state.d = s_2_3;
        // D s_2_5: read-var Rn:u8
        let s_2_5: u8 = fn_state.Rn;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 4u16);
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (s_2_6.value() as i128);
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // D s_2_9: write-var n <= s_2_8
        fn_state.n = s_2_8;
        // D s_2_10: read-var Rm:u8
        let s_2_10: u8 = fn_state.Rm;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 4u16);
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (s_2_11.value() as i128);
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // D s_2_14: write-var m <= s_2_13
        fn_state.m = s_2_13;
        // D s_2_15: read-var S:u8
        let s_2_15: bool = fn_state.S;
        // D s_2_16: cast zx s_2_15 -> bv
        let s_2_16: Bits = Bits::new(s_2_15 as u128, 1u16);
        // C s_2_17: const #1u : u8
        let s_2_17: bool = true;
        // C s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // D s_2_19: cmp-eq s_2_16 s_2_18
        let s_2_19: bool = ((s_2_16) == (s_2_18));
        // D s_2_20: write-var setflags <= s_2_19
        fn_state.setflags = s_2_19;
        // D s_2_21: read-var imm3:u8
        let s_2_21: u8 = fn_state.imm3;
        // D s_2_22: cast zx s_2_21 -> bv
        let s_2_22: Bits = Bits::new(s_2_21 as u128, 3u16);
        // D s_2_23: read-var imm2:u8
        let s_2_23: u8 = fn_state.imm2;
        // D s_2_24: cast zx s_2_23 -> bv
        let s_2_24: Bits = Bits::new(s_2_23 as u128, 2u16);
        // D s_2_25: cast reint s_2_22 -> u128
        let s_2_25: u128 = (s_2_22.value() as u128);
        // D s_2_26: size-of s_2_22
        let s_2_26: u16 = s_2_22.length();
        // D s_2_27: cast reint s_2_24 -> u128
        let s_2_27: u128 = (s_2_24.value() as u128);
        // D s_2_28: size-of s_2_24
        let s_2_28: u16 = s_2_24.length();
        // D s_2_29: lsl s_2_25 s_2_28
        let s_2_29: u128 = s_2_25 << s_2_28;
        // D s_2_30: or s_2_29 s_2_27
        let s_2_30: u128 = ((s_2_29) | (s_2_27));
        // D s_2_31: add s_2_26 s_2_28
        let s_2_31: u16 = (s_2_26 + s_2_28);
        // D s_2_32: create-bits s_2_30 s_2_31
        let s_2_32: Bits = Bits::new(s_2_30, s_2_31);
        // D s_2_33: cast reint s_2_32 -> u8
        let s_2_33: u8 = (s_2_32.value() as u8);
        // D s_2_34: read-var stype:u8
        let s_2_34: u8 = fn_state.stype;
        // D s_2_35: call DecodeImmShift(s_2_34, s_2_33)
        let s_2_35: ProductType396b95aa74979079 = DecodeImmShift(
            state,
            tracer,
            s_2_34,
            s_2_33,
        );
        // D s_2_36: write-var ga#346099 <= s_2_35
        fn_state.ga_346099 = s_2_35;
        // D s_2_37: read-var ga#346099.0:struct
        let s_2_37: u32 = fn_state.ga_346099._0;
        // D s_2_38: read-var ga#346099.1:struct
        let s_2_38: i128 = fn_state.ga_346099._1;
        // D s_2_39: write-var shift_t <= s_2_37
        fn_state.shift_t = s_2_37;
        // D s_2_40: write-var shift_nshadow#7251 <= s_2_38
        fn_state.shift_nshadow_7251 = s_2_38;
        // C s_2_41: const #15s : i
        let s_2_41: i128 = 15;
        // D s_2_42: read-var d:i64
        let s_2_42: i64 = fn_state.d;
        // D s_2_43: cast zx s_2_42 -> i
        let s_2_43: i128 = (i128::try_from(s_2_42).unwrap());
        // D s_2_44: cmp-eq s_2_43 s_2_41
        let s_2_44: bool = ((s_2_43) == (s_2_41));
        // N s_2_45: branch s_2_44 b10 b3
        if s_2_44 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #15s : i
        let s_3_0: i128 = 15;
        // D s_3_1: read-var n:i64
        let s_3_1: i64 = fn_state.n;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // D s_3_4: write-var gs#300074 <= s_3_3
        fn_state.gs_300074 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#300074:u8
        let s_4_0: bool = fn_state.gs_300074;
        // N s_4_1: branch s_4_0 b9 b5
        if s_4_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #15s : i
        let s_5_0: i128 = 15;
        // D s_5_1: read-var m:i64
        let s_5_1: i64 = fn_state.m;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // D s_5_4: write-var gs#300076 <= s_5_3
        fn_state.gs_300076 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#300076:u8
        let s_6_0: bool = fn_state.gs_300076;
        // N s_6_1: branch s_6_0 b8 b7
        if s_6_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var shift_t:u32
        let s_7_0: u32 = fn_state.shift_t;
        // D s_7_1: read-var shift_nshadow#7251:i
        let s_7_1: i128 = fn_state.shift_nshadow_7251;
        // D s_7_2: read-var d:i64
        let s_7_2: i64 = fn_state.d;
        // D s_7_3: read-var m:i64
        let s_7_3: i64 = fn_state.m;
        // D s_7_4: read-var n:i64
        let s_7_4: i64 = fn_state.n;
        // D s_7_5: read-var setflags:u8
        let s_7_5: bool = fn_state.setflags;
        // D s_7_6: call execute_aarch32_instrs_RSB_r_Op_A_txt(s_7_2, s_7_3, s_7_4, s_7_5, s_7_1, s_7_0)
        let s_7_6: () = execute_aarch32_instrs_RSB_r_Op_A_txt(
            state,
            tracer,
            s_7_2,
            s_7_3,
            s_7_4,
            s_7_5,
            s_7_1,
            s_7_0,
        );
        // N s_7_7: return
        return;
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
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#300076 <= s_9_0
        fn_state.gs_300076 = s_9_0;
        // N s_9_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#300074 <= s_10_0
        fn_state.gs_300074 = s_10_0;
        // N s_10_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}

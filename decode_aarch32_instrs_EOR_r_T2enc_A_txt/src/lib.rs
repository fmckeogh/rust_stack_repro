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
use execute_aarch32_instrs_EOR_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_EOR_r_T2enc_A_txt<T: Tracer>(
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
        setflags: bool,
        ga_343650: ProductType396b95aa74979079,
        gs_296672: bool,
        n: i64,
        gs_296676: bool,
        d: i64,
        gs_296674: bool,
        gs_296661: bool,
        shift_nshadow_7151: i128,
        shift_t: u32,
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
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b18 b3
        if s_2_4 {
            return block_18(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#296661 <= s_3_0
        fn_state.gs_296661 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#296661:u8
        let s_4_0: bool = fn_state.gs_296661;
        // N s_4_1: branch s_4_0 b17 b5
        if s_4_0 {
            return block_17(state, tracer, fn_state);
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
        // D s_5_15: read-var S:u8
        let s_5_15: bool = fn_state.S;
        // D s_5_16: cast zx s_5_15 -> bv
        let s_5_16: Bits = Bits::new(s_5_15 as u128, 1u16);
        // C s_5_17: const #1u : u8
        let s_5_17: bool = true;
        // C s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 1u16);
        // D s_5_19: cmp-eq s_5_16 s_5_18
        let s_5_19: bool = ((s_5_16) == (s_5_18));
        // D s_5_20: write-var setflags <= s_5_19
        fn_state.setflags = s_5_19;
        // D s_5_21: read-var imm3:u8
        let s_5_21: u8 = fn_state.imm3;
        // D s_5_22: cast zx s_5_21 -> bv
        let s_5_22: Bits = Bits::new(s_5_21 as u128, 3u16);
        // D s_5_23: read-var imm2:u8
        let s_5_23: u8 = fn_state.imm2;
        // D s_5_24: cast zx s_5_23 -> bv
        let s_5_24: Bits = Bits::new(s_5_23 as u128, 2u16);
        // D s_5_25: cast reint s_5_22 -> u128
        let s_5_25: u128 = (s_5_22.value() as u128);
        // D s_5_26: size-of s_5_22
        let s_5_26: u16 = s_5_22.length();
        // D s_5_27: cast reint s_5_24 -> u128
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
        // D s_5_34: read-var stype:u8
        let s_5_34: u8 = fn_state.stype;
        // D s_5_35: call DecodeImmShift(s_5_34, s_5_33)
        let s_5_35: ProductType396b95aa74979079 = DecodeImmShift(
            state,
            tracer,
            s_5_34,
            s_5_33,
        );
        // D s_5_36: write-var ga#343650 <= s_5_35
        fn_state.ga_343650 = s_5_35;
        // D s_5_37: read-var ga#343650.0:struct
        let s_5_37: u32 = fn_state.ga_343650._0;
        // D s_5_38: read-var ga#343650.1:struct
        let s_5_38: i128 = fn_state.ga_343650._1;
        // D s_5_39: write-var shift_t <= s_5_37
        fn_state.shift_t = s_5_37;
        // D s_5_40: write-var shift_nshadow#7151 <= s_5_38
        fn_state.shift_nshadow_7151 = s_5_38;
        // C s_5_41: const #15s : i
        let s_5_41: i128 = 15;
        // D s_5_42: read-var d:i64
        let s_5_42: i64 = fn_state.d;
        // D s_5_43: cast zx s_5_42 -> i
        let s_5_43: i128 = (i128::try_from(s_5_42).unwrap());
        // D s_5_44: cmp-eq s_5_43 s_5_41
        let s_5_44: bool = ((s_5_43) == (s_5_41));
        // N s_5_45: branch s_5_44 b16 b6
        if s_5_44 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#296672 <= s_6_0
        fn_state.gs_296672 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#296672:u8
        let s_7_0: bool = fn_state.gs_296672;
        // N s_7_1: branch s_7_0 b15 b8
        if s_7_0 {
            return block_15(state, tracer, fn_state);
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
        // D s_8_1: read-var n:i64
        let s_8_1: i64 = fn_state.n;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_0
        let s_8_3: bool = ((s_8_2) == (s_8_0));
        // D s_8_4: write-var gs#296674 <= s_8_3
        fn_state.gs_296674 = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#296674:u8
        let s_9_0: bool = fn_state.gs_296674;
        // N s_9_1: branch s_9_0 b14 b10
        if s_9_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #15s : i
        let s_10_0: i128 = 15;
        // D s_10_1: read-var m:i64
        let s_10_1: i64 = fn_state.m;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cmp-eq s_10_2 s_10_0
        let s_10_3: bool = ((s_10_2) == (s_10_0));
        // D s_10_4: write-var gs#296676 <= s_10_3
        fn_state.gs_296676 = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#296676:u8
        let s_11_0: bool = fn_state.gs_296676;
        // N s_11_1: branch s_11_0 b13 b12
        if s_11_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var d:i64
        let s_12_0: i64 = fn_state.d;
        // D s_12_1: read-var m:i64
        let s_12_1: i64 = fn_state.m;
        // D s_12_2: read-var n:i64
        let s_12_2: i64 = fn_state.n;
        // D s_12_3: read-var setflags:u8
        let s_12_3: bool = fn_state.setflags;
        // D s_12_4: read-var shift_nshadow#7151:i
        let s_12_4: i128 = fn_state.shift_nshadow_7151;
        // D s_12_5: read-var shift_t:u32
        let s_12_5: u32 = fn_state.shift_t;
        // D s_12_6: call execute_aarch32_instrs_EOR_r_Op_A_txt(s_12_0, s_12_1, s_12_2, s_12_3, s_12_4, s_12_5)
        let s_12_6: () = execute_aarch32_instrs_EOR_r_Op_A_txt(
            state,
            tracer,
            s_12_0,
            s_12_1,
            s_12_2,
            s_12_3,
            s_12_4,
            s_12_5,
        );
        // N s_12_7: return
        return;
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
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#296676 <= s_14_0
        fn_state.gs_296676 = s_14_0;
        // N s_14_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#296674 <= s_15_0
        fn_state.gs_296674 = s_15_0;
        // N s_15_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var setflags:u8
        let s_16_0: bool = fn_state.setflags;
        // D s_16_1: not s_16_0
        let s_16_1: bool = !s_16_0;
        // D s_16_2: write-var gs#296672 <= s_16_1
        fn_state.gs_296672 = s_16_1;
        // N s_16_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var S:u8
        let s_18_0: bool = fn_state.S;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #1u : u8
        let s_18_2: bool = true;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: write-var gs#296661 <= s_18_4
        fn_state.gs_296661 = s_18_4;
        // N s_18_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}

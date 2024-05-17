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
use execute_aarch32_instrs_SUB_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_SUB_r_T2enc_A_txt<T: Tracer>(
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
        gs_303251: bool,
        m: i64,
        setflags: bool,
        gs_303265: bool,
        gs_303267: bool,
        n: i64,
        d: i64,
        gs_303263: bool,
        shift_nshadow_7308: i128,
        shift_t: u32,
        ga_348432: ProductType396b95aa74979079,
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
        // N s_2_5: branch s_2_4 b20 b3
        if s_2_4 {
            return block_20(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#303251 <= s_3_0
        fn_state.gs_303251 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#303251:u8
        let s_4_0: bool = fn_state.gs_303251;
        // N s_4_1: branch s_4_0 b19 b5
        if s_4_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Rn:u8
        let s_5_0: u8 = fn_state.Rn;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // C s_5_2: const #13u : u8
        let s_5_2: u8 = 13;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b18 b6
        if s_5_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var Rd:u8
        let s_6_0: u8 = fn_state.Rd;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 4u16);
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (s_6_1.value() as i128);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // D s_6_4: write-var d <= s_6_3
        fn_state.d = s_6_3;
        // D s_6_5: read-var Rn:u8
        let s_6_5: u8 = fn_state.Rn;
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 4u16);
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (s_6_6.value() as i128);
        // D s_6_8: cast reint s_6_7 -> i64
        let s_6_8: i64 = (s_6_7 as i64);
        // D s_6_9: write-var n <= s_6_8
        fn_state.n = s_6_8;
        // D s_6_10: read-var Rm:u8
        let s_6_10: u8 = fn_state.Rm;
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 4u16);
        // D s_6_12: cast zx s_6_11 -> i
        let s_6_12: i128 = (s_6_11.value() as i128);
        // D s_6_13: cast reint s_6_12 -> i64
        let s_6_13: i64 = (s_6_12 as i64);
        // D s_6_14: write-var m <= s_6_13
        fn_state.m = s_6_13;
        // D s_6_15: read-var S:u8
        let s_6_15: bool = fn_state.S;
        // D s_6_16: cast zx s_6_15 -> bv
        let s_6_16: Bits = Bits::new(s_6_15 as u128, 1u16);
        // C s_6_17: const #1u : u8
        let s_6_17: bool = true;
        // C s_6_18: cast zx s_6_17 -> bv
        let s_6_18: Bits = Bits::new(s_6_17 as u128, 1u16);
        // D s_6_19: cmp-eq s_6_16 s_6_18
        let s_6_19: bool = ((s_6_16) == (s_6_18));
        // D s_6_20: write-var setflags <= s_6_19
        fn_state.setflags = s_6_19;
        // D s_6_21: read-var imm3:u8
        let s_6_21: u8 = fn_state.imm3;
        // D s_6_22: cast zx s_6_21 -> bv
        let s_6_22: Bits = Bits::new(s_6_21 as u128, 3u16);
        // D s_6_23: read-var imm2:u8
        let s_6_23: u8 = fn_state.imm2;
        // D s_6_24: cast zx s_6_23 -> bv
        let s_6_24: Bits = Bits::new(s_6_23 as u128, 2u16);
        // D s_6_25: cast reint s_6_22 -> u128
        let s_6_25: u128 = (s_6_22.value() as u128);
        // D s_6_26: size-of s_6_22
        let s_6_26: u16 = s_6_22.length();
        // D s_6_27: cast reint s_6_24 -> u128
        let s_6_27: u128 = (s_6_24.value() as u128);
        // D s_6_28: size-of s_6_24
        let s_6_28: u16 = s_6_24.length();
        // D s_6_29: lsl s_6_25 s_6_28
        let s_6_29: u128 = s_6_25 << s_6_28;
        // D s_6_30: or s_6_29 s_6_27
        let s_6_30: u128 = ((s_6_29) | (s_6_27));
        // D s_6_31: add s_6_26 s_6_28
        let s_6_31: u16 = (s_6_26 + s_6_28);
        // D s_6_32: create-bits s_6_30 s_6_31
        let s_6_32: Bits = Bits::new(s_6_30, s_6_31);
        // D s_6_33: cast reint s_6_32 -> u8
        let s_6_33: u8 = (s_6_32.value() as u8);
        // D s_6_34: read-var stype:u8
        let s_6_34: u8 = fn_state.stype;
        // D s_6_35: call DecodeImmShift(s_6_34, s_6_33)
        let s_6_35: ProductType396b95aa74979079 = DecodeImmShift(
            state,
            tracer,
            s_6_34,
            s_6_33,
        );
        // D s_6_36: write-var ga#348432 <= s_6_35
        fn_state.ga_348432 = s_6_35;
        // D s_6_37: read-var ga#348432.0:struct
        let s_6_37: u32 = fn_state.ga_348432._0;
        // D s_6_38: read-var ga#348432.1:struct
        let s_6_38: i128 = fn_state.ga_348432._1;
        // D s_6_39: write-var shift_t <= s_6_37
        fn_state.shift_t = s_6_37;
        // D s_6_40: write-var shift_nshadow#7308 <= s_6_38
        fn_state.shift_nshadow_7308 = s_6_38;
        // C s_6_41: const #15s : i
        let s_6_41: i128 = 15;
        // D s_6_42: read-var d:i64
        let s_6_42: i64 = fn_state.d;
        // D s_6_43: cast zx s_6_42 -> i
        let s_6_43: i128 = (i128::try_from(s_6_42).unwrap());
        // D s_6_44: cmp-eq s_6_43 s_6_41
        let s_6_44: bool = ((s_6_43) == (s_6_41));
        // N s_6_45: branch s_6_44 b17 b7
        if s_6_44 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#303263 <= s_7_0
        fn_state.gs_303263 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#303263:u8
        let s_8_0: bool = fn_state.gs_303263;
        // N s_8_1: branch s_8_0 b16 b9
        if s_8_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #15s : i
        let s_9_0: i128 = 15;
        // D s_9_1: read-var n:i64
        let s_9_1: i64 = fn_state.n;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_0
        let s_9_3: bool = ((s_9_2) == (s_9_0));
        // D s_9_4: write-var gs#303265 <= s_9_3
        fn_state.gs_303265 = s_9_3;
        // N s_9_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#303265:u8
        let s_10_0: bool = fn_state.gs_303265;
        // N s_10_1: branch s_10_0 b15 b11
        if s_10_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #15s : i
        let s_11_0: i128 = 15;
        // D s_11_1: read-var m:i64
        let s_11_1: i64 = fn_state.m;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cmp-eq s_11_2 s_11_0
        let s_11_3: bool = ((s_11_2) == (s_11_0));
        // D s_11_4: write-var gs#303267 <= s_11_3
        fn_state.gs_303267 = s_11_3;
        // N s_11_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#303267:u8
        let s_12_0: bool = fn_state.gs_303267;
        // N s_12_1: branch s_12_0 b14 b13
        if s_12_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var d:i64
        let s_13_0: i64 = fn_state.d;
        // D s_13_1: read-var m:i64
        let s_13_1: i64 = fn_state.m;
        // D s_13_2: read-var n:i64
        let s_13_2: i64 = fn_state.n;
        // D s_13_3: read-var setflags:u8
        let s_13_3: bool = fn_state.setflags;
        // D s_13_4: read-var shift_nshadow#7308:i
        let s_13_4: i128 = fn_state.shift_nshadow_7308;
        // D s_13_5: read-var shift_t:u32
        let s_13_5: u32 = fn_state.shift_t;
        // D s_13_6: call execute_aarch32_instrs_SUB_r_Op_A_txt(s_13_0, s_13_1, s_13_2, s_13_3, s_13_4, s_13_5)
        let s_13_6: () = execute_aarch32_instrs_SUB_r_Op_A_txt(
            state,
            tracer,
            s_13_0,
            s_13_1,
            s_13_2,
            s_13_3,
            s_13_4,
            s_13_5,
        );
        // N s_13_7: return
        return;
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
        // D s_15_1: write-var gs#303267 <= s_15_0
        fn_state.gs_303267 = s_15_0;
        // N s_15_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#303265 <= s_16_0
        fn_state.gs_303265 = s_16_0;
        // N s_16_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var setflags:u8
        let s_17_0: bool = fn_state.setflags;
        // D s_17_1: not s_17_0
        let s_17_1: bool = !s_17_0;
        // D s_17_2: write-var gs#303263 <= s_17_1
        fn_state.gs_303263 = s_17_1;
        // N s_17_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
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
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var S:u8
        let s_20_0: bool = fn_state.S;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 1u16);
        // C s_20_2: const #1u : u8
        let s_20_2: bool = true;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: write-var gs#303251 <= s_20_4
        fn_state.gs_303251 = s_20_4;
        // N s_20_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}

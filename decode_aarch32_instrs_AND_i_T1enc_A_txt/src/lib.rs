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
use T32ExpandImm_C::*;
use execute_aarch32_instrs_AND_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_AND_i_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i: bool,
    S: bool,
    Rn: u8,
    imm3: u8,
    Rd: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        carry: bool,
        imm32: u32,
        setflags: bool,
        gs_295803: bool,
        gs_295805: bool,
        ga_343071: ProductType4813027798de1e98,
        n: i64,
        gs_295795: bool,
        d: i64,
        i: bool,
        S: bool,
        Rn: u8,
        imm3: u8,
        Rd: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        i,
        S,
        Rn,
        imm3,
        Rd,
        imm8,
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#295795 <= s_3_0
        fn_state.gs_295795 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#295795:u8
        let s_4_0: bool = fn_state.gs_295795;
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
        // D s_5_10: read-var S:u8
        let s_5_10: bool = fn_state.S;
        // D s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 1u16);
        // C s_5_12: const #1u : u8
        let s_5_12: bool = true;
        // C s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 1u16);
        // D s_5_14: cmp-eq s_5_11 s_5_13
        let s_5_14: bool = ((s_5_11) == (s_5_13));
        // D s_5_15: write-var setflags <= s_5_14
        fn_state.setflags = s_5_14;
        // D s_5_16: read-var i:u8
        let s_5_16: bool = fn_state.i;
        // D s_5_17: cast zx s_5_16 -> bv
        let s_5_17: Bits = Bits::new(s_5_16 as u128, 1u16);
        // D s_5_18: read-var imm3:u8
        let s_5_18: u8 = fn_state.imm3;
        // D s_5_19: cast zx s_5_18 -> bv
        let s_5_19: Bits = Bits::new(s_5_18 as u128, 3u16);
        // D s_5_20: cast reint s_5_17 -> u128
        let s_5_20: u128 = (s_5_17.value() as u128);
        // D s_5_21: size-of s_5_17
        let s_5_21: u16 = s_5_17.length();
        // D s_5_22: cast reint s_5_19 -> u128
        let s_5_22: u128 = (s_5_19.value() as u128);
        // D s_5_23: size-of s_5_19
        let s_5_23: u16 = s_5_19.length();
        // D s_5_24: lsl s_5_20 s_5_23
        let s_5_24: u128 = s_5_20 << s_5_23;
        // D s_5_25: or s_5_24 s_5_22
        let s_5_25: u128 = ((s_5_24) | (s_5_22));
        // D s_5_26: add s_5_21 s_5_23
        let s_5_26: u16 = (s_5_21 + s_5_23);
        // D s_5_27: create-bits s_5_25 s_5_26
        let s_5_27: Bits = Bits::new(s_5_25, s_5_26);
        // D s_5_28: cast reint s_5_27 -> u8
        let s_5_28: u8 = (s_5_27.value() as u8);
        // D s_5_29: cast zx s_5_28 -> bv
        let s_5_29: Bits = Bits::new(s_5_28 as u128, 4u16);
        // D s_5_30: read-var imm8:u8
        let s_5_30: u8 = fn_state.imm8;
        // D s_5_31: cast zx s_5_30 -> bv
        let s_5_31: Bits = Bits::new(s_5_30 as u128, 8u16);
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
        // D s_5_40: cast reint s_5_39 -> u12
        let s_5_40: u16 = (s_5_39.value() as u16);
        // C s_5_41: const #16971u : u32
        let s_5_41: u32 = 16971;
        // D s_5_42: read-reg s_5_41:u8
        let s_5_42: bool = {
            let value = state.read_register::<bool>(s_5_41 as isize);
            tracer.read_register(s_5_41 as isize, value);
            value
        };
        // D s_5_43: call T32ExpandImm_C(s_5_40, s_5_42)
        let s_5_43: ProductType4813027798de1e98 = T32ExpandImm_C(
            state,
            tracer,
            s_5_40,
            s_5_42,
        );
        // D s_5_44: write-var ga#343071 <= s_5_43
        fn_state.ga_343071 = s_5_43;
        // D s_5_45: read-var ga#343071.0:struct
        let s_5_45: u32 = fn_state.ga_343071._0;
        // D s_5_46: read-var ga#343071.1:struct
        let s_5_46: bool = fn_state.ga_343071._1;
        // D s_5_47: write-var imm32 <= s_5_45
        fn_state.imm32 = s_5_45;
        // D s_5_48: write-var carry <= s_5_46
        fn_state.carry = s_5_46;
        // C s_5_49: const #15s : i
        let s_5_49: i128 = 15;
        // D s_5_50: read-var d:i64
        let s_5_50: i64 = fn_state.d;
        // D s_5_51: cast zx s_5_50 -> i
        let s_5_51: i128 = (i128::try_from(s_5_50).unwrap());
        // D s_5_52: cmp-eq s_5_51 s_5_49
        let s_5_52: bool = ((s_5_51) == (s_5_49));
        // N s_5_53: branch s_5_52 b13 b6
        if s_5_52 {
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
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#295803 <= s_6_0
        fn_state.gs_295803 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#295803:u8
        let s_7_0: bool = fn_state.gs_295803;
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
        // D s_8_1: read-var n:i64
        let s_8_1: i64 = fn_state.n;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_0
        let s_8_3: bool = ((s_8_2) == (s_8_0));
        // D s_8_4: write-var gs#295805 <= s_8_3
        fn_state.gs_295805 = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#295805:u8
        let s_9_0: bool = fn_state.gs_295805;
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
        // D s_10_0: read-var carry:u8
        let s_10_0: bool = fn_state.carry;
        // D s_10_1: read-var d:i64
        let s_10_1: i64 = fn_state.d;
        // D s_10_2: read-var imm32:u32
        let s_10_2: u32 = fn_state.imm32;
        // D s_10_3: read-var n:i64
        let s_10_3: i64 = fn_state.n;
        // D s_10_4: read-var setflags:u8
        let s_10_4: bool = fn_state.setflags;
        // D s_10_5: call execute_aarch32_instrs_AND_i_Op_A_txt(s_10_0, s_10_1, s_10_2, s_10_3, s_10_4)
        let s_10_5: () = execute_aarch32_instrs_AND_i_Op_A_txt(
            state,
            tracer,
            s_10_0,
            s_10_1,
            s_10_2,
            s_10_3,
            s_10_4,
        );
        // N s_10_6: return
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
        // D s_12_1: write-var gs#295805 <= s_12_0
        fn_state.gs_295805 = s_12_0;
        // N s_12_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var setflags:u8
        let s_13_0: bool = fn_state.setflags;
        // D s_13_1: not s_13_0
        let s_13_1: bool = !s_13_0;
        // D s_13_2: write-var gs#295803 <= s_13_1
        fn_state.gs_295803 = s_13_1;
        // N s_13_3: jump b7
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
        // D s_15_0: read-var S:u8
        let s_15_0: bool = fn_state.S;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 1u16);
        // C s_15_2: const #1u : u8
        let s_15_2: bool = true;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: write-var gs#295795 <= s_15_4
        fn_state.gs_295795 = s_15_4;
        // N s_15_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}

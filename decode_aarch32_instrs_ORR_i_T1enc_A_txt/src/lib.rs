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
use execute_aarch32_instrs_ORR_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_ORR_i_T1enc_A_txt<T: Tracer>(
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
        ga_345341: ProductType4813027798de1e98,
        carry: bool,
        imm32: u32,
        setflags: bool,
        n: i64,
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
        // N s_2_5: branch s_2_4 b6 b3
        if s_2_4 {
            return block_6(state, tracer, fn_state);
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
        // D s_3_10: read-var S:u8
        let s_3_10: bool = fn_state.S;
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 1u16);
        // C s_3_12: const #1u : u8
        let s_3_12: bool = true;
        // C s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 1u16);
        // D s_3_14: cmp-eq s_3_11 s_3_13
        let s_3_14: bool = ((s_3_11) == (s_3_13));
        // D s_3_15: write-var setflags <= s_3_14
        fn_state.setflags = s_3_14;
        // D s_3_16: read-var i:u8
        let s_3_16: bool = fn_state.i;
        // D s_3_17: cast zx s_3_16 -> bv
        let s_3_17: Bits = Bits::new(s_3_16 as u128, 1u16);
        // D s_3_18: read-var imm3:u8
        let s_3_18: u8 = fn_state.imm3;
        // D s_3_19: cast zx s_3_18 -> bv
        let s_3_19: Bits = Bits::new(s_3_18 as u128, 3u16);
        // D s_3_20: cast reint s_3_17 -> u128
        let s_3_20: u128 = (s_3_17.value() as u128);
        // D s_3_21: size-of s_3_17
        let s_3_21: u16 = s_3_17.length();
        // D s_3_22: cast reint s_3_19 -> u128
        let s_3_22: u128 = (s_3_19.value() as u128);
        // D s_3_23: size-of s_3_19
        let s_3_23: u16 = s_3_19.length();
        // D s_3_24: lsl s_3_20 s_3_23
        let s_3_24: u128 = s_3_20 << s_3_23;
        // D s_3_25: or s_3_24 s_3_22
        let s_3_25: u128 = ((s_3_24) | (s_3_22));
        // D s_3_26: add s_3_21 s_3_23
        let s_3_26: u16 = (s_3_21 + s_3_23);
        // D s_3_27: create-bits s_3_25 s_3_26
        let s_3_27: Bits = Bits::new(s_3_25, s_3_26);
        // D s_3_28: cast reint s_3_27 -> u8
        let s_3_28: u8 = (s_3_27.value() as u8);
        // D s_3_29: cast zx s_3_28 -> bv
        let s_3_29: Bits = Bits::new(s_3_28 as u128, 4u16);
        // D s_3_30: read-var imm8:u8
        let s_3_30: u8 = fn_state.imm8;
        // D s_3_31: cast zx s_3_30 -> bv
        let s_3_31: Bits = Bits::new(s_3_30 as u128, 8u16);
        // D s_3_32: cast reint s_3_29 -> u128
        let s_3_32: u128 = (s_3_29.value() as u128);
        // D s_3_33: size-of s_3_29
        let s_3_33: u16 = s_3_29.length();
        // D s_3_34: cast reint s_3_31 -> u128
        let s_3_34: u128 = (s_3_31.value() as u128);
        // D s_3_35: size-of s_3_31
        let s_3_35: u16 = s_3_31.length();
        // D s_3_36: lsl s_3_32 s_3_35
        let s_3_36: u128 = s_3_32 << s_3_35;
        // D s_3_37: or s_3_36 s_3_34
        let s_3_37: u128 = ((s_3_36) | (s_3_34));
        // D s_3_38: add s_3_33 s_3_35
        let s_3_38: u16 = (s_3_33 + s_3_35);
        // D s_3_39: create-bits s_3_37 s_3_38
        let s_3_39: Bits = Bits::new(s_3_37, s_3_38);
        // D s_3_40: cast reint s_3_39 -> u12
        let s_3_40: u16 = (s_3_39.value() as u16);
        // C s_3_41: const #16971u : u32
        let s_3_41: u32 = 16971;
        // D s_3_42: read-reg s_3_41:u8
        let s_3_42: bool = {
            let value = state.read_register::<bool>(s_3_41 as isize);
            tracer.read_register(s_3_41 as isize, value);
            value
        };
        // D s_3_43: call T32ExpandImm_C(s_3_40, s_3_42)
        let s_3_43: ProductType4813027798de1e98 = T32ExpandImm_C(
            state,
            tracer,
            s_3_40,
            s_3_42,
        );
        // D s_3_44: write-var ga#345341 <= s_3_43
        fn_state.ga_345341 = s_3_43;
        // D s_3_45: read-var ga#345341.0:struct
        let s_3_45: u32 = fn_state.ga_345341._0;
        // D s_3_46: read-var ga#345341.1:struct
        let s_3_46: bool = fn_state.ga_345341._1;
        // D s_3_47: write-var imm32 <= s_3_45
        fn_state.imm32 = s_3_45;
        // D s_3_48: write-var carry <= s_3_46
        fn_state.carry = s_3_46;
        // C s_3_49: const #15s : i
        let s_3_49: i128 = 15;
        // D s_3_50: read-var d:i64
        let s_3_50: i64 = fn_state.d;
        // D s_3_51: cast zx s_3_50 -> i
        let s_3_51: i128 = (i128::try_from(s_3_50).unwrap());
        // D s_3_52: cmp-eq s_3_51 s_3_49
        let s_3_52: bool = ((s_3_51) == (s_3_49));
        // N s_3_53: branch s_3_52 b5 b4
        if s_3_52 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var carry:u8
        let s_4_0: bool = fn_state.carry;
        // D s_4_1: read-var d:i64
        let s_4_1: i64 = fn_state.d;
        // D s_4_2: read-var imm32:u32
        let s_4_2: u32 = fn_state.imm32;
        // D s_4_3: read-var n:i64
        let s_4_3: i64 = fn_state.n;
        // D s_4_4: read-var setflags:u8
        let s_4_4: bool = fn_state.setflags;
        // D s_4_5: call execute_aarch32_instrs_ORR_i_Op_A_txt(s_4_0, s_4_1, s_4_2, s_4_3, s_4_4)
        let s_4_5: () = execute_aarch32_instrs_ORR_i_Op_A_txt(
            state,
            tracer,
            s_4_0,
            s_4_1,
            s_4_2,
            s_4_3,
            s_4_4,
        );
        // N s_4_6: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
}

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
use execute_aarch32_instrs_MOV_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_MOV_i_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i: bool,
    S: bool,
    imm3: u8,
    Rd: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        carry: bool,
        imm32: u32,
        setflags: bool,
        ga_345066: ProductType4813027798de1e98,
        d: i64,
        i: bool,
        S: bool,
        imm3: u8,
        Rd: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        i,
        S,
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
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var d <= s_2_3
        fn_state.d = s_2_3;
        // D s_2_5: read-var S:u8
        let s_2_5: bool = fn_state.S;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 1u16);
        // C s_2_7: const #1u : u8
        let s_2_7: bool = true;
        // C s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 1u16);
        // D s_2_9: cmp-eq s_2_6 s_2_8
        let s_2_9: bool = ((s_2_6) == (s_2_8));
        // D s_2_10: write-var setflags <= s_2_9
        fn_state.setflags = s_2_9;
        // D s_2_11: read-var i:u8
        let s_2_11: bool = fn_state.i;
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 1u16);
        // D s_2_13: read-var imm3:u8
        let s_2_13: u8 = fn_state.imm3;
        // D s_2_14: cast zx s_2_13 -> bv
        let s_2_14: Bits = Bits::new(s_2_13 as u128, 3u16);
        // D s_2_15: cast reint s_2_12 -> u128
        let s_2_15: u128 = (s_2_12.value() as u128);
        // D s_2_16: size-of s_2_12
        let s_2_16: u16 = s_2_12.length();
        // D s_2_17: cast reint s_2_14 -> u128
        let s_2_17: u128 = (s_2_14.value() as u128);
        // D s_2_18: size-of s_2_14
        let s_2_18: u16 = s_2_14.length();
        // D s_2_19: lsl s_2_15 s_2_18
        let s_2_19: u128 = s_2_15 << s_2_18;
        // D s_2_20: or s_2_19 s_2_17
        let s_2_20: u128 = ((s_2_19) | (s_2_17));
        // D s_2_21: add s_2_16 s_2_18
        let s_2_21: u16 = (s_2_16 + s_2_18);
        // D s_2_22: create-bits s_2_20 s_2_21
        let s_2_22: Bits = Bits::new(s_2_20, s_2_21);
        // D s_2_23: cast reint s_2_22 -> u8
        let s_2_23: u8 = (s_2_22.value() as u8);
        // D s_2_24: cast zx s_2_23 -> bv
        let s_2_24: Bits = Bits::new(s_2_23 as u128, 4u16);
        // D s_2_25: read-var imm8:u8
        let s_2_25: u8 = fn_state.imm8;
        // D s_2_26: cast zx s_2_25 -> bv
        let s_2_26: Bits = Bits::new(s_2_25 as u128, 8u16);
        // D s_2_27: cast reint s_2_24 -> u128
        let s_2_27: u128 = (s_2_24.value() as u128);
        // D s_2_28: size-of s_2_24
        let s_2_28: u16 = s_2_24.length();
        // D s_2_29: cast reint s_2_26 -> u128
        let s_2_29: u128 = (s_2_26.value() as u128);
        // D s_2_30: size-of s_2_26
        let s_2_30: u16 = s_2_26.length();
        // D s_2_31: lsl s_2_27 s_2_30
        let s_2_31: u128 = s_2_27 << s_2_30;
        // D s_2_32: or s_2_31 s_2_29
        let s_2_32: u128 = ((s_2_31) | (s_2_29));
        // D s_2_33: add s_2_28 s_2_30
        let s_2_33: u16 = (s_2_28 + s_2_30);
        // D s_2_34: create-bits s_2_32 s_2_33
        let s_2_34: Bits = Bits::new(s_2_32, s_2_33);
        // D s_2_35: cast reint s_2_34 -> u12
        let s_2_35: u16 = (s_2_34.value() as u16);
        // C s_2_36: const #16971u : u32
        let s_2_36: u32 = 16971;
        // D s_2_37: read-reg s_2_36:u8
        let s_2_37: bool = {
            let value = state.read_register::<bool>(s_2_36 as isize);
            tracer.read_register(s_2_36 as isize, value);
            value
        };
        // D s_2_38: call T32ExpandImm_C(s_2_35, s_2_37)
        let s_2_38: ProductType4813027798de1e98 = T32ExpandImm_C(
            state,
            tracer,
            s_2_35,
            s_2_37,
        );
        // D s_2_39: write-var ga#345066 <= s_2_38
        fn_state.ga_345066 = s_2_38;
        // D s_2_40: read-var ga#345066.0:struct
        let s_2_40: u32 = fn_state.ga_345066._0;
        // D s_2_41: read-var ga#345066.1:struct
        let s_2_41: bool = fn_state.ga_345066._1;
        // D s_2_42: write-var imm32 <= s_2_40
        fn_state.imm32 = s_2_40;
        // D s_2_43: write-var carry <= s_2_41
        fn_state.carry = s_2_41;
        // C s_2_44: const #15s : i
        let s_2_44: i128 = 15;
        // D s_2_45: read-var d:i64
        let s_2_45: i64 = fn_state.d;
        // D s_2_46: cast zx s_2_45 -> i
        let s_2_46: i128 = (i128::try_from(s_2_45).unwrap());
        // D s_2_47: cmp-eq s_2_46 s_2_44
        let s_2_47: bool = ((s_2_46) == (s_2_44));
        // N s_2_48: branch s_2_47 b4 b3
        if s_2_47 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var imm32:u32
        let s_3_0: u32 = fn_state.imm32;
        // D s_3_1: read-var carry:u8
        let s_3_1: bool = fn_state.carry;
        // D s_3_2: read-var d:i64
        let s_3_2: i64 = fn_state.d;
        // D s_3_3: read-var setflags:u8
        let s_3_3: bool = fn_state.setflags;
        // D s_3_4: call execute_aarch32_instrs_MOV_i_Op_A_txt(s_3_1, s_3_2, s_3_0, s_3_3)
        let s_3_4: () = execute_aarch32_instrs_MOV_i_Op_A_txt(
            state,
            tracer,
            s_3_1,
            s_3_2,
            s_3_0,
            s_3_3,
        );
        // N s_3_5: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
}

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
use execute_aarch32_instrs_TEQ_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_TEQ_i_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i: bool,
    Rn: u8,
    imm3: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_348708: ProductType4813027798de1e98,
        carry: bool,
        imm32: u32,
        n: i64,
        i: bool,
        Rn: u8,
        imm3: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        i,
        Rn,
        imm3,
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
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var n <= s_2_3
        fn_state.n = s_2_3;
        // D s_2_5: read-var i:u8
        let s_2_5: bool = fn_state.i;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 1u16);
        // D s_2_7: read-var imm3:u8
        let s_2_7: u8 = fn_state.imm3;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 3u16);
        // D s_2_9: cast reint s_2_6 -> u128
        let s_2_9: u128 = (s_2_6.value() as u128);
        // D s_2_10: size-of s_2_6
        let s_2_10: u16 = s_2_6.length();
        // D s_2_11: cast reint s_2_8 -> u128
        let s_2_11: u128 = (s_2_8.value() as u128);
        // D s_2_12: size-of s_2_8
        let s_2_12: u16 = s_2_8.length();
        // D s_2_13: lsl s_2_9 s_2_12
        let s_2_13: u128 = s_2_9 << s_2_12;
        // D s_2_14: or s_2_13 s_2_11
        let s_2_14: u128 = ((s_2_13) | (s_2_11));
        // D s_2_15: add s_2_10 s_2_12
        let s_2_15: u16 = (s_2_10 + s_2_12);
        // D s_2_16: create-bits s_2_14 s_2_15
        let s_2_16: Bits = Bits::new(s_2_14, s_2_15);
        // D s_2_17: cast reint s_2_16 -> u8
        let s_2_17: u8 = (s_2_16.value() as u8);
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 4u16);
        // D s_2_19: read-var imm8:u8
        let s_2_19: u8 = fn_state.imm8;
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 8u16);
        // D s_2_21: cast reint s_2_18 -> u128
        let s_2_21: u128 = (s_2_18.value() as u128);
        // D s_2_22: size-of s_2_18
        let s_2_22: u16 = s_2_18.length();
        // D s_2_23: cast reint s_2_20 -> u128
        let s_2_23: u128 = (s_2_20.value() as u128);
        // D s_2_24: size-of s_2_20
        let s_2_24: u16 = s_2_20.length();
        // D s_2_25: lsl s_2_21 s_2_24
        let s_2_25: u128 = s_2_21 << s_2_24;
        // D s_2_26: or s_2_25 s_2_23
        let s_2_26: u128 = ((s_2_25) | (s_2_23));
        // D s_2_27: add s_2_22 s_2_24
        let s_2_27: u16 = (s_2_22 + s_2_24);
        // D s_2_28: create-bits s_2_26 s_2_27
        let s_2_28: Bits = Bits::new(s_2_26, s_2_27);
        // D s_2_29: cast reint s_2_28 -> u12
        let s_2_29: u16 = (s_2_28.value() as u16);
        // C s_2_30: const #16971u : u32
        let s_2_30: u32 = 16971;
        // D s_2_31: read-reg s_2_30:u8
        let s_2_31: bool = {
            let value = state.read_register::<bool>(s_2_30 as isize);
            tracer.read_register(s_2_30 as isize, value);
            value
        };
        // D s_2_32: call T32ExpandImm_C(s_2_29, s_2_31)
        let s_2_32: ProductType4813027798de1e98 = T32ExpandImm_C(
            state,
            tracer,
            s_2_29,
            s_2_31,
        );
        // D s_2_33: write-var ga#348708 <= s_2_32
        fn_state.ga_348708 = s_2_32;
        // D s_2_34: read-var ga#348708.0:struct
        let s_2_34: u32 = fn_state.ga_348708._0;
        // D s_2_35: read-var ga#348708.1:struct
        let s_2_35: bool = fn_state.ga_348708._1;
        // D s_2_36: write-var imm32 <= s_2_34
        fn_state.imm32 = s_2_34;
        // D s_2_37: write-var carry <= s_2_35
        fn_state.carry = s_2_35;
        // C s_2_38: const #15s : i
        let s_2_38: i128 = 15;
        // D s_2_39: read-var n:i64
        let s_2_39: i64 = fn_state.n;
        // D s_2_40: cast zx s_2_39 -> i
        let s_2_40: i128 = (i128::try_from(s_2_39).unwrap());
        // D s_2_41: cmp-eq s_2_40 s_2_38
        let s_2_41: bool = ((s_2_40) == (s_2_38));
        // N s_2_42: branch s_2_41 b4 b3
        if s_2_41 {
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
        // D s_3_2: read-var n:i64
        let s_3_2: i64 = fn_state.n;
        // D s_3_3: call execute_aarch32_instrs_TEQ_i_Op_A_txt(s_3_1, s_3_0, s_3_2)
        let s_3_3: () = execute_aarch32_instrs_TEQ_i_Op_A_txt(
            state,
            tracer,
            s_3_1,
            s_3_0,
            s_3_2,
        );
        // N s_3_4: return
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

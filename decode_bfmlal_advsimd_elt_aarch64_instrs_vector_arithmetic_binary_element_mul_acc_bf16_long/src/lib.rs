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
use execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_bf16_long::*;
use HaveBF16Ext::*;
use common::*;
pub fn decode_bfmlal_advsimd_elt_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_bf16_long<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    H: bool,
    Rm: u8,
    M: bool,
    L: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rd: u8,
        Rn: u8,
        H: bool,
        Rm: u8,
        M: bool,
        L: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        H,
        Rm,
        M,
        L,
        Q,
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
        // S s_0_1: call HaveBF16Ext(s_0_0)
        let s_0_1: bool = HaveBF16Ext(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b2 b1
        if s_0_2 {
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
        // D s_1_0: read-var Rn:u8
        let s_1_0: u8 = fn_state.Rn;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #0u : u8
        let s_1_4: bool = false;
        // C s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 1u16);
        // D s_1_6: read-var Rm:u8
        let s_1_6: u8 = fn_state.Rm;
        // D s_1_7: cast zx s_1_6 -> bv
        let s_1_7: Bits = Bits::new(s_1_6 as u128, 4u16);
        // C s_1_8: cast reint s_1_5 -> u128
        let s_1_8: u128 = (s_1_5.value() as u128);
        // D s_1_9: size-of s_1_5
        let s_1_9: u16 = s_1_5.length();
        // D s_1_10: cast reint s_1_7 -> u128
        let s_1_10: u128 = (s_1_7.value() as u128);
        // D s_1_11: size-of s_1_7
        let s_1_11: u16 = s_1_7.length();
        // D s_1_12: lsl s_1_8 s_1_11
        let s_1_12: u128 = s_1_8 << s_1_11;
        // D s_1_13: or s_1_12 s_1_10
        let s_1_13: u128 = ((s_1_12) | (s_1_10));
        // D s_1_14: add s_1_9 s_1_11
        let s_1_14: u16 = (s_1_9 + s_1_11);
        // D s_1_15: create-bits s_1_13 s_1_14
        let s_1_15: Bits = Bits::new(s_1_13, s_1_14);
        // D s_1_16: cast reint s_1_15 -> u8
        let s_1_16: u8 = (s_1_15.value() as u8);
        // D s_1_17: cast zx s_1_16 -> bv
        let s_1_17: Bits = Bits::new(s_1_16 as u128, 5u16);
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (s_1_17.value() as i128);
        // D s_1_19: cast reint s_1_18 -> i64
        let s_1_19: i64 = (s_1_18 as i64);
        // D s_1_20: read-var Rd:u8
        let s_1_20: u8 = fn_state.Rd;
        // D s_1_21: cast zx s_1_20 -> bv
        let s_1_21: Bits = Bits::new(s_1_20 as u128, 5u16);
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (s_1_21.value() as i128);
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: read-var H:u8
        let s_1_24: bool = fn_state.H;
        // D s_1_25: cast zx s_1_24 -> bv
        let s_1_25: Bits = Bits::new(s_1_24 as u128, 1u16);
        // D s_1_26: read-var L:u8
        let s_1_26: bool = fn_state.L;
        // D s_1_27: cast zx s_1_26 -> bv
        let s_1_27: Bits = Bits::new(s_1_26 as u128, 1u16);
        // D s_1_28: cast reint s_1_25 -> u128
        let s_1_28: u128 = (s_1_25.value() as u128);
        // D s_1_29: size-of s_1_25
        let s_1_29: u16 = s_1_25.length();
        // D s_1_30: cast reint s_1_27 -> u128
        let s_1_30: u128 = (s_1_27.value() as u128);
        // D s_1_31: size-of s_1_27
        let s_1_31: u16 = s_1_27.length();
        // D s_1_32: lsl s_1_28 s_1_31
        let s_1_32: u128 = s_1_28 << s_1_31;
        // D s_1_33: or s_1_32 s_1_30
        let s_1_33: u128 = ((s_1_32) | (s_1_30));
        // D s_1_34: add s_1_29 s_1_31
        let s_1_34: u16 = (s_1_29 + s_1_31);
        // D s_1_35: create-bits s_1_33 s_1_34
        let s_1_35: Bits = Bits::new(s_1_33, s_1_34);
        // D s_1_36: cast reint s_1_35 -> u8
        let s_1_36: u8 = (s_1_35.value() as u8);
        // D s_1_37: cast zx s_1_36 -> bv
        let s_1_37: Bits = Bits::new(s_1_36 as u128, 2u16);
        // D s_1_38: read-var M:u8
        let s_1_38: bool = fn_state.M;
        // D s_1_39: cast zx s_1_38 -> bv
        let s_1_39: Bits = Bits::new(s_1_38 as u128, 1u16);
        // D s_1_40: cast reint s_1_37 -> u128
        let s_1_40: u128 = (s_1_37.value() as u128);
        // D s_1_41: size-of s_1_37
        let s_1_41: u16 = s_1_37.length();
        // D s_1_42: cast reint s_1_39 -> u128
        let s_1_42: u128 = (s_1_39.value() as u128);
        // D s_1_43: size-of s_1_39
        let s_1_43: u16 = s_1_39.length();
        // D s_1_44: lsl s_1_40 s_1_43
        let s_1_44: u128 = s_1_40 << s_1_43;
        // D s_1_45: or s_1_44 s_1_42
        let s_1_45: u128 = ((s_1_44) | (s_1_42));
        // D s_1_46: add s_1_41 s_1_43
        let s_1_46: u16 = (s_1_41 + s_1_43);
        // D s_1_47: create-bits s_1_45 s_1_46
        let s_1_47: Bits = Bits::new(s_1_45, s_1_46);
        // D s_1_48: cast reint s_1_47 -> u8
        let s_1_48: u8 = (s_1_47.value() as u8);
        // D s_1_49: cast zx s_1_48 -> bv
        let s_1_49: Bits = Bits::new(s_1_48 as u128, 3u16);
        // D s_1_50: cast zx s_1_49 -> i
        let s_1_50: i128 = (s_1_49.value() as i128);
        // D s_1_51: cast reint s_1_50 -> i64
        let s_1_51: i64 = (s_1_50 as i64);
        // C s_1_52: const #4s : i64
        let s_1_52: i64 = 4;
        // D s_1_53: read-var Q:u8
        let s_1_53: bool = fn_state.Q;
        // D s_1_54: cast zx s_1_53 -> bv
        let s_1_54: Bits = Bits::new(s_1_53 as u128, 1u16);
        // D s_1_55: cast zx s_1_54 -> i
        let s_1_55: i128 = (s_1_54.value() as i128);
        // D s_1_56: cast reint s_1_55 -> i64
        let s_1_56: i64 = (s_1_55 as i64);
        // D s_1_57: call execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_bf16_long(s_1_23, s_1_52, s_1_51, s_1_19, s_1_3, s_1_56)
        let s_1_57: () = execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_bf16_long(
            state,
            tracer,
            s_1_23,
            s_1_52,
            s_1_51,
            s_1_19,
            s_1_3,
            s_1_56,
        );
        // N s_1_58: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: panic
        panic!("{:?}", ());
        // N s_2_1: return
        return;
    }
}

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
use HaveInt8MatMulExt::*;
use execute_aarch64_instrs_vector_arithmetic_binary_element_mat_mul_int_dotp::*;
use common::*;
pub fn decode_usdot_advsimd_elt_aarch64_instrs_vector_arithmetic_binary_element_mat_mul_int_dotp<
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
    US: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        n: i64,
        d: i64,
        op1_unsigned: bool,
        ga_269703: i64,
        i: i64,
        op2_unsigned: bool,
        Rd: u8,
        Rn: u8,
        H: bool,
        Rm: u8,
        M: bool,
        L: bool,
        US: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        H,
        Rm,
        M,
        L,
        US,
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
        // S s_0_1: call HaveInt8MatMulExt(s_0_0)
        let s_0_1: bool = HaveInt8MatMulExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b5 b1
        if s_0_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var US:u8
        let s_1_0: bool = fn_state.US;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // C s_1_2: const #1u : u8
        let s_1_2: bool = true;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // D s_1_5: write-var op1_unsigned <= s_1_4
        fn_state.op1_unsigned = s_1_4;
        // D s_1_6: read-var US:u8
        let s_1_6: bool = fn_state.US;
        // D s_1_7: cast zx s_1_6 -> bv
        let s_1_7: Bits = Bits::new(s_1_6 as u128, 1u16);
        // C s_1_8: const #0u : u8
        let s_1_8: bool = false;
        // C s_1_9: cast zx s_1_8 -> bv
        let s_1_9: Bits = Bits::new(s_1_8 as u128, 1u16);
        // D s_1_10: cmp-eq s_1_7 s_1_9
        let s_1_10: bool = ((s_1_7) == (s_1_9));
        // D s_1_11: write-var op2_unsigned <= s_1_10
        fn_state.op2_unsigned = s_1_10;
        // D s_1_12: read-var Rn:u8
        let s_1_12: u8 = fn_state.Rn;
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 5u16);
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (s_1_13.value() as i128);
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: write-var n <= s_1_15
        fn_state.n = s_1_15;
        // D s_1_17: read-var M:u8
        let s_1_17: bool = fn_state.M;
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 1u16);
        // D s_1_19: read-var Rm:u8
        let s_1_19: u8 = fn_state.Rm;
        // D s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 4u16);
        // D s_1_21: cast reint s_1_18 -> u128
        let s_1_21: u128 = (s_1_18.value() as u128);
        // D s_1_22: size-of s_1_18
        let s_1_22: u16 = s_1_18.length();
        // D s_1_23: cast reint s_1_20 -> u128
        let s_1_23: u128 = (s_1_20.value() as u128);
        // D s_1_24: size-of s_1_20
        let s_1_24: u16 = s_1_20.length();
        // D s_1_25: lsl s_1_21 s_1_24
        let s_1_25: u128 = s_1_21 << s_1_24;
        // D s_1_26: or s_1_25 s_1_23
        let s_1_26: u128 = ((s_1_25) | (s_1_23));
        // D s_1_27: add s_1_22 s_1_24
        let s_1_27: u16 = (s_1_22 + s_1_24);
        // D s_1_28: create-bits s_1_26 s_1_27
        let s_1_28: Bits = Bits::new(s_1_26, s_1_27);
        // D s_1_29: cast reint s_1_28 -> u8
        let s_1_29: u8 = (s_1_28.value() as u8);
        // D s_1_30: cast zx s_1_29 -> bv
        let s_1_30: Bits = Bits::new(s_1_29 as u128, 5u16);
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (s_1_30.value() as i128);
        // D s_1_32: cast reint s_1_31 -> i64
        let s_1_32: i64 = (s_1_31 as i64);
        // D s_1_33: write-var m <= s_1_32
        fn_state.m = s_1_32;
        // D s_1_34: read-var Rd:u8
        let s_1_34: u8 = fn_state.Rd;
        // D s_1_35: cast zx s_1_34 -> bv
        let s_1_35: Bits = Bits::new(s_1_34 as u128, 5u16);
        // D s_1_36: cast zx s_1_35 -> i
        let s_1_36: i128 = (s_1_35.value() as i128);
        // D s_1_37: cast reint s_1_36 -> i64
        let s_1_37: i64 = (s_1_36 as i64);
        // D s_1_38: write-var d <= s_1_37
        fn_state.d = s_1_37;
        // D s_1_39: read-var H:u8
        let s_1_39: bool = fn_state.H;
        // D s_1_40: cast zx s_1_39 -> bv
        let s_1_40: Bits = Bits::new(s_1_39 as u128, 1u16);
        // D s_1_41: read-var L:u8
        let s_1_41: bool = fn_state.L;
        // D s_1_42: cast zx s_1_41 -> bv
        let s_1_42: Bits = Bits::new(s_1_41 as u128, 1u16);
        // D s_1_43: cast reint s_1_40 -> u128
        let s_1_43: u128 = (s_1_40.value() as u128);
        // D s_1_44: size-of s_1_40
        let s_1_44: u16 = s_1_40.length();
        // D s_1_45: cast reint s_1_42 -> u128
        let s_1_45: u128 = (s_1_42.value() as u128);
        // D s_1_46: size-of s_1_42
        let s_1_46: u16 = s_1_42.length();
        // D s_1_47: lsl s_1_43 s_1_46
        let s_1_47: u128 = s_1_43 << s_1_46;
        // D s_1_48: or s_1_47 s_1_45
        let s_1_48: u128 = ((s_1_47) | (s_1_45));
        // D s_1_49: add s_1_44 s_1_46
        let s_1_49: u16 = (s_1_44 + s_1_46);
        // D s_1_50: create-bits s_1_48 s_1_49
        let s_1_50: Bits = Bits::new(s_1_48, s_1_49);
        // D s_1_51: cast reint s_1_50 -> u8
        let s_1_51: u8 = (s_1_50.value() as u8);
        // D s_1_52: cast zx s_1_51 -> bv
        let s_1_52: Bits = Bits::new(s_1_51 as u128, 2u16);
        // D s_1_53: cast zx s_1_52 -> i
        let s_1_53: i128 = (s_1_52.value() as i128);
        // D s_1_54: cast reint s_1_53 -> i64
        let s_1_54: i64 = (s_1_53 as i64);
        // D s_1_55: write-var i <= s_1_54
        fn_state.i = s_1_54;
        // D s_1_56: read-var Q:u8
        let s_1_56: bool = fn_state.Q;
        // D s_1_57: cast zx s_1_56 -> bv
        let s_1_57: Bits = Bits::new(s_1_56 as u128, 1u16);
        // C s_1_58: const #1u : u8
        let s_1_58: bool = true;
        // C s_1_59: cast zx s_1_58 -> bv
        let s_1_59: Bits = Bits::new(s_1_58 as u128, 1u16);
        // D s_1_60: cmp-eq s_1_57 s_1_59
        let s_1_60: bool = ((s_1_57) == (s_1_59));
        // N s_1_61: branch s_1_60 b4 b2
        if s_1_60 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: write-var ga#269703 <= s_2_0
        fn_state.ga_269703 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#269703:i64
        let s_3_0: i64 = fn_state.ga_269703;
        // C s_3_1: const #32s : i
        let s_3_1: i128 = 32;
        // D s_3_2: cast zx s_3_0 -> i
        let s_3_2: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_3: div s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) / (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: cast zx s_3_0 -> i
        let s_3_5: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // D s_3_7: read-var d:i64
        let s_3_7: i64 = fn_state.d;
        // D s_3_8: read-var i:i64
        let s_3_8: i64 = fn_state.i;
        // D s_3_9: read-var m:i64
        let s_3_9: i64 = fn_state.m;
        // D s_3_10: read-var n:i64
        let s_3_10: i64 = fn_state.n;
        // D s_3_11: read-var op1_unsigned:u8
        let s_3_11: bool = fn_state.op1_unsigned;
        // D s_3_12: read-var op2_unsigned:u8
        let s_3_12: bool = fn_state.op2_unsigned;
        // D s_3_13: call execute_aarch64_instrs_vector_arithmetic_binary_element_mat_mul_int_dotp(s_3_7, s_3_6, s_3_4, s_3_8, s_3_9, s_3_10, s_3_11, s_3_12)
        let s_3_13: () = execute_aarch64_instrs_vector_arithmetic_binary_element_mat_mul_int_dotp(
            state,
            tracer,
            s_3_7,
            s_3_6,
            s_3_4,
            s_3_8,
            s_3_9,
            s_3_10,
            s_3_11,
            s_3_12,
        );
        // N s_3_14: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #128s : i64
        let s_4_0: i64 = 128;
        // D s_4_1: write-var ga#269703 <= s_4_0
        fn_state.ga_269703 = s_4_0;
        // N s_4_2: jump b3
        return block_3(state, tracer, fn_state);
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
}

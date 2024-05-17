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
use execute_aarch64_instrs_vector_arithmetic_binary_element_dotp::*;
use HaveDOTPExt::*;
use common::*;
pub fn decode_sdot_advsimd_elt_aarch64_instrs_vector_arithmetic_binary_element_dotp<
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
    size: u8,
    U: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        n: i64,
        index: i64,
        d: i64,
        is_signed: bool,
        ga_265303: i64,
        Rd: u8,
        Rn: u8,
        H: bool,
        Rm: u8,
        M: bool,
        L: bool,
        size: u8,
        U: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        H,
        Rm,
        M,
        L,
        size,
        U,
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
        // S s_0_1: call HaveDOTPExt(s_0_0)
        let s_0_1: bool = HaveDOTPExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b7 b1
        if s_0_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var size:u8
        let s_1_0: u8 = fn_state.size;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #2u : u8
        let s_1_2: u8 = 2;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // D s_1_4: cmp-ne s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) != (s_1_3));
        // N s_1_5: branch s_1_4 b6 b2
        if s_1_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var U:u8
        let s_2_0: bool = fn_state.U;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // D s_2_5: write-var is_signed <= s_2_4
        fn_state.is_signed = s_2_4;
        // D s_2_6: read-var Rd:u8
        let s_2_6: u8 = fn_state.Rd;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 5u16);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: write-var d <= s_2_9
        fn_state.d = s_2_9;
        // D s_2_11: read-var Rn:u8
        let s_2_11: u8 = fn_state.Rn;
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 5u16);
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (s_2_12.value() as i128);
        // D s_2_14: cast reint s_2_13 -> i64
        let s_2_14: i64 = (s_2_13 as i64);
        // D s_2_15: write-var n <= s_2_14
        fn_state.n = s_2_14;
        // D s_2_16: read-var M:u8
        let s_2_16: bool = fn_state.M;
        // D s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 1u16);
        // D s_2_18: read-var Rm:u8
        let s_2_18: u8 = fn_state.Rm;
        // D s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 4u16);
        // D s_2_20: cast reint s_2_17 -> u128
        let s_2_20: u128 = (s_2_17.value() as u128);
        // D s_2_21: size-of s_2_17
        let s_2_21: u16 = s_2_17.length();
        // D s_2_22: cast reint s_2_19 -> u128
        let s_2_22: u128 = (s_2_19.value() as u128);
        // D s_2_23: size-of s_2_19
        let s_2_23: u16 = s_2_19.length();
        // D s_2_24: lsl s_2_20 s_2_23
        let s_2_24: u128 = s_2_20 << s_2_23;
        // D s_2_25: or s_2_24 s_2_22
        let s_2_25: u128 = ((s_2_24) | (s_2_22));
        // D s_2_26: add s_2_21 s_2_23
        let s_2_26: u16 = (s_2_21 + s_2_23);
        // D s_2_27: create-bits s_2_25 s_2_26
        let s_2_27: Bits = Bits::new(s_2_25, s_2_26);
        // D s_2_28: cast reint s_2_27 -> u8
        let s_2_28: u8 = (s_2_27.value() as u8);
        // D s_2_29: cast zx s_2_28 -> bv
        let s_2_29: Bits = Bits::new(s_2_28 as u128, 5u16);
        // D s_2_30: cast zx s_2_29 -> i
        let s_2_30: i128 = (s_2_29.value() as i128);
        // D s_2_31: cast reint s_2_30 -> i64
        let s_2_31: i64 = (s_2_30 as i64);
        // D s_2_32: write-var m <= s_2_31
        fn_state.m = s_2_31;
        // D s_2_33: read-var H:u8
        let s_2_33: bool = fn_state.H;
        // D s_2_34: cast zx s_2_33 -> bv
        let s_2_34: Bits = Bits::new(s_2_33 as u128, 1u16);
        // D s_2_35: read-var L:u8
        let s_2_35: bool = fn_state.L;
        // D s_2_36: cast zx s_2_35 -> bv
        let s_2_36: Bits = Bits::new(s_2_35 as u128, 1u16);
        // D s_2_37: cast reint s_2_34 -> u128
        let s_2_37: u128 = (s_2_34.value() as u128);
        // D s_2_38: size-of s_2_34
        let s_2_38: u16 = s_2_34.length();
        // D s_2_39: cast reint s_2_36 -> u128
        let s_2_39: u128 = (s_2_36.value() as u128);
        // D s_2_40: size-of s_2_36
        let s_2_40: u16 = s_2_36.length();
        // D s_2_41: lsl s_2_37 s_2_40
        let s_2_41: u128 = s_2_37 << s_2_40;
        // D s_2_42: or s_2_41 s_2_39
        let s_2_42: u128 = ((s_2_41) | (s_2_39));
        // D s_2_43: add s_2_38 s_2_40
        let s_2_43: u16 = (s_2_38 + s_2_40);
        // D s_2_44: create-bits s_2_42 s_2_43
        let s_2_44: Bits = Bits::new(s_2_42, s_2_43);
        // D s_2_45: cast reint s_2_44 -> u8
        let s_2_45: u8 = (s_2_44.value() as u8);
        // D s_2_46: cast zx s_2_45 -> bv
        let s_2_46: Bits = Bits::new(s_2_45 as u128, 2u16);
        // D s_2_47: cast zx s_2_46 -> i
        let s_2_47: i128 = (s_2_46.value() as i128);
        // D s_2_48: cast reint s_2_47 -> i64
        let s_2_48: i64 = (s_2_47 as i64);
        // D s_2_49: write-var index <= s_2_48
        fn_state.index = s_2_48;
        // D s_2_50: read-var size:u8
        let s_2_50: u8 = fn_state.size;
        // D s_2_51: cast zx s_2_50 -> bv
        let s_2_51: Bits = Bits::new(s_2_50 as u128, 2u16);
        // D s_2_52: cast zx s_2_51 -> i
        let s_2_52: i128 = (s_2_51.value() as i128);
        // D s_2_53: cast reint s_2_52 -> i64
        let s_2_53: i64 = (s_2_52 as i64);
        // C s_2_54: const #8s : i64
        let s_2_54: i64 = 8;
        // D s_2_55: lsl s_2_54 s_2_53
        let s_2_55: i64 = s_2_54 << s_2_53;
        // D s_2_56: write-var esize <= s_2_55
        fn_state.esize = s_2_55;
        // D s_2_57: read-var Q:u8
        let s_2_57: bool = fn_state.Q;
        // D s_2_58: cast zx s_2_57 -> bv
        let s_2_58: Bits = Bits::new(s_2_57 as u128, 1u16);
        // C s_2_59: const #1u : u8
        let s_2_59: bool = true;
        // C s_2_60: cast zx s_2_59 -> bv
        let s_2_60: Bits = Bits::new(s_2_59 as u128, 1u16);
        // D s_2_61: cmp-eq s_2_58 s_2_60
        let s_2_61: bool = ((s_2_58) == (s_2_60));
        // N s_2_62: branch s_2_61 b5 b3
        if s_2_61 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: write-var ga#265303 <= s_3_0
        fn_state.ga_265303 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#265303:i64
        let s_4_0: i64 = fn_state.ga_265303;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var esize:i64
        let s_4_2: i64 = fn_state.esize;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: div s_4_1 s_4_3
        let s_4_4: i128 = ((s_4_1) / (s_4_3));
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: cast zx s_4_0 -> i
        let s_4_6: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_7: cast reint s_4_6 -> i64
        let s_4_7: i64 = (s_4_6 as i64);
        // D s_4_8: read-var esize:i64
        let s_4_8: i64 = fn_state.esize;
        // D s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (i128::try_from(s_4_8).unwrap());
        // D s_4_10: cast reint s_4_9 -> i64
        let s_4_10: i64 = (s_4_9 as i64);
        // D s_4_11: cast zx s_4_5 -> i
        let s_4_11: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_12: read-var d:i64
        let s_4_12: i64 = fn_state.d;
        // D s_4_13: read-var index:i64
        let s_4_13: i64 = fn_state.index;
        // D s_4_14: read-var m:i64
        let s_4_14: i64 = fn_state.m;
        // D s_4_15: read-var n:i64
        let s_4_15: i64 = fn_state.n;
        // D s_4_16: read-var is_signed:u8
        let s_4_16: bool = fn_state.is_signed;
        // D s_4_17: call execute_aarch64_instrs_vector_arithmetic_binary_element_dotp(s_4_12, s_4_7, s_4_11, s_4_10, s_4_13, s_4_14, s_4_15, s_4_16)
        let s_4_17: () = execute_aarch64_instrs_vector_arithmetic_binary_element_dotp(
            state,
            tracer,
            s_4_12,
            s_4_7,
            s_4_11,
            s_4_10,
            s_4_13,
            s_4_14,
            s_4_15,
            s_4_16,
        );
        // N s_4_18: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #128s : i64
        let s_5_0: i64 = 128;
        // D s_5_1: write-var ga#265303 <= s_5_0
        fn_state.ga_265303 = s_5_0;
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
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
}

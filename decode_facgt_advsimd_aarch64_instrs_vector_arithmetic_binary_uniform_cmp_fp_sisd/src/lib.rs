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
use execute_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_fp16_sisd::*;
use common::*;
pub fn decode_facgt_advsimd_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_fp_sisd<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    ac: bool,
    Rm: u8,
    sz: bool,
    E: bool,
    U: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        ga_252822: u8,
        cmp: u32,
        esize: i64,
        abs: bool,
        n: i64,
        d: i64,
        datasize: i64,
        Rd: u8,
        Rn: u8,
        ac: bool,
        Rm: u8,
        sz: bool,
        E: bool,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        ac,
        Rm,
        sz,
        E,
        U,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rd:u8
        let s_0_0: u8 = fn_state.Rd;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var d <= s_0_3
        fn_state.d = s_0_3;
        // D s_0_5: read-var Rn:u8
        let s_0_5: u8 = fn_state.Rn;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var n <= s_0_8
        fn_state.n = s_0_8;
        // D s_0_10: read-var Rm:u8
        let s_0_10: u8 = fn_state.Rm;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 5u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: write-var m <= s_0_13
        fn_state.m = s_0_13;
        // D s_0_15: read-var sz:u8
        let s_0_15: bool = fn_state.sz;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 1u16);
        // D s_0_17: cast zx s_0_16 -> i
        let s_0_17: i128 = (s_0_16.value() as i128);
        // D s_0_18: cast reint s_0_17 -> i64
        let s_0_18: i64 = (s_0_17 as i64);
        // C s_0_19: const #32s : i64
        let s_0_19: i64 = 32;
        // D s_0_20: lsl s_0_19 s_0_18
        let s_0_20: i64 = s_0_19 << s_0_18;
        // D s_0_21: write-var esize <= s_0_20
        fn_state.esize = s_0_20;
        // D s_0_22: read-var esize:i64
        let s_0_22: i64 = fn_state.esize;
        // D s_0_23: write-var datasize <= s_0_22
        fn_state.datasize = s_0_22;
        // D s_0_24: read-var E:u8
        let s_0_24: bool = fn_state.E;
        // D s_0_25: cast zx s_0_24 -> bv
        let s_0_25: Bits = Bits::new(s_0_24 as u128, 1u16);
        // D s_0_26: read-var U:u8
        let s_0_26: bool = fn_state.U;
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 1u16);
        // D s_0_28: cast reint s_0_25 -> u128
        let s_0_28: u128 = (s_0_25.value() as u128);
        // D s_0_29: size-of s_0_25
        let s_0_29: u16 = s_0_25.length();
        // D s_0_30: cast reint s_0_27 -> u128
        let s_0_30: u128 = (s_0_27.value() as u128);
        // D s_0_31: size-of s_0_27
        let s_0_31: u16 = s_0_27.length();
        // D s_0_32: lsl s_0_28 s_0_31
        let s_0_32: u128 = s_0_28 << s_0_31;
        // D s_0_33: or s_0_32 s_0_30
        let s_0_33: u128 = ((s_0_32) | (s_0_30));
        // D s_0_34: add s_0_29 s_0_31
        let s_0_34: u16 = (s_0_29 + s_0_31);
        // D s_0_35: create-bits s_0_33 s_0_34
        let s_0_35: Bits = Bits::new(s_0_33, s_0_34);
        // D s_0_36: cast reint s_0_35 -> u8
        let s_0_36: u8 = (s_0_35.value() as u8);
        // D s_0_37: cast zx s_0_36 -> bv
        let s_0_37: Bits = Bits::new(s_0_36 as u128, 2u16);
        // D s_0_38: read-var ac:u8
        let s_0_38: bool = fn_state.ac;
        // D s_0_39: cast zx s_0_38 -> bv
        let s_0_39: Bits = Bits::new(s_0_38 as u128, 1u16);
        // D s_0_40: cast reint s_0_37 -> u128
        let s_0_40: u128 = (s_0_37.value() as u128);
        // D s_0_41: size-of s_0_37
        let s_0_41: u16 = s_0_37.length();
        // D s_0_42: cast reint s_0_39 -> u128
        let s_0_42: u128 = (s_0_39.value() as u128);
        // D s_0_43: size-of s_0_39
        let s_0_43: u16 = s_0_39.length();
        // D s_0_44: lsl s_0_40 s_0_43
        let s_0_44: u128 = s_0_40 << s_0_43;
        // D s_0_45: or s_0_44 s_0_42
        let s_0_45: u128 = ((s_0_44) | (s_0_42));
        // D s_0_46: add s_0_41 s_0_43
        let s_0_46: u16 = (s_0_41 + s_0_43);
        // D s_0_47: create-bits s_0_45 s_0_46
        let s_0_47: Bits = Bits::new(s_0_45, s_0_46);
        // D s_0_48: cast reint s_0_47 -> u8
        let s_0_48: u8 = (s_0_47.value() as u8);
        // D s_0_49: write-var ga#252822 <= s_0_48
        fn_state.ga_252822 = s_0_48;
        // D s_0_50: read-var ga#252822:u8
        let s_0_50: u8 = fn_state.ga_252822;
        // D s_0_51: cast zx s_0_50 -> bv
        let s_0_51: Bits = Bits::new(s_0_50 as u128, 3u16);
        // C s_0_52: const #0u : u8
        let s_0_52: u8 = 0;
        // C s_0_53: cast zx s_0_52 -> bv
        let s_0_53: Bits = Bits::new(s_0_52 as u128, 3u16);
        // D s_0_54: cmp-eq s_0_51 s_0_53
        let s_0_54: bool = ((s_0_51) == (s_0_53));
        // D s_0_55: not s_0_54
        let s_0_55: bool = !s_0_54;
        // N s_0_56: branch s_0_55 b3 b1
        if s_0_55 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #2u : u32
        let s_1_0: u32 = 2;
        // D s_1_1: write-var cmp <= s_1_0
        fn_state.cmp = s_1_0;
        // C s_1_2: const #0u : u8
        let s_1_2: bool = false;
        // D s_1_3: write-var abs <= s_1_2
        fn_state.abs = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var abs:u8
        let s_2_0: bool = fn_state.abs;
        // D s_2_1: read-var cmp:u32
        let s_2_1: u32 = fn_state.cmp;
        // D s_2_2: read-var datasize:i64
        let s_2_2: i64 = fn_state.datasize;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: read-var esize:i64
        let s_2_5: i64 = fn_state.esize;
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_7: cast reint s_2_6 -> i64
        let s_2_7: i64 = (s_2_6 as i64);
        // C s_2_8: const #1s : i
        let s_2_8: i128 = 1;
        // D s_2_9: read-var d:i64
        let s_2_9: i64 = fn_state.d;
        // D s_2_10: read-var m:i64
        let s_2_10: i64 = fn_state.m;
        // D s_2_11: read-var n:i64
        let s_2_11: i64 = fn_state.n;
        // D s_2_12: call execute_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_fp16_sisd(s_2_0, s_2_1, s_2_9, s_2_4, s_2_8, s_2_7, s_2_10, s_2_11)
        let s_2_12: () = execute_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_fp16_sisd(
            state,
            tracer,
            s_2_0,
            s_2_1,
            s_2_9,
            s_2_4,
            s_2_8,
            s_2_7,
            s_2_10,
            s_2_11,
        );
        // N s_2_13: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#252822:u8
        let s_3_0: u8 = fn_state.ga_252822;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 3u16);
        // C s_3_2: const #2u : u8
        let s_3_2: u8 = 2;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 3u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: not s_3_4
        let s_3_5: bool = !s_3_4;
        // N s_3_6: branch s_3_5 b5 b4
        if s_3_5 {
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
        // C s_4_0: const #1u : u32
        let s_4_0: u32 = 1;
        // D s_4_1: write-var cmp <= s_4_0
        fn_state.cmp = s_4_0;
        // C s_4_2: const #0u : u8
        let s_4_2: bool = false;
        // D s_4_3: write-var abs <= s_4_2
        fn_state.abs = s_4_2;
        // N s_4_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#252822:u8
        let s_5_0: u8 = fn_state.ga_252822;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 3u16);
        // C s_5_2: const #3u : u8
        let s_5_2: u8 = 3;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 3u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u32
        let s_6_0: u32 = 1;
        // D s_6_1: write-var cmp <= s_6_0
        fn_state.cmp = s_6_0;
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // D s_6_3: write-var abs <= s_6_2
        fn_state.abs = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#252822:u8
        let s_7_0: u8 = fn_state.ga_252822;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 3u16);
        // C s_7_2: const #6u : u8
        let s_7_2: u8 = 6;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 3u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: not s_7_4
        let s_7_5: bool = !s_7_4;
        // N s_7_6: branch s_7_5 b9 b8
        if s_7_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u32
        let s_8_0: u32 = 0;
        // D s_8_1: write-var cmp <= s_8_0
        fn_state.cmp = s_8_0;
        // C s_8_2: const #0u : u8
        let s_8_2: bool = false;
        // D s_8_3: write-var abs <= s_8_2
        fn_state.abs = s_8_2;
        // N s_8_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#252822:u8
        let s_9_0: u8 = fn_state.ga_252822;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 3u16);
        // C s_9_2: const #7u : u8
        let s_9_2: u8 = 7;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 3u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: not s_9_4
        let s_9_5: bool = !s_9_4;
        // N s_9_6: branch s_9_5 b11 b10
        if s_9_5 {
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
        // C s_10_0: const #0u : u32
        let s_10_0: u32 = 0;
        // D s_10_1: write-var cmp <= s_10_0
        fn_state.cmp = s_10_0;
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // D s_10_3: write-var abs <= s_10_2
        fn_state.abs = s_10_2;
        // N s_10_4: jump b2
        return block_2(state, tracer, fn_state);
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
}

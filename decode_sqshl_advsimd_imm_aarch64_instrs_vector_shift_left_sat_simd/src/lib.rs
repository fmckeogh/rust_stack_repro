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
use u__id::*;
use HighestSetBit::*;
use u_shl_int_general::*;
use execute_aarch64_instrs_vector_shift_left_sat_sisd::*;
use fdiv_int::*;
use common::*;
pub fn decode_sqshl_advsimd_imm_aarch64_instrs_vector_shift_left_sat_simd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    op: bool,
    immb: u8,
    immh: u8,
    U: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        src_unsignedshadow_1960: bool,
        esize: i128,
        ga_268487: i64,
        shift: i128,
        n: i64,
        gs_172144: bool,
        d: i64,
        src_unsigned: bool,
        elements: i128,
        dst_unsignedshadow_1959: bool,
        gs_172142: bool,
        gs_172148: bool,
        datasize: i64,
        dst_unsigned: bool,
        gs_172146: bool,
        ga_268492: u8,
        Rd: u8,
        Rn: u8,
        op: bool,
        immb: u8,
        immh: u8,
        U: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        op,
        immb,
        immh,
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
        // D s_0_10: read-var immh:u8
        let s_0_10: u8 = fn_state.immh;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 4u16);
        // C s_0_12: const #0u : u8
        let s_0_12: u8 = 0;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 4u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b26 b1
        if s_0_14 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #3s : i
        let s_1_0: i128 = 3;
        // D s_1_1: read-var immh:u8
        let s_1_1: u8 = fn_state.immh;
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 4u16);
        // C s_1_3: const #1u : u64
        let s_1_3: u64 = 1;
        // D s_1_4: bit-extract s_1_2 s_1_0 s_1_3
        let s_1_4: Bits = (Bits::new(
            ((s_1_2) >> (s_1_0)).value(),
            u16::try_from(s_1_3).unwrap(),
        ));
        // D s_1_5: cast reint s_1_4 -> u8
        let s_1_5: bool = ((s_1_4.value()) != 0);
        // C s_1_6: const #0s : i
        let s_1_6: i128 = 0;
        // C s_1_7: const #0u : u64
        let s_1_7: u64 = 0;
        // D s_1_8: cast zx s_1_5 -> u64
        let s_1_8: u64 = (s_1_5 as u64);
        // C s_1_9: const #1u : u64
        let s_1_9: u64 = 1;
        // D s_1_10: and s_1_8 s_1_9
        let s_1_10: u64 = ((s_1_8) & (s_1_9));
        // D s_1_11: cmp-eq s_1_10 s_1_9
        let s_1_11: bool = ((s_1_10) == (s_1_9));
        // D s_1_12: lsl s_1_8 s_1_6
        let s_1_12: u64 = s_1_8 << s_1_6;
        // D s_1_13: or s_1_7 s_1_12
        let s_1_13: u64 = ((s_1_7) | (s_1_12));
        // D s_1_14: cmpl s_1_12
        let s_1_14: u64 = !s_1_12;
        // D s_1_15: and s_1_7 s_1_14
        let s_1_15: u64 = ((s_1_7) & (s_1_14));
        // D s_1_16: select s_1_11 s_1_13 s_1_15
        let s_1_16: u64 = if s_1_11 { s_1_13 } else { s_1_15 };
        // D s_1_17: cast trunc s_1_16 -> u8
        let s_1_17: bool = ((s_1_16) != 0);
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 1u16);
        // D s_1_19: read-var Q:u8
        let s_1_19: bool = fn_state.Q;
        // D s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 1u16);
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
        let s_1_30: Bits = Bits::new(s_1_29 as u128, 2u16);
        // C s_1_31: const #2u : u8
        let s_1_31: u8 = 2;
        // C s_1_32: cast zx s_1_31 -> bv
        let s_1_32: Bits = Bits::new(s_1_31 as u128, 2u16);
        // D s_1_33: cmp-eq s_1_30 s_1_32
        let s_1_33: bool = ((s_1_30) == (s_1_32));
        // N s_1_34: branch s_1_33 b25 b2
        if s_1_33 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var immh:u8
        let s_2_0: u8 = fn_state.immh;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: call HighestSetBit(s_2_1)
        let s_2_2: i128 = HighestSetBit(state, tracer, s_2_1);
        // C s_2_3: const #8s : i
        let s_2_3: i128 = 8;
        // D s_2_4: call _shl_int_general(s_2_3, s_2_2)
        let s_2_4: i128 = u_shl_int_general(state, tracer, s_2_3, s_2_2);
        // D s_2_5: write-var esize <= s_2_4
        fn_state.esize = s_2_4;
        // D s_2_6: read-var Q:u8
        let s_2_6: bool = fn_state.Q;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 1u16);
        // C s_2_8: const #1u : u8
        let s_2_8: bool = true;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 1u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // N s_2_11: branch s_2_10 b24 b3
        if s_2_10 {
            return block_24(state, tracer, fn_state);
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
        // D s_3_1: write-var ga#268487 <= s_3_0
        fn_state.ga_268487 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#268487:i64
        let s_4_0: i64 = fn_state.ga_268487;
        // D s_4_1: write-var datasize <= s_4_0
        fn_state.datasize = s_4_0;
        // D s_4_2: read-var datasize:i64
        let s_4_2: i64 = fn_state.datasize;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-var esize:i
        let s_4_4: i128 = fn_state.esize;
        // D s_4_5: call fdiv_int(s_4_3, s_4_4)
        let s_4_5: i128 = fdiv_int(state, tracer, s_4_3, s_4_4);
        // D s_4_6: write-var elements <= s_4_5
        fn_state.elements = s_4_5;
        // D s_4_7: read-var immh:u8
        let s_4_7: u8 = fn_state.immh;
        // D s_4_8: cast zx s_4_7 -> bv
        let s_4_8: Bits = Bits::new(s_4_7 as u128, 4u16);
        // D s_4_9: read-var immb:u8
        let s_4_9: u8 = fn_state.immb;
        // D s_4_10: cast zx s_4_9 -> bv
        let s_4_10: Bits = Bits::new(s_4_9 as u128, 3u16);
        // D s_4_11: cast reint s_4_8 -> u128
        let s_4_11: u128 = (s_4_8.value() as u128);
        // D s_4_12: size-of s_4_8
        let s_4_12: u16 = s_4_8.length();
        // D s_4_13: cast reint s_4_10 -> u128
        let s_4_13: u128 = (s_4_10.value() as u128);
        // D s_4_14: size-of s_4_10
        let s_4_14: u16 = s_4_10.length();
        // D s_4_15: lsl s_4_11 s_4_14
        let s_4_15: u128 = s_4_11 << s_4_14;
        // D s_4_16: or s_4_15 s_4_13
        let s_4_16: u128 = ((s_4_15) | (s_4_13));
        // D s_4_17: add s_4_12 s_4_14
        let s_4_17: u16 = (s_4_12 + s_4_14);
        // D s_4_18: create-bits s_4_16 s_4_17
        let s_4_18: Bits = Bits::new(s_4_16, s_4_17);
        // D s_4_19: cast reint s_4_18 -> u8
        let s_4_19: u8 = (s_4_18.value() as u8);
        // D s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 7u16);
        // D s_4_21: cast zx s_4_20 -> i
        let s_4_21: i128 = (s_4_20.value() as i128);
        // D s_4_22: cast reint s_4_21 -> i64
        let s_4_22: i64 = (s_4_21 as i64);
        // D s_4_23: cast zx s_4_22 -> i
        let s_4_23: i128 = (i128::try_from(s_4_22).unwrap());
        // D s_4_24: read-var esize:i
        let s_4_24: i128 = fn_state.esize;
        // D s_4_25: sub s_4_23 s_4_24
        let s_4_25: i128 = ((s_4_23) - (s_4_24));
        // D s_4_26: write-var shift <= s_4_25
        fn_state.shift = s_4_25;
        // D s_4_27: read-var op:u8
        let s_4_27: bool = fn_state.op;
        // D s_4_28: cast zx s_4_27 -> bv
        let s_4_28: Bits = Bits::new(s_4_27 as u128, 1u16);
        // D s_4_29: read-var U:u8
        let s_4_29: bool = fn_state.U;
        // D s_4_30: cast zx s_4_29 -> bv
        let s_4_30: Bits = Bits::new(s_4_29 as u128, 1u16);
        // D s_4_31: cast reint s_4_28 -> u128
        let s_4_31: u128 = (s_4_28.value() as u128);
        // D s_4_32: size-of s_4_28
        let s_4_32: u16 = s_4_28.length();
        // D s_4_33: cast reint s_4_30 -> u128
        let s_4_33: u128 = (s_4_30.value() as u128);
        // D s_4_34: size-of s_4_30
        let s_4_34: u16 = s_4_30.length();
        // D s_4_35: lsl s_4_31 s_4_34
        let s_4_35: u128 = s_4_31 << s_4_34;
        // D s_4_36: or s_4_35 s_4_33
        let s_4_36: u128 = ((s_4_35) | (s_4_33));
        // D s_4_37: add s_4_32 s_4_34
        let s_4_37: u16 = (s_4_32 + s_4_34);
        // D s_4_38: create-bits s_4_36 s_4_37
        let s_4_38: Bits = Bits::new(s_4_36, s_4_37);
        // D s_4_39: cast reint s_4_38 -> u8
        let s_4_39: u8 = (s_4_38.value() as u8);
        // D s_4_40: write-var ga#268492 <= s_4_39
        fn_state.ga_268492 = s_4_39;
        // D s_4_41: read-var ga#268492:u8
        let s_4_41: u8 = fn_state.ga_268492;
        // D s_4_42: cast zx s_4_41 -> bv
        let s_4_42: Bits = Bits::new(s_4_41 as u128, 2u16);
        // C s_4_43: const #0u : u8
        let s_4_43: u8 = 0;
        // C s_4_44: cast zx s_4_43 -> bv
        let s_4_44: Bits = Bits::new(s_4_43 as u128, 2u16);
        // D s_4_45: cmp-eq s_4_42 s_4_44
        let s_4_45: bool = ((s_4_42) == (s_4_44));
        // D s_4_46: not s_4_45
        let s_4_46: bool = !s_4_45;
        // N s_4_47: branch s_4_46 b6 b5
        if s_4_46 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
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
        // D s_6_0: read-var ga#268492:u8
        let s_6_0: u8 = fn_state.ga_268492;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #1u : u8
        let s_6_2: u8 = 1;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: not s_6_4
        let s_6_5: bool = !s_6_4;
        // N s_6_6: branch s_6_5 b21 b7
        if s_6_5 {
            return block_21(state, tracer, fn_state);
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
        // D s_7_1: write-var src_unsigned <= s_7_0
        fn_state.src_unsigned = s_7_0;
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // D s_7_3: write-var dst_unsigned <= s_7_2
        fn_state.dst_unsigned = s_7_2;
        // N s_7_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var dst_unsigned:u8
        let s_8_0: bool = fn_state.dst_unsigned;
        // D s_8_1: write-var dst_unsignedshadow#1959 <= s_8_0
        fn_state.dst_unsignedshadow_1959 = s_8_0;
        // D s_8_2: read-var src_unsigned:u8
        let s_8_2: bool = fn_state.src_unsigned;
        // D s_8_3: write-var src_unsignedshadow#1960 <= s_8_2
        fn_state.src_unsignedshadow_1960 = s_8_2;
        // D s_8_4: read-var esize:i
        let s_8_4: i128 = fn_state.esize;
        // D s_8_5: call __id(s_8_4)
        let s_8_5: i128 = u__id(state, tracer, s_8_4);
        // C s_8_6: const #4s : i
        let s_8_6: i128 = 4;
        // D s_8_7: cmp-eq s_8_5 s_8_6
        let s_8_7: bool = ((s_8_5) == (s_8_6));
        // N s_8_8: branch s_8_7 b20 b9
        if s_8_7 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esize:i
        let s_9_0: i128 = fn_state.esize;
        // D s_9_1: call __id(s_9_0)
        let s_9_1: i128 = u__id(state, tracer, s_9_0);
        // C s_9_2: const #8s : i
        let s_9_2: i128 = 8;
        // D s_9_3: cmp-eq s_9_1 s_9_2
        let s_9_3: bool = ((s_9_1) == (s_9_2));
        // D s_9_4: write-var gs#172142 <= s_9_3
        fn_state.gs_172142 = s_9_3;
        // N s_9_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#172142:u8
        let s_10_0: bool = fn_state.gs_172142;
        // N s_10_1: branch s_10_0 b19 b11
        if s_10_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esize:i
        let s_11_0: i128 = fn_state.esize;
        // D s_11_1: call __id(s_11_0)
        let s_11_1: i128 = u__id(state, tracer, s_11_0);
        // C s_11_2: const #16s : i
        let s_11_2: i128 = 16;
        // D s_11_3: cmp-eq s_11_1 s_11_2
        let s_11_3: bool = ((s_11_1) == (s_11_2));
        // D s_11_4: write-var gs#172144 <= s_11_3
        fn_state.gs_172144 = s_11_3;
        // N s_11_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#172144:u8
        let s_12_0: bool = fn_state.gs_172144;
        // N s_12_1: branch s_12_0 b18 b13
        if s_12_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var esize:i
        let s_13_0: i128 = fn_state.esize;
        // D s_13_1: call __id(s_13_0)
        let s_13_1: i128 = u__id(state, tracer, s_13_0);
        // C s_13_2: const #32s : i
        let s_13_2: i128 = 32;
        // D s_13_3: cmp-eq s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) == (s_13_2));
        // D s_13_4: write-var gs#172146 <= s_13_3
        fn_state.gs_172146 = s_13_3;
        // N s_13_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#172146:u8
        let s_14_0: bool = fn_state.gs_172146;
        // N s_14_1: branch s_14_0 b17 b15
        if s_14_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var esize:i
        let s_15_0: i128 = fn_state.esize;
        // D s_15_1: call __id(s_15_0)
        let s_15_1: i128 = u__id(state, tracer, s_15_0);
        // C s_15_2: const #64s : i
        let s_15_2: i128 = 64;
        // D s_15_3: cmp-eq s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) == (s_15_2));
        // D s_15_4: write-var gs#172148 <= s_15_3
        fn_state.gs_172148 = s_15_3;
        // N s_15_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#172148:u8
        let s_16_0: bool = fn_state.gs_172148;
        // N s_16_1: assert s_16_0
        let s_16_1: () = assert!(s_16_0);
        // D s_16_2: read-var datasize:i64
        let s_16_2: i64 = fn_state.datasize;
        // D s_16_3: cast zx s_16_2 -> i
        let s_16_3: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_4: cast reint s_16_3 -> i64
        let s_16_4: i64 = (s_16_3 as i64);
        // D s_16_5: read-var esize:i
        let s_16_5: i128 = fn_state.esize;
        // D s_16_6: cast reint s_16_5 -> i64
        let s_16_6: i64 = (s_16_5 as i64);
        // D s_16_7: read-var d:i64
        let s_16_7: i64 = fn_state.d;
        // D s_16_8: read-var dst_unsignedshadow#1959:u8
        let s_16_8: bool = fn_state.dst_unsignedshadow_1959;
        // D s_16_9: read-var elements:i
        let s_16_9: i128 = fn_state.elements;
        // D s_16_10: read-var n:i64
        let s_16_10: i64 = fn_state.n;
        // D s_16_11: read-var shift:i
        let s_16_11: i128 = fn_state.shift;
        // D s_16_12: read-var src_unsignedshadow#1960:u8
        let s_16_12: bool = fn_state.src_unsignedshadow_1960;
        // D s_16_13: call execute_aarch64_instrs_vector_shift_left_sat_sisd(s_16_7, s_16_4, s_16_8, s_16_9, s_16_6, s_16_10, s_16_11, s_16_12)
        let s_16_13: () = execute_aarch64_instrs_vector_shift_left_sat_sisd(
            state,
            tracer,
            s_16_7,
            s_16_4,
            s_16_8,
            s_16_9,
            s_16_6,
            s_16_10,
            s_16_11,
            s_16_12,
        );
        // N s_16_14: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#172148 <= s_17_0
        fn_state.gs_172148 = s_17_0;
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#172146 <= s_18_0
        fn_state.gs_172146 = s_18_0;
        // N s_18_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#172144 <= s_19_0
        fn_state.gs_172144 = s_19_0;
        // N s_19_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#172142 <= s_20_0
        fn_state.gs_172142 = s_20_0;
        // N s_20_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var ga#268492:u8
        let s_21_0: u8 = fn_state.ga_268492;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 2u16);
        // C s_21_2: const #2u : u8
        let s_21_2: u8 = 2;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 2u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: not s_21_4
        let s_21_5: bool = !s_21_4;
        // N s_21_6: branch s_21_5 b23 b22
        if s_21_5 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var src_unsigned <= s_22_0
        fn_state.src_unsigned = s_22_0;
        // C s_22_2: const #0u : u8
        let s_22_2: bool = false;
        // D s_22_3: write-var dst_unsigned <= s_22_2
        fn_state.dst_unsigned = s_22_2;
        // N s_22_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var src_unsigned <= s_23_0
        fn_state.src_unsigned = s_23_0;
        // C s_23_2: const #1u : u8
        let s_23_2: bool = true;
        // D s_23_3: write-var dst_unsigned <= s_23_2
        fn_state.dst_unsigned = s_23_2;
        // N s_23_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #128s : i64
        let s_24_0: i64 = 128;
        // D s_24_1: write-var ga#268487 <= s_24_0
        fn_state.ga_268487 = s_24_0;
        // N s_24_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
}

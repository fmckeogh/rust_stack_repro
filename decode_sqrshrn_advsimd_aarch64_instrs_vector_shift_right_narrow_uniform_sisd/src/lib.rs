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
use execute_aarch64_instrs_vector_shift_right_narrow_uniform_sisd::*;
use u__id::*;
use HighestSetBit::*;
use u_shl_int_general::*;
use common::*;
pub fn decode_sqrshrn_advsimd_aarch64_instrs_vector_shift_right_narrow_uniform_sisd<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    op: bool,
    immb: u8,
    immh: u8,
    U: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_171646: bool,
        gs_171644: bool,
        esize: i128,
        shift: i128,
        gs_171637: bool,
        gs_171647: bool,
        n: i64,
        gs_171633: bool,
        round: bool,
        d: i64,
        is_unsigned: bool,
        gs_171631: bool,
        gs_171640: bool,
        gs_171642: bool,
        datasize: i128,
        gs_171635: bool,
        Rd: u8,
        Rn: u8,
        op: bool,
        immb: u8,
        immh: u8,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        op,
        immb,
        immh,
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
        // N s_0_15: branch s_0_14 b31 b1
        if s_0_14 {
            return block_31(state, tracer, fn_state);
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
        // C s_1_19: const #1u : u8
        let s_1_19: bool = true;
        // C s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 1u16);
        // D s_1_21: cmp-eq s_1_18 s_1_20
        let s_1_21: bool = ((s_1_18) == (s_1_20));
        // N s_1_22: branch s_1_21 b30 b2
        if s_1_21 {
            return block_30(state, tracer, fn_state);
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
        // D s_2_6: read-var esize:i
        let s_2_6: i128 = fn_state.esize;
        // D s_2_7: write-var datasize <= s_2_6
        fn_state.datasize = s_2_6;
        // C s_2_8: const #2s : i
        let s_2_8: i128 = 2;
        // D s_2_9: read-var esize:i
        let s_2_9: i128 = fn_state.esize;
        // D s_2_10: mul s_2_8 s_2_9
        let s_2_10: i128 = ((s_2_8) * (s_2_9));
        // D s_2_11: read-var immh:u8
        let s_2_11: u8 = fn_state.immh;
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 4u16);
        // D s_2_13: read-var immb:u8
        let s_2_13: u8 = fn_state.immb;
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
        let s_2_24: Bits = Bits::new(s_2_23 as u128, 7u16);
        // D s_2_25: cast zx s_2_24 -> i
        let s_2_25: i128 = (s_2_24.value() as i128);
        // D s_2_26: cast reint s_2_25 -> i64
        let s_2_26: i64 = (s_2_25 as i64);
        // D s_2_27: cast zx s_2_26 -> i
        let s_2_27: i128 = (i128::try_from(s_2_26).unwrap());
        // D s_2_28: sub s_2_10 s_2_27
        let s_2_28: i128 = ((s_2_10) - (s_2_27));
        // D s_2_29: write-var shift <= s_2_28
        fn_state.shift = s_2_28;
        // D s_2_30: read-var op:u8
        let s_2_30: bool = fn_state.op;
        // D s_2_31: cast zx s_2_30 -> bv
        let s_2_31: Bits = Bits::new(s_2_30 as u128, 1u16);
        // C s_2_32: const #1u : u8
        let s_2_32: bool = true;
        // C s_2_33: cast zx s_2_32 -> bv
        let s_2_33: Bits = Bits::new(s_2_32 as u128, 1u16);
        // D s_2_34: cmp-eq s_2_31 s_2_33
        let s_2_34: bool = ((s_2_31) == (s_2_33));
        // D s_2_35: write-var round <= s_2_34
        fn_state.round = s_2_34;
        // D s_2_36: read-var U:u8
        let s_2_36: bool = fn_state.U;
        // D s_2_37: cast zx s_2_36 -> bv
        let s_2_37: Bits = Bits::new(s_2_36 as u128, 1u16);
        // C s_2_38: const #1u : u8
        let s_2_38: bool = true;
        // C s_2_39: cast zx s_2_38 -> bv
        let s_2_39: Bits = Bits::new(s_2_38 as u128, 1u16);
        // D s_2_40: cmp-eq s_2_37 s_2_39
        let s_2_40: bool = ((s_2_37) == (s_2_39));
        // D s_2_41: write-var is_unsigned <= s_2_40
        fn_state.is_unsigned = s_2_40;
        // D s_2_42: read-var esize:i
        let s_2_42: i128 = fn_state.esize;
        // D s_2_43: call __id(s_2_42)
        let s_2_43: i128 = u__id(state, tracer, s_2_42);
        // C s_2_44: const #4s : i
        let s_2_44: i128 = 4;
        // D s_2_45: cmp-eq s_2_43 s_2_44
        let s_2_45: bool = ((s_2_43) == (s_2_44));
        // N s_2_46: branch s_2_45 b29 b3
        if s_2_45 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esize:i
        let s_3_0: i128 = fn_state.esize;
        // D s_3_1: call __id(s_3_0)
        let s_3_1: i128 = u__id(state, tracer, s_3_0);
        // C s_3_2: const #8s : i
        let s_3_2: i128 = 8;
        // D s_3_3: cmp-eq s_3_1 s_3_2
        let s_3_3: bool = ((s_3_1) == (s_3_2));
        // D s_3_4: write-var gs#171631 <= s_3_3
        fn_state.gs_171631 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#171631:u8
        let s_4_0: bool = fn_state.gs_171631;
        // N s_4_1: branch s_4_0 b28 b5
        if s_4_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esize:i
        let s_5_0: i128 = fn_state.esize;
        // D s_5_1: call __id(s_5_0)
        let s_5_1: i128 = u__id(state, tracer, s_5_0);
        // C s_5_2: const #16s : i
        let s_5_2: i128 = 16;
        // D s_5_3: cmp-eq s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) == (s_5_2));
        // D s_5_4: write-var gs#171633 <= s_5_3
        fn_state.gs_171633 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#171633:u8
        let s_6_0: bool = fn_state.gs_171633;
        // N s_6_1: branch s_6_0 b27 b7
        if s_6_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esize:i
        let s_7_0: i128 = fn_state.esize;
        // D s_7_1: call __id(s_7_0)
        let s_7_1: i128 = u__id(state, tracer, s_7_0);
        // C s_7_2: const #32s : i
        let s_7_2: i128 = 32;
        // D s_7_3: cmp-eq s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) == (s_7_2));
        // D s_7_4: write-var gs#171635 <= s_7_3
        fn_state.gs_171635 = s_7_3;
        // N s_7_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#171635:u8
        let s_8_0: bool = fn_state.gs_171635;
        // N s_8_1: branch s_8_0 b26 b9
        if s_8_0 {
            return block_26(state, tracer, fn_state);
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
        // C s_9_2: const #64s : i
        let s_9_2: i128 = 64;
        // D s_9_3: cmp-eq s_9_1 s_9_2
        let s_9_3: bool = ((s_9_1) == (s_9_2));
        // D s_9_4: write-var gs#171637 <= s_9_3
        fn_state.gs_171637 = s_9_3;
        // N s_9_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#171637:u8
        let s_10_0: bool = fn_state.gs_171637;
        // N s_10_1: branch s_10_0 b13 b11
        if s_10_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#171647 <= s_11_0
        fn_state.gs_171647 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#171647:u8
        let s_12_0: bool = fn_state.gs_171647;
        // N s_12_1: assert s_12_0
        let s_12_1: () = assert!(s_12_0);
        // D s_12_2: read-var datasize:i
        let s_12_2: i128 = fn_state.datasize;
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // D s_12_4: read-var esize:i
        let s_12_4: i128 = fn_state.esize;
        // D s_12_5: cast reint s_12_4 -> i64
        let s_12_5: i64 = (s_12_4 as i64);
        // C s_12_6: const #1s : i
        let s_12_6: i128 = 1;
        // C s_12_7: const #0s : i64
        let s_12_7: i64 = 0;
        // D s_12_8: read-var d:i64
        let s_12_8: i64 = fn_state.d;
        // D s_12_9: read-var n:i64
        let s_12_9: i64 = fn_state.n;
        // D s_12_10: read-var round:u8
        let s_12_10: bool = fn_state.round;
        // D s_12_11: read-var shift:i
        let s_12_11: i128 = fn_state.shift;
        // D s_12_12: read-var is_unsigned:u8
        let s_12_12: bool = fn_state.is_unsigned;
        // D s_12_13: call execute_aarch64_instrs_vector_shift_right_narrow_uniform_sisd(s_12_8, s_12_3, s_12_6, s_12_5, s_12_9, s_12_7, s_12_10, s_12_11, s_12_12)
        let s_12_13: () = execute_aarch64_instrs_vector_shift_right_narrow_uniform_sisd(
            state,
            tracer,
            s_12_8,
            s_12_3,
            s_12_6,
            s_12_5,
            s_12_9,
            s_12_7,
            s_12_10,
            s_12_11,
            s_12_12,
        );
        // N s_12_14: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var datasize:i
        let s_13_0: i128 = fn_state.datasize;
        // D s_13_1: call __id(s_13_0)
        let s_13_1: i128 = u__id(state, tracer, s_13_0);
        // C s_13_2: const #4s : i
        let s_13_2: i128 = 4;
        // D s_13_3: cmp-eq s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) == (s_13_2));
        // N s_13_4: branch s_13_3 b25 b14
        if s_13_3 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var datasize:i
        let s_14_0: i128 = fn_state.datasize;
        // D s_14_1: call __id(s_14_0)
        let s_14_1: i128 = u__id(state, tracer, s_14_0);
        // C s_14_2: const #8s : i
        let s_14_2: i128 = 8;
        // D s_14_3: cmp-eq s_14_1 s_14_2
        let s_14_3: bool = ((s_14_1) == (s_14_2));
        // D s_14_4: write-var gs#171640 <= s_14_3
        fn_state.gs_171640 = s_14_3;
        // N s_14_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#171640:u8
        let s_15_0: bool = fn_state.gs_171640;
        // N s_15_1: branch s_15_0 b24 b16
        if s_15_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var datasize:i
        let s_16_0: i128 = fn_state.datasize;
        // D s_16_1: call __id(s_16_0)
        let s_16_1: i128 = u__id(state, tracer, s_16_0);
        // C s_16_2: const #16s : i
        let s_16_2: i128 = 16;
        // D s_16_3: cmp-eq s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) == (s_16_2));
        // D s_16_4: write-var gs#171642 <= s_16_3
        fn_state.gs_171642 = s_16_3;
        // N s_16_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#171642:u8
        let s_17_0: bool = fn_state.gs_171642;
        // N s_17_1: branch s_17_0 b23 b18
        if s_17_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var datasize:i
        let s_18_0: i128 = fn_state.datasize;
        // D s_18_1: call __id(s_18_0)
        let s_18_1: i128 = u__id(state, tracer, s_18_0);
        // C s_18_2: const #32s : i
        let s_18_2: i128 = 32;
        // D s_18_3: cmp-eq s_18_1 s_18_2
        let s_18_3: bool = ((s_18_1) == (s_18_2));
        // D s_18_4: write-var gs#171644 <= s_18_3
        fn_state.gs_171644 = s_18_3;
        // N s_18_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#171644:u8
        let s_19_0: bool = fn_state.gs_171644;
        // N s_19_1: branch s_19_0 b22 b20
        if s_19_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var datasize:i
        let s_20_0: i128 = fn_state.datasize;
        // D s_20_1: call __id(s_20_0)
        let s_20_1: i128 = u__id(state, tracer, s_20_0);
        // C s_20_2: const #64s : i
        let s_20_2: i128 = 64;
        // D s_20_3: cmp-eq s_20_1 s_20_2
        let s_20_3: bool = ((s_20_1) == (s_20_2));
        // D s_20_4: write-var gs#171646 <= s_20_3
        fn_state.gs_171646 = s_20_3;
        // N s_20_5: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#171646:u8
        let s_21_0: bool = fn_state.gs_171646;
        // D s_21_1: write-var gs#171647 <= s_21_0
        fn_state.gs_171647 = s_21_0;
        // N s_21_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#171646 <= s_22_0
        fn_state.gs_171646 = s_22_0;
        // N s_22_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#171644 <= s_23_0
        fn_state.gs_171644 = s_23_0;
        // N s_23_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#171642 <= s_24_0
        fn_state.gs_171642 = s_24_0;
        // N s_24_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#171640 <= s_25_0
        fn_state.gs_171640 = s_25_0;
        // N s_25_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#171637 <= s_26_0
        fn_state.gs_171637 = s_26_0;
        // N s_26_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#171635 <= s_27_0
        fn_state.gs_171635 = s_27_0;
        // N s_27_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#171633 <= s_28_0
        fn_state.gs_171633 = s_28_0;
        // N s_28_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#171631 <= s_29_0
        fn_state.gs_171631 = s_29_0;
        // N s_29_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: panic
        panic!("{:?}", ());
        // N s_30_1: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: panic
        panic!("{:?}", ());
        // N s_31_1: return
        return;
    }
}

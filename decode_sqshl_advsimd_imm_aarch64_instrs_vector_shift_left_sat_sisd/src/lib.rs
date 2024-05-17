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
use common::*;
pub fn decode_sqshl_advsimd_imm_aarch64_instrs_vector_shift_left_sat_sisd<T: Tracer>(
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
        gs_172098: bool,
        gs_172094: bool,
        ga_268449: u8,
        gs_172103: bool,
        esize: i128,
        gs_172096: bool,
        gs_172110: bool,
        shift: i128,
        gs_172107: bool,
        n: i64,
        d: i64,
        src_unsigned: bool,
        src_unsignedshadow_1958: bool,
        dst_unsignedshadow_1957: bool,
        gs_172105: bool,
        gs_172109: bool,
        datasize: i128,
        dst_unsigned: bool,
        gs_172101: bool,
        gs_172092: bool,
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
        // N s_0_15: branch s_0_14 b39 b1
        if s_0_14 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var immh:u8
        let s_1_0: u8 = fn_state.immh;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 4u16);
        // D s_1_2: call HighestSetBit(s_1_1)
        let s_1_2: i128 = HighestSetBit(state, tracer, s_1_1);
        // C s_1_3: const #8s : i
        let s_1_3: i128 = 8;
        // D s_1_4: call _shl_int_general(s_1_3, s_1_2)
        let s_1_4: i128 = u_shl_int_general(state, tracer, s_1_3, s_1_2);
        // D s_1_5: write-var esize <= s_1_4
        fn_state.esize = s_1_4;
        // D s_1_6: read-var esize:i
        let s_1_6: i128 = fn_state.esize;
        // D s_1_7: write-var datasize <= s_1_6
        fn_state.datasize = s_1_6;
        // D s_1_8: read-var immh:u8
        let s_1_8: u8 = fn_state.immh;
        // D s_1_9: cast zx s_1_8 -> bv
        let s_1_9: Bits = Bits::new(s_1_8 as u128, 4u16);
        // D s_1_10: read-var immb:u8
        let s_1_10: u8 = fn_state.immb;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 3u16);
        // D s_1_12: cast reint s_1_9 -> u128
        let s_1_12: u128 = (s_1_9.value() as u128);
        // D s_1_13: size-of s_1_9
        let s_1_13: u16 = s_1_9.length();
        // D s_1_14: cast reint s_1_11 -> u128
        let s_1_14: u128 = (s_1_11.value() as u128);
        // D s_1_15: size-of s_1_11
        let s_1_15: u16 = s_1_11.length();
        // D s_1_16: lsl s_1_12 s_1_15
        let s_1_16: u128 = s_1_12 << s_1_15;
        // D s_1_17: or s_1_16 s_1_14
        let s_1_17: u128 = ((s_1_16) | (s_1_14));
        // D s_1_18: add s_1_13 s_1_15
        let s_1_18: u16 = (s_1_13 + s_1_15);
        // D s_1_19: create-bits s_1_17 s_1_18
        let s_1_19: Bits = Bits::new(s_1_17, s_1_18);
        // D s_1_20: cast reint s_1_19 -> u8
        let s_1_20: u8 = (s_1_19.value() as u8);
        // D s_1_21: cast zx s_1_20 -> bv
        let s_1_21: Bits = Bits::new(s_1_20 as u128, 7u16);
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (s_1_21.value() as i128);
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: read-var esize:i
        let s_1_25: i128 = fn_state.esize;
        // D s_1_26: sub s_1_24 s_1_25
        let s_1_26: i128 = ((s_1_24) - (s_1_25));
        // D s_1_27: write-var shift <= s_1_26
        fn_state.shift = s_1_26;
        // D s_1_28: read-var op:u8
        let s_1_28: bool = fn_state.op;
        // D s_1_29: cast zx s_1_28 -> bv
        let s_1_29: Bits = Bits::new(s_1_28 as u128, 1u16);
        // D s_1_30: read-var U:u8
        let s_1_30: bool = fn_state.U;
        // D s_1_31: cast zx s_1_30 -> bv
        let s_1_31: Bits = Bits::new(s_1_30 as u128, 1u16);
        // D s_1_32: cast reint s_1_29 -> u128
        let s_1_32: u128 = (s_1_29.value() as u128);
        // D s_1_33: size-of s_1_29
        let s_1_33: u16 = s_1_29.length();
        // D s_1_34: cast reint s_1_31 -> u128
        let s_1_34: u128 = (s_1_31.value() as u128);
        // D s_1_35: size-of s_1_31
        let s_1_35: u16 = s_1_31.length();
        // D s_1_36: lsl s_1_32 s_1_35
        let s_1_36: u128 = s_1_32 << s_1_35;
        // D s_1_37: or s_1_36 s_1_34
        let s_1_37: u128 = ((s_1_36) | (s_1_34));
        // D s_1_38: add s_1_33 s_1_35
        let s_1_38: u16 = (s_1_33 + s_1_35);
        // D s_1_39: create-bits s_1_37 s_1_38
        let s_1_39: Bits = Bits::new(s_1_37, s_1_38);
        // D s_1_40: cast reint s_1_39 -> u8
        let s_1_40: u8 = (s_1_39.value() as u8);
        // D s_1_41: write-var ga#268449 <= s_1_40
        fn_state.ga_268449 = s_1_40;
        // D s_1_42: read-var ga#268449:u8
        let s_1_42: u8 = fn_state.ga_268449;
        // D s_1_43: cast zx s_1_42 -> bv
        let s_1_43: Bits = Bits::new(s_1_42 as u128, 2u16);
        // C s_1_44: const #0u : u8
        let s_1_44: u8 = 0;
        // C s_1_45: cast zx s_1_44 -> bv
        let s_1_45: Bits = Bits::new(s_1_44 as u128, 2u16);
        // D s_1_46: cmp-eq s_1_43 s_1_45
        let s_1_46: bool = ((s_1_43) == (s_1_45));
        // D s_1_47: not s_1_46
        let s_1_47: bool = !s_1_46;
        // N s_1_48: branch s_1_47 b3 b2
        if s_1_47 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
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
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#268449:u8
        let s_3_0: u8 = fn_state.ga_268449;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #1u : u8
        let s_3_2: u8 = 1;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: not s_3_4
        let s_3_5: bool = !s_3_4;
        // N s_3_6: branch s_3_5 b36 b4
        if s_3_5 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var src_unsigned <= s_4_0
        fn_state.src_unsigned = s_4_0;
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // D s_4_3: write-var dst_unsigned <= s_4_2
        fn_state.dst_unsigned = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var dst_unsigned:u8
        let s_5_0: bool = fn_state.dst_unsigned;
        // D s_5_1: write-var dst_unsignedshadow#1957 <= s_5_0
        fn_state.dst_unsignedshadow_1957 = s_5_0;
        // D s_5_2: read-var src_unsigned:u8
        let s_5_2: bool = fn_state.src_unsigned;
        // D s_5_3: write-var src_unsignedshadow#1958 <= s_5_2
        fn_state.src_unsignedshadow_1958 = s_5_2;
        // D s_5_4: read-var esize:i
        let s_5_4: i128 = fn_state.esize;
        // D s_5_5: call __id(s_5_4)
        let s_5_5: i128 = u__id(state, tracer, s_5_4);
        // C s_5_6: const #4s : i
        let s_5_6: i128 = 4;
        // D s_5_7: cmp-eq s_5_5 s_5_6
        let s_5_7: bool = ((s_5_5) == (s_5_6));
        // N s_5_8: branch s_5_7 b35 b6
        if s_5_7 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esize:i
        let s_6_0: i128 = fn_state.esize;
        // D s_6_1: call __id(s_6_0)
        let s_6_1: i128 = u__id(state, tracer, s_6_0);
        // C s_6_2: const #8s : i
        let s_6_2: i128 = 8;
        // D s_6_3: cmp-eq s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) == (s_6_2));
        // D s_6_4: write-var gs#172092 <= s_6_3
        fn_state.gs_172092 = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#172092:u8
        let s_7_0: bool = fn_state.gs_172092;
        // N s_7_1: branch s_7_0 b34 b8
        if s_7_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esize:i
        let s_8_0: i128 = fn_state.esize;
        // D s_8_1: call __id(s_8_0)
        let s_8_1: i128 = u__id(state, tracer, s_8_0);
        // C s_8_2: const #16s : i
        let s_8_2: i128 = 16;
        // D s_8_3: cmp-eq s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) == (s_8_2));
        // D s_8_4: write-var gs#172094 <= s_8_3
        fn_state.gs_172094 = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#172094:u8
        let s_9_0: bool = fn_state.gs_172094;
        // N s_9_1: branch s_9_0 b33 b10
        if s_9_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esize:i
        let s_10_0: i128 = fn_state.esize;
        // D s_10_1: call __id(s_10_0)
        let s_10_1: i128 = u__id(state, tracer, s_10_0);
        // C s_10_2: const #32s : i
        let s_10_2: i128 = 32;
        // D s_10_3: cmp-eq s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) == (s_10_2));
        // D s_10_4: write-var gs#172096 <= s_10_3
        fn_state.gs_172096 = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#172096:u8
        let s_11_0: bool = fn_state.gs_172096;
        // N s_11_1: branch s_11_0 b32 b12
        if s_11_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var esize:i
        let s_12_0: i128 = fn_state.esize;
        // D s_12_1: call __id(s_12_0)
        let s_12_1: i128 = u__id(state, tracer, s_12_0);
        // C s_12_2: const #64s : i
        let s_12_2: i128 = 64;
        // D s_12_3: cmp-eq s_12_1 s_12_2
        let s_12_3: bool = ((s_12_1) == (s_12_2));
        // D s_12_4: write-var gs#172098 <= s_12_3
        fn_state.gs_172098 = s_12_3;
        // N s_12_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#172098:u8
        let s_13_0: bool = fn_state.gs_172098;
        // N s_13_1: branch s_13_0 b16 b14
        if s_13_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#172110 <= s_14_0
        fn_state.gs_172110 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#172110:u8
        let s_15_0: bool = fn_state.gs_172110;
        // N s_15_1: assert s_15_0
        let s_15_1: () = assert!(s_15_0);
        // D s_15_2: read-var datasize:i
        let s_15_2: i128 = fn_state.datasize;
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // D s_15_4: read-var esize:i
        let s_15_4: i128 = fn_state.esize;
        // D s_15_5: cast reint s_15_4 -> i64
        let s_15_5: i64 = (s_15_4 as i64);
        // C s_15_6: const #1s : i
        let s_15_6: i128 = 1;
        // D s_15_7: read-var d:i64
        let s_15_7: i64 = fn_state.d;
        // D s_15_8: read-var dst_unsignedshadow#1957:u8
        let s_15_8: bool = fn_state.dst_unsignedshadow_1957;
        // D s_15_9: read-var n:i64
        let s_15_9: i64 = fn_state.n;
        // D s_15_10: read-var shift:i
        let s_15_10: i128 = fn_state.shift;
        // D s_15_11: read-var src_unsignedshadow#1958:u8
        let s_15_11: bool = fn_state.src_unsignedshadow_1958;
        // D s_15_12: call execute_aarch64_instrs_vector_shift_left_sat_sisd(s_15_7, s_15_3, s_15_8, s_15_6, s_15_5, s_15_9, s_15_10, s_15_11)
        let s_15_12: () = execute_aarch64_instrs_vector_shift_left_sat_sisd(
            state,
            tracer,
            s_15_7,
            s_15_3,
            s_15_8,
            s_15_6,
            s_15_5,
            s_15_9,
            s_15_10,
            s_15_11,
        );
        // N s_15_13: return
        return;
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
        // C s_16_2: const #4s : i
        let s_16_2: i128 = 4;
        // D s_16_3: cmp-eq s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) == (s_16_2));
        // N s_16_4: branch s_16_3 b31 b17
        if s_16_3 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var datasize:i
        let s_17_0: i128 = fn_state.datasize;
        // D s_17_1: call __id(s_17_0)
        let s_17_1: i128 = u__id(state, tracer, s_17_0);
        // C s_17_2: const #8s : i
        let s_17_2: i128 = 8;
        // D s_17_3: cmp-eq s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) == (s_17_2));
        // D s_17_4: write-var gs#172101 <= s_17_3
        fn_state.gs_172101 = s_17_3;
        // N s_17_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#172101:u8
        let s_18_0: bool = fn_state.gs_172101;
        // N s_18_1: branch s_18_0 b30 b19
        if s_18_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var datasize:i
        let s_19_0: i128 = fn_state.datasize;
        // D s_19_1: call __id(s_19_0)
        let s_19_1: i128 = u__id(state, tracer, s_19_0);
        // C s_19_2: const #16s : i
        let s_19_2: i128 = 16;
        // D s_19_3: cmp-eq s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) == (s_19_2));
        // D s_19_4: write-var gs#172103 <= s_19_3
        fn_state.gs_172103 = s_19_3;
        // N s_19_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#172103:u8
        let s_20_0: bool = fn_state.gs_172103;
        // N s_20_1: branch s_20_0 b29 b21
        if s_20_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var datasize:i
        let s_21_0: i128 = fn_state.datasize;
        // D s_21_1: call __id(s_21_0)
        let s_21_1: i128 = u__id(state, tracer, s_21_0);
        // C s_21_2: const #32s : i
        let s_21_2: i128 = 32;
        // D s_21_3: cmp-eq s_21_1 s_21_2
        let s_21_3: bool = ((s_21_1) == (s_21_2));
        // D s_21_4: write-var gs#172105 <= s_21_3
        fn_state.gs_172105 = s_21_3;
        // N s_21_5: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#172105:u8
        let s_22_0: bool = fn_state.gs_172105;
        // N s_22_1: branch s_22_0 b28 b23
        if s_22_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var datasize:i
        let s_23_0: i128 = fn_state.datasize;
        // D s_23_1: call __id(s_23_0)
        let s_23_1: i128 = u__id(state, tracer, s_23_0);
        // C s_23_2: const #64s : i
        let s_23_2: i128 = 64;
        // D s_23_3: cmp-eq s_23_1 s_23_2
        let s_23_3: bool = ((s_23_1) == (s_23_2));
        // D s_23_4: write-var gs#172107 <= s_23_3
        fn_state.gs_172107 = s_23_3;
        // N s_23_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#172107:u8
        let s_24_0: bool = fn_state.gs_172107;
        // N s_24_1: branch s_24_0 b27 b25
        if s_24_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var datasize:i
        let s_25_0: i128 = fn_state.datasize;
        // D s_25_1: call __id(s_25_0)
        let s_25_1: i128 = u__id(state, tracer, s_25_0);
        // C s_25_2: const #128s : i
        let s_25_2: i128 = 128;
        // D s_25_3: cmp-eq s_25_1 s_25_2
        let s_25_3: bool = ((s_25_1) == (s_25_2));
        // D s_25_4: write-var gs#172109 <= s_25_3
        fn_state.gs_172109 = s_25_3;
        // N s_25_5: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#172109:u8
        let s_26_0: bool = fn_state.gs_172109;
        // D s_26_1: write-var gs#172110 <= s_26_0
        fn_state.gs_172110 = s_26_0;
        // N s_26_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#172109 <= s_27_0
        fn_state.gs_172109 = s_27_0;
        // N s_27_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#172107 <= s_28_0
        fn_state.gs_172107 = s_28_0;
        // N s_28_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#172105 <= s_29_0
        fn_state.gs_172105 = s_29_0;
        // N s_29_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#172103 <= s_30_0
        fn_state.gs_172103 = s_30_0;
        // N s_30_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#172101 <= s_31_0
        fn_state.gs_172101 = s_31_0;
        // N s_31_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#172098 <= s_32_0
        fn_state.gs_172098 = s_32_0;
        // N s_32_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#172096 <= s_33_0
        fn_state.gs_172096 = s_33_0;
        // N s_33_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#172094 <= s_34_0
        fn_state.gs_172094 = s_34_0;
        // N s_34_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#172092 <= s_35_0
        fn_state.gs_172092 = s_35_0;
        // N s_35_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var ga#268449:u8
        let s_36_0: u8 = fn_state.ga_268449;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 2u16);
        // C s_36_2: const #2u : u8
        let s_36_2: u8 = 2;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 2u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: not s_36_4
        let s_36_5: bool = !s_36_4;
        // N s_36_6: branch s_36_5 b38 b37
        if s_36_5 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var src_unsigned <= s_37_0
        fn_state.src_unsigned = s_37_0;
        // C s_37_2: const #0u : u8
        let s_37_2: bool = false;
        // D s_37_3: write-var dst_unsigned <= s_37_2
        fn_state.dst_unsigned = s_37_2;
        // N s_37_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var src_unsigned <= s_38_0
        fn_state.src_unsigned = s_38_0;
        // C s_38_2: const #1u : u8
        let s_38_2: bool = true;
        // D s_38_3: write-var dst_unsigned <= s_38_2
        fn_state.dst_unsigned = s_38_2;
        // N s_38_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: panic
        panic!("{:?}", ());
        // N s_39_1: return
        return;
    }
}

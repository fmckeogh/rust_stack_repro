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
use unsigned_subrange::*;
use execute_aarch64_instrs_vector_transfer_vector_cpy_dup_sisd::*;
use u__id::*;
use LowestSetBit::*;
use u_shl_int_general::*;
use fdiv_int::*;
use common::*;
pub fn decode_dup_advsimd_elt_aarch64_instrs_vector_transfer_vector_cpy_dup_simd<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, imm5: u8, Q: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_148009: bool,
        gs_148011: bool,
        gs_147997: bool,
        esize: i128,
        gs_148007: bool,
        ga_252486: i64,
        n: i64,
        index: i64,
        gs_148015: bool,
        d: i64,
        size: i128,
        gs_148013: bool,
        elements: i128,
        datasize: i64,
        idxdsize: i64,
        ga_252480: i64,
        Rd: u8,
        Rn: u8,
        imm5: u8,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        imm5,
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
        // D s_0_10: read-var imm5:u8
        let s_0_10: u8 = fn_state.imm5;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 5u16);
        // D s_0_12: call LowestSetBit(s_0_11)
        let s_0_12: i128 = LowestSetBit(state, tracer, s_0_11);
        // D s_0_13: write-var size <= s_0_12
        fn_state.size = s_0_12;
        // C s_0_14: const #3s : i
        let s_0_14: i128 = 3;
        // D s_0_15: read-var size:i
        let s_0_15: i128 = fn_state.size;
        // D s_0_16: cmp-gt s_0_15 s_0_14
        let s_0_16: bool = ((s_0_15) > (s_0_14));
        // N s_0_17: branch s_0_16 b28 b1
        if s_0_16 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var size:i
        let s_1_0: i128 = fn_state.size;
        // D s_1_1: call __id(s_1_0)
        let s_1_1: i128 = u__id(state, tracer, s_1_0);
        // C s_1_2: const #1s : i
        let s_1_2: i128 = 1;
        // D s_1_3: add s_1_1 s_1_2
        let s_1_3: i128 = (s_1_1 + s_1_2);
        // C s_1_4: const #0s : i
        let s_1_4: i128 = 0;
        // D s_1_5: cmp-le s_1_4 s_1_3
        let s_1_5: bool = ((s_1_4) <= (s_1_3));
        // N s_1_6: assert s_1_5
        let s_1_6: () = assert!(s_1_5);
        // C s_1_7: const #1s : i
        let s_1_7: i128 = 1;
        // D s_1_8: read-var size:i
        let s_1_8: i128 = fn_state.size;
        // D s_1_9: add s_1_8 s_1_7
        let s_1_9: i128 = (s_1_8 + s_1_7);
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // C s_1_11: const #4s : i
        let s_1_11: i128 = 4;
        // D s_1_12: read-var imm5:u8
        let s_1_12: u8 = fn_state.imm5;
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 5u16);
        // D s_1_14: cast zx s_1_10 -> i
        let s_1_14: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_15: call unsigned_subrange(s_1_13, s_1_11, s_1_14)
        let s_1_15: i128 = unsigned_subrange(state, tracer, s_1_13, s_1_11, s_1_14);
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: write-var index <= s_1_16
        fn_state.index = s_1_16;
        // C s_1_18: const #4s : i
        let s_1_18: i128 = 4;
        // D s_1_19: read-var imm5:u8
        let s_1_19: u8 = fn_state.imm5;
        // D s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 5u16);
        // C s_1_21: const #1u : u64
        let s_1_21: u64 = 1;
        // D s_1_22: bit-extract s_1_20 s_1_18 s_1_21
        let s_1_22: Bits = (Bits::new(
            ((s_1_20) >> (s_1_18)).value(),
            u16::try_from(s_1_21).unwrap(),
        ));
        // D s_1_23: cast reint s_1_22 -> u8
        let s_1_23: bool = ((s_1_22.value()) != 0);
        // C s_1_24: const #0s : i
        let s_1_24: i128 = 0;
        // C s_1_25: const #0u : u64
        let s_1_25: u64 = 0;
        // D s_1_26: cast zx s_1_23 -> u64
        let s_1_26: u64 = (s_1_23 as u64);
        // C s_1_27: const #1u : u64
        let s_1_27: u64 = 1;
        // D s_1_28: and s_1_26 s_1_27
        let s_1_28: u64 = ((s_1_26) & (s_1_27));
        // D s_1_29: cmp-eq s_1_28 s_1_27
        let s_1_29: bool = ((s_1_28) == (s_1_27));
        // D s_1_30: lsl s_1_26 s_1_24
        let s_1_30: u64 = s_1_26 << s_1_24;
        // D s_1_31: or s_1_25 s_1_30
        let s_1_31: u64 = ((s_1_25) | (s_1_30));
        // D s_1_32: cmpl s_1_30
        let s_1_32: u64 = !s_1_30;
        // D s_1_33: and s_1_25 s_1_32
        let s_1_33: u64 = ((s_1_25) & (s_1_32));
        // D s_1_34: select s_1_29 s_1_31 s_1_33
        let s_1_34: u64 = if s_1_29 { s_1_31 } else { s_1_33 };
        // D s_1_35: cast trunc s_1_34 -> u8
        let s_1_35: bool = ((s_1_34) != 0);
        // D s_1_36: cast zx s_1_35 -> bv
        let s_1_36: Bits = Bits::new(s_1_35 as u128, 1u16);
        // C s_1_37: const #1u : u8
        let s_1_37: bool = true;
        // C s_1_38: cast zx s_1_37 -> bv
        let s_1_38: Bits = Bits::new(s_1_37 as u128, 1u16);
        // D s_1_39: cmp-eq s_1_36 s_1_38
        let s_1_39: bool = ((s_1_36) == (s_1_38));
        // N s_1_40: branch s_1_39 b27 b2
        if s_1_39 {
            return block_27(state, tracer, fn_state);
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
        // D s_2_1: write-var ga#252480 <= s_2_0
        fn_state.ga_252480 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#252480:i64
        let s_3_0: i64 = fn_state.ga_252480;
        // D s_3_1: write-var idxdsize <= s_3_0
        fn_state.idxdsize = s_3_0;
        // C s_3_2: const #3s : i
        let s_3_2: i128 = 3;
        // D s_3_3: read-var size:i
        let s_3_3: i128 = fn_state.size;
        // D s_3_4: cmp-eq s_3_3 s_3_2
        let s_3_4: bool = ((s_3_3) == (s_3_2));
        // N s_3_5: branch s_3_4 b26 b4
        if s_3_4 {
            return block_26(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#147997 <= s_4_0
        fn_state.gs_147997 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#147997:u8
        let s_5_0: bool = fn_state.gs_147997;
        // N s_5_1: branch s_5_0 b25 b6
        if s_5_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #8s : i
        let s_6_0: i128 = 8;
        // D s_6_1: read-var size:i
        let s_6_1: i128 = fn_state.size;
        // D s_6_2: call _shl_int_general(s_6_0, s_6_1)
        let s_6_2: i128 = u_shl_int_general(state, tracer, s_6_0, s_6_1);
        // D s_6_3: write-var esize <= s_6_2
        fn_state.esize = s_6_2;
        // D s_6_4: read-var Q:u8
        let s_6_4: bool = fn_state.Q;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // C s_6_6: const #1u : u8
        let s_6_6: bool = true;
        // C s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 1u16);
        // D s_6_8: cmp-eq s_6_5 s_6_7
        let s_6_8: bool = ((s_6_5) == (s_6_7));
        // N s_6_9: branch s_6_8 b24 b7
        if s_6_8 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // D s_7_1: write-var ga#252486 <= s_7_0
        fn_state.ga_252486 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#252486:i64
        let s_8_0: i64 = fn_state.ga_252486;
        // D s_8_1: write-var datasize <= s_8_0
        fn_state.datasize = s_8_0;
        // D s_8_2: read-var datasize:i64
        let s_8_2: i64 = fn_state.datasize;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: read-var esize:i
        let s_8_4: i128 = fn_state.esize;
        // D s_8_5: call fdiv_int(s_8_3, s_8_4)
        let s_8_5: i128 = fdiv_int(state, tracer, s_8_3, s_8_4);
        // D s_8_6: write-var elements <= s_8_5
        fn_state.elements = s_8_5;
        // D s_8_7: read-var esize:i
        let s_8_7: i128 = fn_state.esize;
        // D s_8_8: call __id(s_8_7)
        let s_8_8: i128 = u__id(state, tracer, s_8_7);
        // C s_8_9: const #8s : i
        let s_8_9: i128 = 8;
        // D s_8_10: cmp-eq s_8_8 s_8_9
        let s_8_10: bool = ((s_8_8) == (s_8_9));
        // N s_8_11: branch s_8_10 b23 b9
        if s_8_10 {
            return block_23(state, tracer, fn_state);
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
        // C s_9_2: const #16s : i
        let s_9_2: i128 = 16;
        // D s_9_3: cmp-eq s_9_1 s_9_2
        let s_9_3: bool = ((s_9_1) == (s_9_2));
        // D s_9_4: write-var gs#148007 <= s_9_3
        fn_state.gs_148007 = s_9_3;
        // N s_9_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#148007:u8
        let s_10_0: bool = fn_state.gs_148007;
        // N s_10_1: branch s_10_0 b22 b11
        if s_10_0 {
            return block_22(state, tracer, fn_state);
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
        // C s_11_2: const #32s : i
        let s_11_2: i128 = 32;
        // D s_11_3: cmp-eq s_11_1 s_11_2
        let s_11_3: bool = ((s_11_1) == (s_11_2));
        // D s_11_4: write-var gs#148009 <= s_11_3
        fn_state.gs_148009 = s_11_3;
        // N s_11_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#148009:u8
        let s_12_0: bool = fn_state.gs_148009;
        // N s_12_1: branch s_12_0 b21 b13
        if s_12_0 {
            return block_21(state, tracer, fn_state);
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
        // C s_13_2: const #64s : i
        let s_13_2: i128 = 64;
        // D s_13_3: cmp-eq s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) == (s_13_2));
        // D s_13_4: write-var gs#148011 <= s_13_3
        fn_state.gs_148011 = s_13_3;
        // N s_13_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#148011:u8
        let s_14_0: bool = fn_state.gs_148011;
        // N s_14_1: branch s_14_0 b20 b15
        if s_14_0 {
            return block_20(state, tracer, fn_state);
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
        // C s_15_2: const #128s : i
        let s_15_2: i128 = 128;
        // D s_15_3: cmp-eq s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) == (s_15_2));
        // D s_15_4: write-var gs#148013 <= s_15_3
        fn_state.gs_148013 = s_15_3;
        // N s_15_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#148013:u8
        let s_16_0: bool = fn_state.gs_148013;
        // N s_16_1: branch s_16_0 b19 b17
        if s_16_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var esize:i
        let s_17_0: i128 = fn_state.esize;
        // D s_17_1: call __id(s_17_0)
        let s_17_1: i128 = u__id(state, tracer, s_17_0);
        // C s_17_2: const #256s : i
        let s_17_2: i128 = 256;
        // D s_17_3: cmp-eq s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) == (s_17_2));
        // D s_17_4: write-var gs#148015 <= s_17_3
        fn_state.gs_148015 = s_17_3;
        // N s_17_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#148015:u8
        let s_18_0: bool = fn_state.gs_148015;
        // N s_18_1: assert s_18_0
        let s_18_1: () = assert!(s_18_0);
        // D s_18_2: read-var datasize:i64
        let s_18_2: i64 = fn_state.datasize;
        // D s_18_3: cast zx s_18_2 -> i
        let s_18_3: i128 = (i128::try_from(s_18_2).unwrap());
        // D s_18_4: cast reint s_18_3 -> i64
        let s_18_4: i64 = (s_18_3 as i64);
        // D s_18_5: read-var esize:i
        let s_18_5: i128 = fn_state.esize;
        // D s_18_6: cast reint s_18_5 -> i64
        let s_18_6: i64 = (s_18_5 as i64);
        // D s_18_7: read-var idxdsize:i64
        let s_18_7: i64 = fn_state.idxdsize;
        // D s_18_8: cast zx s_18_7 -> i
        let s_18_8: i128 = (i128::try_from(s_18_7).unwrap());
        // D s_18_9: cast reint s_18_8 -> i64
        let s_18_9: i64 = (s_18_8 as i64);
        // D s_18_10: read-var index:i64
        let s_18_10: i64 = fn_state.index;
        // D s_18_11: cast zx s_18_10 -> i
        let s_18_11: i128 = (i128::try_from(s_18_10).unwrap());
        // D s_18_12: read-var d:i64
        let s_18_12: i64 = fn_state.d;
        // D s_18_13: read-var elements:i
        let s_18_13: i128 = fn_state.elements;
        // D s_18_14: read-var n:i64
        let s_18_14: i64 = fn_state.n;
        // D s_18_15: call execute_aarch64_instrs_vector_transfer_vector_cpy_dup_sisd(s_18_12, s_18_4, s_18_13, s_18_6, s_18_9, s_18_11, s_18_14)
        let s_18_15: () = execute_aarch64_instrs_vector_transfer_vector_cpy_dup_sisd(
            state,
            tracer,
            s_18_12,
            s_18_4,
            s_18_13,
            s_18_6,
            s_18_9,
            s_18_11,
            s_18_14,
        );
        // N s_18_16: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#148015 <= s_19_0
        fn_state.gs_148015 = s_19_0;
        // N s_19_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#148013 <= s_20_0
        fn_state.gs_148013 = s_20_0;
        // N s_20_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#148011 <= s_21_0
        fn_state.gs_148011 = s_21_0;
        // N s_21_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#148009 <= s_22_0
        fn_state.gs_148009 = s_22_0;
        // N s_22_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#148007 <= s_23_0
        fn_state.gs_148007 = s_23_0;
        // N s_23_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #128s : i64
        let s_24_0: i64 = 128;
        // D s_24_1: write-var ga#252486 <= s_24_0
        fn_state.ga_252486 = s_24_0;
        // N s_24_2: jump b8
        return block_8(state, tracer, fn_state);
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
        // D s_26_0: read-var Q:u8
        let s_26_0: bool = fn_state.Q;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #0u : u8
        let s_26_2: bool = false;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#147997 <= s_26_4
        fn_state.gs_147997 = s_26_4;
        // N s_26_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #128s : i64
        let s_27_0: i64 = 128;
        // D s_27_1: write-var ga#252480 <= s_27_0
        fn_state.ga_252480 = s_27_0;
        // N s_27_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
    }
}

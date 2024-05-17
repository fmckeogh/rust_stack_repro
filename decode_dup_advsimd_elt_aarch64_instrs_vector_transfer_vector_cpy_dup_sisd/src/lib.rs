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
use common::*;
pub fn decode_dup_advsimd_elt_aarch64_instrs_vector_transfer_vector_cpy_dup_sisd<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, imm5: u8) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_147956: bool,
        ga_252438: i64,
        gs_147967: bool,
        esize: i128,
        gs_147950: bool,
        n: i64,
        index: i64,
        d: i64,
        size: i128,
        gs_147952: bool,
        gs_147963: bool,
        gs_147969: bool,
        gs_147958: bool,
        gs_147961: bool,
        datasize: i128,
        gs_147954: bool,
        gs_147965: bool,
        idxdsize: i64,
        gs_147970: bool,
        Rd: u8,
        Rn: u8,
        imm5: u8,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        imm5,
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
        // N s_0_17: branch s_0_16 b38 b1
        if s_0_16 {
            return block_38(state, tracer, fn_state);
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
        // N s_1_40: branch s_1_39 b37 b2
        if s_1_39 {
            return block_37(state, tracer, fn_state);
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
        // D s_2_1: write-var ga#252438 <= s_2_0
        fn_state.ga_252438 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#252438:i64
        let s_3_0: i64 = fn_state.ga_252438;
        // D s_3_1: write-var idxdsize <= s_3_0
        fn_state.idxdsize = s_3_0;
        // C s_3_2: const #8s : i
        let s_3_2: i128 = 8;
        // D s_3_3: read-var size:i
        let s_3_3: i128 = fn_state.size;
        // D s_3_4: call _shl_int_general(s_3_2, s_3_3)
        let s_3_4: i128 = u_shl_int_general(state, tracer, s_3_2, s_3_3);
        // D s_3_5: write-var esize <= s_3_4
        fn_state.esize = s_3_4;
        // D s_3_6: read-var esize:i
        let s_3_6: i128 = fn_state.esize;
        // D s_3_7: write-var datasize <= s_3_6
        fn_state.datasize = s_3_6;
        // D s_3_8: read-var esize:i
        let s_3_8: i128 = fn_state.esize;
        // D s_3_9: call __id(s_3_8)
        let s_3_9: i128 = u__id(state, tracer, s_3_8);
        // C s_3_10: const #8s : i
        let s_3_10: i128 = 8;
        // D s_3_11: cmp-eq s_3_9 s_3_10
        let s_3_11: bool = ((s_3_9) == (s_3_10));
        // N s_3_12: branch s_3_11 b36 b4
        if s_3_11 {
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
        // D s_4_0: read-var esize:i
        let s_4_0: i128 = fn_state.esize;
        // D s_4_1: call __id(s_4_0)
        let s_4_1: i128 = u__id(state, tracer, s_4_0);
        // C s_4_2: const #16s : i
        let s_4_2: i128 = 16;
        // D s_4_3: cmp-eq s_4_1 s_4_2
        let s_4_3: bool = ((s_4_1) == (s_4_2));
        // D s_4_4: write-var gs#147950 <= s_4_3
        fn_state.gs_147950 = s_4_3;
        // N s_4_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#147950:u8
        let s_5_0: bool = fn_state.gs_147950;
        // N s_5_1: branch s_5_0 b35 b6
        if s_5_0 {
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
        // C s_6_2: const #32s : i
        let s_6_2: i128 = 32;
        // D s_6_3: cmp-eq s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) == (s_6_2));
        // D s_6_4: write-var gs#147952 <= s_6_3
        fn_state.gs_147952 = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#147952:u8
        let s_7_0: bool = fn_state.gs_147952;
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
        // C s_8_2: const #64s : i
        let s_8_2: i128 = 64;
        // D s_8_3: cmp-eq s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) == (s_8_2));
        // D s_8_4: write-var gs#147954 <= s_8_3
        fn_state.gs_147954 = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#147954:u8
        let s_9_0: bool = fn_state.gs_147954;
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
        // C s_10_2: const #128s : i
        let s_10_2: i128 = 128;
        // D s_10_3: cmp-eq s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) == (s_10_2));
        // D s_10_4: write-var gs#147956 <= s_10_3
        fn_state.gs_147956 = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#147956:u8
        let s_11_0: bool = fn_state.gs_147956;
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
        // C s_12_2: const #256s : i
        let s_12_2: i128 = 256;
        // D s_12_3: cmp-eq s_12_1 s_12_2
        let s_12_3: bool = ((s_12_1) == (s_12_2));
        // D s_12_4: write-var gs#147958 <= s_12_3
        fn_state.gs_147958 = s_12_3;
        // N s_12_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#147958:u8
        let s_13_0: bool = fn_state.gs_147958;
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
        // D s_14_1: write-var gs#147970 <= s_14_0
        fn_state.gs_147970 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#147970:u8
        let s_15_0: bool = fn_state.gs_147970;
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
        // D s_15_6: read-var idxdsize:i64
        let s_15_6: i64 = fn_state.idxdsize;
        // D s_15_7: cast zx s_15_6 -> i
        let s_15_7: i128 = (i128::try_from(s_15_6).unwrap());
        // D s_15_8: cast reint s_15_7 -> i64
        let s_15_8: i64 = (s_15_7 as i64);
        // C s_15_9: const #1s : i
        let s_15_9: i128 = 1;
        // D s_15_10: read-var index:i64
        let s_15_10: i64 = fn_state.index;
        // D s_15_11: cast zx s_15_10 -> i
        let s_15_11: i128 = (i128::try_from(s_15_10).unwrap());
        // D s_15_12: read-var d:i64
        let s_15_12: i64 = fn_state.d;
        // D s_15_13: read-var n:i64
        let s_15_13: i64 = fn_state.n;
        // D s_15_14: call execute_aarch64_instrs_vector_transfer_vector_cpy_dup_sisd(s_15_12, s_15_3, s_15_9, s_15_5, s_15_8, s_15_11, s_15_13)
        let s_15_14: () = execute_aarch64_instrs_vector_transfer_vector_cpy_dup_sisd(
            state,
            tracer,
            s_15_12,
            s_15_3,
            s_15_9,
            s_15_5,
            s_15_8,
            s_15_11,
            s_15_13,
        );
        // N s_15_15: return
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
        // C s_16_2: const #8s : i
        let s_16_2: i128 = 8;
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
        // C s_17_2: const #16s : i
        let s_17_2: i128 = 16;
        // D s_17_3: cmp-eq s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) == (s_17_2));
        // D s_17_4: write-var gs#147961 <= s_17_3
        fn_state.gs_147961 = s_17_3;
        // N s_17_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#147961:u8
        let s_18_0: bool = fn_state.gs_147961;
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
        // C s_19_2: const #32s : i
        let s_19_2: i128 = 32;
        // D s_19_3: cmp-eq s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) == (s_19_2));
        // D s_19_4: write-var gs#147963 <= s_19_3
        fn_state.gs_147963 = s_19_3;
        // N s_19_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#147963:u8
        let s_20_0: bool = fn_state.gs_147963;
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
        // C s_21_2: const #64s : i
        let s_21_2: i128 = 64;
        // D s_21_3: cmp-eq s_21_1 s_21_2
        let s_21_3: bool = ((s_21_1) == (s_21_2));
        // D s_21_4: write-var gs#147965 <= s_21_3
        fn_state.gs_147965 = s_21_3;
        // N s_21_5: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#147965:u8
        let s_22_0: bool = fn_state.gs_147965;
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
        // C s_23_2: const #128s : i
        let s_23_2: i128 = 128;
        // D s_23_3: cmp-eq s_23_1 s_23_2
        let s_23_3: bool = ((s_23_1) == (s_23_2));
        // D s_23_4: write-var gs#147967 <= s_23_3
        fn_state.gs_147967 = s_23_3;
        // N s_23_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#147967:u8
        let s_24_0: bool = fn_state.gs_147967;
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
        // C s_25_2: const #256s : i
        let s_25_2: i128 = 256;
        // D s_25_3: cmp-eq s_25_1 s_25_2
        let s_25_3: bool = ((s_25_1) == (s_25_2));
        // D s_25_4: write-var gs#147969 <= s_25_3
        fn_state.gs_147969 = s_25_3;
        // N s_25_5: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#147969:u8
        let s_26_0: bool = fn_state.gs_147969;
        // D s_26_1: write-var gs#147970 <= s_26_0
        fn_state.gs_147970 = s_26_0;
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
        // D s_27_1: write-var gs#147969 <= s_27_0
        fn_state.gs_147969 = s_27_0;
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
        // D s_28_1: write-var gs#147967 <= s_28_0
        fn_state.gs_147967 = s_28_0;
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
        // D s_29_1: write-var gs#147965 <= s_29_0
        fn_state.gs_147965 = s_29_0;
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
        // D s_30_1: write-var gs#147963 <= s_30_0
        fn_state.gs_147963 = s_30_0;
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
        // D s_31_1: write-var gs#147961 <= s_31_0
        fn_state.gs_147961 = s_31_0;
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
        // D s_32_1: write-var gs#147958 <= s_32_0
        fn_state.gs_147958 = s_32_0;
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
        // D s_33_1: write-var gs#147956 <= s_33_0
        fn_state.gs_147956 = s_33_0;
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
        // D s_34_1: write-var gs#147954 <= s_34_0
        fn_state.gs_147954 = s_34_0;
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
        // D s_35_1: write-var gs#147952 <= s_35_0
        fn_state.gs_147952 = s_35_0;
        // N s_35_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#147950 <= s_36_0
        fn_state.gs_147950 = s_36_0;
        // N s_36_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #128s : i64
        let s_37_0: i64 = 128;
        // D s_37_1: write-var ga#252438 <= s_37_0
        fn_state.ga_252438 = s_37_0;
        // N s_37_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: panic
        panic!("{:?}", ());
        // N s_38_1: return
        return;
    }
}

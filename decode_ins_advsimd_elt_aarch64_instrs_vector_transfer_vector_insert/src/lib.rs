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
use u__id::*;
use LowestSetBit::*;
use execute_aarch64_instrs_vector_transfer_vector_insert::*;
use common::*;
pub fn decode_ins_advsimd_elt_aarch64_instrs_vector_transfer_vector_insert<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    imm4: u8,
    imm5: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        n: i64,
        ga_256843: i64,
        idxdsize: i64,
        d: i64,
        size: i128,
        src_index: i64,
        dst_index: i64,
        Rd: u8,
        Rn: u8,
        imm4: u8,
        imm5: u8,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        imm4,
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
        // N s_0_17: branch s_0_16 b29 b1
        if s_0_16 {
            return block_29(state, tracer, fn_state);
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
        // D s_1_17: write-var dst_index <= s_1_16
        fn_state.dst_index = s_1_16;
        // D s_1_18: read-var size:i
        let s_1_18: i128 = fn_state.size;
        // D s_1_19: call __id(s_1_18)
        let s_1_19: i128 = u__id(state, tracer, s_1_18);
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // C s_1_21: const #0s : i
        let s_1_21: i128 = 0;
        // D s_1_22: cast zx s_1_20 -> i
        let s_1_22: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_23: cmp-le s_1_21 s_1_22
        let s_1_23: bool = ((s_1_21) <= (s_1_22));
        // N s_1_24: assert s_1_23
        let s_1_24: () = assert!(s_1_23);
        // C s_1_25: const #3s : i
        let s_1_25: i128 = 3;
        // D s_1_26: read-var imm4:u8
        let s_1_26: u8 = fn_state.imm4;
        // D s_1_27: cast zx s_1_26 -> bv
        let s_1_27: Bits = Bits::new(s_1_26 as u128, 4u16);
        // D s_1_28: read-var size:i
        let s_1_28: i128 = fn_state.size;
        // D s_1_29: call unsigned_subrange(s_1_27, s_1_25, s_1_28)
        let s_1_29: i128 = unsigned_subrange(state, tracer, s_1_27, s_1_25, s_1_28);
        // D s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // D s_1_31: write-var src_index <= s_1_30
        fn_state.src_index = s_1_30;
        // C s_1_32: const #3s : i
        let s_1_32: i128 = 3;
        // D s_1_33: read-var imm4:u8
        let s_1_33: u8 = fn_state.imm4;
        // D s_1_34: cast zx s_1_33 -> bv
        let s_1_34: Bits = Bits::new(s_1_33 as u128, 4u16);
        // C s_1_35: const #1u : u64
        let s_1_35: u64 = 1;
        // D s_1_36: bit-extract s_1_34 s_1_32 s_1_35
        let s_1_36: Bits = (Bits::new(
            ((s_1_34) >> (s_1_32)).value(),
            u16::try_from(s_1_35).unwrap(),
        ));
        // D s_1_37: cast reint s_1_36 -> u8
        let s_1_37: bool = ((s_1_36.value()) != 0);
        // C s_1_38: const #0s : i
        let s_1_38: i128 = 0;
        // C s_1_39: const #0u : u64
        let s_1_39: u64 = 0;
        // D s_1_40: cast zx s_1_37 -> u64
        let s_1_40: u64 = (s_1_37 as u64);
        // C s_1_41: const #1u : u64
        let s_1_41: u64 = 1;
        // D s_1_42: and s_1_40 s_1_41
        let s_1_42: u64 = ((s_1_40) & (s_1_41));
        // D s_1_43: cmp-eq s_1_42 s_1_41
        let s_1_43: bool = ((s_1_42) == (s_1_41));
        // D s_1_44: lsl s_1_40 s_1_38
        let s_1_44: u64 = s_1_40 << s_1_38;
        // D s_1_45: or s_1_39 s_1_44
        let s_1_45: u64 = ((s_1_39) | (s_1_44));
        // D s_1_46: cmpl s_1_44
        let s_1_46: u64 = !s_1_44;
        // D s_1_47: and s_1_39 s_1_46
        let s_1_47: u64 = ((s_1_39) & (s_1_46));
        // D s_1_48: select s_1_43 s_1_45 s_1_47
        let s_1_48: u64 = if s_1_43 { s_1_45 } else { s_1_47 };
        // D s_1_49: cast trunc s_1_48 -> u8
        let s_1_49: bool = ((s_1_48) != 0);
        // D s_1_50: cast zx s_1_49 -> bv
        let s_1_50: Bits = Bits::new(s_1_49 as u128, 1u16);
        // C s_1_51: const #1u : u8
        let s_1_51: bool = true;
        // C s_1_52: cast zx s_1_51 -> bv
        let s_1_52: Bits = Bits::new(s_1_51 as u128, 1u16);
        // D s_1_53: cmp-eq s_1_50 s_1_52
        let s_1_53: bool = ((s_1_50) == (s_1_52));
        // N s_1_54: branch s_1_53 b28 b2
        if s_1_53 {
            return block_28(state, tracer, fn_state);
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
        // D s_2_1: write-var ga#256843 <= s_2_0
        fn_state.ga_256843 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#256843:i64
        let s_3_0: i64 = fn_state.ga_256843;
        // D s_3_1: write-var idxdsize <= s_3_0
        fn_state.idxdsize = s_3_0;
        // C s_3_2: const #8s : i64
        let s_3_2: i64 = 8;
        // D s_3_3: read-var size:i
        let s_3_3: i128 = fn_state.size;
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: lsl s_3_2 s_3_4
        let s_3_5: i64 = s_3_2 << s_3_4;
        // D s_3_6: write-var esize <= s_3_5
        fn_state.esize = s_3_5;
        // D s_3_7: read-var esize:i64
        let s_3_7: i64 = fn_state.esize;
        // C s_3_8: const #8s : i
        let s_3_8: i128 = 8;
        // D s_3_9: cast zx s_3_7 -> i
        let s_3_9: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_10: cmp-eq s_3_9 s_3_8
        let s_3_10: bool = ((s_3_9) == (s_3_8));
        // D s_3_11: not s_3_10
        let s_3_11: bool = !s_3_10;
        // N s_3_12: branch s_3_11 b9 b4
        if s_3_11 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var idxdsize:i64
        let s_4_0: i64 = fn_state.idxdsize;
        // C s_4_1: const #64s : i
        let s_4_1: i128 = 64;
        // D s_4_2: cast zx s_4_0 -> i
        let s_4_2: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_1
        let s_4_3: bool = ((s_4_2) == (s_4_1));
        // D s_4_4: not s_4_3
        let s_4_4: bool = !s_4_3;
        // N s_4_5: branch s_4_4 b6 b5
        if s_4_4 {
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
        // C s_5_0: const #8s : i64
        let s_5_0: i64 = 8;
        // C s_5_1: const #64s : i64
        let s_5_1: i64 = 64;
        // D s_5_2: read-var dst_index:i64
        let s_5_2: i64 = fn_state.dst_index;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var src_index:i64
        let s_5_4: i64 = fn_state.src_index;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: read-var d:i64
        let s_5_6: i64 = fn_state.d;
        // D s_5_7: read-var n:i64
        let s_5_7: i64 = fn_state.n;
        // D s_5_8: call execute_aarch64_instrs_vector_transfer_vector_insert(s_5_6, s_5_3, s_5_0, s_5_1, s_5_7, s_5_5)
        let s_5_8: () = execute_aarch64_instrs_vector_transfer_vector_insert(
            state,
            tracer,
            s_5_6,
            s_5_3,
            s_5_0,
            s_5_1,
            s_5_7,
            s_5_5,
        );
        // N s_5_9: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var idxdsize:i64
        let s_6_0: i64 = fn_state.idxdsize;
        // C s_6_1: const #128s : i
        let s_6_1: i128 = 128;
        // D s_6_2: cast zx s_6_0 -> i
        let s_6_2: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_1
        let s_6_3: bool = ((s_6_2) == (s_6_1));
        // D s_6_4: not s_6_3
        let s_6_4: bool = !s_6_3;
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #8s : i64
        let s_7_0: i64 = 8;
        // C s_7_1: const #128s : i64
        let s_7_1: i64 = 128;
        // D s_7_2: read-var dst_index:i64
        let s_7_2: i64 = fn_state.dst_index;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var src_index:i64
        let s_7_4: i64 = fn_state.src_index;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var d:i64
        let s_7_6: i64 = fn_state.d;
        // D s_7_7: read-var n:i64
        let s_7_7: i64 = fn_state.n;
        // D s_7_8: call execute_aarch64_instrs_vector_transfer_vector_insert(s_7_6, s_7_3, s_7_0, s_7_1, s_7_7, s_7_5)
        let s_7_8: () = execute_aarch64_instrs_vector_transfer_vector_insert(
            state,
            tracer,
            s_7_6,
            s_7_3,
            s_7_0,
            s_7_1,
            s_7_7,
            s_7_5,
        );
        // N s_7_9: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // N s_8_2: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esize:i64
        let s_9_0: i64 = fn_state.esize;
        // C s_9_1: const #16s : i
        let s_9_1: i128 = 16;
        // D s_9_2: cast zx s_9_0 -> i
        let s_9_2: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_1
        let s_9_3: bool = ((s_9_2) == (s_9_1));
        // D s_9_4: not s_9_3
        let s_9_4: bool = !s_9_3;
        // N s_9_5: branch s_9_4 b15 b10
        if s_9_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var idxdsize:i64
        let s_10_0: i64 = fn_state.idxdsize;
        // C s_10_1: const #64s : i
        let s_10_1: i128 = 64;
        // D s_10_2: cast zx s_10_0 -> i
        let s_10_2: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_3: cmp-eq s_10_2 s_10_1
        let s_10_3: bool = ((s_10_2) == (s_10_1));
        // D s_10_4: not s_10_3
        let s_10_4: bool = !s_10_3;
        // N s_10_5: branch s_10_4 b12 b11
        if s_10_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #16s : i64
        let s_11_0: i64 = 16;
        // C s_11_1: const #64s : i64
        let s_11_1: i64 = 64;
        // D s_11_2: read-var dst_index:i64
        let s_11_2: i64 = fn_state.dst_index;
        // D s_11_3: cast zx s_11_2 -> i
        let s_11_3: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_4: read-var src_index:i64
        let s_11_4: i64 = fn_state.src_index;
        // D s_11_5: cast zx s_11_4 -> i
        let s_11_5: i128 = (i128::try_from(s_11_4).unwrap());
        // D s_11_6: read-var d:i64
        let s_11_6: i64 = fn_state.d;
        // D s_11_7: read-var n:i64
        let s_11_7: i64 = fn_state.n;
        // D s_11_8: call execute_aarch64_instrs_vector_transfer_vector_insert(s_11_6, s_11_3, s_11_0, s_11_1, s_11_7, s_11_5)
        let s_11_8: () = execute_aarch64_instrs_vector_transfer_vector_insert(
            state,
            tracer,
            s_11_6,
            s_11_3,
            s_11_0,
            s_11_1,
            s_11_7,
            s_11_5,
        );
        // N s_11_9: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var idxdsize:i64
        let s_12_0: i64 = fn_state.idxdsize;
        // C s_12_1: const #128s : i
        let s_12_1: i128 = 128;
        // D s_12_2: cast zx s_12_0 -> i
        let s_12_2: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_3: cmp-eq s_12_2 s_12_1
        let s_12_3: bool = ((s_12_2) == (s_12_1));
        // D s_12_4: not s_12_3
        let s_12_4: bool = !s_12_3;
        // N s_12_5: branch s_12_4 b14 b13
        if s_12_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #16s : i64
        let s_13_0: i64 = 16;
        // C s_13_1: const #128s : i64
        let s_13_1: i64 = 128;
        // D s_13_2: read-var dst_index:i64
        let s_13_2: i64 = fn_state.dst_index;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: read-var src_index:i64
        let s_13_4: i64 = fn_state.src_index;
        // D s_13_5: cast zx s_13_4 -> i
        let s_13_5: i128 = (i128::try_from(s_13_4).unwrap());
        // D s_13_6: read-var d:i64
        let s_13_6: i64 = fn_state.d;
        // D s_13_7: read-var n:i64
        let s_13_7: i64 = fn_state.n;
        // D s_13_8: call execute_aarch64_instrs_vector_transfer_vector_insert(s_13_6, s_13_3, s_13_0, s_13_1, s_13_7, s_13_5)
        let s_13_8: () = execute_aarch64_instrs_vector_transfer_vector_insert(
            state,
            tracer,
            s_13_6,
            s_13_3,
            s_13_0,
            s_13_1,
            s_13_7,
            s_13_5,
        );
        // N s_13_9: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // N s_14_1: assert s_14_0
        let s_14_1: () = assert!(s_14_0);
        // N s_14_2: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var esize:i64
        let s_15_0: i64 = fn_state.esize;
        // C s_15_1: const #32s : i
        let s_15_1: i128 = 32;
        // D s_15_2: cast zx s_15_0 -> i
        let s_15_2: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_3: cmp-eq s_15_2 s_15_1
        let s_15_3: bool = ((s_15_2) == (s_15_1));
        // D s_15_4: not s_15_3
        let s_15_4: bool = !s_15_3;
        // N s_15_5: branch s_15_4 b21 b16
        if s_15_4 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var idxdsize:i64
        let s_16_0: i64 = fn_state.idxdsize;
        // C s_16_1: const #64s : i
        let s_16_1: i128 = 64;
        // D s_16_2: cast zx s_16_0 -> i
        let s_16_2: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_3: cmp-eq s_16_2 s_16_1
        let s_16_3: bool = ((s_16_2) == (s_16_1));
        // D s_16_4: not s_16_3
        let s_16_4: bool = !s_16_3;
        // N s_16_5: branch s_16_4 b18 b17
        if s_16_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #32s : i64
        let s_17_0: i64 = 32;
        // C s_17_1: const #64s : i64
        let s_17_1: i64 = 64;
        // D s_17_2: read-var dst_index:i64
        let s_17_2: i64 = fn_state.dst_index;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: read-var src_index:i64
        let s_17_4: i64 = fn_state.src_index;
        // D s_17_5: cast zx s_17_4 -> i
        let s_17_5: i128 = (i128::try_from(s_17_4).unwrap());
        // D s_17_6: read-var d:i64
        let s_17_6: i64 = fn_state.d;
        // D s_17_7: read-var n:i64
        let s_17_7: i64 = fn_state.n;
        // D s_17_8: call execute_aarch64_instrs_vector_transfer_vector_insert(s_17_6, s_17_3, s_17_0, s_17_1, s_17_7, s_17_5)
        let s_17_8: () = execute_aarch64_instrs_vector_transfer_vector_insert(
            state,
            tracer,
            s_17_6,
            s_17_3,
            s_17_0,
            s_17_1,
            s_17_7,
            s_17_5,
        );
        // N s_17_9: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var idxdsize:i64
        let s_18_0: i64 = fn_state.idxdsize;
        // C s_18_1: const #128s : i
        let s_18_1: i128 = 128;
        // D s_18_2: cast zx s_18_0 -> i
        let s_18_2: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_3: cmp-eq s_18_2 s_18_1
        let s_18_3: bool = ((s_18_2) == (s_18_1));
        // D s_18_4: not s_18_3
        let s_18_4: bool = !s_18_3;
        // N s_18_5: branch s_18_4 b20 b19
        if s_18_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #32s : i64
        let s_19_0: i64 = 32;
        // C s_19_1: const #128s : i64
        let s_19_1: i64 = 128;
        // D s_19_2: read-var dst_index:i64
        let s_19_2: i64 = fn_state.dst_index;
        // D s_19_3: cast zx s_19_2 -> i
        let s_19_3: i128 = (i128::try_from(s_19_2).unwrap());
        // D s_19_4: read-var src_index:i64
        let s_19_4: i64 = fn_state.src_index;
        // D s_19_5: cast zx s_19_4 -> i
        let s_19_5: i128 = (i128::try_from(s_19_4).unwrap());
        // D s_19_6: read-var d:i64
        let s_19_6: i64 = fn_state.d;
        // D s_19_7: read-var n:i64
        let s_19_7: i64 = fn_state.n;
        // D s_19_8: call execute_aarch64_instrs_vector_transfer_vector_insert(s_19_6, s_19_3, s_19_0, s_19_1, s_19_7, s_19_5)
        let s_19_8: () = execute_aarch64_instrs_vector_transfer_vector_insert(
            state,
            tracer,
            s_19_6,
            s_19_3,
            s_19_0,
            s_19_1,
            s_19_7,
            s_19_5,
        );
        // N s_19_9: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // N s_20_1: assert s_20_0
        let s_20_1: () = assert!(s_20_0);
        // N s_20_2: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var esize:i64
        let s_21_0: i64 = fn_state.esize;
        // C s_21_1: const #64s : i
        let s_21_1: i128 = 64;
        // D s_21_2: cast zx s_21_0 -> i
        let s_21_2: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_3: cmp-eq s_21_2 s_21_1
        let s_21_3: bool = ((s_21_2) == (s_21_1));
        // D s_21_4: not s_21_3
        let s_21_4: bool = !s_21_3;
        // N s_21_5: branch s_21_4 b27 b22
        if s_21_4 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var idxdsize:i64
        let s_22_0: i64 = fn_state.idxdsize;
        // C s_22_1: const #64s : i
        let s_22_1: i128 = 64;
        // D s_22_2: cast zx s_22_0 -> i
        let s_22_2: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_3: cmp-eq s_22_2 s_22_1
        let s_22_3: bool = ((s_22_2) == (s_22_1));
        // D s_22_4: not s_22_3
        let s_22_4: bool = !s_22_3;
        // N s_22_5: branch s_22_4 b24 b23
        if s_22_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #64s : i64
        let s_23_0: i64 = 64;
        // C s_23_1: const #64s : i64
        let s_23_1: i64 = 64;
        // D s_23_2: read-var dst_index:i64
        let s_23_2: i64 = fn_state.dst_index;
        // D s_23_3: cast zx s_23_2 -> i
        let s_23_3: i128 = (i128::try_from(s_23_2).unwrap());
        // D s_23_4: read-var src_index:i64
        let s_23_4: i64 = fn_state.src_index;
        // D s_23_5: cast zx s_23_4 -> i
        let s_23_5: i128 = (i128::try_from(s_23_4).unwrap());
        // D s_23_6: read-var d:i64
        let s_23_6: i64 = fn_state.d;
        // D s_23_7: read-var n:i64
        let s_23_7: i64 = fn_state.n;
        // D s_23_8: call execute_aarch64_instrs_vector_transfer_vector_insert(s_23_6, s_23_3, s_23_0, s_23_1, s_23_7, s_23_5)
        let s_23_8: () = execute_aarch64_instrs_vector_transfer_vector_insert(
            state,
            tracer,
            s_23_6,
            s_23_3,
            s_23_0,
            s_23_1,
            s_23_7,
            s_23_5,
        );
        // N s_23_9: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var idxdsize:i64
        let s_24_0: i64 = fn_state.idxdsize;
        // C s_24_1: const #128s : i
        let s_24_1: i128 = 128;
        // D s_24_2: cast zx s_24_0 -> i
        let s_24_2: i128 = (i128::try_from(s_24_0).unwrap());
        // D s_24_3: cmp-eq s_24_2 s_24_1
        let s_24_3: bool = ((s_24_2) == (s_24_1));
        // D s_24_4: not s_24_3
        let s_24_4: bool = !s_24_3;
        // N s_24_5: branch s_24_4 b26 b25
        if s_24_4 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #64s : i64
        let s_25_0: i64 = 64;
        // C s_25_1: const #128s : i64
        let s_25_1: i64 = 128;
        // D s_25_2: read-var dst_index:i64
        let s_25_2: i64 = fn_state.dst_index;
        // D s_25_3: cast zx s_25_2 -> i
        let s_25_3: i128 = (i128::try_from(s_25_2).unwrap());
        // D s_25_4: read-var src_index:i64
        let s_25_4: i64 = fn_state.src_index;
        // D s_25_5: cast zx s_25_4 -> i
        let s_25_5: i128 = (i128::try_from(s_25_4).unwrap());
        // D s_25_6: read-var d:i64
        let s_25_6: i64 = fn_state.d;
        // D s_25_7: read-var n:i64
        let s_25_7: i64 = fn_state.n;
        // D s_25_8: call execute_aarch64_instrs_vector_transfer_vector_insert(s_25_6, s_25_3, s_25_0, s_25_1, s_25_7, s_25_5)
        let s_25_8: () = execute_aarch64_instrs_vector_transfer_vector_insert(
            state,
            tracer,
            s_25_6,
            s_25_3,
            s_25_0,
            s_25_1,
            s_25_7,
            s_25_5,
        );
        // N s_25_9: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // N s_26_1: assert s_26_0
        let s_26_1: () = assert!(s_26_0);
        // N s_26_2: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // N s_27_1: assert s_27_0
        let s_27_1: () = assert!(s_27_0);
        // N s_27_2: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #128s : i64
        let s_28_0: i64 = 128;
        // D s_28_1: write-var ga#256843 <= s_28_0
        fn_state.ga_256843 = s_28_0;
        // N s_28_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: panic
        panic!("{:?}", ());
        // N s_29_1: return
        return;
    }
}

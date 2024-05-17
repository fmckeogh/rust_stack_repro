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
use neq_int::*;
use execute_aarch64_instrs_memory_pair_simdfp_post_idx::*;
use u__id::*;
use ConstrainUnpredictable::*;
use EndOfInstruction::*;
use common::*;
pub fn decode_stp_fpsimd_aarch64_instrs_memory_pair_simdfp_offset<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    Rt2: u8,
    imm7: u8,
    L: bool,
    opc: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        tagchecked: bool,
        t: i64,
        t2: i64,
        gs_160965: bool,
        gs_160976: bool,
        n: i64,
        memop: u32,
        gs_160975: bool,
        gs_160970: bool,
        gs_160968: bool,
        offset: u64,
        gs_160972: bool,
        datasize: i128,
        rt_unknown: bool,
        c: u32,
        Rt: u8,
        Rn: u8,
        Rt2: u8,
        imm7: u8,
        L: bool,
        opc: u8,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        Rt2,
        imm7,
        L,
        opc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rn:u8
        let s_0_0: u8 = fn_state.Rn;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var n <= s_0_3
        fn_state.n = s_0_3;
        // D s_0_5: read-var Rt:u8
        let s_0_5: u8 = fn_state.Rt;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var t <= s_0_8
        fn_state.t = s_0_8;
        // D s_0_10: read-var Rt2:u8
        let s_0_10: u8 = fn_state.Rt2;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 5u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: write-var t2 <= s_0_13
        fn_state.t2 = s_0_13;
        // D s_0_15: read-var L:u8
        let s_0_15: bool = fn_state.L;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 1u16);
        // C s_0_17: const #1u : u8
        let s_0_17: bool = true;
        // C s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_19: cmp-eq s_0_16 s_0_18
        let s_0_19: bool = ((s_0_16) == (s_0_18));
        // N s_0_20: branch s_0_19 b33 b1
        if s_0_19 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1u : u32
        let s_1_0: u32 = 1;
        // D s_1_1: write-var memop <= s_1_0
        fn_state.memop = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var opc:u8
        let s_2_0: u8 = fn_state.opc;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #3u : u8
        let s_2_2: u8 = 3;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b32 b3
        if s_2_4 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var opc:u8
        let s_3_0: u8 = fn_state.opc;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: const #2s : i
        let s_3_4: i128 = 2;
        // D s_3_5: cast zx s_3_3 -> i
        let s_3_5: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_6: add s_3_4 s_3_5
        let s_3_6: i128 = (s_3_4 + s_3_5);
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // C s_3_8: const #8s : i
        let s_3_8: i128 = 8;
        // D s_3_9: cast zx s_3_7 -> i
        let s_3_9: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_10: lsl s_3_8 s_3_9
        let s_3_10: i128 = s_3_8 << s_3_9;
        // D s_3_11: write-var datasize <= s_3_10
        fn_state.datasize = s_3_10;
        // C s_3_12: const #64s : i
        let s_3_12: i128 = 64;
        // D s_3_13: read-var imm7:u8
        let s_3_13: u8 = fn_state.imm7;
        // D s_3_14: cast zx s_3_13 -> bv
        let s_3_14: Bits = Bits::new(s_3_13 as u128, 7u16);
        // D s_3_15: bits-cast sx s_3_14 -> bv length s_3_12
        let s_3_15: Bits = s_3_14.sign_extend(s_3_12);
        // D s_3_16: cast reint s_3_15 -> u64
        let s_3_16: u64 = (s_3_15.value() as u64);
        // D s_3_17: cast zx s_3_16 -> bv
        let s_3_17: Bits = Bits::new(s_3_16 as u128, 64u16);
        // D s_3_18: cast zx s_3_7 -> i
        let s_3_18: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_19: lsl s_3_17 s_3_18
        let s_3_19: Bits = s_3_17 << s_3_18;
        // D s_3_20: cast reint s_3_19 -> u64
        let s_3_20: u64 = (s_3_19.value() as u64);
        // D s_3_21: write-var offset <= s_3_20
        fn_state.offset = s_3_20;
        // C s_3_22: const #31s : i
        let s_3_22: i128 = 31;
        // D s_3_23: read-var n:i64
        let s_3_23: i64 = fn_state.n;
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: call neq_int(s_3_24, s_3_22)
        let s_3_25: bool = neq_int(state, tracer, s_3_24, s_3_22);
        // D s_3_26: write-var tagchecked <= s_3_25
        fn_state.tagchecked = s_3_25;
        // C s_3_27: const #0u : u8
        let s_3_27: bool = false;
        // D s_3_28: write-var rt_unknown <= s_3_27
        fn_state.rt_unknown = s_3_27;
        // D s_3_29: read-var memop:u32
        let s_3_29: u32 = fn_state.memop;
        // C s_3_30: const #0u : u32
        let s_3_30: u32 = 0;
        // D s_3_31: cmp-eq s_3_29 s_3_30
        let s_3_31: bool = ((s_3_29) == (s_3_30));
        // N s_3_32: branch s_3_31 b31 b4
        if s_3_31 {
            return block_31(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#160965 <= s_4_0
        fn_state.gs_160965 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#160965:u8
        let s_5_0: bool = fn_state.gs_160965;
        // N s_5_1: branch s_5_0 b17 b6
        if s_5_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var datasize:i
        let s_7_0: i128 = fn_state.datasize;
        // D s_7_1: call __id(s_7_0)
        let s_7_1: i128 = u__id(state, tracer, s_7_0);
        // C s_7_2: const #32s : i
        let s_7_2: i128 = 32;
        // D s_7_3: cmp-eq s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) == (s_7_2));
        // N s_7_4: branch s_7_3 b16 b8
        if s_7_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var datasize:i
        let s_8_0: i128 = fn_state.datasize;
        // D s_8_1: call __id(s_8_0)
        let s_8_1: i128 = u__id(state, tracer, s_8_0);
        // C s_8_2: const #64s : i
        let s_8_2: i128 = 64;
        // D s_8_3: cmp-eq s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) == (s_8_2));
        // D s_8_4: write-var gs#160968 <= s_8_3
        fn_state.gs_160968 = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#160968:u8
        let s_9_0: bool = fn_state.gs_160968;
        // N s_9_1: branch s_9_0 b15 b10
        if s_9_0 {
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
        // D s_10_0: read-var datasize:i
        let s_10_0: i128 = fn_state.datasize;
        // D s_10_1: call __id(s_10_0)
        let s_10_1: i128 = u__id(state, tracer, s_10_0);
        // C s_10_2: const #128s : i
        let s_10_2: i128 = 128;
        // D s_10_3: cmp-eq s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) == (s_10_2));
        // D s_10_4: write-var gs#160970 <= s_10_3
        fn_state.gs_160970 = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#160970:u8
        let s_11_0: bool = fn_state.gs_160970;
        // N s_11_1: branch s_11_0 b14 b12
        if s_11_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var datasize:i
        let s_12_0: i128 = fn_state.datasize;
        // D s_12_1: call __id(s_12_0)
        let s_12_1: i128 = u__id(state, tracer, s_12_0);
        // C s_12_2: const #256s : i
        let s_12_2: i128 = 256;
        // D s_12_3: cmp-eq s_12_1 s_12_2
        let s_12_3: bool = ((s_12_1) == (s_12_2));
        // D s_12_4: write-var gs#160972 <= s_12_3
        fn_state.gs_160972 = s_12_3;
        // N s_12_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#160972:u8
        let s_13_0: bool = fn_state.gs_160972;
        // N s_13_1: assert s_13_0
        let s_13_1: () = assert!(s_13_0);
        // D s_13_2: read-var datasize:i
        let s_13_2: i128 = fn_state.datasize;
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // D s_13_4: read-var memop:u32
        let s_13_4: u32 = fn_state.memop;
        // D s_13_5: read-var n:i64
        let s_13_5: i64 = fn_state.n;
        // C s_13_6: const #0u : u8
        let s_13_6: bool = false;
        // D s_13_7: read-var offset:u64
        let s_13_7: u64 = fn_state.offset;
        // C s_13_8: const #0u : u8
        let s_13_8: bool = false;
        // D s_13_9: read-var rt_unknown:u8
        let s_13_9: bool = fn_state.rt_unknown;
        // D s_13_10: read-var t:i64
        let s_13_10: i64 = fn_state.t;
        // D s_13_11: read-var t2:i64
        let s_13_11: i64 = fn_state.t2;
        // D s_13_12: read-var tagchecked:u8
        let s_13_12: bool = fn_state.tagchecked;
        // C s_13_13: const #0u : u8
        let s_13_13: bool = false;
        // D s_13_14: call execute_aarch64_instrs_memory_pair_simdfp_post_idx(s_13_3, s_13_4, s_13_5, s_13_6, s_13_7, s_13_8, s_13_9, s_13_10, s_13_11, s_13_12, s_13_13)
        let s_13_14: () = execute_aarch64_instrs_memory_pair_simdfp_post_idx(
            state,
            tracer,
            s_13_3,
            s_13_4,
            s_13_5,
            s_13_6,
            s_13_7,
            s_13_8,
            s_13_9,
            s_13_10,
            s_13_11,
            s_13_12,
            s_13_13,
        );
        // N s_13_15: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#160972 <= s_14_0
        fn_state.gs_160972 = s_14_0;
        // N s_14_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#160970 <= s_15_0
        fn_state.gs_160970 = s_15_0;
        // N s_15_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#160968 <= s_16_0
        fn_state.gs_160968 = s_16_0;
        // N s_16_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #3u : u32
        let s_17_0: u32 = 3;
        // S s_17_1: call ConstrainUnpredictable(s_17_0)
        let s_17_1: u32 = ConstrainUnpredictable(state, tracer, s_17_0);
        // D s_17_2: write-var c <= s_17_1
        fn_state.c = s_17_1;
        // D s_17_3: read-var c:u32
        let s_17_3: u32 = fn_state.c;
        // C s_17_4: const #1u : u32
        let s_17_4: u32 = 1;
        // D s_17_5: cmp-eq s_17_3 s_17_4
        let s_17_5: bool = ((s_17_3) == (s_17_4));
        // N s_17_6: branch s_17_5 b30 b18
        if s_17_5 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var c:u32
        let s_18_0: u32 = fn_state.c;
        // C s_18_1: const #2u : u32
        let s_18_1: u32 = 2;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // N s_18_3: branch s_18_2 b29 b19
        if s_18_2 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var c:u32
        let s_19_0: u32 = fn_state.c;
        // C s_19_1: const #4u : u32
        let s_19_1: u32 = 4;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // D s_19_3: write-var gs#160975 <= s_19_2
        fn_state.gs_160975 = s_19_2;
        // N s_19_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#160975:u8
        let s_20_0: bool = fn_state.gs_160975;
        // D s_20_1: write-var gs#160976 <= s_20_0
        fn_state.gs_160976 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#160976:u8
        let s_21_0: bool = fn_state.gs_160976;
        // N s_21_1: assert s_21_0
        let s_21_1: () = assert!(s_21_0);
        // C s_21_2: const #1u : u32
        let s_21_2: u32 = 1;
        // D s_21_3: read-var c:u32
        let s_21_3: u32 = fn_state.c;
        // D s_21_4: cmp-eq s_21_2 s_21_3
        let s_21_4: bool = ((s_21_2) == (s_21_3));
        // D s_21_5: not s_21_4
        let s_21_5: bool = !s_21_4;
        // N s_21_6: branch s_21_5 b24 b22
        if s_21_5 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var rt_unknown <= s_22_0
        fn_state.rt_unknown = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #2u : u32
        let s_24_0: u32 = 2;
        // D s_24_1: read-var c:u32
        let s_24_1: u32 = fn_state.c;
        // D s_24_2: cmp-eq s_24_0 s_24_1
        let s_24_2: bool = ((s_24_0) == (s_24_1));
        // D s_24_3: not s_24_2
        let s_24_3: bool = !s_24_2;
        // N s_24_4: branch s_24_3 b26 b25
        if s_24_3 {
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
        // C s_26_0: const #4u : u32
        let s_26_0: u32 = 4;
        // D s_26_1: read-var c:u32
        let s_26_1: u32 = fn_state.c;
        // D s_26_2: cmp-eq s_26_0 s_26_1
        let s_26_2: bool = ((s_26_0) == (s_26_1));
        // D s_26_3: not s_26_2
        let s_26_3: bool = !s_26_2;
        // N s_26_4: branch s_26_3 b28 b27
        if s_26_3 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call EndOfInstruction(s_27_0)
        let s_27_1: () = EndOfInstruction(state, tracer, s_27_0);
        // N s_27_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#160975 <= s_29_0
        fn_state.gs_160975 = s_29_0;
        // N s_29_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#160976 <= s_30_0
        fn_state.gs_160976 = s_30_0;
        // N s_30_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var t:i64
        let s_31_0: i64 = fn_state.t;
        // D s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_2: read-var t2:i64
        let s_31_2: i64 = fn_state.t2;
        // D s_31_3: cast zx s_31_2 -> i
        let s_31_3: i128 = (i128::try_from(s_31_2).unwrap());
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#160965 <= s_31_4
        fn_state.gs_160965 = s_31_4;
        // N s_31_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_32_0: panic
        panic!("{:?}", ());
        // N s_32_1: return
        return;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u32
        let s_33_0: u32 = 0;
        // D s_33_1: write-var memop <= s_33_0
        fn_state.memop = s_33_0;
        // N s_33_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}

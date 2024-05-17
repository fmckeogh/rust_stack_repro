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
use execute_aarch64_instrs_memory_pair_general_no_alloc::*;
use u__id::*;
use ConstrainUnpredictable::*;
use EndOfInstruction::*;
use common::*;
pub fn decode_stnp_gen_aarch64_instrs_memory_pair_general_no_alloc<T: Tracer>(
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
        gs_160614: bool,
        t: i64,
        t2: i64,
        gs_160618: bool,
        n: i64,
        memop: u32,
        gs_160617: bool,
        offset: u64,
        datasize: i64,
        rt_unknown: bool,
        gs_160611: bool,
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
        // N s_0_20: branch s_0_19 b27 b1
        if s_0_19 {
            return block_27(state, tracer, fn_state);
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
        // C s_2_0: const #0s : i
        let s_2_0: i128 = 0;
        // D s_2_1: read-var opc:u8
        let s_2_1: u8 = fn_state.opc;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 2u16);
        // C s_2_3: const #1u : u64
        let s_2_3: u64 = 1;
        // D s_2_4: bit-extract s_2_2 s_2_0 s_2_3
        let s_2_4: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_3).unwrap(),
        ));
        // D s_2_5: cast reint s_2_4 -> u8
        let s_2_5: bool = ((s_2_4.value()) != 0);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // C s_2_7: const #0u : u64
        let s_2_7: u64 = 0;
        // D s_2_8: cast zx s_2_5 -> u64
        let s_2_8: u64 = (s_2_5 as u64);
        // C s_2_9: const #1u : u64
        let s_2_9: u64 = 1;
        // D s_2_10: and s_2_8 s_2_9
        let s_2_10: u64 = ((s_2_8) & (s_2_9));
        // D s_2_11: cmp-eq s_2_10 s_2_9
        let s_2_11: bool = ((s_2_10) == (s_2_9));
        // D s_2_12: lsl s_2_8 s_2_6
        let s_2_12: u64 = s_2_8 << s_2_6;
        // D s_2_13: or s_2_7 s_2_12
        let s_2_13: u64 = ((s_2_7) | (s_2_12));
        // D s_2_14: cmpl s_2_12
        let s_2_14: u64 = !s_2_12;
        // D s_2_15: and s_2_7 s_2_14
        let s_2_15: u64 = ((s_2_7) & (s_2_14));
        // D s_2_16: select s_2_11 s_2_13 s_2_15
        let s_2_16: u64 = if s_2_11 { s_2_13 } else { s_2_15 };
        // D s_2_17: cast trunc s_2_16 -> u8
        let s_2_17: bool = ((s_2_16) != 0);
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // C s_2_19: const #1u : u8
        let s_2_19: bool = true;
        // C s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 1u16);
        // D s_2_21: cmp-eq s_2_18 s_2_20
        let s_2_21: bool = ((s_2_18) == (s_2_20));
        // N s_2_22: branch s_2_21 b26 b3
        if s_2_21 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1s : i
        let s_3_0: i128 = 1;
        // D s_3_1: read-var opc:u8
        let s_3_1: u8 = fn_state.opc;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 2u16);
        // C s_3_3: const #1u : u64
        let s_3_3: u64 = 1;
        // D s_3_4: bit-extract s_3_2 s_3_0 s_3_3
        let s_3_4: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_3).unwrap(),
        ));
        // D s_3_5: cast reint s_3_4 -> u8
        let s_3_5: bool = ((s_3_4.value()) != 0);
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // C s_3_7: const #0u : u64
        let s_3_7: u64 = 0;
        // D s_3_8: cast zx s_3_5 -> u64
        let s_3_8: u64 = (s_3_5 as u64);
        // C s_3_9: const #1u : u64
        let s_3_9: u64 = 1;
        // D s_3_10: and s_3_8 s_3_9
        let s_3_10: u64 = ((s_3_8) & (s_3_9));
        // D s_3_11: cmp-eq s_3_10 s_3_9
        let s_3_11: bool = ((s_3_10) == (s_3_9));
        // D s_3_12: lsl s_3_8 s_3_6
        let s_3_12: u64 = s_3_8 << s_3_6;
        // D s_3_13: or s_3_7 s_3_12
        let s_3_13: u64 = ((s_3_7) | (s_3_12));
        // D s_3_14: cmpl s_3_12
        let s_3_14: u64 = !s_3_12;
        // D s_3_15: and s_3_7 s_3_14
        let s_3_15: u64 = ((s_3_7) & (s_3_14));
        // D s_3_16: select s_3_11 s_3_13 s_3_15
        let s_3_16: u64 = if s_3_11 { s_3_13 } else { s_3_15 };
        // D s_3_17: cast trunc s_3_16 -> u8
        let s_3_17: bool = ((s_3_16) != 0);
        // D s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 1u16);
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (s_3_18.value() as i128);
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // C s_3_21: const #2s : i
        let s_3_21: i128 = 2;
        // D s_3_22: cast zx s_3_20 -> i
        let s_3_22: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_23: add s_3_21 s_3_22
        let s_3_23: i128 = (s_3_21 + s_3_22);
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // C s_3_25: const #8s : i64
        let s_3_25: i64 = 8;
        // D s_3_26: lsl s_3_25 s_3_24
        let s_3_26: i64 = s_3_25 << s_3_24;
        // D s_3_27: write-var datasize <= s_3_26
        fn_state.datasize = s_3_26;
        // C s_3_28: const #64s : i
        let s_3_28: i128 = 64;
        // D s_3_29: read-var imm7:u8
        let s_3_29: u8 = fn_state.imm7;
        // D s_3_30: cast zx s_3_29 -> bv
        let s_3_30: Bits = Bits::new(s_3_29 as u128, 7u16);
        // D s_3_31: bits-cast sx s_3_30 -> bv length s_3_28
        let s_3_31: Bits = s_3_30.sign_extend(s_3_28);
        // D s_3_32: cast reint s_3_31 -> u64
        let s_3_32: u64 = (s_3_31.value() as u64);
        // D s_3_33: cast zx s_3_32 -> bv
        let s_3_33: Bits = Bits::new(s_3_32 as u128, 64u16);
        // D s_3_34: cast zx s_3_24 -> i
        let s_3_34: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_35: lsl s_3_33 s_3_34
        let s_3_35: Bits = s_3_33 << s_3_34;
        // D s_3_36: cast reint s_3_35 -> u64
        let s_3_36: u64 = (s_3_35.value() as u64);
        // D s_3_37: write-var offset <= s_3_36
        fn_state.offset = s_3_36;
        // C s_3_38: const #31s : i
        let s_3_38: i128 = 31;
        // D s_3_39: read-var n:i64
        let s_3_39: i64 = fn_state.n;
        // D s_3_40: cast zx s_3_39 -> i
        let s_3_40: i128 = (i128::try_from(s_3_39).unwrap());
        // D s_3_41: call neq_int(s_3_40, s_3_38)
        let s_3_41: bool = neq_int(state, tracer, s_3_40, s_3_38);
        // D s_3_42: write-var tagchecked <= s_3_41
        fn_state.tagchecked = s_3_41;
        // C s_3_43: const #0u : u8
        let s_3_43: bool = false;
        // D s_3_44: write-var rt_unknown <= s_3_43
        fn_state.rt_unknown = s_3_43;
        // D s_3_45: read-var memop:u32
        let s_3_45: u32 = fn_state.memop;
        // C s_3_46: const #0u : u32
        let s_3_46: u32 = 0;
        // D s_3_47: cmp-eq s_3_45 s_3_46
        let s_3_47: bool = ((s_3_45) == (s_3_46));
        // N s_3_48: branch s_3_47 b25 b4
        if s_3_47 {
            return block_25(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#160611 <= s_4_0
        fn_state.gs_160611 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#160611:u8
        let s_5_0: bool = fn_state.gs_160611;
        // N s_5_1: branch s_5_0 b11 b6
        if s_5_0 {
            return block_11(state, tracer, fn_state);
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
        // D s_7_0: read-var datasize:i64
        let s_7_0: i64 = fn_state.datasize;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call __id(s_7_1)
        let s_7_2: i128 = u__id(state, tracer, s_7_1);
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: const #32s : i
        let s_7_4: i128 = 32;
        // D s_7_5: cast zx s_7_3 -> i
        let s_7_5: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_6: cmp-eq s_7_5 s_7_4
        let s_7_6: bool = ((s_7_5) == (s_7_4));
        // N s_7_7: branch s_7_6 b10 b8
        if s_7_6 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var datasize:i64
        let s_8_0: i64 = fn_state.datasize;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call __id(s_8_1)
        let s_8_2: i128 = u__id(state, tracer, s_8_1);
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // C s_8_4: const #64s : i
        let s_8_4: i128 = 64;
        // D s_8_5: cast zx s_8_3 -> i
        let s_8_5: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_6: cmp-eq s_8_5 s_8_4
        let s_8_6: bool = ((s_8_5) == (s_8_4));
        // D s_8_7: write-var gs#160614 <= s_8_6
        fn_state.gs_160614 = s_8_6;
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#160614:u8
        let s_9_0: bool = fn_state.gs_160614;
        // N s_9_1: assert s_9_0
        let s_9_1: () = assert!(s_9_0);
        // D s_9_2: read-var datasize:i64
        let s_9_2: i64 = fn_state.datasize;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: cast reint s_9_3 -> i64
        let s_9_4: i64 = (s_9_3 as i64);
        // D s_9_5: read-var memop:u32
        let s_9_5: u32 = fn_state.memop;
        // D s_9_6: read-var n:i64
        let s_9_6: i64 = fn_state.n;
        // C s_9_7: const #1u : u8
        let s_9_7: bool = true;
        // D s_9_8: read-var offset:u64
        let s_9_8: u64 = fn_state.offset;
        // C s_9_9: const #0u : u8
        let s_9_9: bool = false;
        // D s_9_10: read-var rt_unknown:u8
        let s_9_10: bool = fn_state.rt_unknown;
        // D s_9_11: read-var t:i64
        let s_9_11: i64 = fn_state.t;
        // D s_9_12: read-var t2:i64
        let s_9_12: i64 = fn_state.t2;
        // D s_9_13: read-var tagchecked:u8
        let s_9_13: bool = fn_state.tagchecked;
        // C s_9_14: const #0u : u8
        let s_9_14: bool = false;
        // D s_9_15: call execute_aarch64_instrs_memory_pair_general_no_alloc(s_9_4, s_9_5, s_9_6, s_9_7, s_9_8, s_9_9, s_9_10, s_9_11, s_9_12, s_9_13, s_9_14)
        let s_9_15: () = execute_aarch64_instrs_memory_pair_general_no_alloc(
            state,
            tracer,
            s_9_4,
            s_9_5,
            s_9_6,
            s_9_7,
            s_9_8,
            s_9_9,
            s_9_10,
            s_9_11,
            s_9_12,
            s_9_13,
            s_9_14,
        );
        // N s_9_16: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#160614 <= s_10_0
        fn_state.gs_160614 = s_10_0;
        // N s_10_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #3u : u32
        let s_11_0: u32 = 3;
        // S s_11_1: call ConstrainUnpredictable(s_11_0)
        let s_11_1: u32 = ConstrainUnpredictable(state, tracer, s_11_0);
        // D s_11_2: write-var c <= s_11_1
        fn_state.c = s_11_1;
        // D s_11_3: read-var c:u32
        let s_11_3: u32 = fn_state.c;
        // C s_11_4: const #1u : u32
        let s_11_4: u32 = 1;
        // D s_11_5: cmp-eq s_11_3 s_11_4
        let s_11_5: bool = ((s_11_3) == (s_11_4));
        // N s_11_6: branch s_11_5 b24 b12
        if s_11_5 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var c:u32
        let s_12_0: u32 = fn_state.c;
        // C s_12_1: const #2u : u32
        let s_12_1: u32 = 2;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // N s_12_3: branch s_12_2 b23 b13
        if s_12_2 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var c:u32
        let s_13_0: u32 = fn_state.c;
        // C s_13_1: const #4u : u32
        let s_13_1: u32 = 4;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: write-var gs#160617 <= s_13_2
        fn_state.gs_160617 = s_13_2;
        // N s_13_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#160617:u8
        let s_14_0: bool = fn_state.gs_160617;
        // D s_14_1: write-var gs#160618 <= s_14_0
        fn_state.gs_160618 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#160618:u8
        let s_15_0: bool = fn_state.gs_160618;
        // N s_15_1: assert s_15_0
        let s_15_1: () = assert!(s_15_0);
        // C s_15_2: const #1u : u32
        let s_15_2: u32 = 1;
        // D s_15_3: read-var c:u32
        let s_15_3: u32 = fn_state.c;
        // D s_15_4: cmp-eq s_15_2 s_15_3
        let s_15_4: bool = ((s_15_2) == (s_15_3));
        // D s_15_5: not s_15_4
        let s_15_5: bool = !s_15_4;
        // N s_15_6: branch s_15_5 b18 b16
        if s_15_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var rt_unknown <= s_16_0
        fn_state.rt_unknown = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #2u : u32
        let s_18_0: u32 = 2;
        // D s_18_1: read-var c:u32
        let s_18_1: u32 = fn_state.c;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // D s_18_3: not s_18_2
        let s_18_3: bool = !s_18_2;
        // N s_18_4: branch s_18_3 b20 b19
        if s_18_3 {
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
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #4u : u32
        let s_20_0: u32 = 4;
        // D s_20_1: read-var c:u32
        let s_20_1: u32 = fn_state.c;
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // D s_20_3: not s_20_2
        let s_20_3: bool = !s_20_2;
        // N s_20_4: branch s_20_3 b22 b21
        if s_20_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call EndOfInstruction(s_21_0)
        let s_21_1: () = EndOfInstruction(state, tracer, s_21_0);
        // N s_21_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#160617 <= s_23_0
        fn_state.gs_160617 = s_23_0;
        // N s_23_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#160618 <= s_24_0
        fn_state.gs_160618 = s_24_0;
        // N s_24_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var t:i64
        let s_25_0: i64 = fn_state.t;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: read-var t2:i64
        let s_25_2: i64 = fn_state.t2;
        // D s_25_3: cast zx s_25_2 -> i
        let s_25_3: i128 = (i128::try_from(s_25_2).unwrap());
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#160611 <= s_25_4
        fn_state.gs_160611 = s_25_4;
        // N s_25_6: jump b5
        return block_5(state, tracer, fn_state);
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
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u32
        let s_27_0: u32 = 0;
        // D s_27_1: write-var memop <= s_27_0
        fn_state.memop = s_27_0;
        // N s_27_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}

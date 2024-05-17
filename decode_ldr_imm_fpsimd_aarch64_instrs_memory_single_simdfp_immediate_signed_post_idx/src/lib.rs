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
use execute_aarch64_instrs_memory_single_simdfp_immediate_signed_post_idx::*;
use u__id::*;
use common::*;
pub fn decode_ldr_imm_fpsimd_aarch64_instrs_memory_single_simdfp_immediate_signed_post_idx<
    T: Tracer,
>(state: &mut State, tracer: &T, Rt: u8, Rn: u8, imm9: u16, opc: u8, size: u8) -> () {
    #[derive(Default)]
    struct FunctionState {
        tagchecked: bool,
        t: i64,
        gs_161607: bool,
        gs_161610: bool,
        gs_161620: bool,
        gs_161622: bool,
        n: i64,
        memop: u32,
        gs_161616: bool,
        offset: u64,
        gs_161612: bool,
        datasize: i128,
        gs_161618: bool,
        scale: i64,
        gs_161614: bool,
        Rt: u8,
        Rn: u8,
        imm9: u16,
        opc: u8,
        size: u8,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        imm9,
        opc,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1s : i
        let s_0_0: i128 = 1;
        // D s_0_1: read-var opc:u8
        let s_0_1: u8 = fn_state.opc;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #1u : u64
        let s_0_3: u64 = 1;
        // D s_0_4: bit-extract s_0_2 s_0_0 s_0_3
        let s_0_4: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_3).unwrap(),
        ));
        // D s_0_5: cast reint s_0_4 -> u8
        let s_0_5: bool = ((s_0_4.value()) != 0);
        // C s_0_6: const #0s : i
        let s_0_6: i128 = 0;
        // C s_0_7: const #0u : u64
        let s_0_7: u64 = 0;
        // D s_0_8: cast zx s_0_5 -> u64
        let s_0_8: u64 = (s_0_5 as u64);
        // C s_0_9: const #1u : u64
        let s_0_9: u64 = 1;
        // D s_0_10: and s_0_8 s_0_9
        let s_0_10: u64 = ((s_0_8) & (s_0_9));
        // D s_0_11: cmp-eq s_0_10 s_0_9
        let s_0_11: bool = ((s_0_10) == (s_0_9));
        // D s_0_12: lsl s_0_8 s_0_6
        let s_0_12: u64 = s_0_8 << s_0_6;
        // D s_0_13: or s_0_7 s_0_12
        let s_0_13: u64 = ((s_0_7) | (s_0_12));
        // D s_0_14: cmpl s_0_12
        let s_0_14: u64 = !s_0_12;
        // D s_0_15: and s_0_7 s_0_14
        let s_0_15: u64 = ((s_0_7) & (s_0_14));
        // D s_0_16: select s_0_11 s_0_13 s_0_15
        let s_0_16: u64 = if s_0_11 { s_0_13 } else { s_0_15 };
        // D s_0_17: cast trunc s_0_16 -> u8
        let s_0_17: bool = ((s_0_16) != 0);
        // D s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_19: read-var size:u8
        let s_0_19: u8 = fn_state.size;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 2u16);
        // D s_0_21: cast reint s_0_18 -> u128
        let s_0_21: u128 = (s_0_18.value() as u128);
        // D s_0_22: size-of s_0_18
        let s_0_22: u16 = s_0_18.length();
        // D s_0_23: cast reint s_0_20 -> u128
        let s_0_23: u128 = (s_0_20.value() as u128);
        // D s_0_24: size-of s_0_20
        let s_0_24: u16 = s_0_20.length();
        // D s_0_25: lsl s_0_21 s_0_24
        let s_0_25: u128 = s_0_21 << s_0_24;
        // D s_0_26: or s_0_25 s_0_23
        let s_0_26: u128 = ((s_0_25) | (s_0_23));
        // D s_0_27: add s_0_22 s_0_24
        let s_0_27: u16 = (s_0_22 + s_0_24);
        // D s_0_28: create-bits s_0_26 s_0_27
        let s_0_28: Bits = Bits::new(s_0_26, s_0_27);
        // D s_0_29: cast reint s_0_28 -> u8
        let s_0_29: u8 = (s_0_28.value() as u8);
        // D s_0_30: cast zx s_0_29 -> bv
        let s_0_30: Bits = Bits::new(s_0_29 as u128, 3u16);
        // D s_0_31: cast zx s_0_30 -> i
        let s_0_31: i128 = (s_0_30.value() as i128);
        // D s_0_32: cast reint s_0_31 -> i64
        let s_0_32: i64 = (s_0_31 as i64);
        // D s_0_33: write-var scale <= s_0_32
        fn_state.scale = s_0_32;
        // C s_0_34: const #4s : i
        let s_0_34: i128 = 4;
        // D s_0_35: read-var scale:i64
        let s_0_35: i64 = fn_state.scale;
        // D s_0_36: cast zx s_0_35 -> i
        let s_0_36: i128 = (i128::try_from(s_0_35).unwrap());
        // D s_0_37: cmp-gt s_0_36 s_0_34
        let s_0_37: bool = ((s_0_36) > (s_0_34));
        // N s_0_38: branch s_0_37 b29 b1
        if s_0_37 {
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
        // C s_1_0: const #64s : i
        let s_1_0: i128 = 64;
        // D s_1_1: read-var imm9:u9
        let s_1_1: u16 = fn_state.imm9;
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 9u16);
        // D s_1_3: bits-cast sx s_1_2 -> bv length s_1_0
        let s_1_3: Bits = s_1_2.sign_extend(s_1_0);
        // D s_1_4: cast reint s_1_3 -> u64
        let s_1_4: u64 = (s_1_3.value() as u64);
        // D s_1_5: write-var offset <= s_1_4
        fn_state.offset = s_1_4;
        // D s_1_6: read-var Rn:u8
        let s_1_6: u8 = fn_state.Rn;
        // D s_1_7: cast zx s_1_6 -> bv
        let s_1_7: Bits = Bits::new(s_1_6 as u128, 5u16);
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (s_1_7.value() as i128);
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: write-var n <= s_1_9
        fn_state.n = s_1_9;
        // D s_1_11: read-var Rt:u8
        let s_1_11: u8 = fn_state.Rt;
        // D s_1_12: cast zx s_1_11 -> bv
        let s_1_12: Bits = Bits::new(s_1_11 as u128, 5u16);
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (s_1_12.value() as i128);
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: write-var t <= s_1_14
        fn_state.t = s_1_14;
        // C s_1_16: const #0s : i
        let s_1_16: i128 = 0;
        // D s_1_17: read-var opc:u8
        let s_1_17: u8 = fn_state.opc;
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 2u16);
        // C s_1_19: const #1u : u64
        let s_1_19: u64 = 1;
        // D s_1_20: bit-extract s_1_18 s_1_16 s_1_19
        let s_1_20: Bits = (Bits::new(
            ((s_1_18) >> (s_1_16)).value(),
            u16::try_from(s_1_19).unwrap(),
        ));
        // D s_1_21: cast reint s_1_20 -> u8
        let s_1_21: bool = ((s_1_20.value()) != 0);
        // C s_1_22: const #0s : i
        let s_1_22: i128 = 0;
        // C s_1_23: const #0u : u64
        let s_1_23: u64 = 0;
        // D s_1_24: cast zx s_1_21 -> u64
        let s_1_24: u64 = (s_1_21 as u64);
        // C s_1_25: const #1u : u64
        let s_1_25: u64 = 1;
        // D s_1_26: and s_1_24 s_1_25
        let s_1_26: u64 = ((s_1_24) & (s_1_25));
        // D s_1_27: cmp-eq s_1_26 s_1_25
        let s_1_27: bool = ((s_1_26) == (s_1_25));
        // D s_1_28: lsl s_1_24 s_1_22
        let s_1_28: u64 = s_1_24 << s_1_22;
        // D s_1_29: or s_1_23 s_1_28
        let s_1_29: u64 = ((s_1_23) | (s_1_28));
        // D s_1_30: cmpl s_1_28
        let s_1_30: u64 = !s_1_28;
        // D s_1_31: and s_1_23 s_1_30
        let s_1_31: u64 = ((s_1_23) & (s_1_30));
        // D s_1_32: select s_1_27 s_1_29 s_1_31
        let s_1_32: u64 = if s_1_27 { s_1_29 } else { s_1_31 };
        // D s_1_33: cast trunc s_1_32 -> u8
        let s_1_33: bool = ((s_1_32) != 0);
        // D s_1_34: cast zx s_1_33 -> bv
        let s_1_34: Bits = Bits::new(s_1_33 as u128, 1u16);
        // C s_1_35: const #1u : u8
        let s_1_35: bool = true;
        // C s_1_36: cast zx s_1_35 -> bv
        let s_1_36: Bits = Bits::new(s_1_35 as u128, 1u16);
        // D s_1_37: cmp-eq s_1_34 s_1_36
        let s_1_37: bool = ((s_1_34) == (s_1_36));
        // N s_1_38: branch s_1_37 b28 b2
        if s_1_37 {
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
        // C s_2_0: const #1u : u32
        let s_2_0: u32 = 1;
        // D s_2_1: write-var memop <= s_2_0
        fn_state.memop = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // D s_3_1: read-var scale:i64
        let s_3_1: i64 = fn_state.scale;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: lsl s_3_0 s_3_2
        let s_3_3: i128 = s_3_0 << s_3_2;
        // D s_3_4: write-var datasize <= s_3_3
        fn_state.datasize = s_3_3;
        // D s_3_5: read-var memop:u32
        let s_3_5: u32 = fn_state.memop;
        // C s_3_6: const #2u : u32
        let s_3_6: u32 = 2;
        // D s_3_7: cmp-eq s_3_5 s_3_6
        let s_3_7: bool = ((s_3_5) == (s_3_6));
        // N s_3_8: branch s_3_7 b27 b4
        if s_3_7 {
            return block_27(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#161607 <= s_4_0
        fn_state.gs_161607 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#161607:u8
        let s_5_0: bool = fn_state.gs_161607;
        // D s_5_1: write-var tagchecked <= s_5_0
        fn_state.tagchecked = s_5_0;
        // D s_5_2: read-var datasize:i
        let s_5_2: i128 = fn_state.datasize;
        // D s_5_3: call __id(s_5_2)
        let s_5_3: i128 = u__id(state, tracer, s_5_2);
        // C s_5_4: const #8s : i
        let s_5_4: i128 = 8;
        // D s_5_5: cmp-eq s_5_3 s_5_4
        let s_5_5: bool = ((s_5_3) == (s_5_4));
        // N s_5_6: branch s_5_5 b26 b6
        if s_5_5 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var datasize:i
        let s_6_0: i128 = fn_state.datasize;
        // D s_6_1: call __id(s_6_0)
        let s_6_1: i128 = u__id(state, tracer, s_6_0);
        // C s_6_2: const #16s : i
        let s_6_2: i128 = 16;
        // D s_6_3: cmp-eq s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) == (s_6_2));
        // D s_6_4: write-var gs#161610 <= s_6_3
        fn_state.gs_161610 = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#161610:u8
        let s_7_0: bool = fn_state.gs_161610;
        // N s_7_1: branch s_7_0 b25 b8
        if s_7_0 {
            return block_25(state, tracer, fn_state);
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
        // C s_8_2: const #32s : i
        let s_8_2: i128 = 32;
        // D s_8_3: cmp-eq s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) == (s_8_2));
        // D s_8_4: write-var gs#161612 <= s_8_3
        fn_state.gs_161612 = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#161612:u8
        let s_9_0: bool = fn_state.gs_161612;
        // N s_9_1: branch s_9_0 b24 b10
        if s_9_0 {
            return block_24(state, tracer, fn_state);
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
        // C s_10_2: const #64s : i
        let s_10_2: i128 = 64;
        // D s_10_3: cmp-eq s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) == (s_10_2));
        // D s_10_4: write-var gs#161614 <= s_10_3
        fn_state.gs_161614 = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#161614:u8
        let s_11_0: bool = fn_state.gs_161614;
        // N s_11_1: branch s_11_0 b23 b12
        if s_11_0 {
            return block_23(state, tracer, fn_state);
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
        // C s_12_2: const #128s : i
        let s_12_2: i128 = 128;
        // D s_12_3: cmp-eq s_12_1 s_12_2
        let s_12_3: bool = ((s_12_1) == (s_12_2));
        // D s_12_4: write-var gs#161616 <= s_12_3
        fn_state.gs_161616 = s_12_3;
        // N s_12_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#161616:u8
        let s_13_0: bool = fn_state.gs_161616;
        // N s_13_1: branch s_13_0 b22 b14
        if s_13_0 {
            return block_22(state, tracer, fn_state);
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
        // C s_14_2: const #256s : i
        let s_14_2: i128 = 256;
        // D s_14_3: cmp-eq s_14_1 s_14_2
        let s_14_3: bool = ((s_14_1) == (s_14_2));
        // D s_14_4: write-var gs#161618 <= s_14_3
        fn_state.gs_161618 = s_14_3;
        // N s_14_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#161618:u8
        let s_15_0: bool = fn_state.gs_161618;
        // N s_15_1: branch s_15_0 b21 b16
        if s_15_0 {
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
        // D s_16_0: read-var datasize:i
        let s_16_0: i128 = fn_state.datasize;
        // D s_16_1: call __id(s_16_0)
        let s_16_1: i128 = u__id(state, tracer, s_16_0);
        // C s_16_2: const #512s : i
        let s_16_2: i128 = 512;
        // D s_16_3: cmp-eq s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) == (s_16_2));
        // D s_16_4: write-var gs#161620 <= s_16_3
        fn_state.gs_161620 = s_16_3;
        // N s_16_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#161620:u8
        let s_17_0: bool = fn_state.gs_161620;
        // N s_17_1: branch s_17_0 b20 b18
        if s_17_0 {
            return block_20(state, tracer, fn_state);
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
        // C s_18_2: const #1024s : i
        let s_18_2: i128 = 1024;
        // D s_18_3: cmp-eq s_18_1 s_18_2
        let s_18_3: bool = ((s_18_1) == (s_18_2));
        // D s_18_4: write-var gs#161622 <= s_18_3
        fn_state.gs_161622 = s_18_3;
        // N s_18_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#161622:u8
        let s_19_0: bool = fn_state.gs_161622;
        // N s_19_1: assert s_19_0
        let s_19_1: () = assert!(s_19_0);
        // D s_19_2: read-var datasize:i
        let s_19_2: i128 = fn_state.datasize;
        // D s_19_3: cast reint s_19_2 -> i64
        let s_19_3: i64 = (s_19_2 as i64);
        // D s_19_4: read-var memop:u32
        let s_19_4: u32 = fn_state.memop;
        // D s_19_5: read-var n:i64
        let s_19_5: i64 = fn_state.n;
        // C s_19_6: const #0u : u8
        let s_19_6: bool = false;
        // D s_19_7: read-var offset:u64
        let s_19_7: u64 = fn_state.offset;
        // C s_19_8: const #1u : u8
        let s_19_8: bool = true;
        // D s_19_9: read-var t:i64
        let s_19_9: i64 = fn_state.t;
        // D s_19_10: read-var tagchecked:u8
        let s_19_10: bool = fn_state.tagchecked;
        // C s_19_11: const #1u : u8
        let s_19_11: bool = true;
        // D s_19_12: call execute_aarch64_instrs_memory_single_simdfp_immediate_signed_post_idx(s_19_3, s_19_4, s_19_5, s_19_6, s_19_7, s_19_8, s_19_9, s_19_10, s_19_11)
        let s_19_12: () = execute_aarch64_instrs_memory_single_simdfp_immediate_signed_post_idx(
            state,
            tracer,
            s_19_3,
            s_19_4,
            s_19_5,
            s_19_6,
            s_19_7,
            s_19_8,
            s_19_9,
            s_19_10,
            s_19_11,
        );
        // N s_19_13: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#161622 <= s_20_0
        fn_state.gs_161622 = s_20_0;
        // N s_20_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#161620 <= s_21_0
        fn_state.gs_161620 = s_21_0;
        // N s_21_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#161618 <= s_22_0
        fn_state.gs_161618 = s_22_0;
        // N s_22_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#161616 <= s_23_0
        fn_state.gs_161616 = s_23_0;
        // N s_23_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#161614 <= s_24_0
        fn_state.gs_161614 = s_24_0;
        // N s_24_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#161612 <= s_25_0
        fn_state.gs_161612 = s_25_0;
        // N s_25_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#161610 <= s_26_0
        fn_state.gs_161610 = s_26_0;
        // N s_26_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#161607 <= s_27_0
        fn_state.gs_161607 = s_27_0;
        // N s_27_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u32
        let s_28_0: u32 = 0;
        // D s_28_1: write-var memop <= s_28_0
        fn_state.memop = s_28_0;
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

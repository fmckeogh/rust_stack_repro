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
use execute_aarch64_instrs_memory_single_general_immediate_signed_offset_unpriv::*;
use neq_int::*;
use common::*;
pub fn decode_sttr_aarch64_instrs_memory_single_general_immediate_signed_offset_unpriv<
    T: Tracer,
>(state: &mut State, tracer: &T, Rt: u8, Rn: u8, imm9: u16, opc: u8, size: u8) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        regsizeshadow_1730: i64,
        gs_164370: bool,
        n: i64,
        memop: u32,
        regsize: i64,
        offset: u64,
        datasize: i64,
        gs_164351: bool,
        scale: i64,
        is_signed: bool,
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
        // D s_0_0: read-var size:u8
        let s_0_0: u8 = fn_state.size;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 2u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var scale <= s_0_3
        fn_state.scale = s_0_3;
        // C s_0_5: const #64s : i
        let s_0_5: i128 = 64;
        // D s_0_6: read-var imm9:u9
        let s_0_6: u16 = fn_state.imm9;
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 9u16);
        // D s_0_8: bits-cast sx s_0_7 -> bv length s_0_5
        let s_0_8: Bits = s_0_7.sign_extend(s_0_5);
        // D s_0_9: cast reint s_0_8 -> u64
        let s_0_9: u64 = (s_0_8.value() as u64);
        // D s_0_10: write-var offset <= s_0_9
        fn_state.offset = s_0_9;
        // D s_0_11: read-var Rn:u8
        let s_0_11: u8 = fn_state.Rn;
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 5u16);
        // D s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (s_0_12.value() as i128);
        // D s_0_14: cast reint s_0_13 -> i64
        let s_0_14: i64 = (s_0_13 as i64);
        // D s_0_15: write-var n <= s_0_14
        fn_state.n = s_0_14;
        // D s_0_16: read-var Rt:u8
        let s_0_16: u8 = fn_state.Rt;
        // D s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 5u16);
        // D s_0_18: cast zx s_0_17 -> i
        let s_0_18: i128 = (s_0_17.value() as i128);
        // D s_0_19: cast reint s_0_18 -> i64
        let s_0_19: i64 = (s_0_18 as i64);
        // D s_0_20: write-var t <= s_0_19
        fn_state.t = s_0_19;
        // C s_0_21: const #32s : i64
        let s_0_21: i64 = 32;
        // D s_0_22: write-var regsize <= s_0_21
        fn_state.regsize = s_0_21;
        // C s_0_23: const #1s : i
        let s_0_23: i128 = 1;
        // D s_0_24: read-var opc:u8
        let s_0_24: u8 = fn_state.opc;
        // D s_0_25: cast zx s_0_24 -> bv
        let s_0_25: Bits = Bits::new(s_0_24 as u128, 2u16);
        // C s_0_26: const #1u : u64
        let s_0_26: u64 = 1;
        // D s_0_27: bit-extract s_0_25 s_0_23 s_0_26
        let s_0_27: Bits = (Bits::new(
            ((s_0_25) >> (s_0_23)).value(),
            u16::try_from(s_0_26).unwrap(),
        ));
        // D s_0_28: cast reint s_0_27 -> u8
        let s_0_28: bool = ((s_0_27.value()) != 0);
        // C s_0_29: const #0s : i
        let s_0_29: i128 = 0;
        // C s_0_30: const #0u : u64
        let s_0_30: u64 = 0;
        // D s_0_31: cast zx s_0_28 -> u64
        let s_0_31: u64 = (s_0_28 as u64);
        // C s_0_32: const #1u : u64
        let s_0_32: u64 = 1;
        // D s_0_33: and s_0_31 s_0_32
        let s_0_33: u64 = ((s_0_31) & (s_0_32));
        // D s_0_34: cmp-eq s_0_33 s_0_32
        let s_0_34: bool = ((s_0_33) == (s_0_32));
        // D s_0_35: lsl s_0_31 s_0_29
        let s_0_35: u64 = s_0_31 << s_0_29;
        // D s_0_36: or s_0_30 s_0_35
        let s_0_36: u64 = ((s_0_30) | (s_0_35));
        // D s_0_37: cmpl s_0_35
        let s_0_37: u64 = !s_0_35;
        // D s_0_38: and s_0_30 s_0_37
        let s_0_38: u64 = ((s_0_30) & (s_0_37));
        // D s_0_39: select s_0_34 s_0_36 s_0_38
        let s_0_39: u64 = if s_0_34 { s_0_36 } else { s_0_38 };
        // D s_0_40: cast trunc s_0_39 -> u8
        let s_0_40: bool = ((s_0_39) != 0);
        // D s_0_41: cast zx s_0_40 -> bv
        let s_0_41: Bits = Bits::new(s_0_40 as u128, 1u16);
        // C s_0_42: const #0u : u8
        let s_0_42: bool = false;
        // C s_0_43: cast zx s_0_42 -> bv
        let s_0_43: Bits = Bits::new(s_0_42 as u128, 1u16);
        // D s_0_44: cmp-eq s_0_41 s_0_43
        let s_0_44: bool = ((s_0_41) == (s_0_43));
        // N s_0_45: branch s_0_44 b16 b1
        if s_0_44 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var size:u8
        let s_1_0: u8 = fn_state.size;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #3u : u8
        let s_1_2: u8 = 3;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b15 b2
        if s_1_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u32
        let s_2_0: u32 = 0;
        // D s_2_1: write-var memop <= s_2_0
        fn_state.memop = s_2_0;
        // D s_2_2: read-var size:u8
        let s_2_2: u8 = fn_state.size;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // C s_2_4: const #2u : u8
        let s_2_4: u8 = 2;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 2u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b14 b3
        if s_2_6 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#164351 <= s_3_0
        fn_state.gs_164351 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#164351:u8
        let s_4_0: bool = fn_state.gs_164351;
        // N s_4_1: branch s_4_0 b13 b5
        if s_4_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i
        let s_5_0: i128 = 0;
        // D s_5_1: read-var opc:u8
        let s_5_1: u8 = fn_state.opc;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 2u16);
        // C s_5_3: const #1u : u64
        let s_5_3: u64 = 1;
        // D s_5_4: bit-extract s_5_2 s_5_0 s_5_3
        let s_5_4: Bits = (Bits::new(
            ((s_5_2) >> (s_5_0)).value(),
            u16::try_from(s_5_3).unwrap(),
        ));
        // D s_5_5: cast reint s_5_4 -> u8
        let s_5_5: bool = ((s_5_4.value()) != 0);
        // C s_5_6: const #0s : i
        let s_5_6: i128 = 0;
        // C s_5_7: const #0u : u64
        let s_5_7: u64 = 0;
        // D s_5_8: cast zx s_5_5 -> u64
        let s_5_8: u64 = (s_5_5 as u64);
        // C s_5_9: const #1u : u64
        let s_5_9: u64 = 1;
        // D s_5_10: and s_5_8 s_5_9
        let s_5_10: u64 = ((s_5_8) & (s_5_9));
        // D s_5_11: cmp-eq s_5_10 s_5_9
        let s_5_11: bool = ((s_5_10) == (s_5_9));
        // D s_5_12: lsl s_5_8 s_5_6
        let s_5_12: u64 = s_5_8 << s_5_6;
        // D s_5_13: or s_5_7 s_5_12
        let s_5_13: u64 = ((s_5_7) | (s_5_12));
        // D s_5_14: cmpl s_5_12
        let s_5_14: u64 = !s_5_12;
        // D s_5_15: and s_5_7 s_5_14
        let s_5_15: u64 = ((s_5_7) & (s_5_14));
        // D s_5_16: select s_5_11 s_5_13 s_5_15
        let s_5_16: u64 = if s_5_11 { s_5_13 } else { s_5_15 };
        // D s_5_17: cast trunc s_5_16 -> u8
        let s_5_17: bool = ((s_5_16) != 0);
        // D s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 1u16);
        // C s_5_19: const #1u : u8
        let s_5_19: bool = true;
        // C s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 1u16);
        // D s_5_21: cmp-eq s_5_18 s_5_20
        let s_5_21: bool = ((s_5_18) == (s_5_20));
        // N s_5_22: branch s_5_21 b12 b6
        if s_5_21 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // D s_6_1: write-var regsize <= s_6_0
        fn_state.regsize = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var is_signed <= s_7_0
        fn_state.is_signed = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var regsize:i64
        let s_8_0: i64 = fn_state.regsize;
        // D s_8_1: write-var regsizeshadow#1730 <= s_8_0
        fn_state.regsizeshadow_1730 = s_8_0;
        // C s_8_2: const #8s : i64
        let s_8_2: i64 = 8;
        // D s_8_3: read-var scale:i64
        let s_8_3: i64 = fn_state.scale;
        // D s_8_4: lsl s_8_2 s_8_3
        let s_8_4: i64 = s_8_2 << s_8_3;
        // D s_8_5: write-var datasize <= s_8_4
        fn_state.datasize = s_8_4;
        // D s_8_6: read-var memop:u32
        let s_8_6: u32 = fn_state.memop;
        // C s_8_7: const #2u : u32
        let s_8_7: u32 = 2;
        // D s_8_8: cmp-eq s_8_6 s_8_7
        let s_8_8: bool = ((s_8_6) == (s_8_7));
        // N s_8_9: branch s_8_8 b11 b9
        if s_8_8 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#164370 <= s_9_0
        fn_state.gs_164370 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#164370:u8
        let s_10_0: bool = fn_state.gs_164370;
        // D s_10_1: read-var datasize:i64
        let s_10_1: i64 = fn_state.datasize;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // D s_10_4: read-var regsizeshadow#1730:i64
        let s_10_4: i64 = fn_state.regsizeshadow_1730;
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: cast reint s_10_5 -> i64
        let s_10_6: i64 = (s_10_5 as i64);
        // D s_10_7: read-var memop:u32
        let s_10_7: u32 = fn_state.memop;
        // D s_10_8: read-var n:i64
        let s_10_8: i64 = fn_state.n;
        // C s_10_9: const #0u : u8
        let s_10_9: bool = false;
        // D s_10_10: read-var offset:u64
        let s_10_10: u64 = fn_state.offset;
        // C s_10_11: const #0u : u8
        let s_10_11: bool = false;
        // C s_10_12: const #0u : u8
        let s_10_12: bool = false;
        // D s_10_13: read-var is_signed:u8
        let s_10_13: bool = fn_state.is_signed;
        // D s_10_14: read-var t:i64
        let s_10_14: i64 = fn_state.t;
        // C s_10_15: const #0u : u8
        let s_10_15: bool = false;
        // C s_10_16: const #0u : u8
        let s_10_16: bool = false;
        // D s_10_17: call execute_aarch64_instrs_memory_single_general_immediate_signed_offset_unpriv(s_10_3, s_10_7, s_10_8, s_10_9, s_10_10, s_10_11, s_10_6, s_10_12, s_10_13, s_10_14, s_10_0, s_10_15, s_10_16)
        let s_10_17: () = execute_aarch64_instrs_memory_single_general_immediate_signed_offset_unpriv(
            state,
            tracer,
            s_10_3,
            s_10_7,
            s_10_8,
            s_10_9,
            s_10_10,
            s_10_11,
            s_10_6,
            s_10_12,
            s_10_13,
            s_10_14,
            s_10_0,
            s_10_15,
            s_10_16,
        );
        // N s_10_18: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #31s : i
        let s_11_0: i128 = 31;
        // D s_11_1: read-var n:i64
        let s_11_1: i64 = fn_state.n;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: call neq_int(s_11_2, s_11_0)
        let s_11_3: bool = neq_int(state, tracer, s_11_2, s_11_0);
        // D s_11_4: write-var gs#164370 <= s_11_3
        fn_state.gs_164370 = s_11_3;
        // N s_11_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #32s : i64
        let s_12_0: i64 = 32;
        // D s_12_1: write-var regsize <= s_12_0
        fn_state.regsize = s_12_0;
        // N s_12_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0s : i
        let s_14_0: i128 = 0;
        // D s_14_1: read-var opc:u8
        let s_14_1: u8 = fn_state.opc;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 2u16);
        // C s_14_3: const #1u : u64
        let s_14_3: u64 = 1;
        // D s_14_4: bit-extract s_14_2 s_14_0 s_14_3
        let s_14_4: Bits = (Bits::new(
            ((s_14_2) >> (s_14_0)).value(),
            u16::try_from(s_14_3).unwrap(),
        ));
        // D s_14_5: cast reint s_14_4 -> u8
        let s_14_5: bool = ((s_14_4.value()) != 0);
        // C s_14_6: const #0s : i
        let s_14_6: i128 = 0;
        // C s_14_7: const #0u : u64
        let s_14_7: u64 = 0;
        // D s_14_8: cast zx s_14_5 -> u64
        let s_14_8: u64 = (s_14_5 as u64);
        // C s_14_9: const #1u : u64
        let s_14_9: u64 = 1;
        // D s_14_10: and s_14_8 s_14_9
        let s_14_10: u64 = ((s_14_8) & (s_14_9));
        // D s_14_11: cmp-eq s_14_10 s_14_9
        let s_14_11: bool = ((s_14_10) == (s_14_9));
        // D s_14_12: lsl s_14_8 s_14_6
        let s_14_12: u64 = s_14_8 << s_14_6;
        // D s_14_13: or s_14_7 s_14_12
        let s_14_13: u64 = ((s_14_7) | (s_14_12));
        // D s_14_14: cmpl s_14_12
        let s_14_14: u64 = !s_14_12;
        // D s_14_15: and s_14_7 s_14_14
        let s_14_15: u64 = ((s_14_7) & (s_14_14));
        // D s_14_16: select s_14_11 s_14_13 s_14_15
        let s_14_16: u64 = if s_14_11 { s_14_13 } else { s_14_15 };
        // D s_14_17: cast trunc s_14_16 -> u8
        let s_14_17: bool = ((s_14_16) != 0);
        // D s_14_18: cast zx s_14_17 -> bv
        let s_14_18: Bits = Bits::new(s_14_17 as u128, 1u16);
        // C s_14_19: const #1u : u8
        let s_14_19: bool = true;
        // C s_14_20: cast zx s_14_19 -> bv
        let s_14_20: Bits = Bits::new(s_14_19 as u128, 1u16);
        // D s_14_21: cmp-eq s_14_18 s_14_20
        let s_14_21: bool = ((s_14_18) == (s_14_20));
        // D s_14_22: write-var gs#164351 <= s_14_21
        fn_state.gs_164351 = s_14_21;
        // N s_14_23: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0s : i
        let s_16_0: i128 = 0;
        // D s_16_1: read-var opc:u8
        let s_16_1: u8 = fn_state.opc;
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 2u16);
        // C s_16_3: const #1u : u64
        let s_16_3: u64 = 1;
        // D s_16_4: bit-extract s_16_2 s_16_0 s_16_3
        let s_16_4: Bits = (Bits::new(
            ((s_16_2) >> (s_16_0)).value(),
            u16::try_from(s_16_3).unwrap(),
        ));
        // D s_16_5: cast reint s_16_4 -> u8
        let s_16_5: bool = ((s_16_4.value()) != 0);
        // C s_16_6: const #0s : i
        let s_16_6: i128 = 0;
        // C s_16_7: const #0u : u64
        let s_16_7: u64 = 0;
        // D s_16_8: cast zx s_16_5 -> u64
        let s_16_8: u64 = (s_16_5 as u64);
        // C s_16_9: const #1u : u64
        let s_16_9: u64 = 1;
        // D s_16_10: and s_16_8 s_16_9
        let s_16_10: u64 = ((s_16_8) & (s_16_9));
        // D s_16_11: cmp-eq s_16_10 s_16_9
        let s_16_11: bool = ((s_16_10) == (s_16_9));
        // D s_16_12: lsl s_16_8 s_16_6
        let s_16_12: u64 = s_16_8 << s_16_6;
        // D s_16_13: or s_16_7 s_16_12
        let s_16_13: u64 = ((s_16_7) | (s_16_12));
        // D s_16_14: cmpl s_16_12
        let s_16_14: u64 = !s_16_12;
        // D s_16_15: and s_16_7 s_16_14
        let s_16_15: u64 = ((s_16_7) & (s_16_14));
        // D s_16_16: select s_16_11 s_16_13 s_16_15
        let s_16_16: u64 = if s_16_11 { s_16_13 } else { s_16_15 };
        // D s_16_17: cast trunc s_16_16 -> u8
        let s_16_17: bool = ((s_16_16) != 0);
        // D s_16_18: cast zx s_16_17 -> bv
        let s_16_18: Bits = Bits::new(s_16_17 as u128, 1u16);
        // C s_16_19: const #1u : u8
        let s_16_19: bool = true;
        // C s_16_20: cast zx s_16_19 -> bv
        let s_16_20: Bits = Bits::new(s_16_19 as u128, 1u16);
        // D s_16_21: cmp-eq s_16_18 s_16_20
        let s_16_21: bool = ((s_16_18) == (s_16_20));
        // N s_16_22: branch s_16_21 b22 b17
        if s_16_21 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u32
        let s_17_0: u32 = 1;
        // D s_17_1: write-var memop <= s_17_0
        fn_state.memop = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var size:u8
        let s_18_0: u8 = fn_state.size;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 2u16);
        // C s_18_2: const #3u : u8
        let s_18_2: u8 = 3;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 2u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // N s_18_5: branch s_18_4 b21 b19
        if s_18_4 {
            return block_21(state, tracer, fn_state);
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
        // D s_19_1: write-var regsize <= s_19_0
        fn_state.regsize = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var is_signed <= s_20_0
        fn_state.is_signed = s_20_0;
        // N s_20_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #64s : i64
        let s_21_0: i64 = 64;
        // D s_21_1: write-var regsize <= s_21_0
        fn_state.regsize = s_21_0;
        // N s_21_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u32
        let s_22_0: u32 = 0;
        // D s_22_1: write-var memop <= s_22_0
        fn_state.memop = s_22_0;
        // N s_22_2: jump b18
        return block_18(state, tracer, fn_state);
    }
}

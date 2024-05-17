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
use execute_aarch64_instrs_memory_single_general_immediate_signed_post_idx::*;
use ConstrainUnpredictable::*;
use EndOfInstruction::*;
use common::*;
pub fn decode_ldr_imm_gen_aarch64_instrs_memory_single_general_immediate_signed_post_idx<
    T: Tracer,
>(state: &mut State, tracer: &T, Rt: u8, Rn: u8, imm9: u16, opc: u8, size: u8) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_161918: bool,
        t: i64,
        gs_161956: bool,
        n: i64,
        memop: u32,
        gs_161936: bool,
        gs_161945: bool,
        cshadow_1661: u32,
        gs_161939: bool,
        gs_161941: bool,
        c: u32,
        gs_161957: bool,
        scale: i64,
        is_signed: bool,
        gs_161947: bool,
        tagchecked: bool,
        gs_161937: bool,
        gs_161940: bool,
        gs_161946: bool,
        gs_161958: bool,
        gs_161943: bool,
        regsize: i64,
        offset: u64,
        wback: bool,
        datasize: i64,
        rt_unknown: bool,
        gs_161935: bool,
        wb_unknown: bool,
        regsizeshadow_1660: i64,
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
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // D s_0_1: write-var wback <= s_0_0
        fn_state.wback = s_0_0;
        // D s_0_2: read-var size:u8
        let s_0_2: u8 = fn_state.size;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 2u16);
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (s_0_3.value() as i128);
        // D s_0_5: cast reint s_0_4 -> i64
        let s_0_5: i64 = (s_0_4 as i64);
        // D s_0_6: write-var scale <= s_0_5
        fn_state.scale = s_0_5;
        // C s_0_7: const #64s : i
        let s_0_7: i128 = 64;
        // D s_0_8: read-var imm9:u9
        let s_0_8: u16 = fn_state.imm9;
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 9u16);
        // D s_0_10: bits-cast sx s_0_9 -> bv length s_0_7
        let s_0_10: Bits = s_0_9.sign_extend(s_0_7);
        // D s_0_11: cast reint s_0_10 -> u64
        let s_0_11: u64 = (s_0_10.value() as u64);
        // D s_0_12: write-var offset <= s_0_11
        fn_state.offset = s_0_11;
        // D s_0_13: read-var Rn:u8
        let s_0_13: u8 = fn_state.Rn;
        // D s_0_14: cast zx s_0_13 -> bv
        let s_0_14: Bits = Bits::new(s_0_13 as u128, 5u16);
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (s_0_14.value() as i128);
        // D s_0_16: cast reint s_0_15 -> i64
        let s_0_16: i64 = (s_0_15 as i64);
        // D s_0_17: write-var n <= s_0_16
        fn_state.n = s_0_16;
        // D s_0_18: read-var Rt:u8
        let s_0_18: u8 = fn_state.Rt;
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 5u16);
        // D s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (s_0_19.value() as i128);
        // D s_0_21: cast reint s_0_20 -> i64
        let s_0_21: i64 = (s_0_20 as i64);
        // D s_0_22: write-var t <= s_0_21
        fn_state.t = s_0_21;
        // C s_0_23: const #32s : i64
        let s_0_23: i64 = 32;
        // D s_0_24: write-var regsize <= s_0_23
        fn_state.regsize = s_0_23;
        // C s_0_25: const #1s : i
        let s_0_25: i128 = 1;
        // D s_0_26: read-var opc:u8
        let s_0_26: u8 = fn_state.opc;
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 2u16);
        // C s_0_28: const #1u : u64
        let s_0_28: u64 = 1;
        // D s_0_29: bit-extract s_0_27 s_0_25 s_0_28
        let s_0_29: Bits = (Bits::new(
            ((s_0_27) >> (s_0_25)).value(),
            u16::try_from(s_0_28).unwrap(),
        ));
        // D s_0_30: cast reint s_0_29 -> u8
        let s_0_30: bool = ((s_0_29.value()) != 0);
        // C s_0_31: const #0s : i
        let s_0_31: i128 = 0;
        // C s_0_32: const #0u : u64
        let s_0_32: u64 = 0;
        // D s_0_33: cast zx s_0_30 -> u64
        let s_0_33: u64 = (s_0_30 as u64);
        // C s_0_34: const #1u : u64
        let s_0_34: u64 = 1;
        // D s_0_35: and s_0_33 s_0_34
        let s_0_35: u64 = ((s_0_33) & (s_0_34));
        // D s_0_36: cmp-eq s_0_35 s_0_34
        let s_0_36: bool = ((s_0_35) == (s_0_34));
        // D s_0_37: lsl s_0_33 s_0_31
        let s_0_37: u64 = s_0_33 << s_0_31;
        // D s_0_38: or s_0_32 s_0_37
        let s_0_38: u64 = ((s_0_32) | (s_0_37));
        // D s_0_39: cmpl s_0_37
        let s_0_39: u64 = !s_0_37;
        // D s_0_40: and s_0_32 s_0_39
        let s_0_40: u64 = ((s_0_32) & (s_0_39));
        // D s_0_41: select s_0_36 s_0_38 s_0_40
        let s_0_41: u64 = if s_0_36 { s_0_38 } else { s_0_40 };
        // D s_0_42: cast trunc s_0_41 -> u8
        let s_0_42: bool = ((s_0_41) != 0);
        // D s_0_43: cast zx s_0_42 -> bv
        let s_0_43: Bits = Bits::new(s_0_42 as u128, 1u16);
        // C s_0_44: const #0u : u8
        let s_0_44: bool = false;
        // C s_0_45: cast zx s_0_44 -> bv
        let s_0_45: Bits = Bits::new(s_0_44 as u128, 1u16);
        // D s_0_46: cmp-eq s_0_43 s_0_45
        let s_0_46: bool = ((s_0_43) == (s_0_45));
        // N s_0_47: branch s_0_46 b76 b1
        if s_0_46 {
            return block_76(state, tracer, fn_state);
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
        // N s_1_5: branch s_1_4 b75 b2
        if s_1_4 {
            return block_75(state, tracer, fn_state);
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
        // N s_2_7: branch s_2_6 b74 b3
        if s_2_6 {
            return block_74(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#161918 <= s_3_0
        fn_state.gs_161918 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#161918:u8
        let s_4_0: bool = fn_state.gs_161918;
        // N s_4_1: branch s_4_0 b73 b5
        if s_4_0 {
            return block_73(state, tracer, fn_state);
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
        // N s_5_22: branch s_5_21 b72 b6
        if s_5_21 {
            return block_72(state, tracer, fn_state);
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
        // D s_8_1: write-var regsizeshadow#1660 <= s_8_0
        fn_state.regsizeshadow_1660 = s_8_0;
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
        // N s_8_9: branch s_8_8 b71 b9
        if s_8_8 {
            return block_71(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#161935 <= s_9_0
        fn_state.gs_161935 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#161935:u8
        let s_10_0: bool = fn_state.gs_161935;
        // D s_10_1: write-var tagchecked <= s_10_0
        fn_state.tagchecked = s_10_0;
        // C s_10_2: const #0u : u8
        let s_10_2: bool = false;
        // D s_10_3: write-var wb_unknown <= s_10_2
        fn_state.wb_unknown = s_10_2;
        // C s_10_4: const #0u : u8
        let s_10_4: bool = false;
        // D s_10_5: write-var rt_unknown <= s_10_4
        fn_state.rt_unknown = s_10_4;
        // D s_10_6: read-var memop:u32
        let s_10_6: u32 = fn_state.memop;
        // C s_10_7: const #0u : u32
        let s_10_7: u32 = 0;
        // D s_10_8: cmp-eq s_10_6 s_10_7
        let s_10_8: bool = ((s_10_6) == (s_10_7));
        // N s_10_9: branch s_10_8 b70 b11
        if s_10_8 {
            return block_70(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#161936 <= s_11_0
        fn_state.gs_161936 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#161936:u8
        let s_12_0: bool = fn_state.gs_161936;
        // N s_12_1: branch s_12_0 b69 b13
        if s_12_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#161937 <= s_13_0
        fn_state.gs_161937 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#161937:u8
        let s_14_0: bool = fn_state.gs_161937;
        // N s_14_1: branch s_14_0 b68 b15
        if s_14_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#161939 <= s_15_0
        fn_state.gs_161939 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#161939:u8
        let s_16_0: bool = fn_state.gs_161939;
        // N s_16_1: branch s_16_0 b49 b17
        if s_16_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var memop:u32
        let s_18_0: u32 = fn_state.memop;
        // C s_18_1: const #1u : u32
        let s_18_1: u32 = 1;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // N s_18_3: branch s_18_2 b48 b19
        if s_18_2 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#161940 <= s_19_0
        fn_state.gs_161940 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#161940:u8
        let s_20_0: bool = fn_state.gs_161940;
        // N s_20_1: branch s_20_0 b47 b21
        if s_20_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#161941 <= s_21_0
        fn_state.gs_161941 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#161941:u8
        let s_22_0: bool = fn_state.gs_161941;
        // N s_22_1: branch s_22_0 b46 b23
        if s_22_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#161943 <= s_23_0
        fn_state.gs_161943 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#161943:u8
        let s_24_0: bool = fn_state.gs_161943;
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
        // N s_25_0: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var datasize:i64
        let s_26_0: i64 = fn_state.datasize;
        // D s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_2: cast reint s_26_1 -> i64
        let s_26_2: i64 = (s_26_1 as i64);
        // D s_26_3: read-var regsizeshadow#1660:i64
        let s_26_3: i64 = fn_state.regsizeshadow_1660;
        // D s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // D s_26_5: cast reint s_26_4 -> i64
        let s_26_5: i64 = (s_26_4 as i64);
        // D s_26_6: read-var memop:u32
        let s_26_6: u32 = fn_state.memop;
        // D s_26_7: read-var n:i64
        let s_26_7: i64 = fn_state.n;
        // C s_26_8: const #0u : u8
        let s_26_8: bool = false;
        // D s_26_9: read-var offset:u64
        let s_26_9: u64 = fn_state.offset;
        // C s_26_10: const #1u : u8
        let s_26_10: bool = true;
        // D s_26_11: read-var rt_unknown:u8
        let s_26_11: bool = fn_state.rt_unknown;
        // D s_26_12: read-var is_signed:u8
        let s_26_12: bool = fn_state.is_signed;
        // D s_26_13: read-var t:i64
        let s_26_13: i64 = fn_state.t;
        // D s_26_14: read-var tagchecked:u8
        let s_26_14: bool = fn_state.tagchecked;
        // D s_26_15: read-var wb_unknown:u8
        let s_26_15: bool = fn_state.wb_unknown;
        // D s_26_16: read-var wback:u8
        let s_26_16: bool = fn_state.wback;
        // D s_26_17: call execute_aarch64_instrs_memory_single_general_immediate_signed_post_idx(s_26_2, s_26_6, s_26_7, s_26_8, s_26_9, s_26_10, s_26_5, s_26_11, s_26_12, s_26_13, s_26_14, s_26_15, s_26_16)
        let s_26_17: () = execute_aarch64_instrs_memory_single_general_immediate_signed_post_idx(
            state,
            tracer,
            s_26_2,
            s_26_6,
            s_26_7,
            s_26_8,
            s_26_9,
            s_26_10,
            s_26_5,
            s_26_11,
            s_26_12,
            s_26_13,
            s_26_14,
            s_26_15,
            s_26_16,
        );
        // N s_26_18: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #2u : u32
        let s_27_0: u32 = 2;
        // S s_27_1: call ConstrainUnpredictable(s_27_0)
        let s_27_1: u32 = ConstrainUnpredictable(state, tracer, s_27_0);
        // D s_27_2: write-var cshadow#1661 <= s_27_1
        fn_state.cshadow_1661 = s_27_1;
        // D s_27_3: read-var cshadow#1661:u32
        let s_27_3: u32 = fn_state.cshadow_1661;
        // C s_27_4: const #0u : u32
        let s_27_4: u32 = 0;
        // D s_27_5: cmp-eq s_27_3 s_27_4
        let s_27_5: bool = ((s_27_3) == (s_27_4));
        // N s_27_6: branch s_27_5 b45 b28
        if s_27_5 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var cshadow#1661:u32
        let s_28_0: u32 = fn_state.cshadow_1661;
        // C s_28_1: const #1u : u32
        let s_28_1: u32 = 1;
        // D s_28_2: cmp-eq s_28_0 s_28_1
        let s_28_2: bool = ((s_28_0) == (s_28_1));
        // N s_28_3: branch s_28_2 b44 b29
        if s_28_2 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var cshadow#1661:u32
        let s_29_0: u32 = fn_state.cshadow_1661;
        // C s_29_1: const #2u : u32
        let s_29_1: u32 = 2;
        // D s_29_2: cmp-eq s_29_0 s_29_1
        let s_29_2: bool = ((s_29_0) == (s_29_1));
        // N s_29_3: branch s_29_2 b43 b30
        if s_29_2 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var cshadow#1661:u32
        let s_30_0: u32 = fn_state.cshadow_1661;
        // C s_30_1: const #4u : u32
        let s_30_1: u32 = 4;
        // D s_30_2: cmp-eq s_30_0 s_30_1
        let s_30_2: bool = ((s_30_0) == (s_30_1));
        // D s_30_3: write-var gs#161945 <= s_30_2
        fn_state.gs_161945 = s_30_2;
        // N s_30_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#161945:u8
        let s_31_0: bool = fn_state.gs_161945;
        // D s_31_1: write-var gs#161946 <= s_31_0
        fn_state.gs_161946 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#161946:u8
        let s_32_0: bool = fn_state.gs_161946;
        // D s_32_1: write-var gs#161947 <= s_32_0
        fn_state.gs_161947 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#161947:u8
        let s_33_0: bool = fn_state.gs_161947;
        // N s_33_1: assert s_33_0
        let s_33_1: () = assert!(s_33_0);
        // C s_33_2: const #0u : u32
        let s_33_2: u32 = 0;
        // D s_33_3: read-var cshadow#1661:u32
        let s_33_3: u32 = fn_state.cshadow_1661;
        // D s_33_4: cmp-eq s_33_2 s_33_3
        let s_33_4: bool = ((s_33_2) == (s_33_3));
        // D s_33_5: not s_33_4
        let s_33_5: bool = !s_33_4;
        // N s_33_6: branch s_33_5 b36 b34
        if s_33_5 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var rt_unknown <= s_34_0
        fn_state.rt_unknown = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u32
        let s_36_0: u32 = 1;
        // D s_36_1: read-var cshadow#1661:u32
        let s_36_1: u32 = fn_state.cshadow_1661;
        // D s_36_2: cmp-eq s_36_0 s_36_1
        let s_36_2: bool = ((s_36_0) == (s_36_1));
        // D s_36_3: not s_36_2
        let s_36_3: bool = !s_36_2;
        // N s_36_4: branch s_36_3 b38 b37
        if s_36_3 {
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
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var rt_unknown <= s_37_0
        fn_state.rt_unknown = s_37_0;
        // N s_37_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #2u : u32
        let s_38_0: u32 = 2;
        // D s_38_1: read-var cshadow#1661:u32
        let s_38_1: u32 = fn_state.cshadow_1661;
        // D s_38_2: cmp-eq s_38_0 s_38_1
        let s_38_2: bool = ((s_38_0) == (s_38_1));
        // D s_38_3: not s_38_2
        let s_38_3: bool = !s_38_2;
        // N s_38_4: branch s_38_3 b40 b39
        if s_38_3 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
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
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #4u : u32
        let s_40_0: u32 = 4;
        // D s_40_1: read-var cshadow#1661:u32
        let s_40_1: u32 = fn_state.cshadow_1661;
        // D s_40_2: cmp-eq s_40_0 s_40_1
        let s_40_2: bool = ((s_40_0) == (s_40_1));
        // D s_40_3: not s_40_2
        let s_40_3: bool = !s_40_2;
        // N s_40_4: branch s_40_3 b42 b41
        if s_40_3 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call EndOfInstruction(s_41_0)
        let s_41_1: () = EndOfInstruction(state, tracer, s_41_0);
        // N s_41_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_42_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#161945 <= s_43_0
        fn_state.gs_161945 = s_43_0;
        // N s_43_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#161946 <= s_44_0
        fn_state.gs_161946 = s_44_0;
        // N s_44_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#161947 <= s_45_0
        fn_state.gs_161947 = s_45_0;
        // N s_45_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #31s : i
        let s_46_0: i128 = 31;
        // D s_46_1: read-var n:i64
        let s_46_1: i64 = fn_state.n;
        // D s_46_2: cast zx s_46_1 -> i
        let s_46_2: i128 = (i128::try_from(s_46_1).unwrap());
        // D s_46_3: call neq_int(s_46_2, s_46_0)
        let s_46_3: bool = neq_int(state, tracer, s_46_2, s_46_0);
        // D s_46_4: write-var gs#161943 <= s_46_3
        fn_state.gs_161943 = s_46_3;
        // N s_46_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var n:i64
        let s_47_0: i64 = fn_state.n;
        // D s_47_1: cast zx s_47_0 -> i
        let s_47_1: i128 = (i128::try_from(s_47_0).unwrap());
        // D s_47_2: read-var t:i64
        let s_47_2: i64 = fn_state.t;
        // D s_47_3: cast zx s_47_2 -> i
        let s_47_3: i128 = (i128::try_from(s_47_2).unwrap());
        // D s_47_4: cmp-eq s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) == (s_47_3));
        // D s_47_5: write-var gs#161941 <= s_47_4
        fn_state.gs_161941 = s_47_4;
        // N s_47_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var wback:u8
        let s_48_0: bool = fn_state.wback;
        // D s_48_1: write-var gs#161940 <= s_48_0
        fn_state.gs_161940 = s_48_0;
        // N s_48_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u32
        let s_49_0: u32 = 1;
        // S s_49_1: call ConstrainUnpredictable(s_49_0)
        let s_49_1: u32 = ConstrainUnpredictable(state, tracer, s_49_0);
        // D s_49_2: write-var c <= s_49_1
        fn_state.c = s_49_1;
        // D s_49_3: read-var c:u32
        let s_49_3: u32 = fn_state.c;
        // C s_49_4: const #11u : u32
        let s_49_4: u32 = 11;
        // D s_49_5: cmp-eq s_49_3 s_49_4
        let s_49_5: bool = ((s_49_3) == (s_49_4));
        // N s_49_6: branch s_49_5 b67 b50
        if s_49_5 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var c:u32
        let s_50_0: u32 = fn_state.c;
        // C s_50_1: const #1u : u32
        let s_50_1: u32 = 1;
        // D s_50_2: cmp-eq s_50_0 s_50_1
        let s_50_2: bool = ((s_50_0) == (s_50_1));
        // N s_50_3: branch s_50_2 b66 b51
        if s_50_2 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var c:u32
        let s_51_0: u32 = fn_state.c;
        // C s_51_1: const #2u : u32
        let s_51_1: u32 = 2;
        // D s_51_2: cmp-eq s_51_0 s_51_1
        let s_51_2: bool = ((s_51_0) == (s_51_1));
        // N s_51_3: branch s_51_2 b65 b52
        if s_51_2 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var c:u32
        let s_52_0: u32 = fn_state.c;
        // C s_52_1: const #4u : u32
        let s_52_1: u32 = 4;
        // D s_52_2: cmp-eq s_52_0 s_52_1
        let s_52_2: bool = ((s_52_0) == (s_52_1));
        // D s_52_3: write-var gs#161956 <= s_52_2
        fn_state.gs_161956 = s_52_2;
        // N s_52_4: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#161956:u8
        let s_53_0: bool = fn_state.gs_161956;
        // D s_53_1: write-var gs#161957 <= s_53_0
        fn_state.gs_161957 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#161957:u8
        let s_54_0: bool = fn_state.gs_161957;
        // D s_54_1: write-var gs#161958 <= s_54_0
        fn_state.gs_161958 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#161958:u8
        let s_55_0: bool = fn_state.gs_161958;
        // N s_55_1: assert s_55_0
        let s_55_1: () = assert!(s_55_0);
        // C s_55_2: const #11u : u32
        let s_55_2: u32 = 11;
        // D s_55_3: read-var c:u32
        let s_55_3: u32 = fn_state.c;
        // D s_55_4: cmp-eq s_55_2 s_55_3
        let s_55_4: bool = ((s_55_2) == (s_55_3));
        // D s_55_5: not s_55_4
        let s_55_5: bool = !s_55_4;
        // N s_55_6: branch s_55_5 b58 b56
        if s_55_5 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var wback <= s_56_0
        fn_state.wback = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_57_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #1u : u32
        let s_58_0: u32 = 1;
        // D s_58_1: read-var c:u32
        let s_58_1: u32 = fn_state.c;
        // D s_58_2: cmp-eq s_58_0 s_58_1
        let s_58_2: bool = ((s_58_0) == (s_58_1));
        // D s_58_3: not s_58_2
        let s_58_3: bool = !s_58_2;
        // N s_58_4: branch s_58_3 b60 b59
        if s_58_3 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #1u : u8
        let s_59_0: bool = true;
        // D s_59_1: write-var wb_unknown <= s_59_0
        fn_state.wb_unknown = s_59_0;
        // N s_59_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #2u : u32
        let s_60_0: u32 = 2;
        // D s_60_1: read-var c:u32
        let s_60_1: u32 = fn_state.c;
        // D s_60_2: cmp-eq s_60_0 s_60_1
        let s_60_2: bool = ((s_60_0) == (s_60_1));
        // D s_60_3: not s_60_2
        let s_60_3: bool = !s_60_2;
        // N s_60_4: branch s_60_3 b62 b61
        if s_60_3 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_61_0: panic
        panic!("{:?}", ());
        // N s_61_1: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #4u : u32
        let s_62_0: u32 = 4;
        // D s_62_1: read-var c:u32
        let s_62_1: u32 = fn_state.c;
        // D s_62_2: cmp-eq s_62_0 s_62_1
        let s_62_2: bool = ((s_62_0) == (s_62_1));
        // D s_62_3: not s_62_2
        let s_62_3: bool = !s_62_2;
        // N s_62_4: branch s_62_3 b64 b63
        if s_62_3 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #() : ()
        let s_63_0: () = ();
        // S s_63_1: call EndOfInstruction(s_63_0)
        let s_63_1: () = EndOfInstruction(state, tracer, s_63_0);
        // N s_63_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_64_0: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#161956 <= s_65_0
        fn_state.gs_161956 = s_65_0;
        // N s_65_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #1u : u8
        let s_66_0: bool = true;
        // D s_66_1: write-var gs#161957 <= s_66_0
        fn_state.gs_161957 = s_66_0;
        // N s_66_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // D s_67_1: write-var gs#161958 <= s_67_0
        fn_state.gs_161958 = s_67_0;
        // N s_67_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #31s : i
        let s_68_0: i128 = 31;
        // D s_68_1: read-var n:i64
        let s_68_1: i64 = fn_state.n;
        // D s_68_2: cast zx s_68_1 -> i
        let s_68_2: i128 = (i128::try_from(s_68_1).unwrap());
        // D s_68_3: call neq_int(s_68_2, s_68_0)
        let s_68_3: bool = neq_int(state, tracer, s_68_2, s_68_0);
        // D s_68_4: write-var gs#161939 <= s_68_3
        fn_state.gs_161939 = s_68_3;
        // N s_68_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var n:i64
        let s_69_0: i64 = fn_state.n;
        // D s_69_1: cast zx s_69_0 -> i
        let s_69_1: i128 = (i128::try_from(s_69_0).unwrap());
        // D s_69_2: read-var t:i64
        let s_69_2: i64 = fn_state.t;
        // D s_69_3: cast zx s_69_2 -> i
        let s_69_3: i128 = (i128::try_from(s_69_2).unwrap());
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#161937 <= s_69_4
        fn_state.gs_161937 = s_69_4;
        // N s_69_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: write-var gs#161936 <= s_70_0
        fn_state.gs_161936 = s_70_0;
        // N s_70_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var gs#161935 <= s_71_0
        fn_state.gs_161935 = s_71_0;
        // N s_71_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #32s : i64
        let s_72_0: i64 = 32;
        // D s_72_1: write-var regsize <= s_72_0
        fn_state.regsize = s_72_0;
        // N s_72_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_73_0: panic
        panic!("{:?}", ());
        // N s_73_1: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #0s : i
        let s_74_0: i128 = 0;
        // D s_74_1: read-var opc:u8
        let s_74_1: u8 = fn_state.opc;
        // D s_74_2: cast zx s_74_1 -> bv
        let s_74_2: Bits = Bits::new(s_74_1 as u128, 2u16);
        // C s_74_3: const #1u : u64
        let s_74_3: u64 = 1;
        // D s_74_4: bit-extract s_74_2 s_74_0 s_74_3
        let s_74_4: Bits = (Bits::new(
            ((s_74_2) >> (s_74_0)).value(),
            u16::try_from(s_74_3).unwrap(),
        ));
        // D s_74_5: cast reint s_74_4 -> u8
        let s_74_5: bool = ((s_74_4.value()) != 0);
        // C s_74_6: const #0s : i
        let s_74_6: i128 = 0;
        // C s_74_7: const #0u : u64
        let s_74_7: u64 = 0;
        // D s_74_8: cast zx s_74_5 -> u64
        let s_74_8: u64 = (s_74_5 as u64);
        // C s_74_9: const #1u : u64
        let s_74_9: u64 = 1;
        // D s_74_10: and s_74_8 s_74_9
        let s_74_10: u64 = ((s_74_8) & (s_74_9));
        // D s_74_11: cmp-eq s_74_10 s_74_9
        let s_74_11: bool = ((s_74_10) == (s_74_9));
        // D s_74_12: lsl s_74_8 s_74_6
        let s_74_12: u64 = s_74_8 << s_74_6;
        // D s_74_13: or s_74_7 s_74_12
        let s_74_13: u64 = ((s_74_7) | (s_74_12));
        // D s_74_14: cmpl s_74_12
        let s_74_14: u64 = !s_74_12;
        // D s_74_15: and s_74_7 s_74_14
        let s_74_15: u64 = ((s_74_7) & (s_74_14));
        // D s_74_16: select s_74_11 s_74_13 s_74_15
        let s_74_16: u64 = if s_74_11 { s_74_13 } else { s_74_15 };
        // D s_74_17: cast trunc s_74_16 -> u8
        let s_74_17: bool = ((s_74_16) != 0);
        // D s_74_18: cast zx s_74_17 -> bv
        let s_74_18: Bits = Bits::new(s_74_17 as u128, 1u16);
        // C s_74_19: const #1u : u8
        let s_74_19: bool = true;
        // C s_74_20: cast zx s_74_19 -> bv
        let s_74_20: Bits = Bits::new(s_74_19 as u128, 1u16);
        // D s_74_21: cmp-eq s_74_18 s_74_20
        let s_74_21: bool = ((s_74_18) == (s_74_20));
        // D s_74_22: write-var gs#161918 <= s_74_21
        fn_state.gs_161918 = s_74_21;
        // N s_74_23: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_75_0: panic
        panic!("{:?}", ());
        // N s_75_1: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #0s : i
        let s_76_0: i128 = 0;
        // D s_76_1: read-var opc:u8
        let s_76_1: u8 = fn_state.opc;
        // D s_76_2: cast zx s_76_1 -> bv
        let s_76_2: Bits = Bits::new(s_76_1 as u128, 2u16);
        // C s_76_3: const #1u : u64
        let s_76_3: u64 = 1;
        // D s_76_4: bit-extract s_76_2 s_76_0 s_76_3
        let s_76_4: Bits = (Bits::new(
            ((s_76_2) >> (s_76_0)).value(),
            u16::try_from(s_76_3).unwrap(),
        ));
        // D s_76_5: cast reint s_76_4 -> u8
        let s_76_5: bool = ((s_76_4.value()) != 0);
        // C s_76_6: const #0s : i
        let s_76_6: i128 = 0;
        // C s_76_7: const #0u : u64
        let s_76_7: u64 = 0;
        // D s_76_8: cast zx s_76_5 -> u64
        let s_76_8: u64 = (s_76_5 as u64);
        // C s_76_9: const #1u : u64
        let s_76_9: u64 = 1;
        // D s_76_10: and s_76_8 s_76_9
        let s_76_10: u64 = ((s_76_8) & (s_76_9));
        // D s_76_11: cmp-eq s_76_10 s_76_9
        let s_76_11: bool = ((s_76_10) == (s_76_9));
        // D s_76_12: lsl s_76_8 s_76_6
        let s_76_12: u64 = s_76_8 << s_76_6;
        // D s_76_13: or s_76_7 s_76_12
        let s_76_13: u64 = ((s_76_7) | (s_76_12));
        // D s_76_14: cmpl s_76_12
        let s_76_14: u64 = !s_76_12;
        // D s_76_15: and s_76_7 s_76_14
        let s_76_15: u64 = ((s_76_7) & (s_76_14));
        // D s_76_16: select s_76_11 s_76_13 s_76_15
        let s_76_16: u64 = if s_76_11 { s_76_13 } else { s_76_15 };
        // D s_76_17: cast trunc s_76_16 -> u8
        let s_76_17: bool = ((s_76_16) != 0);
        // D s_76_18: cast zx s_76_17 -> bv
        let s_76_18: Bits = Bits::new(s_76_17 as u128, 1u16);
        // C s_76_19: const #1u : u8
        let s_76_19: bool = true;
        // C s_76_20: cast zx s_76_19 -> bv
        let s_76_20: Bits = Bits::new(s_76_19 as u128, 1u16);
        // D s_76_21: cmp-eq s_76_18 s_76_20
        let s_76_21: bool = ((s_76_18) == (s_76_20));
        // N s_76_22: branch s_76_21 b82 b77
        if s_76_21 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #1u : u32
        let s_77_0: u32 = 1;
        // D s_77_1: write-var memop <= s_77_0
        fn_state.memop = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var size:u8
        let s_78_0: u8 = fn_state.size;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 2u16);
        // C s_78_2: const #3u : u8
        let s_78_2: u8 = 3;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 2u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // N s_78_5: branch s_78_4 b81 b79
        if s_78_4 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #32s : i64
        let s_79_0: i64 = 32;
        // D s_79_1: write-var regsize <= s_79_0
        fn_state.regsize = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #0u : u8
        let s_80_0: bool = false;
        // D s_80_1: write-var is_signed <= s_80_0
        fn_state.is_signed = s_80_0;
        // N s_80_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #64s : i64
        let s_81_0: i64 = 64;
        // D s_81_1: write-var regsize <= s_81_0
        fn_state.regsize = s_81_0;
        // N s_81_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #0u : u32
        let s_82_0: u32 = 0;
        // D s_82_1: write-var memop <= s_82_0
        fn_state.memop = s_82_0;
        // N s_82_2: jump b78
        return block_78(state, tracer, fn_state);
    }
}

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
use decode_aarch32_instrs_LDR_r_A1enc_A_txt::*;
use decode_aarch32_instrs_STRT_A2enc_A_txt::*;
use decode_aarch32_instrs_LDRB_r_A1enc_A_txt::*;
use decode_aarch32_instrs_STR_r_A1enc_A_txt::*;
use decode_aarch32_instrs_STRBT_A2enc_A_txt::*;
use decode_aarch32_instrs_LDRT_A2enc_A_txt::*;
use decode_aarch32_instrs_LDRBT_A2enc_A_txt::*;
use decode_aarch32_instrs_STRB_r_A1enc_A_txt::*;
use common::*;
pub fn u__DecodeA32_LoadStoreReg<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_408817: i128,
    gs_408818: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_408962: bool,
        gs_408930: bool,
        gs_409030: bool,
        gs_409003: bool,
        u_33351: u32,
        gs_408969: bool,
        gs_408963: bool,
        gs_409065: bool,
        gs_408895: bool,
        gs_408868: bool,
        u_33391: u32,
        u_33371: u32,
        gs_408897: bool,
        gs_409069: bool,
        gs_408834: bool,
        gs_408829: bool,
        gs_408902: bool,
        u_33400: u32,
        gs_409031: bool,
        gs_408997: bool,
        gs_408864: bool,
        gs_409029: bool,
        u__opcode: u32,
        merge_var: ProductType7b8639ca40b2f578,
        gs_409002: bool,
        gs_408835: bool,
        gs_408863: bool,
        gs_408901: bool,
        gs_409036: bool,
        gs_408828: bool,
        u_33380: u32,
        gs_408896: bool,
        gs_408968: bool,
        gs_408964: bool,
        gs_408869: bool,
        gs_409070: bool,
        gs_409064: bool,
        gs_409035: bool,
        gs_408830: bool,
        gs_408931: bool,
        gs_408936: bool,
        u_33411: u32,
        u_33360: u32,
        gs_408935: bool,
        gs_408998: bool,
        gs_408817: i128,
        gs_408818: u32,
    }
    let fn_state = FunctionState {
        gs_408817,
        gs_408818,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var gs#408817:i
        let s_0_0: i128 = fn_state.gs_408817;
        // D s_0_1: write-var merge#var.0 <= s_0_0
        fn_state.merge_var._0 = s_0_0;
        // D s_0_2: read-var gs#408818:u32
        let s_0_2: u32 = fn_state.gs_408818;
        // D s_0_3: write-var merge#var.1 <= s_0_2
        fn_state.merge_var._1 = s_0_2;
        // D s_0_4: read-var merge#var.1:struct
        let s_0_4: u32 = fn_state.merge_var._1;
        // D s_0_5: write-var __opcode <= s_0_4
        fn_state.u__opcode = s_0_4;
        // C s_0_6: const #25s : i
        let s_0_6: i128 = 25;
        // D s_0_7: read-var __opcode:u32
        let s_0_7: u32 = fn_state.u__opcode;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 32u16);
        // C s_0_9: const #1s : i64
        let s_0_9: i64 = 1;
        // C s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (i128::try_from(s_0_9).unwrap());
        // C s_0_11: const #2s : i
        let s_0_11: i128 = 2;
        // C s_0_12: add s_0_11 s_0_10
        let s_0_12: i128 = (s_0_11 + s_0_10);
        // D s_0_13: bit-extract s_0_8 s_0_6 s_0_12
        let s_0_13: Bits = (Bits::new(
            ((s_0_8) >> (s_0_6)).value(),
            u16::try_from(s_0_12).unwrap(),
        ));
        // D s_0_14: cast reint s_0_13 -> u8
        let s_0_14: u8 = (s_0_13.value() as u8);
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 3u16);
        // C s_0_16: const #3u : u8
        let s_0_16: u8 = 3;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 3u16);
        // D s_0_18: cmp-eq s_0_15 s_0_17
        let s_0_18: bool = ((s_0_15) == (s_0_17));
        // N s_0_19: branch s_0_18 b118 b1
        if s_0_18 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#408830 <= s_1_0
        fn_state.gs_408830 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#408830:u8
        let s_2_0: bool = fn_state.gs_408830;
        // N s_2_1: branch s_2_0 b114 b3
        if s_2_0 {
            return block_114(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#408835 <= s_3_0
        fn_state.gs_408835 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#408835:u8
        let s_4_0: bool = fn_state.gs_408835;
        // D s_4_1: not s_4_0
        let s_4_1: bool = !s_4_0;
        // N s_4_2: branch s_4_1 b6 b5
        if s_4_1 {
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
        // C s_5_0: const #2929s : i
        let s_5_0: i128 = 2929;
        // C s_5_1: const #14696u : u32
        let s_5_1: u32 = 14696;
        // N s_5_2: write-reg s_5_1 <= s_5_0
        let s_5_2: () = {
            state.write_register::<i128>(s_5_1 as isize, s_5_0);
            tracer.write_register(s_5_1 as isize, s_5_0);
        };
        // C s_5_3: const #28s : i
        let s_5_3: i128 = 28;
        // C s_5_4: const #4s : i
        let s_5_4: i128 = 4;
        // D s_5_5: read-var __opcode:u32
        let s_5_5: u32 = fn_state.u__opcode;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 32u16);
        // D s_5_7: bit-extract s_5_6 s_5_3 s_5_4
        let s_5_7: Bits = (Bits::new(
            ((s_5_6) >> (s_5_3)).value(),
            u16::try_from(s_5_4).unwrap(),
        ));
        // D s_5_8: cast reint s_5_7 -> u8
        let s_5_8: u8 = (s_5_7.value() as u8);
        // C s_5_9: const #24s : i
        let s_5_9: i128 = 24;
        // C s_5_10: const #1s : i
        let s_5_10: i128 = 1;
        // D s_5_11: read-var __opcode:u32
        let s_5_11: u32 = fn_state.u__opcode;
        // D s_5_12: cast zx s_5_11 -> bv
        let s_5_12: Bits = Bits::new(s_5_11 as u128, 32u16);
        // D s_5_13: bit-extract s_5_12 s_5_9 s_5_10
        let s_5_13: Bits = (Bits::new(
            ((s_5_12) >> (s_5_9)).value(),
            u16::try_from(s_5_10).unwrap(),
        ));
        // D s_5_14: cast reint s_5_13 -> u8
        let s_5_14: bool = ((s_5_13.value()) != 0);
        // C s_5_15: const #23s : i
        let s_5_15: i128 = 23;
        // C s_5_16: const #1s : i
        let s_5_16: i128 = 1;
        // D s_5_17: read-var __opcode:u32
        let s_5_17: u32 = fn_state.u__opcode;
        // D s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 32u16);
        // D s_5_19: bit-extract s_5_18 s_5_15 s_5_16
        let s_5_19: Bits = (Bits::new(
            ((s_5_18) >> (s_5_15)).value(),
            u16::try_from(s_5_16).unwrap(),
        ));
        // D s_5_20: cast reint s_5_19 -> u8
        let s_5_20: bool = ((s_5_19.value()) != 0);
        // C s_5_21: const #21s : i
        let s_5_21: i128 = 21;
        // C s_5_22: const #1s : i
        let s_5_22: i128 = 1;
        // D s_5_23: read-var __opcode:u32
        let s_5_23: u32 = fn_state.u__opcode;
        // D s_5_24: cast zx s_5_23 -> bv
        let s_5_24: Bits = Bits::new(s_5_23 as u128, 32u16);
        // D s_5_25: bit-extract s_5_24 s_5_21 s_5_22
        let s_5_25: Bits = (Bits::new(
            ((s_5_24) >> (s_5_21)).value(),
            u16::try_from(s_5_22).unwrap(),
        ));
        // D s_5_26: cast reint s_5_25 -> u8
        let s_5_26: bool = ((s_5_25.value()) != 0);
        // C s_5_27: const #16s : i
        let s_5_27: i128 = 16;
        // C s_5_28: const #4s : i
        let s_5_28: i128 = 4;
        // D s_5_29: read-var __opcode:u32
        let s_5_29: u32 = fn_state.u__opcode;
        // D s_5_30: cast zx s_5_29 -> bv
        let s_5_30: Bits = Bits::new(s_5_29 as u128, 32u16);
        // D s_5_31: bit-extract s_5_30 s_5_27 s_5_28
        let s_5_31: Bits = (Bits::new(
            ((s_5_30) >> (s_5_27)).value(),
            u16::try_from(s_5_28).unwrap(),
        ));
        // D s_5_32: cast reint s_5_31 -> u8
        let s_5_32: u8 = (s_5_31.value() as u8);
        // C s_5_33: const #12s : i
        let s_5_33: i128 = 12;
        // C s_5_34: const #4s : i
        let s_5_34: i128 = 4;
        // D s_5_35: read-var __opcode:u32
        let s_5_35: u32 = fn_state.u__opcode;
        // D s_5_36: cast zx s_5_35 -> bv
        let s_5_36: Bits = Bits::new(s_5_35 as u128, 32u16);
        // D s_5_37: bit-extract s_5_36 s_5_33 s_5_34
        let s_5_37: Bits = (Bits::new(
            ((s_5_36) >> (s_5_33)).value(),
            u16::try_from(s_5_34).unwrap(),
        ));
        // D s_5_38: cast reint s_5_37 -> u8
        let s_5_38: u8 = (s_5_37.value() as u8);
        // C s_5_39: const #7s : i
        let s_5_39: i128 = 7;
        // C s_5_40: const #5s : i
        let s_5_40: i128 = 5;
        // D s_5_41: read-var __opcode:u32
        let s_5_41: u32 = fn_state.u__opcode;
        // D s_5_42: cast zx s_5_41 -> bv
        let s_5_42: Bits = Bits::new(s_5_41 as u128, 32u16);
        // D s_5_43: bit-extract s_5_42 s_5_39 s_5_40
        let s_5_43: Bits = (Bits::new(
            ((s_5_42) >> (s_5_39)).value(),
            u16::try_from(s_5_40).unwrap(),
        ));
        // D s_5_44: cast reint s_5_43 -> u8
        let s_5_44: u8 = (s_5_43.value() as u8);
        // C s_5_45: const #5s : i
        let s_5_45: i128 = 5;
        // C s_5_46: const #2s : i
        let s_5_46: i128 = 2;
        // D s_5_47: read-var __opcode:u32
        let s_5_47: u32 = fn_state.u__opcode;
        // D s_5_48: cast zx s_5_47 -> bv
        let s_5_48: Bits = Bits::new(s_5_47 as u128, 32u16);
        // D s_5_49: bit-extract s_5_48 s_5_45 s_5_46
        let s_5_49: Bits = (Bits::new(
            ((s_5_48) >> (s_5_45)).value(),
            u16::try_from(s_5_46).unwrap(),
        ));
        // D s_5_50: cast reint s_5_49 -> u8
        let s_5_50: u8 = (s_5_49.value() as u8);
        // C s_5_51: const #0s : i
        let s_5_51: i128 = 0;
        // C s_5_52: const #4s : i
        let s_5_52: i128 = 4;
        // D s_5_53: read-var __opcode:u32
        let s_5_53: u32 = fn_state.u__opcode;
        // D s_5_54: cast zx s_5_53 -> bv
        let s_5_54: Bits = Bits::new(s_5_53 as u128, 32u16);
        // D s_5_55: bit-extract s_5_54 s_5_51 s_5_52
        let s_5_55: Bits = (Bits::new(
            ((s_5_54) >> (s_5_51)).value(),
            u16::try_from(s_5_52).unwrap(),
        ));
        // D s_5_56: cast reint s_5_55 -> u8
        let s_5_56: u8 = (s_5_55.value() as u8);
        // D s_5_57: call decode_aarch32_instrs_LDRB_r_A1enc_A_txt(s_5_8, s_5_14, s_5_20, s_5_26, s_5_32, s_5_38, s_5_44, s_5_50, s_5_56)
        let s_5_57: () = decode_aarch32_instrs_LDRB_r_A1enc_A_txt(
            state,
            tracer,
            s_5_8,
            s_5_14,
            s_5_20,
            s_5_26,
            s_5_32,
            s_5_38,
            s_5_44,
            s_5_50,
            s_5_56,
        );
        // N s_5_58: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var merge#var.1:struct
        let s_6_0: u32 = fn_state.merge_var._1;
        // D s_6_1: write-var u#33351 <= s_6_0
        fn_state.u_33351 = s_6_0;
        // C s_6_2: const #24s : i
        let s_6_2: i128 = 24;
        // D s_6_3: read-var u#33351:u32
        let s_6_3: u32 = fn_state.u_33351;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 32u16);
        // C s_6_5: const #1s : i64
        let s_6_5: i64 = 1;
        // C s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // C s_6_7: const #3s : i
        let s_6_7: i128 = 3;
        // C s_6_8: add s_6_7 s_6_6
        let s_6_8: i128 = (s_6_7 + s_6_6);
        // D s_6_9: bit-extract s_6_4 s_6_2 s_6_8
        let s_6_9: Bits = (Bits::new(
            ((s_6_4) >> (s_6_2)).value(),
            u16::try_from(s_6_8).unwrap(),
        ));
        // D s_6_10: cast reint s_6_9 -> u8
        let s_6_10: u8 = (s_6_9.value() as u8);
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 4u16);
        // C s_6_12: const #6u : u8
        let s_6_12: u8 = 6;
        // C s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 4u16);
        // D s_6_14: cmp-eq s_6_11 s_6_13
        let s_6_14: bool = ((s_6_11) == (s_6_13));
        // N s_6_15: branch s_6_14 b110 b7
        if s_6_14 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#408864 <= s_7_0
        fn_state.gs_408864 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#408864:u8
        let s_8_0: bool = fn_state.gs_408864;
        // N s_8_1: branch s_8_0 b106 b9
        if s_8_0 {
            return block_106(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#408869 <= s_9_0
        fn_state.gs_408869 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#408869:u8
        let s_10_0: bool = fn_state.gs_408869;
        // D s_10_1: not s_10_0
        let s_10_1: bool = !s_10_0;
        // N s_10_2: branch s_10_1 b12 b11
        if s_10_1 {
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
        // C s_11_0: const #2933s : i
        let s_11_0: i128 = 2933;
        // C s_11_1: const #14696u : u32
        let s_11_1: u32 = 14696;
        // N s_11_2: write-reg s_11_1 <= s_11_0
        let s_11_2: () = {
            state.write_register::<i128>(s_11_1 as isize, s_11_0);
            tracer.write_register(s_11_1 as isize, s_11_0);
        };
        // C s_11_3: const #28s : i
        let s_11_3: i128 = 28;
        // C s_11_4: const #4s : i
        let s_11_4: i128 = 4;
        // D s_11_5: read-var u#33351:u32
        let s_11_5: u32 = fn_state.u_33351;
        // D s_11_6: cast zx s_11_5 -> bv
        let s_11_6: Bits = Bits::new(s_11_5 as u128, 32u16);
        // D s_11_7: bit-extract s_11_6 s_11_3 s_11_4
        let s_11_7: Bits = (Bits::new(
            ((s_11_6) >> (s_11_3)).value(),
            u16::try_from(s_11_4).unwrap(),
        ));
        // D s_11_8: cast reint s_11_7 -> u8
        let s_11_8: u8 = (s_11_7.value() as u8);
        // C s_11_9: const #23s : i
        let s_11_9: i128 = 23;
        // C s_11_10: const #1s : i
        let s_11_10: i128 = 1;
        // D s_11_11: read-var u#33351:u32
        let s_11_11: u32 = fn_state.u_33351;
        // D s_11_12: cast zx s_11_11 -> bv
        let s_11_12: Bits = Bits::new(s_11_11 as u128, 32u16);
        // D s_11_13: bit-extract s_11_12 s_11_9 s_11_10
        let s_11_13: Bits = (Bits::new(
            ((s_11_12) >> (s_11_9)).value(),
            u16::try_from(s_11_10).unwrap(),
        ));
        // D s_11_14: cast reint s_11_13 -> u8
        let s_11_14: bool = ((s_11_13.value()) != 0);
        // C s_11_15: const #16s : i
        let s_11_15: i128 = 16;
        // C s_11_16: const #4s : i
        let s_11_16: i128 = 4;
        // D s_11_17: read-var u#33351:u32
        let s_11_17: u32 = fn_state.u_33351;
        // D s_11_18: cast zx s_11_17 -> bv
        let s_11_18: Bits = Bits::new(s_11_17 as u128, 32u16);
        // D s_11_19: bit-extract s_11_18 s_11_15 s_11_16
        let s_11_19: Bits = (Bits::new(
            ((s_11_18) >> (s_11_15)).value(),
            u16::try_from(s_11_16).unwrap(),
        ));
        // D s_11_20: cast reint s_11_19 -> u8
        let s_11_20: u8 = (s_11_19.value() as u8);
        // C s_11_21: const #12s : i
        let s_11_21: i128 = 12;
        // C s_11_22: const #4s : i
        let s_11_22: i128 = 4;
        // D s_11_23: read-var u#33351:u32
        let s_11_23: u32 = fn_state.u_33351;
        // D s_11_24: cast zx s_11_23 -> bv
        let s_11_24: Bits = Bits::new(s_11_23 as u128, 32u16);
        // D s_11_25: bit-extract s_11_24 s_11_21 s_11_22
        let s_11_25: Bits = (Bits::new(
            ((s_11_24) >> (s_11_21)).value(),
            u16::try_from(s_11_22).unwrap(),
        ));
        // D s_11_26: cast reint s_11_25 -> u8
        let s_11_26: u8 = (s_11_25.value() as u8);
        // C s_11_27: const #7s : i
        let s_11_27: i128 = 7;
        // C s_11_28: const #5s : i
        let s_11_28: i128 = 5;
        // D s_11_29: read-var u#33351:u32
        let s_11_29: u32 = fn_state.u_33351;
        // D s_11_30: cast zx s_11_29 -> bv
        let s_11_30: Bits = Bits::new(s_11_29 as u128, 32u16);
        // D s_11_31: bit-extract s_11_30 s_11_27 s_11_28
        let s_11_31: Bits = (Bits::new(
            ((s_11_30) >> (s_11_27)).value(),
            u16::try_from(s_11_28).unwrap(),
        ));
        // D s_11_32: cast reint s_11_31 -> u8
        let s_11_32: u8 = (s_11_31.value() as u8);
        // C s_11_33: const #5s : i
        let s_11_33: i128 = 5;
        // C s_11_34: const #2s : i
        let s_11_34: i128 = 2;
        // D s_11_35: read-var u#33351:u32
        let s_11_35: u32 = fn_state.u_33351;
        // D s_11_36: cast zx s_11_35 -> bv
        let s_11_36: Bits = Bits::new(s_11_35 as u128, 32u16);
        // D s_11_37: bit-extract s_11_36 s_11_33 s_11_34
        let s_11_37: Bits = (Bits::new(
            ((s_11_36) >> (s_11_33)).value(),
            u16::try_from(s_11_34).unwrap(),
        ));
        // D s_11_38: cast reint s_11_37 -> u8
        let s_11_38: u8 = (s_11_37.value() as u8);
        // C s_11_39: const #0s : i
        let s_11_39: i128 = 0;
        // C s_11_40: const #4s : i
        let s_11_40: i128 = 4;
        // D s_11_41: read-var u#33351:u32
        let s_11_41: u32 = fn_state.u_33351;
        // D s_11_42: cast zx s_11_41 -> bv
        let s_11_42: Bits = Bits::new(s_11_41 as u128, 32u16);
        // D s_11_43: bit-extract s_11_42 s_11_39 s_11_40
        let s_11_43: Bits = (Bits::new(
            ((s_11_42) >> (s_11_39)).value(),
            u16::try_from(s_11_40).unwrap(),
        ));
        // D s_11_44: cast reint s_11_43 -> u8
        let s_11_44: u8 = (s_11_43.value() as u8);
        // D s_11_45: call decode_aarch32_instrs_LDRBT_A2enc_A_txt(s_11_8, s_11_14, s_11_20, s_11_26, s_11_32, s_11_38, s_11_44)
        let s_11_45: () = decode_aarch32_instrs_LDRBT_A2enc_A_txt(
            state,
            tracer,
            s_11_8,
            s_11_14,
            s_11_20,
            s_11_26,
            s_11_32,
            s_11_38,
            s_11_44,
        );
        // N s_11_46: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var merge#var.1:struct
        let s_12_0: u32 = fn_state.merge_var._1;
        // D s_12_1: write-var u#33360 <= s_12_0
        fn_state.u_33360 = s_12_0;
        // C s_12_2: const #25s : i
        let s_12_2: i128 = 25;
        // D s_12_3: read-var u#33360:u32
        let s_12_3: u32 = fn_state.u_33360;
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 32u16);
        // C s_12_5: const #1s : i64
        let s_12_5: i64 = 1;
        // C s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // C s_12_7: const #2s : i
        let s_12_7: i128 = 2;
        // C s_12_8: add s_12_7 s_12_6
        let s_12_8: i128 = (s_12_7 + s_12_6);
        // D s_12_9: bit-extract s_12_4 s_12_2 s_12_8
        let s_12_9: Bits = (Bits::new(
            ((s_12_4) >> (s_12_2)).value(),
            u16::try_from(s_12_8).unwrap(),
        ));
        // D s_12_10: cast reint s_12_9 -> u8
        let s_12_10: u8 = (s_12_9.value() as u8);
        // D s_12_11: cast zx s_12_10 -> bv
        let s_12_11: Bits = Bits::new(s_12_10 as u128, 3u16);
        // C s_12_12: const #3u : u8
        let s_12_12: u8 = 3;
        // C s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 3u16);
        // D s_12_14: cmp-eq s_12_11 s_12_13
        let s_12_14: bool = ((s_12_11) == (s_12_13));
        // N s_12_15: branch s_12_14 b99 b13
        if s_12_14 {
            return block_99(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#408897 <= s_13_0
        fn_state.gs_408897 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#408897:u8
        let s_14_0: bool = fn_state.gs_408897;
        // N s_14_1: branch s_14_0 b95 b15
        if s_14_0 {
            return block_95(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#408902 <= s_15_0
        fn_state.gs_408902 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#408902:u8
        let s_16_0: bool = fn_state.gs_408902;
        // D s_16_1: not s_16_0
        let s_16_1: bool = !s_16_0;
        // N s_16_2: branch s_16_1 b18 b17
        if s_16_1 {
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
        // C s_17_0: const #2968s : i
        let s_17_0: i128 = 2968;
        // C s_17_1: const #14696u : u32
        let s_17_1: u32 = 14696;
        // N s_17_2: write-reg s_17_1 <= s_17_0
        let s_17_2: () = {
            state.write_register::<i128>(s_17_1 as isize, s_17_0);
            tracer.write_register(s_17_1 as isize, s_17_0);
        };
        // C s_17_3: const #28s : i
        let s_17_3: i128 = 28;
        // C s_17_4: const #4s : i
        let s_17_4: i128 = 4;
        // D s_17_5: read-var u#33360:u32
        let s_17_5: u32 = fn_state.u_33360;
        // D s_17_6: cast zx s_17_5 -> bv
        let s_17_6: Bits = Bits::new(s_17_5 as u128, 32u16);
        // D s_17_7: bit-extract s_17_6 s_17_3 s_17_4
        let s_17_7: Bits = (Bits::new(
            ((s_17_6) >> (s_17_3)).value(),
            u16::try_from(s_17_4).unwrap(),
        ));
        // D s_17_8: cast reint s_17_7 -> u8
        let s_17_8: u8 = (s_17_7.value() as u8);
        // C s_17_9: const #24s : i
        let s_17_9: i128 = 24;
        // C s_17_10: const #1s : i
        let s_17_10: i128 = 1;
        // D s_17_11: read-var u#33360:u32
        let s_17_11: u32 = fn_state.u_33360;
        // D s_17_12: cast zx s_17_11 -> bv
        let s_17_12: Bits = Bits::new(s_17_11 as u128, 32u16);
        // D s_17_13: bit-extract s_17_12 s_17_9 s_17_10
        let s_17_13: Bits = (Bits::new(
            ((s_17_12) >> (s_17_9)).value(),
            u16::try_from(s_17_10).unwrap(),
        ));
        // D s_17_14: cast reint s_17_13 -> u8
        let s_17_14: bool = ((s_17_13.value()) != 0);
        // C s_17_15: const #23s : i
        let s_17_15: i128 = 23;
        // C s_17_16: const #1s : i
        let s_17_16: i128 = 1;
        // D s_17_17: read-var u#33360:u32
        let s_17_17: u32 = fn_state.u_33360;
        // D s_17_18: cast zx s_17_17 -> bv
        let s_17_18: Bits = Bits::new(s_17_17 as u128, 32u16);
        // D s_17_19: bit-extract s_17_18 s_17_15 s_17_16
        let s_17_19: Bits = (Bits::new(
            ((s_17_18) >> (s_17_15)).value(),
            u16::try_from(s_17_16).unwrap(),
        ));
        // D s_17_20: cast reint s_17_19 -> u8
        let s_17_20: bool = ((s_17_19.value()) != 0);
        // C s_17_21: const #21s : i
        let s_17_21: i128 = 21;
        // C s_17_22: const #1s : i
        let s_17_22: i128 = 1;
        // D s_17_23: read-var u#33360:u32
        let s_17_23: u32 = fn_state.u_33360;
        // D s_17_24: cast zx s_17_23 -> bv
        let s_17_24: Bits = Bits::new(s_17_23 as u128, 32u16);
        // D s_17_25: bit-extract s_17_24 s_17_21 s_17_22
        let s_17_25: Bits = (Bits::new(
            ((s_17_24) >> (s_17_21)).value(),
            u16::try_from(s_17_22).unwrap(),
        ));
        // D s_17_26: cast reint s_17_25 -> u8
        let s_17_26: bool = ((s_17_25.value()) != 0);
        // C s_17_27: const #16s : i
        let s_17_27: i128 = 16;
        // C s_17_28: const #4s : i
        let s_17_28: i128 = 4;
        // D s_17_29: read-var u#33360:u32
        let s_17_29: u32 = fn_state.u_33360;
        // D s_17_30: cast zx s_17_29 -> bv
        let s_17_30: Bits = Bits::new(s_17_29 as u128, 32u16);
        // D s_17_31: bit-extract s_17_30 s_17_27 s_17_28
        let s_17_31: Bits = (Bits::new(
            ((s_17_30) >> (s_17_27)).value(),
            u16::try_from(s_17_28).unwrap(),
        ));
        // D s_17_32: cast reint s_17_31 -> u8
        let s_17_32: u8 = (s_17_31.value() as u8);
        // C s_17_33: const #12s : i
        let s_17_33: i128 = 12;
        // C s_17_34: const #4s : i
        let s_17_34: i128 = 4;
        // D s_17_35: read-var u#33360:u32
        let s_17_35: u32 = fn_state.u_33360;
        // D s_17_36: cast zx s_17_35 -> bv
        let s_17_36: Bits = Bits::new(s_17_35 as u128, 32u16);
        // D s_17_37: bit-extract s_17_36 s_17_33 s_17_34
        let s_17_37: Bits = (Bits::new(
            ((s_17_36) >> (s_17_33)).value(),
            u16::try_from(s_17_34).unwrap(),
        ));
        // D s_17_38: cast reint s_17_37 -> u8
        let s_17_38: u8 = (s_17_37.value() as u8);
        // C s_17_39: const #7s : i
        let s_17_39: i128 = 7;
        // C s_17_40: const #5s : i
        let s_17_40: i128 = 5;
        // D s_17_41: read-var u#33360:u32
        let s_17_41: u32 = fn_state.u_33360;
        // D s_17_42: cast zx s_17_41 -> bv
        let s_17_42: Bits = Bits::new(s_17_41 as u128, 32u16);
        // D s_17_43: bit-extract s_17_42 s_17_39 s_17_40
        let s_17_43: Bits = (Bits::new(
            ((s_17_42) >> (s_17_39)).value(),
            u16::try_from(s_17_40).unwrap(),
        ));
        // D s_17_44: cast reint s_17_43 -> u8
        let s_17_44: u8 = (s_17_43.value() as u8);
        // C s_17_45: const #5s : i
        let s_17_45: i128 = 5;
        // C s_17_46: const #2s : i
        let s_17_46: i128 = 2;
        // D s_17_47: read-var u#33360:u32
        let s_17_47: u32 = fn_state.u_33360;
        // D s_17_48: cast zx s_17_47 -> bv
        let s_17_48: Bits = Bits::new(s_17_47 as u128, 32u16);
        // D s_17_49: bit-extract s_17_48 s_17_45 s_17_46
        let s_17_49: Bits = (Bits::new(
            ((s_17_48) >> (s_17_45)).value(),
            u16::try_from(s_17_46).unwrap(),
        ));
        // D s_17_50: cast reint s_17_49 -> u8
        let s_17_50: u8 = (s_17_49.value() as u8);
        // C s_17_51: const #0s : i
        let s_17_51: i128 = 0;
        // C s_17_52: const #4s : i
        let s_17_52: i128 = 4;
        // D s_17_53: read-var u#33360:u32
        let s_17_53: u32 = fn_state.u_33360;
        // D s_17_54: cast zx s_17_53 -> bv
        let s_17_54: Bits = Bits::new(s_17_53 as u128, 32u16);
        // D s_17_55: bit-extract s_17_54 s_17_51 s_17_52
        let s_17_55: Bits = (Bits::new(
            ((s_17_54) >> (s_17_51)).value(),
            u16::try_from(s_17_52).unwrap(),
        ));
        // D s_17_56: cast reint s_17_55 -> u8
        let s_17_56: u8 = (s_17_55.value() as u8);
        // D s_17_57: call decode_aarch32_instrs_LDR_r_A1enc_A_txt(s_17_8, s_17_14, s_17_20, s_17_26, s_17_32, s_17_38, s_17_44, s_17_50, s_17_56)
        let s_17_57: () = decode_aarch32_instrs_LDR_r_A1enc_A_txt(
            state,
            tracer,
            s_17_8,
            s_17_14,
            s_17_20,
            s_17_26,
            s_17_32,
            s_17_38,
            s_17_44,
            s_17_50,
            s_17_56,
        );
        // N s_17_58: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var merge#var.1:struct
        let s_18_0: u32 = fn_state.merge_var._1;
        // D s_18_1: write-var u#33371 <= s_18_0
        fn_state.u_33371 = s_18_0;
        // C s_18_2: const #24s : i
        let s_18_2: i128 = 24;
        // D s_18_3: read-var u#33371:u32
        let s_18_3: u32 = fn_state.u_33371;
        // D s_18_4: cast zx s_18_3 -> bv
        let s_18_4: Bits = Bits::new(s_18_3 as u128, 32u16);
        // C s_18_5: const #1s : i64
        let s_18_5: i64 = 1;
        // C s_18_6: cast zx s_18_5 -> i
        let s_18_6: i128 = (i128::try_from(s_18_5).unwrap());
        // C s_18_7: const #3s : i
        let s_18_7: i128 = 3;
        // C s_18_8: add s_18_7 s_18_6
        let s_18_8: i128 = (s_18_7 + s_18_6);
        // D s_18_9: bit-extract s_18_4 s_18_2 s_18_8
        let s_18_9: Bits = (Bits::new(
            ((s_18_4) >> (s_18_2)).value(),
            u16::try_from(s_18_8).unwrap(),
        ));
        // D s_18_10: cast reint s_18_9 -> u8
        let s_18_10: u8 = (s_18_9.value() as u8);
        // D s_18_11: cast zx s_18_10 -> bv
        let s_18_11: Bits = Bits::new(s_18_10 as u128, 4u16);
        // C s_18_12: const #6u : u8
        let s_18_12: u8 = 6;
        // C s_18_13: cast zx s_18_12 -> bv
        let s_18_13: Bits = Bits::new(s_18_12 as u128, 4u16);
        // D s_18_14: cmp-eq s_18_11 s_18_13
        let s_18_14: bool = ((s_18_11) == (s_18_13));
        // N s_18_15: branch s_18_14 b91 b19
        if s_18_14 {
            return block_91(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#408931 <= s_19_0
        fn_state.gs_408931 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#408931:u8
        let s_20_0: bool = fn_state.gs_408931;
        // N s_20_1: branch s_20_0 b87 b21
        if s_20_0 {
            return block_87(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#408936 <= s_21_0
        fn_state.gs_408936 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#408936:u8
        let s_22_0: bool = fn_state.gs_408936;
        // D s_22_1: not s_22_0
        let s_22_1: bool = !s_22_0;
        // N s_22_2: branch s_22_1 b24 b23
        if s_22_1 {
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
        // C s_23_0: const #2994s : i
        let s_23_0: i128 = 2994;
        // C s_23_1: const #14696u : u32
        let s_23_1: u32 = 14696;
        // N s_23_2: write-reg s_23_1 <= s_23_0
        let s_23_2: () = {
            state.write_register::<i128>(s_23_1 as isize, s_23_0);
            tracer.write_register(s_23_1 as isize, s_23_0);
        };
        // C s_23_3: const #28s : i
        let s_23_3: i128 = 28;
        // C s_23_4: const #4s : i
        let s_23_4: i128 = 4;
        // D s_23_5: read-var u#33371:u32
        let s_23_5: u32 = fn_state.u_33371;
        // D s_23_6: cast zx s_23_5 -> bv
        let s_23_6: Bits = Bits::new(s_23_5 as u128, 32u16);
        // D s_23_7: bit-extract s_23_6 s_23_3 s_23_4
        let s_23_7: Bits = (Bits::new(
            ((s_23_6) >> (s_23_3)).value(),
            u16::try_from(s_23_4).unwrap(),
        ));
        // D s_23_8: cast reint s_23_7 -> u8
        let s_23_8: u8 = (s_23_7.value() as u8);
        // C s_23_9: const #23s : i
        let s_23_9: i128 = 23;
        // C s_23_10: const #1s : i
        let s_23_10: i128 = 1;
        // D s_23_11: read-var u#33371:u32
        let s_23_11: u32 = fn_state.u_33371;
        // D s_23_12: cast zx s_23_11 -> bv
        let s_23_12: Bits = Bits::new(s_23_11 as u128, 32u16);
        // D s_23_13: bit-extract s_23_12 s_23_9 s_23_10
        let s_23_13: Bits = (Bits::new(
            ((s_23_12) >> (s_23_9)).value(),
            u16::try_from(s_23_10).unwrap(),
        ));
        // D s_23_14: cast reint s_23_13 -> u8
        let s_23_14: bool = ((s_23_13.value()) != 0);
        // C s_23_15: const #16s : i
        let s_23_15: i128 = 16;
        // C s_23_16: const #4s : i
        let s_23_16: i128 = 4;
        // D s_23_17: read-var u#33371:u32
        let s_23_17: u32 = fn_state.u_33371;
        // D s_23_18: cast zx s_23_17 -> bv
        let s_23_18: Bits = Bits::new(s_23_17 as u128, 32u16);
        // D s_23_19: bit-extract s_23_18 s_23_15 s_23_16
        let s_23_19: Bits = (Bits::new(
            ((s_23_18) >> (s_23_15)).value(),
            u16::try_from(s_23_16).unwrap(),
        ));
        // D s_23_20: cast reint s_23_19 -> u8
        let s_23_20: u8 = (s_23_19.value() as u8);
        // C s_23_21: const #12s : i
        let s_23_21: i128 = 12;
        // C s_23_22: const #4s : i
        let s_23_22: i128 = 4;
        // D s_23_23: read-var u#33371:u32
        let s_23_23: u32 = fn_state.u_33371;
        // D s_23_24: cast zx s_23_23 -> bv
        let s_23_24: Bits = Bits::new(s_23_23 as u128, 32u16);
        // D s_23_25: bit-extract s_23_24 s_23_21 s_23_22
        let s_23_25: Bits = (Bits::new(
            ((s_23_24) >> (s_23_21)).value(),
            u16::try_from(s_23_22).unwrap(),
        ));
        // D s_23_26: cast reint s_23_25 -> u8
        let s_23_26: u8 = (s_23_25.value() as u8);
        // C s_23_27: const #7s : i
        let s_23_27: i128 = 7;
        // C s_23_28: const #5s : i
        let s_23_28: i128 = 5;
        // D s_23_29: read-var u#33371:u32
        let s_23_29: u32 = fn_state.u_33371;
        // D s_23_30: cast zx s_23_29 -> bv
        let s_23_30: Bits = Bits::new(s_23_29 as u128, 32u16);
        // D s_23_31: bit-extract s_23_30 s_23_27 s_23_28
        let s_23_31: Bits = (Bits::new(
            ((s_23_30) >> (s_23_27)).value(),
            u16::try_from(s_23_28).unwrap(),
        ));
        // D s_23_32: cast reint s_23_31 -> u8
        let s_23_32: u8 = (s_23_31.value() as u8);
        // C s_23_33: const #5s : i
        let s_23_33: i128 = 5;
        // C s_23_34: const #2s : i
        let s_23_34: i128 = 2;
        // D s_23_35: read-var u#33371:u32
        let s_23_35: u32 = fn_state.u_33371;
        // D s_23_36: cast zx s_23_35 -> bv
        let s_23_36: Bits = Bits::new(s_23_35 as u128, 32u16);
        // D s_23_37: bit-extract s_23_36 s_23_33 s_23_34
        let s_23_37: Bits = (Bits::new(
            ((s_23_36) >> (s_23_33)).value(),
            u16::try_from(s_23_34).unwrap(),
        ));
        // D s_23_38: cast reint s_23_37 -> u8
        let s_23_38: u8 = (s_23_37.value() as u8);
        // C s_23_39: const #0s : i
        let s_23_39: i128 = 0;
        // C s_23_40: const #4s : i
        let s_23_40: i128 = 4;
        // D s_23_41: read-var u#33371:u32
        let s_23_41: u32 = fn_state.u_33371;
        // D s_23_42: cast zx s_23_41 -> bv
        let s_23_42: Bits = Bits::new(s_23_41 as u128, 32u16);
        // D s_23_43: bit-extract s_23_42 s_23_39 s_23_40
        let s_23_43: Bits = (Bits::new(
            ((s_23_42) >> (s_23_39)).value(),
            u16::try_from(s_23_40).unwrap(),
        ));
        // D s_23_44: cast reint s_23_43 -> u8
        let s_23_44: u8 = (s_23_43.value() as u8);
        // D s_23_45: call decode_aarch32_instrs_LDRT_A2enc_A_txt(s_23_8, s_23_14, s_23_20, s_23_26, s_23_32, s_23_38, s_23_44)
        let s_23_45: () = decode_aarch32_instrs_LDRT_A2enc_A_txt(
            state,
            tracer,
            s_23_8,
            s_23_14,
            s_23_20,
            s_23_26,
            s_23_32,
            s_23_38,
            s_23_44,
        );
        // N s_23_46: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var merge#var.1:struct
        let s_24_0: u32 = fn_state.merge_var._1;
        // D s_24_1: write-var u#33380 <= s_24_0
        fn_state.u_33380 = s_24_0;
        // C s_24_2: const #25s : i
        let s_24_2: i128 = 25;
        // D s_24_3: read-var u#33380:u32
        let s_24_3: u32 = fn_state.u_33380;
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 32u16);
        // C s_24_5: const #1s : i64
        let s_24_5: i64 = 1;
        // C s_24_6: cast zx s_24_5 -> i
        let s_24_6: i128 = (i128::try_from(s_24_5).unwrap());
        // C s_24_7: const #2s : i
        let s_24_7: i128 = 2;
        // C s_24_8: add s_24_7 s_24_6
        let s_24_8: i128 = (s_24_7 + s_24_6);
        // D s_24_9: bit-extract s_24_4 s_24_2 s_24_8
        let s_24_9: Bits = (Bits::new(
            ((s_24_4) >> (s_24_2)).value(),
            u16::try_from(s_24_8).unwrap(),
        ));
        // D s_24_10: cast reint s_24_9 -> u8
        let s_24_10: u8 = (s_24_9.value() as u8);
        // D s_24_11: cast zx s_24_10 -> bv
        let s_24_11: Bits = Bits::new(s_24_10 as u128, 3u16);
        // C s_24_12: const #3u : u8
        let s_24_12: u8 = 3;
        // C s_24_13: cast zx s_24_12 -> bv
        let s_24_13: Bits = Bits::new(s_24_12 as u128, 3u16);
        // D s_24_14: cmp-eq s_24_11 s_24_13
        let s_24_14: bool = ((s_24_11) == (s_24_13));
        // N s_24_15: branch s_24_14 b80 b25
        if s_24_14 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#408964 <= s_25_0
        fn_state.gs_408964 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#408964:u8
        let s_26_0: bool = fn_state.gs_408964;
        // N s_26_1: branch s_26_0 b76 b27
        if s_26_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#408969 <= s_27_0
        fn_state.gs_408969 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#408969:u8
        let s_28_0: bool = fn_state.gs_408969;
        // D s_28_1: not s_28_0
        let s_28_1: bool = !s_28_0;
        // N s_28_2: branch s_28_1 b30 b29
        if s_28_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #3197s : i
        let s_29_0: i128 = 3197;
        // C s_29_1: const #14696u : u32
        let s_29_1: u32 = 14696;
        // N s_29_2: write-reg s_29_1 <= s_29_0
        let s_29_2: () = {
            state.write_register::<i128>(s_29_1 as isize, s_29_0);
            tracer.write_register(s_29_1 as isize, s_29_0);
        };
        // C s_29_3: const #28s : i
        let s_29_3: i128 = 28;
        // C s_29_4: const #4s : i
        let s_29_4: i128 = 4;
        // D s_29_5: read-var u#33380:u32
        let s_29_5: u32 = fn_state.u_33380;
        // D s_29_6: cast zx s_29_5 -> bv
        let s_29_6: Bits = Bits::new(s_29_5 as u128, 32u16);
        // D s_29_7: bit-extract s_29_6 s_29_3 s_29_4
        let s_29_7: Bits = (Bits::new(
            ((s_29_6) >> (s_29_3)).value(),
            u16::try_from(s_29_4).unwrap(),
        ));
        // D s_29_8: cast reint s_29_7 -> u8
        let s_29_8: u8 = (s_29_7.value() as u8);
        // C s_29_9: const #24s : i
        let s_29_9: i128 = 24;
        // C s_29_10: const #1s : i
        let s_29_10: i128 = 1;
        // D s_29_11: read-var u#33380:u32
        let s_29_11: u32 = fn_state.u_33380;
        // D s_29_12: cast zx s_29_11 -> bv
        let s_29_12: Bits = Bits::new(s_29_11 as u128, 32u16);
        // D s_29_13: bit-extract s_29_12 s_29_9 s_29_10
        let s_29_13: Bits = (Bits::new(
            ((s_29_12) >> (s_29_9)).value(),
            u16::try_from(s_29_10).unwrap(),
        ));
        // D s_29_14: cast reint s_29_13 -> u8
        let s_29_14: bool = ((s_29_13.value()) != 0);
        // C s_29_15: const #23s : i
        let s_29_15: i128 = 23;
        // C s_29_16: const #1s : i
        let s_29_16: i128 = 1;
        // D s_29_17: read-var u#33380:u32
        let s_29_17: u32 = fn_state.u_33380;
        // D s_29_18: cast zx s_29_17 -> bv
        let s_29_18: Bits = Bits::new(s_29_17 as u128, 32u16);
        // D s_29_19: bit-extract s_29_18 s_29_15 s_29_16
        let s_29_19: Bits = (Bits::new(
            ((s_29_18) >> (s_29_15)).value(),
            u16::try_from(s_29_16).unwrap(),
        ));
        // D s_29_20: cast reint s_29_19 -> u8
        let s_29_20: bool = ((s_29_19.value()) != 0);
        // C s_29_21: const #21s : i
        let s_29_21: i128 = 21;
        // C s_29_22: const #1s : i
        let s_29_22: i128 = 1;
        // D s_29_23: read-var u#33380:u32
        let s_29_23: u32 = fn_state.u_33380;
        // D s_29_24: cast zx s_29_23 -> bv
        let s_29_24: Bits = Bits::new(s_29_23 as u128, 32u16);
        // D s_29_25: bit-extract s_29_24 s_29_21 s_29_22
        let s_29_25: Bits = (Bits::new(
            ((s_29_24) >> (s_29_21)).value(),
            u16::try_from(s_29_22).unwrap(),
        ));
        // D s_29_26: cast reint s_29_25 -> u8
        let s_29_26: bool = ((s_29_25.value()) != 0);
        // C s_29_27: const #16s : i
        let s_29_27: i128 = 16;
        // C s_29_28: const #4s : i
        let s_29_28: i128 = 4;
        // D s_29_29: read-var u#33380:u32
        let s_29_29: u32 = fn_state.u_33380;
        // D s_29_30: cast zx s_29_29 -> bv
        let s_29_30: Bits = Bits::new(s_29_29 as u128, 32u16);
        // D s_29_31: bit-extract s_29_30 s_29_27 s_29_28
        let s_29_31: Bits = (Bits::new(
            ((s_29_30) >> (s_29_27)).value(),
            u16::try_from(s_29_28).unwrap(),
        ));
        // D s_29_32: cast reint s_29_31 -> u8
        let s_29_32: u8 = (s_29_31.value() as u8);
        // C s_29_33: const #12s : i
        let s_29_33: i128 = 12;
        // C s_29_34: const #4s : i
        let s_29_34: i128 = 4;
        // D s_29_35: read-var u#33380:u32
        let s_29_35: u32 = fn_state.u_33380;
        // D s_29_36: cast zx s_29_35 -> bv
        let s_29_36: Bits = Bits::new(s_29_35 as u128, 32u16);
        // D s_29_37: bit-extract s_29_36 s_29_33 s_29_34
        let s_29_37: Bits = (Bits::new(
            ((s_29_36) >> (s_29_33)).value(),
            u16::try_from(s_29_34).unwrap(),
        ));
        // D s_29_38: cast reint s_29_37 -> u8
        let s_29_38: u8 = (s_29_37.value() as u8);
        // C s_29_39: const #7s : i
        let s_29_39: i128 = 7;
        // C s_29_40: const #5s : i
        let s_29_40: i128 = 5;
        // D s_29_41: read-var u#33380:u32
        let s_29_41: u32 = fn_state.u_33380;
        // D s_29_42: cast zx s_29_41 -> bv
        let s_29_42: Bits = Bits::new(s_29_41 as u128, 32u16);
        // D s_29_43: bit-extract s_29_42 s_29_39 s_29_40
        let s_29_43: Bits = (Bits::new(
            ((s_29_42) >> (s_29_39)).value(),
            u16::try_from(s_29_40).unwrap(),
        ));
        // D s_29_44: cast reint s_29_43 -> u8
        let s_29_44: u8 = (s_29_43.value() as u8);
        // C s_29_45: const #5s : i
        let s_29_45: i128 = 5;
        // C s_29_46: const #2s : i
        let s_29_46: i128 = 2;
        // D s_29_47: read-var u#33380:u32
        let s_29_47: u32 = fn_state.u_33380;
        // D s_29_48: cast zx s_29_47 -> bv
        let s_29_48: Bits = Bits::new(s_29_47 as u128, 32u16);
        // D s_29_49: bit-extract s_29_48 s_29_45 s_29_46
        let s_29_49: Bits = (Bits::new(
            ((s_29_48) >> (s_29_45)).value(),
            u16::try_from(s_29_46).unwrap(),
        ));
        // D s_29_50: cast reint s_29_49 -> u8
        let s_29_50: u8 = (s_29_49.value() as u8);
        // C s_29_51: const #0s : i
        let s_29_51: i128 = 0;
        // C s_29_52: const #4s : i
        let s_29_52: i128 = 4;
        // D s_29_53: read-var u#33380:u32
        let s_29_53: u32 = fn_state.u_33380;
        // D s_29_54: cast zx s_29_53 -> bv
        let s_29_54: Bits = Bits::new(s_29_53 as u128, 32u16);
        // D s_29_55: bit-extract s_29_54 s_29_51 s_29_52
        let s_29_55: Bits = (Bits::new(
            ((s_29_54) >> (s_29_51)).value(),
            u16::try_from(s_29_52).unwrap(),
        ));
        // D s_29_56: cast reint s_29_55 -> u8
        let s_29_56: u8 = (s_29_55.value() as u8);
        // D s_29_57: call decode_aarch32_instrs_STRB_r_A1enc_A_txt(s_29_8, s_29_14, s_29_20, s_29_26, s_29_32, s_29_38, s_29_44, s_29_50, s_29_56)
        let s_29_57: () = decode_aarch32_instrs_STRB_r_A1enc_A_txt(
            state,
            tracer,
            s_29_8,
            s_29_14,
            s_29_20,
            s_29_26,
            s_29_32,
            s_29_38,
            s_29_44,
            s_29_50,
            s_29_56,
        );
        // N s_29_58: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var merge#var.1:struct
        let s_30_0: u32 = fn_state.merge_var._1;
        // D s_30_1: write-var u#33391 <= s_30_0
        fn_state.u_33391 = s_30_0;
        // C s_30_2: const #24s : i
        let s_30_2: i128 = 24;
        // D s_30_3: read-var u#33391:u32
        let s_30_3: u32 = fn_state.u_33391;
        // D s_30_4: cast zx s_30_3 -> bv
        let s_30_4: Bits = Bits::new(s_30_3 as u128, 32u16);
        // C s_30_5: const #1s : i64
        let s_30_5: i64 = 1;
        // C s_30_6: cast zx s_30_5 -> i
        let s_30_6: i128 = (i128::try_from(s_30_5).unwrap());
        // C s_30_7: const #3s : i
        let s_30_7: i128 = 3;
        // C s_30_8: add s_30_7 s_30_6
        let s_30_8: i128 = (s_30_7 + s_30_6);
        // D s_30_9: bit-extract s_30_4 s_30_2 s_30_8
        let s_30_9: Bits = (Bits::new(
            ((s_30_4) >> (s_30_2)).value(),
            u16::try_from(s_30_8).unwrap(),
        ));
        // D s_30_10: cast reint s_30_9 -> u8
        let s_30_10: u8 = (s_30_9.value() as u8);
        // D s_30_11: cast zx s_30_10 -> bv
        let s_30_11: Bits = Bits::new(s_30_10 as u128, 4u16);
        // C s_30_12: const #6u : u8
        let s_30_12: u8 = 6;
        // C s_30_13: cast zx s_30_12 -> bv
        let s_30_13: Bits = Bits::new(s_30_12 as u128, 4u16);
        // D s_30_14: cmp-eq s_30_11 s_30_13
        let s_30_14: bool = ((s_30_11) == (s_30_13));
        // N s_30_15: branch s_30_14 b72 b31
        if s_30_14 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#408998 <= s_31_0
        fn_state.gs_408998 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#408998:u8
        let s_32_0: bool = fn_state.gs_408998;
        // N s_32_1: branch s_32_0 b68 b33
        if s_32_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#409003 <= s_33_0
        fn_state.gs_409003 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#409003:u8
        let s_34_0: bool = fn_state.gs_409003;
        // D s_34_1: not s_34_0
        let s_34_1: bool = !s_34_0;
        // N s_34_2: branch s_34_1 b36 b35
        if s_34_1 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #3201s : i
        let s_35_0: i128 = 3201;
        // C s_35_1: const #14696u : u32
        let s_35_1: u32 = 14696;
        // N s_35_2: write-reg s_35_1 <= s_35_0
        let s_35_2: () = {
            state.write_register::<i128>(s_35_1 as isize, s_35_0);
            tracer.write_register(s_35_1 as isize, s_35_0);
        };
        // C s_35_3: const #28s : i
        let s_35_3: i128 = 28;
        // C s_35_4: const #4s : i
        let s_35_4: i128 = 4;
        // D s_35_5: read-var u#33391:u32
        let s_35_5: u32 = fn_state.u_33391;
        // D s_35_6: cast zx s_35_5 -> bv
        let s_35_6: Bits = Bits::new(s_35_5 as u128, 32u16);
        // D s_35_7: bit-extract s_35_6 s_35_3 s_35_4
        let s_35_7: Bits = (Bits::new(
            ((s_35_6) >> (s_35_3)).value(),
            u16::try_from(s_35_4).unwrap(),
        ));
        // D s_35_8: cast reint s_35_7 -> u8
        let s_35_8: u8 = (s_35_7.value() as u8);
        // C s_35_9: const #23s : i
        let s_35_9: i128 = 23;
        // C s_35_10: const #1s : i
        let s_35_10: i128 = 1;
        // D s_35_11: read-var u#33391:u32
        let s_35_11: u32 = fn_state.u_33391;
        // D s_35_12: cast zx s_35_11 -> bv
        let s_35_12: Bits = Bits::new(s_35_11 as u128, 32u16);
        // D s_35_13: bit-extract s_35_12 s_35_9 s_35_10
        let s_35_13: Bits = (Bits::new(
            ((s_35_12) >> (s_35_9)).value(),
            u16::try_from(s_35_10).unwrap(),
        ));
        // D s_35_14: cast reint s_35_13 -> u8
        let s_35_14: bool = ((s_35_13.value()) != 0);
        // C s_35_15: const #16s : i
        let s_35_15: i128 = 16;
        // C s_35_16: const #4s : i
        let s_35_16: i128 = 4;
        // D s_35_17: read-var u#33391:u32
        let s_35_17: u32 = fn_state.u_33391;
        // D s_35_18: cast zx s_35_17 -> bv
        let s_35_18: Bits = Bits::new(s_35_17 as u128, 32u16);
        // D s_35_19: bit-extract s_35_18 s_35_15 s_35_16
        let s_35_19: Bits = (Bits::new(
            ((s_35_18) >> (s_35_15)).value(),
            u16::try_from(s_35_16).unwrap(),
        ));
        // D s_35_20: cast reint s_35_19 -> u8
        let s_35_20: u8 = (s_35_19.value() as u8);
        // C s_35_21: const #12s : i
        let s_35_21: i128 = 12;
        // C s_35_22: const #4s : i
        let s_35_22: i128 = 4;
        // D s_35_23: read-var u#33391:u32
        let s_35_23: u32 = fn_state.u_33391;
        // D s_35_24: cast zx s_35_23 -> bv
        let s_35_24: Bits = Bits::new(s_35_23 as u128, 32u16);
        // D s_35_25: bit-extract s_35_24 s_35_21 s_35_22
        let s_35_25: Bits = (Bits::new(
            ((s_35_24) >> (s_35_21)).value(),
            u16::try_from(s_35_22).unwrap(),
        ));
        // D s_35_26: cast reint s_35_25 -> u8
        let s_35_26: u8 = (s_35_25.value() as u8);
        // C s_35_27: const #7s : i
        let s_35_27: i128 = 7;
        // C s_35_28: const #5s : i
        let s_35_28: i128 = 5;
        // D s_35_29: read-var u#33391:u32
        let s_35_29: u32 = fn_state.u_33391;
        // D s_35_30: cast zx s_35_29 -> bv
        let s_35_30: Bits = Bits::new(s_35_29 as u128, 32u16);
        // D s_35_31: bit-extract s_35_30 s_35_27 s_35_28
        let s_35_31: Bits = (Bits::new(
            ((s_35_30) >> (s_35_27)).value(),
            u16::try_from(s_35_28).unwrap(),
        ));
        // D s_35_32: cast reint s_35_31 -> u8
        let s_35_32: u8 = (s_35_31.value() as u8);
        // C s_35_33: const #5s : i
        let s_35_33: i128 = 5;
        // C s_35_34: const #2s : i
        let s_35_34: i128 = 2;
        // D s_35_35: read-var u#33391:u32
        let s_35_35: u32 = fn_state.u_33391;
        // D s_35_36: cast zx s_35_35 -> bv
        let s_35_36: Bits = Bits::new(s_35_35 as u128, 32u16);
        // D s_35_37: bit-extract s_35_36 s_35_33 s_35_34
        let s_35_37: Bits = (Bits::new(
            ((s_35_36) >> (s_35_33)).value(),
            u16::try_from(s_35_34).unwrap(),
        ));
        // D s_35_38: cast reint s_35_37 -> u8
        let s_35_38: u8 = (s_35_37.value() as u8);
        // C s_35_39: const #0s : i
        let s_35_39: i128 = 0;
        // C s_35_40: const #4s : i
        let s_35_40: i128 = 4;
        // D s_35_41: read-var u#33391:u32
        let s_35_41: u32 = fn_state.u_33391;
        // D s_35_42: cast zx s_35_41 -> bv
        let s_35_42: Bits = Bits::new(s_35_41 as u128, 32u16);
        // D s_35_43: bit-extract s_35_42 s_35_39 s_35_40
        let s_35_43: Bits = (Bits::new(
            ((s_35_42) >> (s_35_39)).value(),
            u16::try_from(s_35_40).unwrap(),
        ));
        // D s_35_44: cast reint s_35_43 -> u8
        let s_35_44: u8 = (s_35_43.value() as u8);
        // D s_35_45: call decode_aarch32_instrs_STRBT_A2enc_A_txt(s_35_8, s_35_14, s_35_20, s_35_26, s_35_32, s_35_38, s_35_44)
        let s_35_45: () = decode_aarch32_instrs_STRBT_A2enc_A_txt(
            state,
            tracer,
            s_35_8,
            s_35_14,
            s_35_20,
            s_35_26,
            s_35_32,
            s_35_38,
            s_35_44,
        );
        // N s_35_46: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var merge#var.1:struct
        let s_36_0: u32 = fn_state.merge_var._1;
        // D s_36_1: write-var u#33400 <= s_36_0
        fn_state.u_33400 = s_36_0;
        // C s_36_2: const #25s : i
        let s_36_2: i128 = 25;
        // D s_36_3: read-var u#33400:u32
        let s_36_3: u32 = fn_state.u_33400;
        // D s_36_4: cast zx s_36_3 -> bv
        let s_36_4: Bits = Bits::new(s_36_3 as u128, 32u16);
        // C s_36_5: const #1s : i64
        let s_36_5: i64 = 1;
        // C s_36_6: cast zx s_36_5 -> i
        let s_36_6: i128 = (i128::try_from(s_36_5).unwrap());
        // C s_36_7: const #2s : i
        let s_36_7: i128 = 2;
        // C s_36_8: add s_36_7 s_36_6
        let s_36_8: i128 = (s_36_7 + s_36_6);
        // D s_36_9: bit-extract s_36_4 s_36_2 s_36_8
        let s_36_9: Bits = (Bits::new(
            ((s_36_4) >> (s_36_2)).value(),
            u16::try_from(s_36_8).unwrap(),
        ));
        // D s_36_10: cast reint s_36_9 -> u8
        let s_36_10: u8 = (s_36_9.value() as u8);
        // D s_36_11: cast zx s_36_10 -> bv
        let s_36_11: Bits = Bits::new(s_36_10 as u128, 3u16);
        // C s_36_12: const #3u : u8
        let s_36_12: u8 = 3;
        // C s_36_13: cast zx s_36_12 -> bv
        let s_36_13: Bits = Bits::new(s_36_12 as u128, 3u16);
        // D s_36_14: cmp-eq s_36_11 s_36_13
        let s_36_14: bool = ((s_36_11) == (s_36_13));
        // N s_36_15: branch s_36_14 b61 b37
        if s_36_14 {
            return block_61(state, tracer, fn_state);
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
        // D s_37_1: write-var gs#409031 <= s_37_0
        fn_state.gs_409031 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#409031:u8
        let s_38_0: bool = fn_state.gs_409031;
        // N s_38_1: branch s_38_0 b57 b39
        if s_38_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#409036 <= s_39_0
        fn_state.gs_409036 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#409036:u8
        let s_40_0: bool = fn_state.gs_409036;
        // D s_40_1: not s_40_0
        let s_40_1: bool = !s_40_0;
        // N s_40_2: branch s_40_1 b42 b41
        if s_40_1 {
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
        // C s_41_0: const #3229s : i
        let s_41_0: i128 = 3229;
        // C s_41_1: const #14696u : u32
        let s_41_1: u32 = 14696;
        // N s_41_2: write-reg s_41_1 <= s_41_0
        let s_41_2: () = {
            state.write_register::<i128>(s_41_1 as isize, s_41_0);
            tracer.write_register(s_41_1 as isize, s_41_0);
        };
        // C s_41_3: const #28s : i
        let s_41_3: i128 = 28;
        // C s_41_4: const #4s : i
        let s_41_4: i128 = 4;
        // D s_41_5: read-var u#33400:u32
        let s_41_5: u32 = fn_state.u_33400;
        // D s_41_6: cast zx s_41_5 -> bv
        let s_41_6: Bits = Bits::new(s_41_5 as u128, 32u16);
        // D s_41_7: bit-extract s_41_6 s_41_3 s_41_4
        let s_41_7: Bits = (Bits::new(
            ((s_41_6) >> (s_41_3)).value(),
            u16::try_from(s_41_4).unwrap(),
        ));
        // D s_41_8: cast reint s_41_7 -> u8
        let s_41_8: u8 = (s_41_7.value() as u8);
        // C s_41_9: const #24s : i
        let s_41_9: i128 = 24;
        // C s_41_10: const #1s : i
        let s_41_10: i128 = 1;
        // D s_41_11: read-var u#33400:u32
        let s_41_11: u32 = fn_state.u_33400;
        // D s_41_12: cast zx s_41_11 -> bv
        let s_41_12: Bits = Bits::new(s_41_11 as u128, 32u16);
        // D s_41_13: bit-extract s_41_12 s_41_9 s_41_10
        let s_41_13: Bits = (Bits::new(
            ((s_41_12) >> (s_41_9)).value(),
            u16::try_from(s_41_10).unwrap(),
        ));
        // D s_41_14: cast reint s_41_13 -> u8
        let s_41_14: bool = ((s_41_13.value()) != 0);
        // C s_41_15: const #23s : i
        let s_41_15: i128 = 23;
        // C s_41_16: const #1s : i
        let s_41_16: i128 = 1;
        // D s_41_17: read-var u#33400:u32
        let s_41_17: u32 = fn_state.u_33400;
        // D s_41_18: cast zx s_41_17 -> bv
        let s_41_18: Bits = Bits::new(s_41_17 as u128, 32u16);
        // D s_41_19: bit-extract s_41_18 s_41_15 s_41_16
        let s_41_19: Bits = (Bits::new(
            ((s_41_18) >> (s_41_15)).value(),
            u16::try_from(s_41_16).unwrap(),
        ));
        // D s_41_20: cast reint s_41_19 -> u8
        let s_41_20: bool = ((s_41_19.value()) != 0);
        // C s_41_21: const #21s : i
        let s_41_21: i128 = 21;
        // C s_41_22: const #1s : i
        let s_41_22: i128 = 1;
        // D s_41_23: read-var u#33400:u32
        let s_41_23: u32 = fn_state.u_33400;
        // D s_41_24: cast zx s_41_23 -> bv
        let s_41_24: Bits = Bits::new(s_41_23 as u128, 32u16);
        // D s_41_25: bit-extract s_41_24 s_41_21 s_41_22
        let s_41_25: Bits = (Bits::new(
            ((s_41_24) >> (s_41_21)).value(),
            u16::try_from(s_41_22).unwrap(),
        ));
        // D s_41_26: cast reint s_41_25 -> u8
        let s_41_26: bool = ((s_41_25.value()) != 0);
        // C s_41_27: const #16s : i
        let s_41_27: i128 = 16;
        // C s_41_28: const #4s : i
        let s_41_28: i128 = 4;
        // D s_41_29: read-var u#33400:u32
        let s_41_29: u32 = fn_state.u_33400;
        // D s_41_30: cast zx s_41_29 -> bv
        let s_41_30: Bits = Bits::new(s_41_29 as u128, 32u16);
        // D s_41_31: bit-extract s_41_30 s_41_27 s_41_28
        let s_41_31: Bits = (Bits::new(
            ((s_41_30) >> (s_41_27)).value(),
            u16::try_from(s_41_28).unwrap(),
        ));
        // D s_41_32: cast reint s_41_31 -> u8
        let s_41_32: u8 = (s_41_31.value() as u8);
        // C s_41_33: const #12s : i
        let s_41_33: i128 = 12;
        // C s_41_34: const #4s : i
        let s_41_34: i128 = 4;
        // D s_41_35: read-var u#33400:u32
        let s_41_35: u32 = fn_state.u_33400;
        // D s_41_36: cast zx s_41_35 -> bv
        let s_41_36: Bits = Bits::new(s_41_35 as u128, 32u16);
        // D s_41_37: bit-extract s_41_36 s_41_33 s_41_34
        let s_41_37: Bits = (Bits::new(
            ((s_41_36) >> (s_41_33)).value(),
            u16::try_from(s_41_34).unwrap(),
        ));
        // D s_41_38: cast reint s_41_37 -> u8
        let s_41_38: u8 = (s_41_37.value() as u8);
        // C s_41_39: const #7s : i
        let s_41_39: i128 = 7;
        // C s_41_40: const #5s : i
        let s_41_40: i128 = 5;
        // D s_41_41: read-var u#33400:u32
        let s_41_41: u32 = fn_state.u_33400;
        // D s_41_42: cast zx s_41_41 -> bv
        let s_41_42: Bits = Bits::new(s_41_41 as u128, 32u16);
        // D s_41_43: bit-extract s_41_42 s_41_39 s_41_40
        let s_41_43: Bits = (Bits::new(
            ((s_41_42) >> (s_41_39)).value(),
            u16::try_from(s_41_40).unwrap(),
        ));
        // D s_41_44: cast reint s_41_43 -> u8
        let s_41_44: u8 = (s_41_43.value() as u8);
        // C s_41_45: const #5s : i
        let s_41_45: i128 = 5;
        // C s_41_46: const #2s : i
        let s_41_46: i128 = 2;
        // D s_41_47: read-var u#33400:u32
        let s_41_47: u32 = fn_state.u_33400;
        // D s_41_48: cast zx s_41_47 -> bv
        let s_41_48: Bits = Bits::new(s_41_47 as u128, 32u16);
        // D s_41_49: bit-extract s_41_48 s_41_45 s_41_46
        let s_41_49: Bits = (Bits::new(
            ((s_41_48) >> (s_41_45)).value(),
            u16::try_from(s_41_46).unwrap(),
        ));
        // D s_41_50: cast reint s_41_49 -> u8
        let s_41_50: u8 = (s_41_49.value() as u8);
        // C s_41_51: const #0s : i
        let s_41_51: i128 = 0;
        // C s_41_52: const #4s : i
        let s_41_52: i128 = 4;
        // D s_41_53: read-var u#33400:u32
        let s_41_53: u32 = fn_state.u_33400;
        // D s_41_54: cast zx s_41_53 -> bv
        let s_41_54: Bits = Bits::new(s_41_53 as u128, 32u16);
        // D s_41_55: bit-extract s_41_54 s_41_51 s_41_52
        let s_41_55: Bits = (Bits::new(
            ((s_41_54) >> (s_41_51)).value(),
            u16::try_from(s_41_52).unwrap(),
        ));
        // D s_41_56: cast reint s_41_55 -> u8
        let s_41_56: u8 = (s_41_55.value() as u8);
        // D s_41_57: call decode_aarch32_instrs_STR_r_A1enc_A_txt(s_41_8, s_41_14, s_41_20, s_41_26, s_41_32, s_41_38, s_41_44, s_41_50, s_41_56)
        let s_41_57: () = decode_aarch32_instrs_STR_r_A1enc_A_txt(
            state,
            tracer,
            s_41_8,
            s_41_14,
            s_41_20,
            s_41_26,
            s_41_32,
            s_41_38,
            s_41_44,
            s_41_50,
            s_41_56,
        );
        // N s_41_58: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var merge#var.1:struct
        let s_42_0: u32 = fn_state.merge_var._1;
        // D s_42_1: write-var u#33411 <= s_42_0
        fn_state.u_33411 = s_42_0;
        // C s_42_2: const #24s : i
        let s_42_2: i128 = 24;
        // D s_42_3: read-var u#33411:u32
        let s_42_3: u32 = fn_state.u_33411;
        // D s_42_4: cast zx s_42_3 -> bv
        let s_42_4: Bits = Bits::new(s_42_3 as u128, 32u16);
        // C s_42_5: const #1s : i64
        let s_42_5: i64 = 1;
        // C s_42_6: cast zx s_42_5 -> i
        let s_42_6: i128 = (i128::try_from(s_42_5).unwrap());
        // C s_42_7: const #3s : i
        let s_42_7: i128 = 3;
        // C s_42_8: add s_42_7 s_42_6
        let s_42_8: i128 = (s_42_7 + s_42_6);
        // D s_42_9: bit-extract s_42_4 s_42_2 s_42_8
        let s_42_9: Bits = (Bits::new(
            ((s_42_4) >> (s_42_2)).value(),
            u16::try_from(s_42_8).unwrap(),
        ));
        // D s_42_10: cast reint s_42_9 -> u8
        let s_42_10: u8 = (s_42_9.value() as u8);
        // D s_42_11: cast zx s_42_10 -> bv
        let s_42_11: Bits = Bits::new(s_42_10 as u128, 4u16);
        // C s_42_12: const #6u : u8
        let s_42_12: u8 = 6;
        // C s_42_13: cast zx s_42_12 -> bv
        let s_42_13: Bits = Bits::new(s_42_12 as u128, 4u16);
        // D s_42_14: cmp-eq s_42_11 s_42_13
        let s_42_14: bool = ((s_42_11) == (s_42_13));
        // N s_42_15: branch s_42_14 b53 b43
        if s_42_14 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#409065 <= s_43_0
        fn_state.gs_409065 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#409065:u8
        let s_44_0: bool = fn_state.gs_409065;
        // N s_44_1: branch s_44_0 b49 b45
        if s_44_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#409070 <= s_45_0
        fn_state.gs_409070 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#409070:u8
        let s_46_0: bool = fn_state.gs_409070;
        // D s_46_1: not s_46_0
        let s_46_1: bool = !s_46_0;
        // N s_46_2: branch s_46_1 b48 b47
        if s_46_1 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #3233s : i
        let s_47_0: i128 = 3233;
        // C s_47_1: const #14696u : u32
        let s_47_1: u32 = 14696;
        // N s_47_2: write-reg s_47_1 <= s_47_0
        let s_47_2: () = {
            state.write_register::<i128>(s_47_1 as isize, s_47_0);
            tracer.write_register(s_47_1 as isize, s_47_0);
        };
        // C s_47_3: const #28s : i
        let s_47_3: i128 = 28;
        // C s_47_4: const #4s : i
        let s_47_4: i128 = 4;
        // D s_47_5: read-var u#33411:u32
        let s_47_5: u32 = fn_state.u_33411;
        // D s_47_6: cast zx s_47_5 -> bv
        let s_47_6: Bits = Bits::new(s_47_5 as u128, 32u16);
        // D s_47_7: bit-extract s_47_6 s_47_3 s_47_4
        let s_47_7: Bits = (Bits::new(
            ((s_47_6) >> (s_47_3)).value(),
            u16::try_from(s_47_4).unwrap(),
        ));
        // D s_47_8: cast reint s_47_7 -> u8
        let s_47_8: u8 = (s_47_7.value() as u8);
        // C s_47_9: const #23s : i
        let s_47_9: i128 = 23;
        // C s_47_10: const #1s : i
        let s_47_10: i128 = 1;
        // D s_47_11: read-var u#33411:u32
        let s_47_11: u32 = fn_state.u_33411;
        // D s_47_12: cast zx s_47_11 -> bv
        let s_47_12: Bits = Bits::new(s_47_11 as u128, 32u16);
        // D s_47_13: bit-extract s_47_12 s_47_9 s_47_10
        let s_47_13: Bits = (Bits::new(
            ((s_47_12) >> (s_47_9)).value(),
            u16::try_from(s_47_10).unwrap(),
        ));
        // D s_47_14: cast reint s_47_13 -> u8
        let s_47_14: bool = ((s_47_13.value()) != 0);
        // C s_47_15: const #16s : i
        let s_47_15: i128 = 16;
        // C s_47_16: const #4s : i
        let s_47_16: i128 = 4;
        // D s_47_17: read-var u#33411:u32
        let s_47_17: u32 = fn_state.u_33411;
        // D s_47_18: cast zx s_47_17 -> bv
        let s_47_18: Bits = Bits::new(s_47_17 as u128, 32u16);
        // D s_47_19: bit-extract s_47_18 s_47_15 s_47_16
        let s_47_19: Bits = (Bits::new(
            ((s_47_18) >> (s_47_15)).value(),
            u16::try_from(s_47_16).unwrap(),
        ));
        // D s_47_20: cast reint s_47_19 -> u8
        let s_47_20: u8 = (s_47_19.value() as u8);
        // C s_47_21: const #12s : i
        let s_47_21: i128 = 12;
        // C s_47_22: const #4s : i
        let s_47_22: i128 = 4;
        // D s_47_23: read-var u#33411:u32
        let s_47_23: u32 = fn_state.u_33411;
        // D s_47_24: cast zx s_47_23 -> bv
        let s_47_24: Bits = Bits::new(s_47_23 as u128, 32u16);
        // D s_47_25: bit-extract s_47_24 s_47_21 s_47_22
        let s_47_25: Bits = (Bits::new(
            ((s_47_24) >> (s_47_21)).value(),
            u16::try_from(s_47_22).unwrap(),
        ));
        // D s_47_26: cast reint s_47_25 -> u8
        let s_47_26: u8 = (s_47_25.value() as u8);
        // C s_47_27: const #7s : i
        let s_47_27: i128 = 7;
        // C s_47_28: const #5s : i
        let s_47_28: i128 = 5;
        // D s_47_29: read-var u#33411:u32
        let s_47_29: u32 = fn_state.u_33411;
        // D s_47_30: cast zx s_47_29 -> bv
        let s_47_30: Bits = Bits::new(s_47_29 as u128, 32u16);
        // D s_47_31: bit-extract s_47_30 s_47_27 s_47_28
        let s_47_31: Bits = (Bits::new(
            ((s_47_30) >> (s_47_27)).value(),
            u16::try_from(s_47_28).unwrap(),
        ));
        // D s_47_32: cast reint s_47_31 -> u8
        let s_47_32: u8 = (s_47_31.value() as u8);
        // C s_47_33: const #5s : i
        let s_47_33: i128 = 5;
        // C s_47_34: const #2s : i
        let s_47_34: i128 = 2;
        // D s_47_35: read-var u#33411:u32
        let s_47_35: u32 = fn_state.u_33411;
        // D s_47_36: cast zx s_47_35 -> bv
        let s_47_36: Bits = Bits::new(s_47_35 as u128, 32u16);
        // D s_47_37: bit-extract s_47_36 s_47_33 s_47_34
        let s_47_37: Bits = (Bits::new(
            ((s_47_36) >> (s_47_33)).value(),
            u16::try_from(s_47_34).unwrap(),
        ));
        // D s_47_38: cast reint s_47_37 -> u8
        let s_47_38: u8 = (s_47_37.value() as u8);
        // C s_47_39: const #0s : i
        let s_47_39: i128 = 0;
        // C s_47_40: const #4s : i
        let s_47_40: i128 = 4;
        // D s_47_41: read-var u#33411:u32
        let s_47_41: u32 = fn_state.u_33411;
        // D s_47_42: cast zx s_47_41 -> bv
        let s_47_42: Bits = Bits::new(s_47_41 as u128, 32u16);
        // D s_47_43: bit-extract s_47_42 s_47_39 s_47_40
        let s_47_43: Bits = (Bits::new(
            ((s_47_42) >> (s_47_39)).value(),
            u16::try_from(s_47_40).unwrap(),
        ));
        // D s_47_44: cast reint s_47_43 -> u8
        let s_47_44: u8 = (s_47_43.value() as u8);
        // D s_47_45: call decode_aarch32_instrs_STRT_A2enc_A_txt(s_47_8, s_47_14, s_47_20, s_47_26, s_47_32, s_47_38, s_47_44)
        let s_47_45: () = decode_aarch32_instrs_STRT_A2enc_A_txt(
            state,
            tracer,
            s_47_8,
            s_47_14,
            s_47_20,
            s_47_26,
            s_47_32,
            s_47_38,
            s_47_44,
        );
        // N s_47_46: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_48_0: panic
        panic!("{:?}", ());
        // N s_48_1: return
        return;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #28s : i
        let s_49_0: i128 = 28;
        // C s_49_1: const #4s : i
        let s_49_1: i128 = 4;
        // D s_49_2: read-var u#33411:u32
        let s_49_2: u32 = fn_state.u_33411;
        // D s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 32u16);
        // D s_49_4: bit-extract s_49_3 s_49_0 s_49_1
        let s_49_4: Bits = (Bits::new(
            ((s_49_3) >> (s_49_0)).value(),
            u16::try_from(s_49_1).unwrap(),
        ));
        // D s_49_5: cast reint s_49_4 -> u8
        let s_49_5: u8 = (s_49_4.value() as u8);
        // D s_49_6: cast zx s_49_5 -> bv
        let s_49_6: Bits = Bits::new(s_49_5 as u128, 4u16);
        // C s_49_7: const #15u : u8
        let s_49_7: u8 = 15;
        // C s_49_8: cast zx s_49_7 -> bv
        let s_49_8: Bits = Bits::new(s_49_7 as u128, 4u16);
        // D s_49_9: cmp-ne s_49_6 s_49_8
        let s_49_9: bool = ((s_49_6) != (s_49_8));
        // N s_49_10: branch s_49_9 b52 b50
        if s_49_9 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var gs#409069 <= s_50_0
        fn_state.gs_409069 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#409069:u8
        let s_51_0: bool = fn_state.gs_409069;
        // D s_51_1: write-var gs#409070 <= s_51_0
        fn_state.gs_409070 = s_51_0;
        // N s_51_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #3233s : i
        let s_52_0: i128 = 3233;
        // C s_52_1: const #14696u : u32
        let s_52_1: u32 = 14696;
        // D s_52_2: read-reg s_52_1:i
        let s_52_2: i128 = {
            let value = state.read_register::<i128>(s_52_1 as isize);
            tracer.read_register(s_52_1 as isize, value);
            value
        };
        // D s_52_3: cmp-lt s_52_2 s_52_0
        let s_52_3: bool = ((s_52_2) < (s_52_0));
        // D s_52_4: write-var gs#409069 <= s_52_3
        fn_state.gs_409069 = s_52_3;
        // N s_52_5: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #20s : i
        let s_53_0: i128 = 20;
        // D s_53_1: read-var u#33411:u32
        let s_53_1: u32 = fn_state.u_33411;
        // D s_53_2: cast zx s_53_1 -> bv
        let s_53_2: Bits = Bits::new(s_53_1 as u128, 32u16);
        // C s_53_3: const #1s : i64
        let s_53_3: i64 = 1;
        // C s_53_4: cast zx s_53_3 -> i
        let s_53_4: i128 = (i128::try_from(s_53_3).unwrap());
        // C s_53_5: const #2s : i
        let s_53_5: i128 = 2;
        // C s_53_6: add s_53_5 s_53_4
        let s_53_6: i128 = (s_53_5 + s_53_4);
        // D s_53_7: bit-extract s_53_2 s_53_0 s_53_6
        let s_53_7: Bits = (Bits::new(
            ((s_53_2) >> (s_53_0)).value(),
            u16::try_from(s_53_6).unwrap(),
        ));
        // D s_53_8: cast reint s_53_7 -> u8
        let s_53_8: u8 = (s_53_7.value() as u8);
        // D s_53_9: cast zx s_53_8 -> bv
        let s_53_9: Bits = Bits::new(s_53_8 as u128, 3u16);
        // C s_53_10: const #2u : u8
        let s_53_10: u8 = 2;
        // C s_53_11: cast zx s_53_10 -> bv
        let s_53_11: Bits = Bits::new(s_53_10 as u128, 3u16);
        // D s_53_12: cmp-eq s_53_9 s_53_11
        let s_53_12: bool = ((s_53_9) == (s_53_11));
        // N s_53_13: branch s_53_12 b56 b54
        if s_53_12 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#409064 <= s_54_0
        fn_state.gs_409064 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#409064:u8
        let s_55_0: bool = fn_state.gs_409064;
        // D s_55_1: write-var gs#409065 <= s_55_0
        fn_state.gs_409065 = s_55_0;
        // N s_55_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #4s : i
        let s_56_0: i128 = 4;
        // D s_56_1: read-var u#33411:u32
        let s_56_1: u32 = fn_state.u_33411;
        // D s_56_2: cast zx s_56_1 -> bv
        let s_56_2: Bits = Bits::new(s_56_1 as u128, 32u16);
        // C s_56_3: const #1s : i64
        let s_56_3: i64 = 1;
        // C s_56_4: cast zx s_56_3 -> i
        let s_56_4: i128 = (i128::try_from(s_56_3).unwrap());
        // C s_56_5: const #0s : i
        let s_56_5: i128 = 0;
        // C s_56_6: add s_56_5 s_56_4
        let s_56_6: i128 = (s_56_5 + s_56_4);
        // D s_56_7: bit-extract s_56_2 s_56_0 s_56_6
        let s_56_7: Bits = (Bits::new(
            ((s_56_2) >> (s_56_0)).value(),
            u16::try_from(s_56_6).unwrap(),
        ));
        // D s_56_8: cast reint s_56_7 -> u8
        let s_56_8: bool = ((s_56_7.value()) != 0);
        // D s_56_9: cast zx s_56_8 -> bv
        let s_56_9: Bits = Bits::new(s_56_8 as u128, 1u16);
        // C s_56_10: const #0u : u8
        let s_56_10: bool = false;
        // C s_56_11: cast zx s_56_10 -> bv
        let s_56_11: Bits = Bits::new(s_56_10 as u128, 1u16);
        // D s_56_12: cmp-eq s_56_9 s_56_11
        let s_56_12: bool = ((s_56_9) == (s_56_11));
        // D s_56_13: write-var gs#409064 <= s_56_12
        fn_state.gs_409064 = s_56_12;
        // N s_56_14: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #28s : i
        let s_57_0: i128 = 28;
        // C s_57_1: const #4s : i
        let s_57_1: i128 = 4;
        // D s_57_2: read-var u#33400:u32
        let s_57_2: u32 = fn_state.u_33400;
        // D s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 32u16);
        // D s_57_4: bit-extract s_57_3 s_57_0 s_57_1
        let s_57_4: Bits = (Bits::new(
            ((s_57_3) >> (s_57_0)).value(),
            u16::try_from(s_57_1).unwrap(),
        ));
        // D s_57_5: cast reint s_57_4 -> u8
        let s_57_5: u8 = (s_57_4.value() as u8);
        // D s_57_6: cast zx s_57_5 -> bv
        let s_57_6: Bits = Bits::new(s_57_5 as u128, 4u16);
        // C s_57_7: const #15u : u8
        let s_57_7: u8 = 15;
        // C s_57_8: cast zx s_57_7 -> bv
        let s_57_8: Bits = Bits::new(s_57_7 as u128, 4u16);
        // D s_57_9: cmp-ne s_57_6 s_57_8
        let s_57_9: bool = ((s_57_6) != (s_57_8));
        // N s_57_10: branch s_57_9 b60 b58
        if s_57_9 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#409035 <= s_58_0
        fn_state.gs_409035 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#409035:u8
        let s_59_0: bool = fn_state.gs_409035;
        // D s_59_1: write-var gs#409036 <= s_59_0
        fn_state.gs_409036 = s_59_0;
        // N s_59_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #3229s : i
        let s_60_0: i128 = 3229;
        // C s_60_1: const #14696u : u32
        let s_60_1: u32 = 14696;
        // D s_60_2: read-reg s_60_1:i
        let s_60_2: i128 = {
            let value = state.read_register::<i128>(s_60_1 as isize);
            tracer.read_register(s_60_1 as isize, value);
            value
        };
        // D s_60_3: cmp-lt s_60_2 s_60_0
        let s_60_3: bool = ((s_60_2) < (s_60_0));
        // D s_60_4: write-var gs#409035 <= s_60_3
        fn_state.gs_409035 = s_60_3;
        // N s_60_5: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #22s : i
        let s_61_0: i128 = 22;
        // D s_61_1: read-var u#33400:u32
        let s_61_1: u32 = fn_state.u_33400;
        // D s_61_2: cast zx s_61_1 -> bv
        let s_61_2: Bits = Bits::new(s_61_1 as u128, 32u16);
        // C s_61_3: const #1s : i64
        let s_61_3: i64 = 1;
        // C s_61_4: cast zx s_61_3 -> i
        let s_61_4: i128 = (i128::try_from(s_61_3).unwrap());
        // C s_61_5: const #0s : i
        let s_61_5: i128 = 0;
        // C s_61_6: add s_61_5 s_61_4
        let s_61_6: i128 = (s_61_5 + s_61_4);
        // D s_61_7: bit-extract s_61_2 s_61_0 s_61_6
        let s_61_7: Bits = (Bits::new(
            ((s_61_2) >> (s_61_0)).value(),
            u16::try_from(s_61_6).unwrap(),
        ));
        // D s_61_8: cast reint s_61_7 -> u8
        let s_61_8: bool = ((s_61_7.value()) != 0);
        // D s_61_9: cast zx s_61_8 -> bv
        let s_61_9: Bits = Bits::new(s_61_8 as u128, 1u16);
        // C s_61_10: const #0u : u8
        let s_61_10: bool = false;
        // C s_61_11: cast zx s_61_10 -> bv
        let s_61_11: Bits = Bits::new(s_61_10 as u128, 1u16);
        // D s_61_12: cmp-eq s_61_9 s_61_11
        let s_61_12: bool = ((s_61_9) == (s_61_11));
        // N s_61_13: branch s_61_12 b64 b62
        if s_61_12 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #0u : u8
        let s_62_0: bool = false;
        // D s_62_1: write-var gs#409030 <= s_62_0
        fn_state.gs_409030 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#409030:u8
        let s_63_0: bool = fn_state.gs_409030;
        // D s_63_1: write-var gs#409031 <= s_63_0
        fn_state.gs_409031 = s_63_0;
        // N s_63_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #20s : i
        let s_64_0: i128 = 20;
        // D s_64_1: read-var u#33400:u32
        let s_64_1: u32 = fn_state.u_33400;
        // D s_64_2: cast zx s_64_1 -> bv
        let s_64_2: Bits = Bits::new(s_64_1 as u128, 32u16);
        // C s_64_3: const #1s : i64
        let s_64_3: i64 = 1;
        // C s_64_4: cast zx s_64_3 -> i
        let s_64_4: i128 = (i128::try_from(s_64_3).unwrap());
        // C s_64_5: const #0s : i
        let s_64_5: i128 = 0;
        // C s_64_6: add s_64_5 s_64_4
        let s_64_6: i128 = (s_64_5 + s_64_4);
        // D s_64_7: bit-extract s_64_2 s_64_0 s_64_6
        let s_64_7: Bits = (Bits::new(
            ((s_64_2) >> (s_64_0)).value(),
            u16::try_from(s_64_6).unwrap(),
        ));
        // D s_64_8: cast reint s_64_7 -> u8
        let s_64_8: bool = ((s_64_7.value()) != 0);
        // D s_64_9: cast zx s_64_8 -> bv
        let s_64_9: Bits = Bits::new(s_64_8 as u128, 1u16);
        // C s_64_10: const #0u : u8
        let s_64_10: bool = false;
        // C s_64_11: cast zx s_64_10 -> bv
        let s_64_11: Bits = Bits::new(s_64_10 as u128, 1u16);
        // D s_64_12: cmp-eq s_64_9 s_64_11
        let s_64_12: bool = ((s_64_9) == (s_64_11));
        // N s_64_13: branch s_64_12 b67 b65
        if s_64_12 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#409029 <= s_65_0
        fn_state.gs_409029 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#409029:u8
        let s_66_0: bool = fn_state.gs_409029;
        // D s_66_1: write-var gs#409030 <= s_66_0
        fn_state.gs_409030 = s_66_0;
        // N s_66_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #4s : i
        let s_67_0: i128 = 4;
        // D s_67_1: read-var u#33400:u32
        let s_67_1: u32 = fn_state.u_33400;
        // D s_67_2: cast zx s_67_1 -> bv
        let s_67_2: Bits = Bits::new(s_67_1 as u128, 32u16);
        // C s_67_3: const #1s : i64
        let s_67_3: i64 = 1;
        // C s_67_4: cast zx s_67_3 -> i
        let s_67_4: i128 = (i128::try_from(s_67_3).unwrap());
        // C s_67_5: const #0s : i
        let s_67_5: i128 = 0;
        // C s_67_6: add s_67_5 s_67_4
        let s_67_6: i128 = (s_67_5 + s_67_4);
        // D s_67_7: bit-extract s_67_2 s_67_0 s_67_6
        let s_67_7: Bits = (Bits::new(
            ((s_67_2) >> (s_67_0)).value(),
            u16::try_from(s_67_6).unwrap(),
        ));
        // D s_67_8: cast reint s_67_7 -> u8
        let s_67_8: bool = ((s_67_7.value()) != 0);
        // D s_67_9: cast zx s_67_8 -> bv
        let s_67_9: Bits = Bits::new(s_67_8 as u128, 1u16);
        // C s_67_10: const #0u : u8
        let s_67_10: bool = false;
        // C s_67_11: cast zx s_67_10 -> bv
        let s_67_11: Bits = Bits::new(s_67_10 as u128, 1u16);
        // D s_67_12: cmp-eq s_67_9 s_67_11
        let s_67_12: bool = ((s_67_9) == (s_67_11));
        // D s_67_13: write-var gs#409029 <= s_67_12
        fn_state.gs_409029 = s_67_12;
        // N s_67_14: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #28s : i
        let s_68_0: i128 = 28;
        // C s_68_1: const #4s : i
        let s_68_1: i128 = 4;
        // D s_68_2: read-var u#33391:u32
        let s_68_2: u32 = fn_state.u_33391;
        // D s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 32u16);
        // D s_68_4: bit-extract s_68_3 s_68_0 s_68_1
        let s_68_4: Bits = (Bits::new(
            ((s_68_3) >> (s_68_0)).value(),
            u16::try_from(s_68_1).unwrap(),
        ));
        // D s_68_5: cast reint s_68_4 -> u8
        let s_68_5: u8 = (s_68_4.value() as u8);
        // D s_68_6: cast zx s_68_5 -> bv
        let s_68_6: Bits = Bits::new(s_68_5 as u128, 4u16);
        // C s_68_7: const #15u : u8
        let s_68_7: u8 = 15;
        // C s_68_8: cast zx s_68_7 -> bv
        let s_68_8: Bits = Bits::new(s_68_7 as u128, 4u16);
        // D s_68_9: cmp-ne s_68_6 s_68_8
        let s_68_9: bool = ((s_68_6) != (s_68_8));
        // N s_68_10: branch s_68_9 b71 b69
        if s_68_9 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#409002 <= s_69_0
        fn_state.gs_409002 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#409002:u8
        let s_70_0: bool = fn_state.gs_409002;
        // D s_70_1: write-var gs#409003 <= s_70_0
        fn_state.gs_409003 = s_70_0;
        // N s_70_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #3201s : i
        let s_71_0: i128 = 3201;
        // C s_71_1: const #14696u : u32
        let s_71_1: u32 = 14696;
        // D s_71_2: read-reg s_71_1:i
        let s_71_2: i128 = {
            let value = state.read_register::<i128>(s_71_1 as isize);
            tracer.read_register(s_71_1 as isize, value);
            value
        };
        // D s_71_3: cmp-lt s_71_2 s_71_0
        let s_71_3: bool = ((s_71_2) < (s_71_0));
        // D s_71_4: write-var gs#409002 <= s_71_3
        fn_state.gs_409002 = s_71_3;
        // N s_71_5: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #20s : i
        let s_72_0: i128 = 20;
        // D s_72_1: read-var u#33391:u32
        let s_72_1: u32 = fn_state.u_33391;
        // D s_72_2: cast zx s_72_1 -> bv
        let s_72_2: Bits = Bits::new(s_72_1 as u128, 32u16);
        // C s_72_3: const #1s : i64
        let s_72_3: i64 = 1;
        // C s_72_4: cast zx s_72_3 -> i
        let s_72_4: i128 = (i128::try_from(s_72_3).unwrap());
        // C s_72_5: const #2s : i
        let s_72_5: i128 = 2;
        // C s_72_6: add s_72_5 s_72_4
        let s_72_6: i128 = (s_72_5 + s_72_4);
        // D s_72_7: bit-extract s_72_2 s_72_0 s_72_6
        let s_72_7: Bits = (Bits::new(
            ((s_72_2) >> (s_72_0)).value(),
            u16::try_from(s_72_6).unwrap(),
        ));
        // D s_72_8: cast reint s_72_7 -> u8
        let s_72_8: u8 = (s_72_7.value() as u8);
        // D s_72_9: cast zx s_72_8 -> bv
        let s_72_9: Bits = Bits::new(s_72_8 as u128, 3u16);
        // C s_72_10: const #6u : u8
        let s_72_10: u8 = 6;
        // C s_72_11: cast zx s_72_10 -> bv
        let s_72_11: Bits = Bits::new(s_72_10 as u128, 3u16);
        // D s_72_12: cmp-eq s_72_9 s_72_11
        let s_72_12: bool = ((s_72_9) == (s_72_11));
        // N s_72_13: branch s_72_12 b75 b73
        if s_72_12 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#408997 <= s_73_0
        fn_state.gs_408997 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#408997:u8
        let s_74_0: bool = fn_state.gs_408997;
        // D s_74_1: write-var gs#408998 <= s_74_0
        fn_state.gs_408998 = s_74_0;
        // N s_74_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #4s : i
        let s_75_0: i128 = 4;
        // D s_75_1: read-var u#33391:u32
        let s_75_1: u32 = fn_state.u_33391;
        // D s_75_2: cast zx s_75_1 -> bv
        let s_75_2: Bits = Bits::new(s_75_1 as u128, 32u16);
        // C s_75_3: const #1s : i64
        let s_75_3: i64 = 1;
        // C s_75_4: cast zx s_75_3 -> i
        let s_75_4: i128 = (i128::try_from(s_75_3).unwrap());
        // C s_75_5: const #0s : i
        let s_75_5: i128 = 0;
        // C s_75_6: add s_75_5 s_75_4
        let s_75_6: i128 = (s_75_5 + s_75_4);
        // D s_75_7: bit-extract s_75_2 s_75_0 s_75_6
        let s_75_7: Bits = (Bits::new(
            ((s_75_2) >> (s_75_0)).value(),
            u16::try_from(s_75_6).unwrap(),
        ));
        // D s_75_8: cast reint s_75_7 -> u8
        let s_75_8: bool = ((s_75_7.value()) != 0);
        // D s_75_9: cast zx s_75_8 -> bv
        let s_75_9: Bits = Bits::new(s_75_8 as u128, 1u16);
        // C s_75_10: const #0u : u8
        let s_75_10: bool = false;
        // C s_75_11: cast zx s_75_10 -> bv
        let s_75_11: Bits = Bits::new(s_75_10 as u128, 1u16);
        // D s_75_12: cmp-eq s_75_9 s_75_11
        let s_75_12: bool = ((s_75_9) == (s_75_11));
        // D s_75_13: write-var gs#408997 <= s_75_12
        fn_state.gs_408997 = s_75_12;
        // N s_75_14: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #28s : i
        let s_76_0: i128 = 28;
        // C s_76_1: const #4s : i
        let s_76_1: i128 = 4;
        // D s_76_2: read-var u#33380:u32
        let s_76_2: u32 = fn_state.u_33380;
        // D s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 32u16);
        // D s_76_4: bit-extract s_76_3 s_76_0 s_76_1
        let s_76_4: Bits = (Bits::new(
            ((s_76_3) >> (s_76_0)).value(),
            u16::try_from(s_76_1).unwrap(),
        ));
        // D s_76_5: cast reint s_76_4 -> u8
        let s_76_5: u8 = (s_76_4.value() as u8);
        // D s_76_6: cast zx s_76_5 -> bv
        let s_76_6: Bits = Bits::new(s_76_5 as u128, 4u16);
        // C s_76_7: const #15u : u8
        let s_76_7: u8 = 15;
        // C s_76_8: cast zx s_76_7 -> bv
        let s_76_8: Bits = Bits::new(s_76_7 as u128, 4u16);
        // D s_76_9: cmp-ne s_76_6 s_76_8
        let s_76_9: bool = ((s_76_6) != (s_76_8));
        // N s_76_10: branch s_76_9 b79 b77
        if s_76_9 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#408968 <= s_77_0
        fn_state.gs_408968 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#408968:u8
        let s_78_0: bool = fn_state.gs_408968;
        // D s_78_1: write-var gs#408969 <= s_78_0
        fn_state.gs_408969 = s_78_0;
        // N s_78_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #3197s : i
        let s_79_0: i128 = 3197;
        // C s_79_1: const #14696u : u32
        let s_79_1: u32 = 14696;
        // D s_79_2: read-reg s_79_1:i
        let s_79_2: i128 = {
            let value = state.read_register::<i128>(s_79_1 as isize);
            tracer.read_register(s_79_1 as isize, value);
            value
        };
        // D s_79_3: cmp-lt s_79_2 s_79_0
        let s_79_3: bool = ((s_79_2) < (s_79_0));
        // D s_79_4: write-var gs#408968 <= s_79_3
        fn_state.gs_408968 = s_79_3;
        // N s_79_5: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #22s : i
        let s_80_0: i128 = 22;
        // D s_80_1: read-var u#33380:u32
        let s_80_1: u32 = fn_state.u_33380;
        // D s_80_2: cast zx s_80_1 -> bv
        let s_80_2: Bits = Bits::new(s_80_1 as u128, 32u16);
        // C s_80_3: const #1s : i64
        let s_80_3: i64 = 1;
        // C s_80_4: cast zx s_80_3 -> i
        let s_80_4: i128 = (i128::try_from(s_80_3).unwrap());
        // C s_80_5: const #0s : i
        let s_80_5: i128 = 0;
        // C s_80_6: add s_80_5 s_80_4
        let s_80_6: i128 = (s_80_5 + s_80_4);
        // D s_80_7: bit-extract s_80_2 s_80_0 s_80_6
        let s_80_7: Bits = (Bits::new(
            ((s_80_2) >> (s_80_0)).value(),
            u16::try_from(s_80_6).unwrap(),
        ));
        // D s_80_8: cast reint s_80_7 -> u8
        let s_80_8: bool = ((s_80_7.value()) != 0);
        // D s_80_9: cast zx s_80_8 -> bv
        let s_80_9: Bits = Bits::new(s_80_8 as u128, 1u16);
        // C s_80_10: const #1u : u8
        let s_80_10: bool = true;
        // C s_80_11: cast zx s_80_10 -> bv
        let s_80_11: Bits = Bits::new(s_80_10 as u128, 1u16);
        // D s_80_12: cmp-eq s_80_9 s_80_11
        let s_80_12: bool = ((s_80_9) == (s_80_11));
        // N s_80_13: branch s_80_12 b83 b81
        if s_80_12 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#408963 <= s_81_0
        fn_state.gs_408963 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#408963:u8
        let s_82_0: bool = fn_state.gs_408963;
        // D s_82_1: write-var gs#408964 <= s_82_0
        fn_state.gs_408964 = s_82_0;
        // N s_82_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #20s : i
        let s_83_0: i128 = 20;
        // D s_83_1: read-var u#33380:u32
        let s_83_1: u32 = fn_state.u_33380;
        // D s_83_2: cast zx s_83_1 -> bv
        let s_83_2: Bits = Bits::new(s_83_1 as u128, 32u16);
        // C s_83_3: const #1s : i64
        let s_83_3: i64 = 1;
        // C s_83_4: cast zx s_83_3 -> i
        let s_83_4: i128 = (i128::try_from(s_83_3).unwrap());
        // C s_83_5: const #0s : i
        let s_83_5: i128 = 0;
        // C s_83_6: add s_83_5 s_83_4
        let s_83_6: i128 = (s_83_5 + s_83_4);
        // D s_83_7: bit-extract s_83_2 s_83_0 s_83_6
        let s_83_7: Bits = (Bits::new(
            ((s_83_2) >> (s_83_0)).value(),
            u16::try_from(s_83_6).unwrap(),
        ));
        // D s_83_8: cast reint s_83_7 -> u8
        let s_83_8: bool = ((s_83_7.value()) != 0);
        // D s_83_9: cast zx s_83_8 -> bv
        let s_83_9: Bits = Bits::new(s_83_8 as u128, 1u16);
        // C s_83_10: const #0u : u8
        let s_83_10: bool = false;
        // C s_83_11: cast zx s_83_10 -> bv
        let s_83_11: Bits = Bits::new(s_83_10 as u128, 1u16);
        // D s_83_12: cmp-eq s_83_9 s_83_11
        let s_83_12: bool = ((s_83_9) == (s_83_11));
        // N s_83_13: branch s_83_12 b86 b84
        if s_83_12 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #0u : u8
        let s_84_0: bool = false;
        // D s_84_1: write-var gs#408962 <= s_84_0
        fn_state.gs_408962 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#408962:u8
        let s_85_0: bool = fn_state.gs_408962;
        // D s_85_1: write-var gs#408963 <= s_85_0
        fn_state.gs_408963 = s_85_0;
        // N s_85_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #4s : i
        let s_86_0: i128 = 4;
        // D s_86_1: read-var u#33380:u32
        let s_86_1: u32 = fn_state.u_33380;
        // D s_86_2: cast zx s_86_1 -> bv
        let s_86_2: Bits = Bits::new(s_86_1 as u128, 32u16);
        // C s_86_3: const #1s : i64
        let s_86_3: i64 = 1;
        // C s_86_4: cast zx s_86_3 -> i
        let s_86_4: i128 = (i128::try_from(s_86_3).unwrap());
        // C s_86_5: const #0s : i
        let s_86_5: i128 = 0;
        // C s_86_6: add s_86_5 s_86_4
        let s_86_6: i128 = (s_86_5 + s_86_4);
        // D s_86_7: bit-extract s_86_2 s_86_0 s_86_6
        let s_86_7: Bits = (Bits::new(
            ((s_86_2) >> (s_86_0)).value(),
            u16::try_from(s_86_6).unwrap(),
        ));
        // D s_86_8: cast reint s_86_7 -> u8
        let s_86_8: bool = ((s_86_7.value()) != 0);
        // D s_86_9: cast zx s_86_8 -> bv
        let s_86_9: Bits = Bits::new(s_86_8 as u128, 1u16);
        // C s_86_10: const #0u : u8
        let s_86_10: bool = false;
        // C s_86_11: cast zx s_86_10 -> bv
        let s_86_11: Bits = Bits::new(s_86_10 as u128, 1u16);
        // D s_86_12: cmp-eq s_86_9 s_86_11
        let s_86_12: bool = ((s_86_9) == (s_86_11));
        // D s_86_13: write-var gs#408962 <= s_86_12
        fn_state.gs_408962 = s_86_12;
        // N s_86_14: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #28s : i
        let s_87_0: i128 = 28;
        // C s_87_1: const #4s : i
        let s_87_1: i128 = 4;
        // D s_87_2: read-var u#33371:u32
        let s_87_2: u32 = fn_state.u_33371;
        // D s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 32u16);
        // D s_87_4: bit-extract s_87_3 s_87_0 s_87_1
        let s_87_4: Bits = (Bits::new(
            ((s_87_3) >> (s_87_0)).value(),
            u16::try_from(s_87_1).unwrap(),
        ));
        // D s_87_5: cast reint s_87_4 -> u8
        let s_87_5: u8 = (s_87_4.value() as u8);
        // D s_87_6: cast zx s_87_5 -> bv
        let s_87_6: Bits = Bits::new(s_87_5 as u128, 4u16);
        // C s_87_7: const #15u : u8
        let s_87_7: u8 = 15;
        // C s_87_8: cast zx s_87_7 -> bv
        let s_87_8: Bits = Bits::new(s_87_7 as u128, 4u16);
        // D s_87_9: cmp-ne s_87_6 s_87_8
        let s_87_9: bool = ((s_87_6) != (s_87_8));
        // N s_87_10: branch s_87_9 b90 b88
        if s_87_9 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var gs#408935 <= s_88_0
        fn_state.gs_408935 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#408935:u8
        let s_89_0: bool = fn_state.gs_408935;
        // D s_89_1: write-var gs#408936 <= s_89_0
        fn_state.gs_408936 = s_89_0;
        // N s_89_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #2994s : i
        let s_90_0: i128 = 2994;
        // C s_90_1: const #14696u : u32
        let s_90_1: u32 = 14696;
        // D s_90_2: read-reg s_90_1:i
        let s_90_2: i128 = {
            let value = state.read_register::<i128>(s_90_1 as isize);
            tracer.read_register(s_90_1 as isize, value);
            value
        };
        // D s_90_3: cmp-lt s_90_2 s_90_0
        let s_90_3: bool = ((s_90_2) < (s_90_0));
        // D s_90_4: write-var gs#408935 <= s_90_3
        fn_state.gs_408935 = s_90_3;
        // N s_90_5: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #20s : i
        let s_91_0: i128 = 20;
        // D s_91_1: read-var u#33371:u32
        let s_91_1: u32 = fn_state.u_33371;
        // D s_91_2: cast zx s_91_1 -> bv
        let s_91_2: Bits = Bits::new(s_91_1 as u128, 32u16);
        // C s_91_3: const #1s : i64
        let s_91_3: i64 = 1;
        // C s_91_4: cast zx s_91_3 -> i
        let s_91_4: i128 = (i128::try_from(s_91_3).unwrap());
        // C s_91_5: const #2s : i
        let s_91_5: i128 = 2;
        // C s_91_6: add s_91_5 s_91_4
        let s_91_6: i128 = (s_91_5 + s_91_4);
        // D s_91_7: bit-extract s_91_2 s_91_0 s_91_6
        let s_91_7: Bits = (Bits::new(
            ((s_91_2) >> (s_91_0)).value(),
            u16::try_from(s_91_6).unwrap(),
        ));
        // D s_91_8: cast reint s_91_7 -> u8
        let s_91_8: u8 = (s_91_7.value() as u8);
        // D s_91_9: cast zx s_91_8 -> bv
        let s_91_9: Bits = Bits::new(s_91_8 as u128, 3u16);
        // C s_91_10: const #3u : u8
        let s_91_10: u8 = 3;
        // C s_91_11: cast zx s_91_10 -> bv
        let s_91_11: Bits = Bits::new(s_91_10 as u128, 3u16);
        // D s_91_12: cmp-eq s_91_9 s_91_11
        let s_91_12: bool = ((s_91_9) == (s_91_11));
        // N s_91_13: branch s_91_12 b94 b92
        if s_91_12 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #0u : u8
        let s_92_0: bool = false;
        // D s_92_1: write-var gs#408930 <= s_92_0
        fn_state.gs_408930 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#408930:u8
        let s_93_0: bool = fn_state.gs_408930;
        // D s_93_1: write-var gs#408931 <= s_93_0
        fn_state.gs_408931 = s_93_0;
        // N s_93_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #4s : i
        let s_94_0: i128 = 4;
        // D s_94_1: read-var u#33371:u32
        let s_94_1: u32 = fn_state.u_33371;
        // D s_94_2: cast zx s_94_1 -> bv
        let s_94_2: Bits = Bits::new(s_94_1 as u128, 32u16);
        // C s_94_3: const #1s : i64
        let s_94_3: i64 = 1;
        // C s_94_4: cast zx s_94_3 -> i
        let s_94_4: i128 = (i128::try_from(s_94_3).unwrap());
        // C s_94_5: const #0s : i
        let s_94_5: i128 = 0;
        // C s_94_6: add s_94_5 s_94_4
        let s_94_6: i128 = (s_94_5 + s_94_4);
        // D s_94_7: bit-extract s_94_2 s_94_0 s_94_6
        let s_94_7: Bits = (Bits::new(
            ((s_94_2) >> (s_94_0)).value(),
            u16::try_from(s_94_6).unwrap(),
        ));
        // D s_94_8: cast reint s_94_7 -> u8
        let s_94_8: bool = ((s_94_7.value()) != 0);
        // D s_94_9: cast zx s_94_8 -> bv
        let s_94_9: Bits = Bits::new(s_94_8 as u128, 1u16);
        // C s_94_10: const #0u : u8
        let s_94_10: bool = false;
        // C s_94_11: cast zx s_94_10 -> bv
        let s_94_11: Bits = Bits::new(s_94_10 as u128, 1u16);
        // D s_94_12: cmp-eq s_94_9 s_94_11
        let s_94_12: bool = ((s_94_9) == (s_94_11));
        // D s_94_13: write-var gs#408930 <= s_94_12
        fn_state.gs_408930 = s_94_12;
        // N s_94_14: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #28s : i
        let s_95_0: i128 = 28;
        // C s_95_1: const #4s : i
        let s_95_1: i128 = 4;
        // D s_95_2: read-var u#33360:u32
        let s_95_2: u32 = fn_state.u_33360;
        // D s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 32u16);
        // D s_95_4: bit-extract s_95_3 s_95_0 s_95_1
        let s_95_4: Bits = (Bits::new(
            ((s_95_3) >> (s_95_0)).value(),
            u16::try_from(s_95_1).unwrap(),
        ));
        // D s_95_5: cast reint s_95_4 -> u8
        let s_95_5: u8 = (s_95_4.value() as u8);
        // D s_95_6: cast zx s_95_5 -> bv
        let s_95_6: Bits = Bits::new(s_95_5 as u128, 4u16);
        // C s_95_7: const #15u : u8
        let s_95_7: u8 = 15;
        // C s_95_8: cast zx s_95_7 -> bv
        let s_95_8: Bits = Bits::new(s_95_7 as u128, 4u16);
        // D s_95_9: cmp-ne s_95_6 s_95_8
        let s_95_9: bool = ((s_95_6) != (s_95_8));
        // N s_95_10: branch s_95_9 b98 b96
        if s_95_9 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #0u : u8
        let s_96_0: bool = false;
        // D s_96_1: write-var gs#408901 <= s_96_0
        fn_state.gs_408901 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#408901:u8
        let s_97_0: bool = fn_state.gs_408901;
        // D s_97_1: write-var gs#408902 <= s_97_0
        fn_state.gs_408902 = s_97_0;
        // N s_97_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #2968s : i
        let s_98_0: i128 = 2968;
        // C s_98_1: const #14696u : u32
        let s_98_1: u32 = 14696;
        // D s_98_2: read-reg s_98_1:i
        let s_98_2: i128 = {
            let value = state.read_register::<i128>(s_98_1 as isize);
            tracer.read_register(s_98_1 as isize, value);
            value
        };
        // D s_98_3: cmp-lt s_98_2 s_98_0
        let s_98_3: bool = ((s_98_2) < (s_98_0));
        // D s_98_4: write-var gs#408901 <= s_98_3
        fn_state.gs_408901 = s_98_3;
        // N s_98_5: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #22s : i
        let s_99_0: i128 = 22;
        // D s_99_1: read-var u#33360:u32
        let s_99_1: u32 = fn_state.u_33360;
        // D s_99_2: cast zx s_99_1 -> bv
        let s_99_2: Bits = Bits::new(s_99_1 as u128, 32u16);
        // C s_99_3: const #1s : i64
        let s_99_3: i64 = 1;
        // C s_99_4: cast zx s_99_3 -> i
        let s_99_4: i128 = (i128::try_from(s_99_3).unwrap());
        // C s_99_5: const #0s : i
        let s_99_5: i128 = 0;
        // C s_99_6: add s_99_5 s_99_4
        let s_99_6: i128 = (s_99_5 + s_99_4);
        // D s_99_7: bit-extract s_99_2 s_99_0 s_99_6
        let s_99_7: Bits = (Bits::new(
            ((s_99_2) >> (s_99_0)).value(),
            u16::try_from(s_99_6).unwrap(),
        ));
        // D s_99_8: cast reint s_99_7 -> u8
        let s_99_8: bool = ((s_99_7.value()) != 0);
        // D s_99_9: cast zx s_99_8 -> bv
        let s_99_9: Bits = Bits::new(s_99_8 as u128, 1u16);
        // C s_99_10: const #0u : u8
        let s_99_10: bool = false;
        // C s_99_11: cast zx s_99_10 -> bv
        let s_99_11: Bits = Bits::new(s_99_10 as u128, 1u16);
        // D s_99_12: cmp-eq s_99_9 s_99_11
        let s_99_12: bool = ((s_99_9) == (s_99_11));
        // N s_99_13: branch s_99_12 b102 b100
        if s_99_12 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #0u : u8
        let s_100_0: bool = false;
        // D s_100_1: write-var gs#408896 <= s_100_0
        fn_state.gs_408896 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#408896:u8
        let s_101_0: bool = fn_state.gs_408896;
        // D s_101_1: write-var gs#408897 <= s_101_0
        fn_state.gs_408897 = s_101_0;
        // N s_101_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #20s : i
        let s_102_0: i128 = 20;
        // D s_102_1: read-var u#33360:u32
        let s_102_1: u32 = fn_state.u_33360;
        // D s_102_2: cast zx s_102_1 -> bv
        let s_102_2: Bits = Bits::new(s_102_1 as u128, 32u16);
        // C s_102_3: const #1s : i64
        let s_102_3: i64 = 1;
        // C s_102_4: cast zx s_102_3 -> i
        let s_102_4: i128 = (i128::try_from(s_102_3).unwrap());
        // C s_102_5: const #0s : i
        let s_102_5: i128 = 0;
        // C s_102_6: add s_102_5 s_102_4
        let s_102_6: i128 = (s_102_5 + s_102_4);
        // D s_102_7: bit-extract s_102_2 s_102_0 s_102_6
        let s_102_7: Bits = (Bits::new(
            ((s_102_2) >> (s_102_0)).value(),
            u16::try_from(s_102_6).unwrap(),
        ));
        // D s_102_8: cast reint s_102_7 -> u8
        let s_102_8: bool = ((s_102_7.value()) != 0);
        // D s_102_9: cast zx s_102_8 -> bv
        let s_102_9: Bits = Bits::new(s_102_8 as u128, 1u16);
        // C s_102_10: const #1u : u8
        let s_102_10: bool = true;
        // C s_102_11: cast zx s_102_10 -> bv
        let s_102_11: Bits = Bits::new(s_102_10 as u128, 1u16);
        // D s_102_12: cmp-eq s_102_9 s_102_11
        let s_102_12: bool = ((s_102_9) == (s_102_11));
        // N s_102_13: branch s_102_12 b105 b103
        if s_102_12 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // D s_103_1: write-var gs#408895 <= s_103_0
        fn_state.gs_408895 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#408895:u8
        let s_104_0: bool = fn_state.gs_408895;
        // D s_104_1: write-var gs#408896 <= s_104_0
        fn_state.gs_408896 = s_104_0;
        // N s_104_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #4s : i
        let s_105_0: i128 = 4;
        // D s_105_1: read-var u#33360:u32
        let s_105_1: u32 = fn_state.u_33360;
        // D s_105_2: cast zx s_105_1 -> bv
        let s_105_2: Bits = Bits::new(s_105_1 as u128, 32u16);
        // C s_105_3: const #1s : i64
        let s_105_3: i64 = 1;
        // C s_105_4: cast zx s_105_3 -> i
        let s_105_4: i128 = (i128::try_from(s_105_3).unwrap());
        // C s_105_5: const #0s : i
        let s_105_5: i128 = 0;
        // C s_105_6: add s_105_5 s_105_4
        let s_105_6: i128 = (s_105_5 + s_105_4);
        // D s_105_7: bit-extract s_105_2 s_105_0 s_105_6
        let s_105_7: Bits = (Bits::new(
            ((s_105_2) >> (s_105_0)).value(),
            u16::try_from(s_105_6).unwrap(),
        ));
        // D s_105_8: cast reint s_105_7 -> u8
        let s_105_8: bool = ((s_105_7.value()) != 0);
        // D s_105_9: cast zx s_105_8 -> bv
        let s_105_9: Bits = Bits::new(s_105_8 as u128, 1u16);
        // C s_105_10: const #0u : u8
        let s_105_10: bool = false;
        // C s_105_11: cast zx s_105_10 -> bv
        let s_105_11: Bits = Bits::new(s_105_10 as u128, 1u16);
        // D s_105_12: cmp-eq s_105_9 s_105_11
        let s_105_12: bool = ((s_105_9) == (s_105_11));
        // D s_105_13: write-var gs#408895 <= s_105_12
        fn_state.gs_408895 = s_105_12;
        // N s_105_14: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #28s : i
        let s_106_0: i128 = 28;
        // C s_106_1: const #4s : i
        let s_106_1: i128 = 4;
        // D s_106_2: read-var u#33351:u32
        let s_106_2: u32 = fn_state.u_33351;
        // D s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 32u16);
        // D s_106_4: bit-extract s_106_3 s_106_0 s_106_1
        let s_106_4: Bits = (Bits::new(
            ((s_106_3) >> (s_106_0)).value(),
            u16::try_from(s_106_1).unwrap(),
        ));
        // D s_106_5: cast reint s_106_4 -> u8
        let s_106_5: u8 = (s_106_4.value() as u8);
        // D s_106_6: cast zx s_106_5 -> bv
        let s_106_6: Bits = Bits::new(s_106_5 as u128, 4u16);
        // C s_106_7: const #15u : u8
        let s_106_7: u8 = 15;
        // C s_106_8: cast zx s_106_7 -> bv
        let s_106_8: Bits = Bits::new(s_106_7 as u128, 4u16);
        // D s_106_9: cmp-ne s_106_6 s_106_8
        let s_106_9: bool = ((s_106_6) != (s_106_8));
        // N s_106_10: branch s_106_9 b109 b107
        if s_106_9 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #0u : u8
        let s_107_0: bool = false;
        // D s_107_1: write-var gs#408868 <= s_107_0
        fn_state.gs_408868 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#408868:u8
        let s_108_0: bool = fn_state.gs_408868;
        // D s_108_1: write-var gs#408869 <= s_108_0
        fn_state.gs_408869 = s_108_0;
        // N s_108_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #2933s : i
        let s_109_0: i128 = 2933;
        // C s_109_1: const #14696u : u32
        let s_109_1: u32 = 14696;
        // D s_109_2: read-reg s_109_1:i
        let s_109_2: i128 = {
            let value = state.read_register::<i128>(s_109_1 as isize);
            tracer.read_register(s_109_1 as isize, value);
            value
        };
        // D s_109_3: cmp-lt s_109_2 s_109_0
        let s_109_3: bool = ((s_109_2) < (s_109_0));
        // D s_109_4: write-var gs#408868 <= s_109_3
        fn_state.gs_408868 = s_109_3;
        // N s_109_5: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #20s : i
        let s_110_0: i128 = 20;
        // D s_110_1: read-var u#33351:u32
        let s_110_1: u32 = fn_state.u_33351;
        // D s_110_2: cast zx s_110_1 -> bv
        let s_110_2: Bits = Bits::new(s_110_1 as u128, 32u16);
        // C s_110_3: const #1s : i64
        let s_110_3: i64 = 1;
        // C s_110_4: cast zx s_110_3 -> i
        let s_110_4: i128 = (i128::try_from(s_110_3).unwrap());
        // C s_110_5: const #2s : i
        let s_110_5: i128 = 2;
        // C s_110_6: add s_110_5 s_110_4
        let s_110_6: i128 = (s_110_5 + s_110_4);
        // D s_110_7: bit-extract s_110_2 s_110_0 s_110_6
        let s_110_7: Bits = (Bits::new(
            ((s_110_2) >> (s_110_0)).value(),
            u16::try_from(s_110_6).unwrap(),
        ));
        // D s_110_8: cast reint s_110_7 -> u8
        let s_110_8: u8 = (s_110_7.value() as u8);
        // D s_110_9: cast zx s_110_8 -> bv
        let s_110_9: Bits = Bits::new(s_110_8 as u128, 3u16);
        // C s_110_10: const #7u : u8
        let s_110_10: u8 = 7;
        // C s_110_11: cast zx s_110_10 -> bv
        let s_110_11: Bits = Bits::new(s_110_10 as u128, 3u16);
        // D s_110_12: cmp-eq s_110_9 s_110_11
        let s_110_12: bool = ((s_110_9) == (s_110_11));
        // N s_110_13: branch s_110_12 b113 b111
        if s_110_12 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #0u : u8
        let s_111_0: bool = false;
        // D s_111_1: write-var gs#408863 <= s_111_0
        fn_state.gs_408863 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#408863:u8
        let s_112_0: bool = fn_state.gs_408863;
        // D s_112_1: write-var gs#408864 <= s_112_0
        fn_state.gs_408864 = s_112_0;
        // N s_112_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #4s : i
        let s_113_0: i128 = 4;
        // D s_113_1: read-var u#33351:u32
        let s_113_1: u32 = fn_state.u_33351;
        // D s_113_2: cast zx s_113_1 -> bv
        let s_113_2: Bits = Bits::new(s_113_1 as u128, 32u16);
        // C s_113_3: const #1s : i64
        let s_113_3: i64 = 1;
        // C s_113_4: cast zx s_113_3 -> i
        let s_113_4: i128 = (i128::try_from(s_113_3).unwrap());
        // C s_113_5: const #0s : i
        let s_113_5: i128 = 0;
        // C s_113_6: add s_113_5 s_113_4
        let s_113_6: i128 = (s_113_5 + s_113_4);
        // D s_113_7: bit-extract s_113_2 s_113_0 s_113_6
        let s_113_7: Bits = (Bits::new(
            ((s_113_2) >> (s_113_0)).value(),
            u16::try_from(s_113_6).unwrap(),
        ));
        // D s_113_8: cast reint s_113_7 -> u8
        let s_113_8: bool = ((s_113_7.value()) != 0);
        // D s_113_9: cast zx s_113_8 -> bv
        let s_113_9: Bits = Bits::new(s_113_8 as u128, 1u16);
        // C s_113_10: const #0u : u8
        let s_113_10: bool = false;
        // C s_113_11: cast zx s_113_10 -> bv
        let s_113_11: Bits = Bits::new(s_113_10 as u128, 1u16);
        // D s_113_12: cmp-eq s_113_9 s_113_11
        let s_113_12: bool = ((s_113_9) == (s_113_11));
        // D s_113_13: write-var gs#408863 <= s_113_12
        fn_state.gs_408863 = s_113_12;
        // N s_113_14: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #28s : i
        let s_114_0: i128 = 28;
        // C s_114_1: const #4s : i
        let s_114_1: i128 = 4;
        // D s_114_2: read-var __opcode:u32
        let s_114_2: u32 = fn_state.u__opcode;
        // D s_114_3: cast zx s_114_2 -> bv
        let s_114_3: Bits = Bits::new(s_114_2 as u128, 32u16);
        // D s_114_4: bit-extract s_114_3 s_114_0 s_114_1
        let s_114_4: Bits = (Bits::new(
            ((s_114_3) >> (s_114_0)).value(),
            u16::try_from(s_114_1).unwrap(),
        ));
        // D s_114_5: cast reint s_114_4 -> u8
        let s_114_5: u8 = (s_114_4.value() as u8);
        // D s_114_6: cast zx s_114_5 -> bv
        let s_114_6: Bits = Bits::new(s_114_5 as u128, 4u16);
        // C s_114_7: const #15u : u8
        let s_114_7: u8 = 15;
        // C s_114_8: cast zx s_114_7 -> bv
        let s_114_8: Bits = Bits::new(s_114_7 as u128, 4u16);
        // D s_114_9: cmp-ne s_114_6 s_114_8
        let s_114_9: bool = ((s_114_6) != (s_114_8));
        // N s_114_10: branch s_114_9 b117 b115
        if s_114_9 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #0u : u8
        let s_115_0: bool = false;
        // D s_115_1: write-var gs#408834 <= s_115_0
        fn_state.gs_408834 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#408834:u8
        let s_116_0: bool = fn_state.gs_408834;
        // D s_116_1: write-var gs#408835 <= s_116_0
        fn_state.gs_408835 = s_116_0;
        // N s_116_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #2929s : i
        let s_117_0: i128 = 2929;
        // C s_117_1: const #14696u : u32
        let s_117_1: u32 = 14696;
        // D s_117_2: read-reg s_117_1:i
        let s_117_2: i128 = {
            let value = state.read_register::<i128>(s_117_1 as isize);
            tracer.read_register(s_117_1 as isize, value);
            value
        };
        // D s_117_3: cmp-lt s_117_2 s_117_0
        let s_117_3: bool = ((s_117_2) < (s_117_0));
        // D s_117_4: write-var gs#408834 <= s_117_3
        fn_state.gs_408834 = s_117_3;
        // N s_117_5: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #22s : i
        let s_118_0: i128 = 22;
        // D s_118_1: read-var __opcode:u32
        let s_118_1: u32 = fn_state.u__opcode;
        // D s_118_2: cast zx s_118_1 -> bv
        let s_118_2: Bits = Bits::new(s_118_1 as u128, 32u16);
        // C s_118_3: const #1s : i64
        let s_118_3: i64 = 1;
        // C s_118_4: cast zx s_118_3 -> i
        let s_118_4: i128 = (i128::try_from(s_118_3).unwrap());
        // C s_118_5: const #0s : i
        let s_118_5: i128 = 0;
        // C s_118_6: add s_118_5 s_118_4
        let s_118_6: i128 = (s_118_5 + s_118_4);
        // D s_118_7: bit-extract s_118_2 s_118_0 s_118_6
        let s_118_7: Bits = (Bits::new(
            ((s_118_2) >> (s_118_0)).value(),
            u16::try_from(s_118_6).unwrap(),
        ));
        // D s_118_8: cast reint s_118_7 -> u8
        let s_118_8: bool = ((s_118_7.value()) != 0);
        // D s_118_9: cast zx s_118_8 -> bv
        let s_118_9: Bits = Bits::new(s_118_8 as u128, 1u16);
        // C s_118_10: const #1u : u8
        let s_118_10: bool = true;
        // C s_118_11: cast zx s_118_10 -> bv
        let s_118_11: Bits = Bits::new(s_118_10 as u128, 1u16);
        // D s_118_12: cmp-eq s_118_9 s_118_11
        let s_118_12: bool = ((s_118_9) == (s_118_11));
        // N s_118_13: branch s_118_12 b121 b119
        if s_118_12 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #0u : u8
        let s_119_0: bool = false;
        // D s_119_1: write-var gs#408829 <= s_119_0
        fn_state.gs_408829 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#408829:u8
        let s_120_0: bool = fn_state.gs_408829;
        // D s_120_1: write-var gs#408830 <= s_120_0
        fn_state.gs_408830 = s_120_0;
        // N s_120_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #20s : i
        let s_121_0: i128 = 20;
        // D s_121_1: read-var __opcode:u32
        let s_121_1: u32 = fn_state.u__opcode;
        // D s_121_2: cast zx s_121_1 -> bv
        let s_121_2: Bits = Bits::new(s_121_1 as u128, 32u16);
        // C s_121_3: const #1s : i64
        let s_121_3: i64 = 1;
        // C s_121_4: cast zx s_121_3 -> i
        let s_121_4: i128 = (i128::try_from(s_121_3).unwrap());
        // C s_121_5: const #0s : i
        let s_121_5: i128 = 0;
        // C s_121_6: add s_121_5 s_121_4
        let s_121_6: i128 = (s_121_5 + s_121_4);
        // D s_121_7: bit-extract s_121_2 s_121_0 s_121_6
        let s_121_7: Bits = (Bits::new(
            ((s_121_2) >> (s_121_0)).value(),
            u16::try_from(s_121_6).unwrap(),
        ));
        // D s_121_8: cast reint s_121_7 -> u8
        let s_121_8: bool = ((s_121_7.value()) != 0);
        // D s_121_9: cast zx s_121_8 -> bv
        let s_121_9: Bits = Bits::new(s_121_8 as u128, 1u16);
        // C s_121_10: const #1u : u8
        let s_121_10: bool = true;
        // C s_121_11: cast zx s_121_10 -> bv
        let s_121_11: Bits = Bits::new(s_121_10 as u128, 1u16);
        // D s_121_12: cmp-eq s_121_9 s_121_11
        let s_121_12: bool = ((s_121_9) == (s_121_11));
        // N s_121_13: branch s_121_12 b124 b122
        if s_121_12 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #0u : u8
        let s_122_0: bool = false;
        // D s_122_1: write-var gs#408828 <= s_122_0
        fn_state.gs_408828 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#408828:u8
        let s_123_0: bool = fn_state.gs_408828;
        // D s_123_1: write-var gs#408829 <= s_123_0
        fn_state.gs_408829 = s_123_0;
        // N s_123_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #4s : i
        let s_124_0: i128 = 4;
        // D s_124_1: read-var __opcode:u32
        let s_124_1: u32 = fn_state.u__opcode;
        // D s_124_2: cast zx s_124_1 -> bv
        let s_124_2: Bits = Bits::new(s_124_1 as u128, 32u16);
        // C s_124_3: const #1s : i64
        let s_124_3: i64 = 1;
        // C s_124_4: cast zx s_124_3 -> i
        let s_124_4: i128 = (i128::try_from(s_124_3).unwrap());
        // C s_124_5: const #0s : i
        let s_124_5: i128 = 0;
        // C s_124_6: add s_124_5 s_124_4
        let s_124_6: i128 = (s_124_5 + s_124_4);
        // D s_124_7: bit-extract s_124_2 s_124_0 s_124_6
        let s_124_7: Bits = (Bits::new(
            ((s_124_2) >> (s_124_0)).value(),
            u16::try_from(s_124_6).unwrap(),
        ));
        // D s_124_8: cast reint s_124_7 -> u8
        let s_124_8: bool = ((s_124_7.value()) != 0);
        // D s_124_9: cast zx s_124_8 -> bv
        let s_124_9: Bits = Bits::new(s_124_8 as u128, 1u16);
        // C s_124_10: const #0u : u8
        let s_124_10: bool = false;
        // C s_124_11: cast zx s_124_10 -> bv
        let s_124_11: Bits = Bits::new(s_124_10 as u128, 1u16);
        // D s_124_12: cmp-eq s_124_9 s_124_11
        let s_124_12: bool = ((s_124_9) == (s_124_11));
        // D s_124_13: write-var gs#408828 <= s_124_12
        fn_state.gs_408828 = s_124_12;
        // N s_124_14: jump b123
        return block_123(state, tracer, fn_state);
    }
}

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
use decode_adr_aarch64_instrs_integer_arithmetic_address_pc_rel::*;
use decode_sbfm_aarch64_instrs_integer_bitfield::*;
use decode_add_addsub_imm_aarch64_instrs_integer_arithmetic_add_sub_immediate::*;
use decode_subs_addsub_imm_aarch64_instrs_integer_arithmetic_add_sub_immediate::*;
use decode_movz_aarch64_instrs_integer_ins_ext_insert_movewide::*;
use decode_addg_aarch64_instrs_integer_tags_mcaddtag::*;
use decode_movk_aarch64_instrs_integer_ins_ext_insert_movewide::*;
use decode_movn_aarch64_instrs_integer_ins_ext_insert_movewide::*;
use decode_ubfm_aarch64_instrs_integer_bitfield::*;
use decode_smax_imm_aarch64_instrs_integer_arithmetic_max_min_smax_imm::*;
use decode_eor_log_imm_aarch64_instrs_integer_logical_immediate::*;
use decode_sub_addsub_imm_aarch64_instrs_integer_arithmetic_add_sub_immediate::*;
use decode_ands_log_imm_aarch64_instrs_integer_logical_immediate::*;
use decode_orr_log_imm_aarch64_instrs_integer_logical_immediate::*;
use decode_adrp_aarch64_instrs_integer_arithmetic_address_pc_rel::*;
use decode_adds_addsub_imm_aarch64_instrs_integer_arithmetic_add_sub_immediate::*;
use decode_bfm_aarch64_instrs_integer_bitfield::*;
use decode_smin_imm_aarch64_instrs_integer_arithmetic_max_min_smin_imm::*;
use decode_extr_aarch64_instrs_integer_ins_ext_extract_immediate::*;
use decode_umin_imm_aarch64_instrs_integer_arithmetic_max_min_umin_imm::*;
use decode_and_log_imm_aarch64_instrs_integer_logical_immediate::*;
use decode_subg_aarch64_instrs_integer_tags_mcsubtag::*;
use decode_umax_imm_aarch64_instrs_integer_arithmetic_max_min_umax_imm::*;
use common::*;
pub fn u__DecodeA64_DataProcImm<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_373162: i128,
    gs_373163: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u_23044: u32,
        gs_373253: bool,
        gs_373583: bool,
        u_22949: u32,
        gs_373376: bool,
        u_23032: u8,
        gs_373313: bool,
        gs_373272: bool,
        u_22925: u32,
        u_23013: u32,
        gs_373334: bool,
        u_23020: u32,
        Xn: u8,
        gs_373499: bool,
        uimm6: u8,
        gs_373603: bool,
        u_23006: u32,
        gs_373482: bool,
        u_23038: u32,
        gs_373566: bool,
        u_23036: u8,
        u_22914: u32,
        gs_373296: bool,
        gs_373588: bool,
        gs_373564: bool,
        gs_373397: bool,
        u_22940: u32,
        u_22935: u32,
        gs_373231: bool,
        u_22967: u32,
        u_22985: u32,
        u_23031: u32,
        gs_373548: bool,
        u_22958: u32,
        gs_373280: bool,
        gs_373461: bool,
        gs_373355: bool,
        u_22994: u32,
        gs_373418: bool,
        u__opcode: u32,
        gs_373210: bool,
        merge_var: ProductType7b8639ca40b2f578,
        u_23001: u32,
        u_22976: u32,
        u_22905: u32,
        gs_373439: bool,
        gs_373168: bool,
        u_23034: u8,
        u_22923: u32,
        gs_373463: bool,
        u_23025: u32,
        u_22929: u32,
        gs_373189: bool,
        uimm4: u8,
        u_23035: u8,
        Xd: u8,
        op3: u8,
        u_23033: u8,
        gs_373298: bool,
        u_22896: u32,
        gs_373533: bool,
        gs_373278: bool,
        gs_373255: bool,
        gs_373516: bool,
        gs_373162: i128,
        gs_373163: u32,
    }
    let fn_state = FunctionState {
        gs_373162,
        gs_373163,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var gs#373162:i
        let s_0_0: i128 = fn_state.gs_373162;
        // D s_0_1: write-var merge#var.0 <= s_0_0
        fn_state.merge_var._0 = s_0_0;
        // D s_0_2: read-var gs#373163:u32
        let s_0_2: u32 = fn_state.gs_373163;
        // D s_0_3: write-var merge#var.1 <= s_0_2
        fn_state.merge_var._1 = s_0_2;
        // D s_0_4: read-var merge#var.1:struct
        let s_0_4: u32 = fn_state.merge_var._1;
        // D s_0_5: write-var __opcode <= s_0_4
        fn_state.u__opcode = s_0_4;
        // C s_0_6: const #23s : i
        let s_0_6: i128 = 23;
        // D s_0_7: read-var __opcode:u32
        let s_0_7: u32 = fn_state.u__opcode;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 32u16);
        // C s_0_9: const #1s : i64
        let s_0_9: i64 = 1;
        // C s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (i128::try_from(s_0_9).unwrap());
        // C s_0_11: const #7s : i
        let s_0_11: i128 = 7;
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
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 8u16);
        // C s_0_16: const #34u : u8
        let s_0_16: u8 = 34;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 8u16);
        // D s_0_18: cmp-eq s_0_15 s_0_17
        let s_0_18: bool = ((s_0_15) == (s_0_17));
        // N s_0_19: branch s_0_18 b140 b1
        if s_0_18 {
            return block_140(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#373168 <= s_1_0
        fn_state.gs_373168 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#373168:u8
        let s_2_0: bool = fn_state.gs_373168;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b4 b3
        if s_2_1 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #18s : i
        let s_3_0: i128 = 18;
        // C s_3_1: const #14696u : u32
        let s_3_1: u32 = 14696;
        // N s_3_2: write-reg s_3_1 <= s_3_0
        let s_3_2: () = {
            state.write_register::<i128>(s_3_1 as isize, s_3_0);
            tracer.write_register(s_3_1 as isize, s_3_0);
        };
        // C s_3_3: const #0s : i
        let s_3_3: i128 = 0;
        // C s_3_4: const #5s : i
        let s_3_4: i128 = 5;
        // D s_3_5: read-var __opcode:u32
        let s_3_5: u32 = fn_state.u__opcode;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 32u16);
        // D s_3_7: bit-extract s_3_6 s_3_3 s_3_4
        let s_3_7: Bits = (Bits::new(
            ((s_3_6) >> (s_3_3)).value(),
            u16::try_from(s_3_4).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u8
        let s_3_8: u8 = (s_3_7.value() as u8);
        // C s_3_9: const #5s : i
        let s_3_9: i128 = 5;
        // C s_3_10: const #5s : i
        let s_3_10: i128 = 5;
        // D s_3_11: read-var __opcode:u32
        let s_3_11: u32 = fn_state.u__opcode;
        // D s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 32u16);
        // D s_3_13: bit-extract s_3_12 s_3_9 s_3_10
        let s_3_13: Bits = (Bits::new(
            ((s_3_12) >> (s_3_9)).value(),
            u16::try_from(s_3_10).unwrap(),
        ));
        // D s_3_14: cast reint s_3_13 -> u8
        let s_3_14: u8 = (s_3_13.value() as u8);
        // C s_3_15: const #10s : i
        let s_3_15: i128 = 10;
        // C s_3_16: const #12s : i
        let s_3_16: i128 = 12;
        // D s_3_17: read-var __opcode:u32
        let s_3_17: u32 = fn_state.u__opcode;
        // D s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 32u16);
        // D s_3_19: bit-extract s_3_18 s_3_15 s_3_16
        let s_3_19: Bits = (Bits::new(
            ((s_3_18) >> (s_3_15)).value(),
            u16::try_from(s_3_16).unwrap(),
        ));
        // D s_3_20: cast reint s_3_19 -> u12
        let s_3_20: u16 = (s_3_19.value() as u16);
        // C s_3_21: const #22s : i
        let s_3_21: i128 = 22;
        // C s_3_22: const #1s : i
        let s_3_22: i128 = 1;
        // D s_3_23: read-var __opcode:u32
        let s_3_23: u32 = fn_state.u__opcode;
        // D s_3_24: cast zx s_3_23 -> bv
        let s_3_24: Bits = Bits::new(s_3_23 as u128, 32u16);
        // D s_3_25: bit-extract s_3_24 s_3_21 s_3_22
        let s_3_25: Bits = (Bits::new(
            ((s_3_24) >> (s_3_21)).value(),
            u16::try_from(s_3_22).unwrap(),
        ));
        // D s_3_26: cast reint s_3_25 -> u8
        let s_3_26: bool = ((s_3_25.value()) != 0);
        // C s_3_27: const #29s : i
        let s_3_27: i128 = 29;
        // C s_3_28: const #1s : i
        let s_3_28: i128 = 1;
        // D s_3_29: read-var __opcode:u32
        let s_3_29: u32 = fn_state.u__opcode;
        // D s_3_30: cast zx s_3_29 -> bv
        let s_3_30: Bits = Bits::new(s_3_29 as u128, 32u16);
        // D s_3_31: bit-extract s_3_30 s_3_27 s_3_28
        let s_3_31: Bits = (Bits::new(
            ((s_3_30) >> (s_3_27)).value(),
            u16::try_from(s_3_28).unwrap(),
        ));
        // D s_3_32: cast reint s_3_31 -> u8
        let s_3_32: bool = ((s_3_31.value()) != 0);
        // C s_3_33: const #30s : i
        let s_3_33: i128 = 30;
        // C s_3_34: const #1s : i
        let s_3_34: i128 = 1;
        // D s_3_35: read-var __opcode:u32
        let s_3_35: u32 = fn_state.u__opcode;
        // D s_3_36: cast zx s_3_35 -> bv
        let s_3_36: Bits = Bits::new(s_3_35 as u128, 32u16);
        // D s_3_37: bit-extract s_3_36 s_3_33 s_3_34
        let s_3_37: Bits = (Bits::new(
            ((s_3_36) >> (s_3_33)).value(),
            u16::try_from(s_3_34).unwrap(),
        ));
        // D s_3_38: cast reint s_3_37 -> u8
        let s_3_38: bool = ((s_3_37.value()) != 0);
        // C s_3_39: const #31s : i
        let s_3_39: i128 = 31;
        // C s_3_40: const #1s : i
        let s_3_40: i128 = 1;
        // D s_3_41: read-var __opcode:u32
        let s_3_41: u32 = fn_state.u__opcode;
        // D s_3_42: cast zx s_3_41 -> bv
        let s_3_42: Bits = Bits::new(s_3_41 as u128, 32u16);
        // D s_3_43: bit-extract s_3_42 s_3_39 s_3_40
        let s_3_43: Bits = (Bits::new(
            ((s_3_42) >> (s_3_39)).value(),
            u16::try_from(s_3_40).unwrap(),
        ));
        // D s_3_44: cast reint s_3_43 -> u8
        let s_3_44: bool = ((s_3_43.value()) != 0);
        // D s_3_45: call decode_add_addsub_imm_aarch64_instrs_integer_arithmetic_add_sub_immediate(s_3_8, s_3_14, s_3_20, s_3_26, s_3_32, s_3_38, s_3_44)
        let s_3_45: () = decode_add_addsub_imm_aarch64_instrs_integer_arithmetic_add_sub_immediate(
            state,
            tracer,
            s_3_8,
            s_3_14,
            s_3_20,
            s_3_26,
            s_3_32,
            s_3_38,
            s_3_44,
        );
        // N s_3_46: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var merge#var.1:struct
        let s_4_0: u32 = fn_state.merge_var._1;
        // D s_4_1: write-var u#22896 <= s_4_0
        fn_state.u_22896 = s_4_0;
        // C s_4_2: const #23s : i
        let s_4_2: i128 = 23;
        // D s_4_3: read-var u#22896:u32
        let s_4_3: u32 = fn_state.u_22896;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 32u16);
        // C s_4_5: const #1s : i64
        let s_4_5: i64 = 1;
        // C s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // C s_4_7: const #7s : i
        let s_4_7: i128 = 7;
        // C s_4_8: add s_4_7 s_4_6
        let s_4_8: i128 = (s_4_7 + s_4_6);
        // D s_4_9: bit-extract s_4_4 s_4_2 s_4_8
        let s_4_9: Bits = (Bits::new(
            ((s_4_4) >> (s_4_2)).value(),
            u16::try_from(s_4_8).unwrap(),
        ));
        // D s_4_10: cast reint s_4_9 -> u8
        let s_4_10: u8 = (s_4_9.value() as u8);
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 8u16);
        // C s_4_12: const #98u : u8
        let s_4_12: u8 = 98;
        // C s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 8u16);
        // D s_4_14: cmp-eq s_4_11 s_4_13
        let s_4_14: bool = ((s_4_11) == (s_4_13));
        // N s_4_15: branch s_4_14 b139 b5
        if s_4_14 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#373189 <= s_5_0
        fn_state.gs_373189 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#373189:u8
        let s_6_0: bool = fn_state.gs_373189;
        // D s_6_1: not s_6_0
        let s_6_1: bool = !s_6_0;
        // N s_6_2: branch s_6_1 b8 b7
        if s_6_1 {
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
        // C s_7_0: const #19s : i
        let s_7_0: i128 = 19;
        // C s_7_1: const #14696u : u32
        let s_7_1: u32 = 14696;
        // N s_7_2: write-reg s_7_1 <= s_7_0
        let s_7_2: () = {
            state.write_register::<i128>(s_7_1 as isize, s_7_0);
            tracer.write_register(s_7_1 as isize, s_7_0);
        };
        // C s_7_3: const #0s : i
        let s_7_3: i128 = 0;
        // C s_7_4: const #5s : i
        let s_7_4: i128 = 5;
        // D s_7_5: read-var u#22896:u32
        let s_7_5: u32 = fn_state.u_22896;
        // D s_7_6: cast zx s_7_5 -> bv
        let s_7_6: Bits = Bits::new(s_7_5 as u128, 32u16);
        // D s_7_7: bit-extract s_7_6 s_7_3 s_7_4
        let s_7_7: Bits = (Bits::new(
            ((s_7_6) >> (s_7_3)).value(),
            u16::try_from(s_7_4).unwrap(),
        ));
        // D s_7_8: cast reint s_7_7 -> u8
        let s_7_8: u8 = (s_7_7.value() as u8);
        // C s_7_9: const #5s : i
        let s_7_9: i128 = 5;
        // C s_7_10: const #5s : i
        let s_7_10: i128 = 5;
        // D s_7_11: read-var u#22896:u32
        let s_7_11: u32 = fn_state.u_22896;
        // D s_7_12: cast zx s_7_11 -> bv
        let s_7_12: Bits = Bits::new(s_7_11 as u128, 32u16);
        // D s_7_13: bit-extract s_7_12 s_7_9 s_7_10
        let s_7_13: Bits = (Bits::new(
            ((s_7_12) >> (s_7_9)).value(),
            u16::try_from(s_7_10).unwrap(),
        ));
        // D s_7_14: cast reint s_7_13 -> u8
        let s_7_14: u8 = (s_7_13.value() as u8);
        // C s_7_15: const #10s : i
        let s_7_15: i128 = 10;
        // C s_7_16: const #12s : i
        let s_7_16: i128 = 12;
        // D s_7_17: read-var u#22896:u32
        let s_7_17: u32 = fn_state.u_22896;
        // D s_7_18: cast zx s_7_17 -> bv
        let s_7_18: Bits = Bits::new(s_7_17 as u128, 32u16);
        // D s_7_19: bit-extract s_7_18 s_7_15 s_7_16
        let s_7_19: Bits = (Bits::new(
            ((s_7_18) >> (s_7_15)).value(),
            u16::try_from(s_7_16).unwrap(),
        ));
        // D s_7_20: cast reint s_7_19 -> u12
        let s_7_20: u16 = (s_7_19.value() as u16);
        // C s_7_21: const #22s : i
        let s_7_21: i128 = 22;
        // C s_7_22: const #1s : i
        let s_7_22: i128 = 1;
        // D s_7_23: read-var u#22896:u32
        let s_7_23: u32 = fn_state.u_22896;
        // D s_7_24: cast zx s_7_23 -> bv
        let s_7_24: Bits = Bits::new(s_7_23 as u128, 32u16);
        // D s_7_25: bit-extract s_7_24 s_7_21 s_7_22
        let s_7_25: Bits = (Bits::new(
            ((s_7_24) >> (s_7_21)).value(),
            u16::try_from(s_7_22).unwrap(),
        ));
        // D s_7_26: cast reint s_7_25 -> u8
        let s_7_26: bool = ((s_7_25.value()) != 0);
        // C s_7_27: const #29s : i
        let s_7_27: i128 = 29;
        // C s_7_28: const #1s : i
        let s_7_28: i128 = 1;
        // D s_7_29: read-var u#22896:u32
        let s_7_29: u32 = fn_state.u_22896;
        // D s_7_30: cast zx s_7_29 -> bv
        let s_7_30: Bits = Bits::new(s_7_29 as u128, 32u16);
        // D s_7_31: bit-extract s_7_30 s_7_27 s_7_28
        let s_7_31: Bits = (Bits::new(
            ((s_7_30) >> (s_7_27)).value(),
            u16::try_from(s_7_28).unwrap(),
        ));
        // D s_7_32: cast reint s_7_31 -> u8
        let s_7_32: bool = ((s_7_31.value()) != 0);
        // C s_7_33: const #30s : i
        let s_7_33: i128 = 30;
        // C s_7_34: const #1s : i
        let s_7_34: i128 = 1;
        // D s_7_35: read-var u#22896:u32
        let s_7_35: u32 = fn_state.u_22896;
        // D s_7_36: cast zx s_7_35 -> bv
        let s_7_36: Bits = Bits::new(s_7_35 as u128, 32u16);
        // D s_7_37: bit-extract s_7_36 s_7_33 s_7_34
        let s_7_37: Bits = (Bits::new(
            ((s_7_36) >> (s_7_33)).value(),
            u16::try_from(s_7_34).unwrap(),
        ));
        // D s_7_38: cast reint s_7_37 -> u8
        let s_7_38: bool = ((s_7_37.value()) != 0);
        // C s_7_39: const #31s : i
        let s_7_39: i128 = 31;
        // C s_7_40: const #1s : i
        let s_7_40: i128 = 1;
        // D s_7_41: read-var u#22896:u32
        let s_7_41: u32 = fn_state.u_22896;
        // D s_7_42: cast zx s_7_41 -> bv
        let s_7_42: Bits = Bits::new(s_7_41 as u128, 32u16);
        // D s_7_43: bit-extract s_7_42 s_7_39 s_7_40
        let s_7_43: Bits = (Bits::new(
            ((s_7_42) >> (s_7_39)).value(),
            u16::try_from(s_7_40).unwrap(),
        ));
        // D s_7_44: cast reint s_7_43 -> u8
        let s_7_44: bool = ((s_7_43.value()) != 0);
        // D s_7_45: call decode_adds_addsub_imm_aarch64_instrs_integer_arithmetic_add_sub_immediate(s_7_8, s_7_14, s_7_20, s_7_26, s_7_32, s_7_38, s_7_44)
        let s_7_45: () = decode_adds_addsub_imm_aarch64_instrs_integer_arithmetic_add_sub_immediate(
            state,
            tracer,
            s_7_8,
            s_7_14,
            s_7_20,
            s_7_26,
            s_7_32,
            s_7_38,
            s_7_44,
        );
        // N s_7_46: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var merge#var.1:struct
        let s_8_0: u32 = fn_state.merge_var._1;
        // D s_8_1: write-var u#22905 <= s_8_0
        fn_state.u_22905 = s_8_0;
        // C s_8_2: const #23s : i
        let s_8_2: i128 = 23;
        // D s_8_3: read-var u#22905:u32
        let s_8_3: u32 = fn_state.u_22905;
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 32u16);
        // C s_8_5: const #1s : i64
        let s_8_5: i64 = 1;
        // C s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // C s_8_7: const #7s : i
        let s_8_7: i128 = 7;
        // C s_8_8: add s_8_7 s_8_6
        let s_8_8: i128 = (s_8_7 + s_8_6);
        // D s_8_9: bit-extract s_8_4 s_8_2 s_8_8
        let s_8_9: Bits = (Bits::new(
            ((s_8_4) >> (s_8_2)).value(),
            u16::try_from(s_8_8).unwrap(),
        ));
        // D s_8_10: cast reint s_8_9 -> u8
        let s_8_10: u8 = (s_8_9.value() as u8);
        // D s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 8u16);
        // C s_8_12: const #162u : u8
        let s_8_12: u8 = 162;
        // C s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 8u16);
        // D s_8_14: cmp-eq s_8_11 s_8_13
        let s_8_14: bool = ((s_8_11) == (s_8_13));
        // N s_8_15: branch s_8_14 b138 b9
        if s_8_14 {
            return block_138(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#373210 <= s_9_0
        fn_state.gs_373210 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#373210:u8
        let s_10_0: bool = fn_state.gs_373210;
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
        // C s_11_0: const #20s : i
        let s_11_0: i128 = 20;
        // C s_11_1: const #14696u : u32
        let s_11_1: u32 = 14696;
        // N s_11_2: write-reg s_11_1 <= s_11_0
        let s_11_2: () = {
            state.write_register::<i128>(s_11_1 as isize, s_11_0);
            tracer.write_register(s_11_1 as isize, s_11_0);
        };
        // C s_11_3: const #0s : i
        let s_11_3: i128 = 0;
        // C s_11_4: const #5s : i
        let s_11_4: i128 = 5;
        // D s_11_5: read-var u#22905:u32
        let s_11_5: u32 = fn_state.u_22905;
        // D s_11_6: cast zx s_11_5 -> bv
        let s_11_6: Bits = Bits::new(s_11_5 as u128, 32u16);
        // D s_11_7: bit-extract s_11_6 s_11_3 s_11_4
        let s_11_7: Bits = (Bits::new(
            ((s_11_6) >> (s_11_3)).value(),
            u16::try_from(s_11_4).unwrap(),
        ));
        // D s_11_8: cast reint s_11_7 -> u8
        let s_11_8: u8 = (s_11_7.value() as u8);
        // C s_11_9: const #5s : i
        let s_11_9: i128 = 5;
        // C s_11_10: const #5s : i
        let s_11_10: i128 = 5;
        // D s_11_11: read-var u#22905:u32
        let s_11_11: u32 = fn_state.u_22905;
        // D s_11_12: cast zx s_11_11 -> bv
        let s_11_12: Bits = Bits::new(s_11_11 as u128, 32u16);
        // D s_11_13: bit-extract s_11_12 s_11_9 s_11_10
        let s_11_13: Bits = (Bits::new(
            ((s_11_12) >> (s_11_9)).value(),
            u16::try_from(s_11_10).unwrap(),
        ));
        // D s_11_14: cast reint s_11_13 -> u8
        let s_11_14: u8 = (s_11_13.value() as u8);
        // C s_11_15: const #10s : i
        let s_11_15: i128 = 10;
        // C s_11_16: const #12s : i
        let s_11_16: i128 = 12;
        // D s_11_17: read-var u#22905:u32
        let s_11_17: u32 = fn_state.u_22905;
        // D s_11_18: cast zx s_11_17 -> bv
        let s_11_18: Bits = Bits::new(s_11_17 as u128, 32u16);
        // D s_11_19: bit-extract s_11_18 s_11_15 s_11_16
        let s_11_19: Bits = (Bits::new(
            ((s_11_18) >> (s_11_15)).value(),
            u16::try_from(s_11_16).unwrap(),
        ));
        // D s_11_20: cast reint s_11_19 -> u12
        let s_11_20: u16 = (s_11_19.value() as u16);
        // C s_11_21: const #22s : i
        let s_11_21: i128 = 22;
        // C s_11_22: const #1s : i
        let s_11_22: i128 = 1;
        // D s_11_23: read-var u#22905:u32
        let s_11_23: u32 = fn_state.u_22905;
        // D s_11_24: cast zx s_11_23 -> bv
        let s_11_24: Bits = Bits::new(s_11_23 as u128, 32u16);
        // D s_11_25: bit-extract s_11_24 s_11_21 s_11_22
        let s_11_25: Bits = (Bits::new(
            ((s_11_24) >> (s_11_21)).value(),
            u16::try_from(s_11_22).unwrap(),
        ));
        // D s_11_26: cast reint s_11_25 -> u8
        let s_11_26: bool = ((s_11_25.value()) != 0);
        // C s_11_27: const #29s : i
        let s_11_27: i128 = 29;
        // C s_11_28: const #1s : i
        let s_11_28: i128 = 1;
        // D s_11_29: read-var u#22905:u32
        let s_11_29: u32 = fn_state.u_22905;
        // D s_11_30: cast zx s_11_29 -> bv
        let s_11_30: Bits = Bits::new(s_11_29 as u128, 32u16);
        // D s_11_31: bit-extract s_11_30 s_11_27 s_11_28
        let s_11_31: Bits = (Bits::new(
            ((s_11_30) >> (s_11_27)).value(),
            u16::try_from(s_11_28).unwrap(),
        ));
        // D s_11_32: cast reint s_11_31 -> u8
        let s_11_32: bool = ((s_11_31.value()) != 0);
        // C s_11_33: const #30s : i
        let s_11_33: i128 = 30;
        // C s_11_34: const #1s : i
        let s_11_34: i128 = 1;
        // D s_11_35: read-var u#22905:u32
        let s_11_35: u32 = fn_state.u_22905;
        // D s_11_36: cast zx s_11_35 -> bv
        let s_11_36: Bits = Bits::new(s_11_35 as u128, 32u16);
        // D s_11_37: bit-extract s_11_36 s_11_33 s_11_34
        let s_11_37: Bits = (Bits::new(
            ((s_11_36) >> (s_11_33)).value(),
            u16::try_from(s_11_34).unwrap(),
        ));
        // D s_11_38: cast reint s_11_37 -> u8
        let s_11_38: bool = ((s_11_37.value()) != 0);
        // C s_11_39: const #31s : i
        let s_11_39: i128 = 31;
        // C s_11_40: const #1s : i
        let s_11_40: i128 = 1;
        // D s_11_41: read-var u#22905:u32
        let s_11_41: u32 = fn_state.u_22905;
        // D s_11_42: cast zx s_11_41 -> bv
        let s_11_42: Bits = Bits::new(s_11_41 as u128, 32u16);
        // D s_11_43: bit-extract s_11_42 s_11_39 s_11_40
        let s_11_43: Bits = (Bits::new(
            ((s_11_42) >> (s_11_39)).value(),
            u16::try_from(s_11_40).unwrap(),
        ));
        // D s_11_44: cast reint s_11_43 -> u8
        let s_11_44: bool = ((s_11_43.value()) != 0);
        // D s_11_45: call decode_sub_addsub_imm_aarch64_instrs_integer_arithmetic_add_sub_immediate(s_11_8, s_11_14, s_11_20, s_11_26, s_11_32, s_11_38, s_11_44)
        let s_11_45: () = decode_sub_addsub_imm_aarch64_instrs_integer_arithmetic_add_sub_immediate(
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
        // D s_12_1: write-var u#22914 <= s_12_0
        fn_state.u_22914 = s_12_0;
        // C s_12_2: const #23s : i
        let s_12_2: i128 = 23;
        // D s_12_3: read-var u#22914:u32
        let s_12_3: u32 = fn_state.u_22914;
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 32u16);
        // C s_12_5: const #1s : i64
        let s_12_5: i64 = 1;
        // C s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // C s_12_7: const #7s : i
        let s_12_7: i128 = 7;
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
        let s_12_11: Bits = Bits::new(s_12_10 as u128, 8u16);
        // C s_12_12: const #226u : u8
        let s_12_12: u8 = 226;
        // C s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 8u16);
        // D s_12_14: cmp-eq s_12_11 s_12_13
        let s_12_14: bool = ((s_12_11) == (s_12_13));
        // N s_12_15: branch s_12_14 b137 b13
        if s_12_14 {
            return block_137(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#373231 <= s_13_0
        fn_state.gs_373231 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#373231:u8
        let s_14_0: bool = fn_state.gs_373231;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // N s_14_2: branch s_14_1 b16 b15
        if s_14_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #21s : i
        let s_15_0: i128 = 21;
        // C s_15_1: const #14696u : u32
        let s_15_1: u32 = 14696;
        // N s_15_2: write-reg s_15_1 <= s_15_0
        let s_15_2: () = {
            state.write_register::<i128>(s_15_1 as isize, s_15_0);
            tracer.write_register(s_15_1 as isize, s_15_0);
        };
        // C s_15_3: const #0s : i
        let s_15_3: i128 = 0;
        // C s_15_4: const #5s : i
        let s_15_4: i128 = 5;
        // D s_15_5: read-var u#22914:u32
        let s_15_5: u32 = fn_state.u_22914;
        // D s_15_6: cast zx s_15_5 -> bv
        let s_15_6: Bits = Bits::new(s_15_5 as u128, 32u16);
        // D s_15_7: bit-extract s_15_6 s_15_3 s_15_4
        let s_15_7: Bits = (Bits::new(
            ((s_15_6) >> (s_15_3)).value(),
            u16::try_from(s_15_4).unwrap(),
        ));
        // D s_15_8: cast reint s_15_7 -> u8
        let s_15_8: u8 = (s_15_7.value() as u8);
        // C s_15_9: const #5s : i
        let s_15_9: i128 = 5;
        // C s_15_10: const #5s : i
        let s_15_10: i128 = 5;
        // D s_15_11: read-var u#22914:u32
        let s_15_11: u32 = fn_state.u_22914;
        // D s_15_12: cast zx s_15_11 -> bv
        let s_15_12: Bits = Bits::new(s_15_11 as u128, 32u16);
        // D s_15_13: bit-extract s_15_12 s_15_9 s_15_10
        let s_15_13: Bits = (Bits::new(
            ((s_15_12) >> (s_15_9)).value(),
            u16::try_from(s_15_10).unwrap(),
        ));
        // D s_15_14: cast reint s_15_13 -> u8
        let s_15_14: u8 = (s_15_13.value() as u8);
        // C s_15_15: const #10s : i
        let s_15_15: i128 = 10;
        // C s_15_16: const #12s : i
        let s_15_16: i128 = 12;
        // D s_15_17: read-var u#22914:u32
        let s_15_17: u32 = fn_state.u_22914;
        // D s_15_18: cast zx s_15_17 -> bv
        let s_15_18: Bits = Bits::new(s_15_17 as u128, 32u16);
        // D s_15_19: bit-extract s_15_18 s_15_15 s_15_16
        let s_15_19: Bits = (Bits::new(
            ((s_15_18) >> (s_15_15)).value(),
            u16::try_from(s_15_16).unwrap(),
        ));
        // D s_15_20: cast reint s_15_19 -> u12
        let s_15_20: u16 = (s_15_19.value() as u16);
        // C s_15_21: const #22s : i
        let s_15_21: i128 = 22;
        // C s_15_22: const #1s : i
        let s_15_22: i128 = 1;
        // D s_15_23: read-var u#22914:u32
        let s_15_23: u32 = fn_state.u_22914;
        // D s_15_24: cast zx s_15_23 -> bv
        let s_15_24: Bits = Bits::new(s_15_23 as u128, 32u16);
        // D s_15_25: bit-extract s_15_24 s_15_21 s_15_22
        let s_15_25: Bits = (Bits::new(
            ((s_15_24) >> (s_15_21)).value(),
            u16::try_from(s_15_22).unwrap(),
        ));
        // D s_15_26: cast reint s_15_25 -> u8
        let s_15_26: bool = ((s_15_25.value()) != 0);
        // C s_15_27: const #29s : i
        let s_15_27: i128 = 29;
        // C s_15_28: const #1s : i
        let s_15_28: i128 = 1;
        // D s_15_29: read-var u#22914:u32
        let s_15_29: u32 = fn_state.u_22914;
        // D s_15_30: cast zx s_15_29 -> bv
        let s_15_30: Bits = Bits::new(s_15_29 as u128, 32u16);
        // D s_15_31: bit-extract s_15_30 s_15_27 s_15_28
        let s_15_31: Bits = (Bits::new(
            ((s_15_30) >> (s_15_27)).value(),
            u16::try_from(s_15_28).unwrap(),
        ));
        // D s_15_32: cast reint s_15_31 -> u8
        let s_15_32: bool = ((s_15_31.value()) != 0);
        // C s_15_33: const #30s : i
        let s_15_33: i128 = 30;
        // C s_15_34: const #1s : i
        let s_15_34: i128 = 1;
        // D s_15_35: read-var u#22914:u32
        let s_15_35: u32 = fn_state.u_22914;
        // D s_15_36: cast zx s_15_35 -> bv
        let s_15_36: Bits = Bits::new(s_15_35 as u128, 32u16);
        // D s_15_37: bit-extract s_15_36 s_15_33 s_15_34
        let s_15_37: Bits = (Bits::new(
            ((s_15_36) >> (s_15_33)).value(),
            u16::try_from(s_15_34).unwrap(),
        ));
        // D s_15_38: cast reint s_15_37 -> u8
        let s_15_38: bool = ((s_15_37.value()) != 0);
        // C s_15_39: const #31s : i
        let s_15_39: i128 = 31;
        // C s_15_40: const #1s : i
        let s_15_40: i128 = 1;
        // D s_15_41: read-var u#22914:u32
        let s_15_41: u32 = fn_state.u_22914;
        // D s_15_42: cast zx s_15_41 -> bv
        let s_15_42: Bits = Bits::new(s_15_41 as u128, 32u16);
        // D s_15_43: bit-extract s_15_42 s_15_39 s_15_40
        let s_15_43: Bits = (Bits::new(
            ((s_15_42) >> (s_15_39)).value(),
            u16::try_from(s_15_40).unwrap(),
        ));
        // D s_15_44: cast reint s_15_43 -> u8
        let s_15_44: bool = ((s_15_43.value()) != 0);
        // D s_15_45: call decode_subs_addsub_imm_aarch64_instrs_integer_arithmetic_add_sub_immediate(s_15_8, s_15_14, s_15_20, s_15_26, s_15_32, s_15_38, s_15_44)
        let s_15_45: () = decode_subs_addsub_imm_aarch64_instrs_integer_arithmetic_add_sub_immediate(
            state,
            tracer,
            s_15_8,
            s_15_14,
            s_15_20,
            s_15_26,
            s_15_32,
            s_15_38,
            s_15_44,
        );
        // N s_15_46: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var merge#var.1:struct
        let s_16_0: u32 = fn_state.merge_var._1;
        // D s_16_1: write-var u#22923 <= s_16_0
        fn_state.u_22923 = s_16_0;
        // C s_16_2: const #22s : i
        let s_16_2: i128 = 22;
        // D s_16_3: read-var u#22923:u32
        let s_16_3: u32 = fn_state.u_22923;
        // D s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 32u16);
        // C s_16_5: const #1s : i64
        let s_16_5: i64 = 1;
        // C s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (i128::try_from(s_16_5).unwrap());
        // C s_16_7: const #9s : i
        let s_16_7: i128 = 9;
        // C s_16_8: add s_16_7 s_16_6
        let s_16_8: i128 = (s_16_7 + s_16_6);
        // D s_16_9: bit-extract s_16_4 s_16_2 s_16_8
        let s_16_9: Bits = (Bits::new(
            ((s_16_4) >> (s_16_2)).value(),
            u16::try_from(s_16_8).unwrap(),
        ));
        // D s_16_10: cast reint s_16_9 -> u10
        let s_16_10: u16 = (s_16_9.value() as u16);
        // D s_16_11: cast zx s_16_10 -> bv
        let s_16_11: Bits = Bits::new(s_16_10 as u128, 10u16);
        // C s_16_12: const #582u : u10
        let s_16_12: u16 = 582;
        // C s_16_13: cast zx s_16_12 -> bv
        let s_16_13: Bits = Bits::new(s_16_12 as u128, 10u16);
        // D s_16_14: cmp-eq s_16_11 s_16_13
        let s_16_14: bool = ((s_16_11) == (s_16_13));
        // N s_16_15: branch s_16_14 b136 b17
        if s_16_14 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#373253 <= s_17_0
        fn_state.gs_373253 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#373253:u8
        let s_18_0: bool = fn_state.gs_373253;
        // N s_18_1: branch s_18_0 b135 b19
        if s_18_0 {
            return block_135(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#373255 <= s_19_0
        fn_state.gs_373255 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#373255:u8
        let s_20_0: bool = fn_state.gs_373255;
        // D s_20_1: not s_20_0
        let s_20_1: bool = !s_20_0;
        // N s_20_2: branch s_20_1 b27 b21
        if s_20_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #26s : i
        let s_21_0: i128 = 26;
        // C s_21_1: const #14696u : u32
        let s_21_1: u32 = 14696;
        // N s_21_2: write-reg s_21_1 <= s_21_0
        let s_21_2: () = {
            state.write_register::<i128>(s_21_1 as isize, s_21_0);
            tracer.write_register(s_21_1 as isize, s_21_0);
        };
        // C s_21_3: const #0s : i
        let s_21_3: i128 = 0;
        // C s_21_4: const #5s : i
        let s_21_4: i128 = 5;
        // D s_21_5: read-var u#22923:u32
        let s_21_5: u32 = fn_state.u_22923;
        // D s_21_6: cast zx s_21_5 -> bv
        let s_21_6: Bits = Bits::new(s_21_5 as u128, 32u16);
        // D s_21_7: bit-extract s_21_6 s_21_3 s_21_4
        let s_21_7: Bits = (Bits::new(
            ((s_21_6) >> (s_21_3)).value(),
            u16::try_from(s_21_4).unwrap(),
        ));
        // D s_21_8: cast reint s_21_7 -> u8
        let s_21_8: u8 = (s_21_7.value() as u8);
        // D s_21_9: write-var Xd <= s_21_8
        fn_state.Xd = s_21_8;
        // C s_21_10: const #5s : i
        let s_21_10: i128 = 5;
        // C s_21_11: const #5s : i
        let s_21_11: i128 = 5;
        // D s_21_12: read-var u#22923:u32
        let s_21_12: u32 = fn_state.u_22923;
        // D s_21_13: cast zx s_21_12 -> bv
        let s_21_13: Bits = Bits::new(s_21_12 as u128, 32u16);
        // D s_21_14: bit-extract s_21_13 s_21_10 s_21_11
        let s_21_14: Bits = (Bits::new(
            ((s_21_13) >> (s_21_10)).value(),
            u16::try_from(s_21_11).unwrap(),
        ));
        // D s_21_15: cast reint s_21_14 -> u8
        let s_21_15: u8 = (s_21_14.value() as u8);
        // D s_21_16: write-var Xn <= s_21_15
        fn_state.Xn = s_21_15;
        // C s_21_17: const #10s : i
        let s_21_17: i128 = 10;
        // C s_21_18: const #4s : i
        let s_21_18: i128 = 4;
        // D s_21_19: read-var u#22923:u32
        let s_21_19: u32 = fn_state.u_22923;
        // D s_21_20: cast zx s_21_19 -> bv
        let s_21_20: Bits = Bits::new(s_21_19 as u128, 32u16);
        // D s_21_21: bit-extract s_21_20 s_21_17 s_21_18
        let s_21_21: Bits = (Bits::new(
            ((s_21_20) >> (s_21_17)).value(),
            u16::try_from(s_21_18).unwrap(),
        ));
        // D s_21_22: cast reint s_21_21 -> u8
        let s_21_22: u8 = (s_21_21.value() as u8);
        // D s_21_23: write-var uimm4 <= s_21_22
        fn_state.uimm4 = s_21_22;
        // C s_21_24: const #14s : i
        let s_21_24: i128 = 14;
        // C s_21_25: const #2s : i
        let s_21_25: i128 = 2;
        // D s_21_26: read-var u#22923:u32
        let s_21_26: u32 = fn_state.u_22923;
        // D s_21_27: cast zx s_21_26 -> bv
        let s_21_27: Bits = Bits::new(s_21_26 as u128, 32u16);
        // D s_21_28: bit-extract s_21_27 s_21_24 s_21_25
        let s_21_28: Bits = (Bits::new(
            ((s_21_27) >> (s_21_24)).value(),
            u16::try_from(s_21_25).unwrap(),
        ));
        // D s_21_29: cast reint s_21_28 -> u8
        let s_21_29: u8 = (s_21_28.value() as u8);
        // D s_21_30: write-var op3 <= s_21_29
        fn_state.op3 = s_21_29;
        // C s_21_31: const #16s : i
        let s_21_31: i128 = 16;
        // C s_21_32: const #6s : i
        let s_21_32: i128 = 6;
        // D s_21_33: read-var u#22923:u32
        let s_21_33: u32 = fn_state.u_22923;
        // D s_21_34: cast zx s_21_33 -> bv
        let s_21_34: Bits = Bits::new(s_21_33 as u128, 32u16);
        // D s_21_35: bit-extract s_21_34 s_21_31 s_21_32
        let s_21_35: Bits = (Bits::new(
            ((s_21_34) >> (s_21_31)).value(),
            u16::try_from(s_21_32).unwrap(),
        ));
        // D s_21_36: cast reint s_21_35 -> u8
        let s_21_36: u8 = (s_21_35.value() as u8);
        // D s_21_37: write-var uimm6 <= s_21_36
        fn_state.uimm6 = s_21_36;
        // C s_21_38: const #14s : i
        let s_21_38: i128 = 14;
        // D s_21_39: read-var u#22923:u32
        let s_21_39: u32 = fn_state.u_22923;
        // D s_21_40: cast zx s_21_39 -> bv
        let s_21_40: Bits = Bits::new(s_21_39 as u128, 32u16);
        // C s_21_41: const #1u : u64
        let s_21_41: u64 = 1;
        // D s_21_42: bit-extract s_21_40 s_21_38 s_21_41
        let s_21_42: Bits = (Bits::new(
            ((s_21_40) >> (s_21_38)).value(),
            u16::try_from(s_21_41).unwrap(),
        ));
        // D s_21_43: cast reint s_21_42 -> u8
        let s_21_43: bool = ((s_21_42.value()) != 0);
        // C s_21_44: const #0s : i
        let s_21_44: i128 = 0;
        // C s_21_45: const #0u : u64
        let s_21_45: u64 = 0;
        // D s_21_46: cast zx s_21_43 -> u64
        let s_21_46: u64 = (s_21_43 as u64);
        // C s_21_47: const #1u : u64
        let s_21_47: u64 = 1;
        // D s_21_48: and s_21_46 s_21_47
        let s_21_48: u64 = ((s_21_46) & (s_21_47));
        // D s_21_49: cmp-eq s_21_48 s_21_47
        let s_21_49: bool = ((s_21_48) == (s_21_47));
        // D s_21_50: lsl s_21_46 s_21_44
        let s_21_50: u64 = s_21_46 << s_21_44;
        // D s_21_51: or s_21_45 s_21_50
        let s_21_51: u64 = ((s_21_45) | (s_21_50));
        // D s_21_52: cmpl s_21_50
        let s_21_52: u64 = !s_21_50;
        // D s_21_53: and s_21_45 s_21_52
        let s_21_53: u64 = ((s_21_45) & (s_21_52));
        // D s_21_54: select s_21_49 s_21_51 s_21_53
        let s_21_54: u64 = if s_21_49 { s_21_51 } else { s_21_53 };
        // D s_21_55: cast trunc s_21_54 -> u8
        let s_21_55: bool = ((s_21_54) != 0);
        // D s_21_56: cast zx s_21_55 -> bv
        let s_21_56: Bits = Bits::new(s_21_55 as u128, 1u16);
        // C s_21_57: const #0u : u8
        let s_21_57: bool = false;
        // C s_21_58: cast zx s_21_57 -> bv
        let s_21_58: Bits = Bits::new(s_21_57 as u128, 1u16);
        // D s_21_59: cmp-ne s_21_56 s_21_58
        let s_21_59: bool = ((s_21_56) != (s_21_58));
        // N s_21_60: branch s_21_59 b26 b22
        if s_21_59 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #15s : i
        let s_22_0: i128 = 15;
        // D s_22_1: read-var u#22923:u32
        let s_22_1: u32 = fn_state.u_22923;
        // D s_22_2: cast zx s_22_1 -> bv
        let s_22_2: Bits = Bits::new(s_22_1 as u128, 32u16);
        // C s_22_3: const #1u : u64
        let s_22_3: u64 = 1;
        // D s_22_4: bit-extract s_22_2 s_22_0 s_22_3
        let s_22_4: Bits = (Bits::new(
            ((s_22_2) >> (s_22_0)).value(),
            u16::try_from(s_22_3).unwrap(),
        ));
        // D s_22_5: cast reint s_22_4 -> u8
        let s_22_5: bool = ((s_22_4.value()) != 0);
        // C s_22_6: const #0s : i
        let s_22_6: i128 = 0;
        // C s_22_7: const #0u : u64
        let s_22_7: u64 = 0;
        // D s_22_8: cast zx s_22_5 -> u64
        let s_22_8: u64 = (s_22_5 as u64);
        // C s_22_9: const #1u : u64
        let s_22_9: u64 = 1;
        // D s_22_10: and s_22_8 s_22_9
        let s_22_10: u64 = ((s_22_8) & (s_22_9));
        // D s_22_11: cmp-eq s_22_10 s_22_9
        let s_22_11: bool = ((s_22_10) == (s_22_9));
        // D s_22_12: lsl s_22_8 s_22_6
        let s_22_12: u64 = s_22_8 << s_22_6;
        // D s_22_13: or s_22_7 s_22_12
        let s_22_13: u64 = ((s_22_7) | (s_22_12));
        // D s_22_14: cmpl s_22_12
        let s_22_14: u64 = !s_22_12;
        // D s_22_15: and s_22_7 s_22_14
        let s_22_15: u64 = ((s_22_7) & (s_22_14));
        // D s_22_16: select s_22_11 s_22_13 s_22_15
        let s_22_16: u64 = if s_22_11 { s_22_13 } else { s_22_15 };
        // D s_22_17: cast trunc s_22_16 -> u8
        let s_22_17: bool = ((s_22_16) != 0);
        // D s_22_18: cast zx s_22_17 -> bv
        let s_22_18: Bits = Bits::new(s_22_17 as u128, 1u16);
        // C s_22_19: const #0u : u8
        let s_22_19: bool = false;
        // C s_22_20: cast zx s_22_19 -> bv
        let s_22_20: Bits = Bits::new(s_22_19 as u128, 1u16);
        // D s_22_21: cmp-ne s_22_18 s_22_20
        let s_22_21: bool = ((s_22_18) != (s_22_20));
        // D s_22_22: write-var gs#373272 <= s_22_21
        fn_state.gs_373272 = s_22_21;
        // N s_22_23: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#373272:u8
        let s_23_0: bool = fn_state.gs_373272;
        // N s_23_1: branch s_23_0 b25 b24
        if s_23_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var Xd:u8
        let s_24_0: u8 = fn_state.Xd;
        // D s_24_1: read-var Xn:u8
        let s_24_1: u8 = fn_state.Xn;
        // D s_24_2: read-var uimm4:u8
        let s_24_2: u8 = fn_state.uimm4;
        // D s_24_3: read-var op3:u8
        let s_24_3: u8 = fn_state.op3;
        // D s_24_4: read-var uimm6:u8
        let s_24_4: u8 = fn_state.uimm6;
        // D s_24_5: call decode_addg_aarch64_instrs_integer_tags_mcaddtag(s_24_0, s_24_1, s_24_2, s_24_3, s_24_4)
        let s_24_5: () = decode_addg_aarch64_instrs_integer_tags_mcaddtag(
            state,
            tracer,
            s_24_0,
            s_24_1,
            s_24_2,
            s_24_3,
            s_24_4,
        );
        // N s_24_6: return
        return;
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
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#373272 <= s_26_0
        fn_state.gs_373272 = s_26_0;
        // N s_26_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var merge#var.1:struct
        let s_27_0: u32 = fn_state.merge_var._1;
        // D s_27_1: write-var u#22925 <= s_27_0
        fn_state.u_22925 = s_27_0;
        // C s_27_2: const #31s : i
        let s_27_2: i128 = 31;
        // D s_27_3: read-var u#22925:u32
        let s_27_3: u32 = fn_state.u_22925;
        // D s_27_4: cast zx s_27_3 -> bv
        let s_27_4: Bits = Bits::new(s_27_3 as u128, 32u16);
        // C s_27_5: const #1s : i64
        let s_27_5: i64 = 1;
        // C s_27_6: cast zx s_27_5 -> i
        let s_27_6: i128 = (i128::try_from(s_27_5).unwrap());
        // C s_27_7: const #0s : i
        let s_27_7: i128 = 0;
        // C s_27_8: add s_27_7 s_27_6
        let s_27_8: i128 = (s_27_7 + s_27_6);
        // D s_27_9: bit-extract s_27_4 s_27_2 s_27_8
        let s_27_9: Bits = (Bits::new(
            ((s_27_4) >> (s_27_2)).value(),
            u16::try_from(s_27_8).unwrap(),
        ));
        // D s_27_10: cast reint s_27_9 -> u8
        let s_27_10: bool = ((s_27_9.value()) != 0);
        // D s_27_11: cast zx s_27_10 -> bv
        let s_27_11: Bits = Bits::new(s_27_10 as u128, 1u16);
        // C s_27_12: const #0u : u8
        let s_27_12: bool = false;
        // C s_27_13: cast zx s_27_12 -> bv
        let s_27_13: Bits = Bits::new(s_27_12 as u128, 1u16);
        // D s_27_14: cmp-eq s_27_11 s_27_13
        let s_27_14: bool = ((s_27_11) == (s_27_13));
        // N s_27_15: branch s_27_14 b134 b28
        if s_27_14 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#373278 <= s_28_0
        fn_state.gs_373278 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#373278:u8
        let s_29_0: bool = fn_state.gs_373278;
        // N s_29_1: branch s_29_0 b133 b30
        if s_29_0 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#373280 <= s_30_0
        fn_state.gs_373280 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#373280:u8
        let s_31_0: bool = fn_state.gs_373280;
        // D s_31_1: not s_31_0
        let s_31_1: bool = !s_31_0;
        // N s_31_2: branch s_31_1 b33 b32
        if s_31_1 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #34s : i
        let s_32_0: i128 = 34;
        // C s_32_1: const #14696u : u32
        let s_32_1: u32 = 14696;
        // N s_32_2: write-reg s_32_1 <= s_32_0
        let s_32_2: () = {
            state.write_register::<i128>(s_32_1 as isize, s_32_0);
            tracer.write_register(s_32_1 as isize, s_32_0);
        };
        // C s_32_3: const #0s : i
        let s_32_3: i128 = 0;
        // C s_32_4: const #5s : i
        let s_32_4: i128 = 5;
        // D s_32_5: read-var u#22925:u32
        let s_32_5: u32 = fn_state.u_22925;
        // D s_32_6: cast zx s_32_5 -> bv
        let s_32_6: Bits = Bits::new(s_32_5 as u128, 32u16);
        // D s_32_7: bit-extract s_32_6 s_32_3 s_32_4
        let s_32_7: Bits = (Bits::new(
            ((s_32_6) >> (s_32_3)).value(),
            u16::try_from(s_32_4).unwrap(),
        ));
        // D s_32_8: cast reint s_32_7 -> u8
        let s_32_8: u8 = (s_32_7.value() as u8);
        // C s_32_9: const #5s : i
        let s_32_9: i128 = 5;
        // C s_32_10: const #19s : i
        let s_32_10: i128 = 19;
        // D s_32_11: read-var u#22925:u32
        let s_32_11: u32 = fn_state.u_22925;
        // D s_32_12: cast zx s_32_11 -> bv
        let s_32_12: Bits = Bits::new(s_32_11 as u128, 32u16);
        // D s_32_13: bit-extract s_32_12 s_32_9 s_32_10
        let s_32_13: Bits = (Bits::new(
            ((s_32_12) >> (s_32_9)).value(),
            u16::try_from(s_32_10).unwrap(),
        ));
        // D s_32_14: cast reint s_32_13 -> u19
        let s_32_14: u32 = (s_32_13.value() as u32);
        // C s_32_15: const #29s : i
        let s_32_15: i128 = 29;
        // C s_32_16: const #2s : i
        let s_32_16: i128 = 2;
        // D s_32_17: read-var u#22925:u32
        let s_32_17: u32 = fn_state.u_22925;
        // D s_32_18: cast zx s_32_17 -> bv
        let s_32_18: Bits = Bits::new(s_32_17 as u128, 32u16);
        // D s_32_19: bit-extract s_32_18 s_32_15 s_32_16
        let s_32_19: Bits = (Bits::new(
            ((s_32_18) >> (s_32_15)).value(),
            u16::try_from(s_32_16).unwrap(),
        ));
        // D s_32_20: cast reint s_32_19 -> u8
        let s_32_20: u8 = (s_32_19.value() as u8);
        // C s_32_21: const #31s : i
        let s_32_21: i128 = 31;
        // C s_32_22: const #1s : i
        let s_32_22: i128 = 1;
        // D s_32_23: read-var u#22925:u32
        let s_32_23: u32 = fn_state.u_22925;
        // D s_32_24: cast zx s_32_23 -> bv
        let s_32_24: Bits = Bits::new(s_32_23 as u128, 32u16);
        // D s_32_25: bit-extract s_32_24 s_32_21 s_32_22
        let s_32_25: Bits = (Bits::new(
            ((s_32_24) >> (s_32_21)).value(),
            u16::try_from(s_32_22).unwrap(),
        ));
        // D s_32_26: cast reint s_32_25 -> u8
        let s_32_26: bool = ((s_32_25.value()) != 0);
        // D s_32_27: call decode_adr_aarch64_instrs_integer_arithmetic_address_pc_rel(s_32_8, s_32_14, s_32_20, s_32_26)
        let s_32_27: () = decode_adr_aarch64_instrs_integer_arithmetic_address_pc_rel(
            state,
            tracer,
            s_32_8,
            s_32_14,
            s_32_20,
            s_32_26,
        );
        // N s_32_28: return
        return;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var merge#var.1:struct
        let s_33_0: u32 = fn_state.merge_var._1;
        // D s_33_1: write-var u#22929 <= s_33_0
        fn_state.u_22929 = s_33_0;
        // C s_33_2: const #31s : i
        let s_33_2: i128 = 31;
        // D s_33_3: read-var u#22929:u32
        let s_33_3: u32 = fn_state.u_22929;
        // D s_33_4: cast zx s_33_3 -> bv
        let s_33_4: Bits = Bits::new(s_33_3 as u128, 32u16);
        // C s_33_5: const #1s : i64
        let s_33_5: i64 = 1;
        // C s_33_6: cast zx s_33_5 -> i
        let s_33_6: i128 = (i128::try_from(s_33_5).unwrap());
        // C s_33_7: const #0s : i
        let s_33_7: i128 = 0;
        // C s_33_8: add s_33_7 s_33_6
        let s_33_8: i128 = (s_33_7 + s_33_6);
        // D s_33_9: bit-extract s_33_4 s_33_2 s_33_8
        let s_33_9: Bits = (Bits::new(
            ((s_33_4) >> (s_33_2)).value(),
            u16::try_from(s_33_8).unwrap(),
        ));
        // D s_33_10: cast reint s_33_9 -> u8
        let s_33_10: bool = ((s_33_9.value()) != 0);
        // D s_33_11: cast zx s_33_10 -> bv
        let s_33_11: Bits = Bits::new(s_33_10 as u128, 1u16);
        // C s_33_12: const #1u : u8
        let s_33_12: bool = true;
        // C s_33_13: cast zx s_33_12 -> bv
        let s_33_13: Bits = Bits::new(s_33_12 as u128, 1u16);
        // D s_33_14: cmp-eq s_33_11 s_33_13
        let s_33_14: bool = ((s_33_11) == (s_33_13));
        // N s_33_15: branch s_33_14 b132 b34
        if s_33_14 {
            return block_132(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#373296 <= s_34_0
        fn_state.gs_373296 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#373296:u8
        let s_35_0: bool = fn_state.gs_373296;
        // N s_35_1: branch s_35_0 b131 b36
        if s_35_0 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#373298 <= s_36_0
        fn_state.gs_373298 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#373298:u8
        let s_37_0: bool = fn_state.gs_373298;
        // D s_37_1: not s_37_0
        let s_37_1: bool = !s_37_0;
        // N s_37_2: branch s_37_1 b39 b38
        if s_37_1 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #35s : i
        let s_38_0: i128 = 35;
        // C s_38_1: const #14696u : u32
        let s_38_1: u32 = 14696;
        // N s_38_2: write-reg s_38_1 <= s_38_0
        let s_38_2: () = {
            state.write_register::<i128>(s_38_1 as isize, s_38_0);
            tracer.write_register(s_38_1 as isize, s_38_0);
        };
        // C s_38_3: const #0s : i
        let s_38_3: i128 = 0;
        // C s_38_4: const #5s : i
        let s_38_4: i128 = 5;
        // D s_38_5: read-var u#22929:u32
        let s_38_5: u32 = fn_state.u_22929;
        // D s_38_6: cast zx s_38_5 -> bv
        let s_38_6: Bits = Bits::new(s_38_5 as u128, 32u16);
        // D s_38_7: bit-extract s_38_6 s_38_3 s_38_4
        let s_38_7: Bits = (Bits::new(
            ((s_38_6) >> (s_38_3)).value(),
            u16::try_from(s_38_4).unwrap(),
        ));
        // D s_38_8: cast reint s_38_7 -> u8
        let s_38_8: u8 = (s_38_7.value() as u8);
        // C s_38_9: const #5s : i
        let s_38_9: i128 = 5;
        // C s_38_10: const #19s : i
        let s_38_10: i128 = 19;
        // D s_38_11: read-var u#22929:u32
        let s_38_11: u32 = fn_state.u_22929;
        // D s_38_12: cast zx s_38_11 -> bv
        let s_38_12: Bits = Bits::new(s_38_11 as u128, 32u16);
        // D s_38_13: bit-extract s_38_12 s_38_9 s_38_10
        let s_38_13: Bits = (Bits::new(
            ((s_38_12) >> (s_38_9)).value(),
            u16::try_from(s_38_10).unwrap(),
        ));
        // D s_38_14: cast reint s_38_13 -> u19
        let s_38_14: u32 = (s_38_13.value() as u32);
        // C s_38_15: const #29s : i
        let s_38_15: i128 = 29;
        // C s_38_16: const #2s : i
        let s_38_16: i128 = 2;
        // D s_38_17: read-var u#22929:u32
        let s_38_17: u32 = fn_state.u_22929;
        // D s_38_18: cast zx s_38_17 -> bv
        let s_38_18: Bits = Bits::new(s_38_17 as u128, 32u16);
        // D s_38_19: bit-extract s_38_18 s_38_15 s_38_16
        let s_38_19: Bits = (Bits::new(
            ((s_38_18) >> (s_38_15)).value(),
            u16::try_from(s_38_16).unwrap(),
        ));
        // D s_38_20: cast reint s_38_19 -> u8
        let s_38_20: u8 = (s_38_19.value() as u8);
        // C s_38_21: const #31s : i
        let s_38_21: i128 = 31;
        // C s_38_22: const #1s : i
        let s_38_22: i128 = 1;
        // D s_38_23: read-var u#22929:u32
        let s_38_23: u32 = fn_state.u_22929;
        // D s_38_24: cast zx s_38_23 -> bv
        let s_38_24: Bits = Bits::new(s_38_23 as u128, 32u16);
        // D s_38_25: bit-extract s_38_24 s_38_21 s_38_22
        let s_38_25: Bits = (Bits::new(
            ((s_38_24) >> (s_38_21)).value(),
            u16::try_from(s_38_22).unwrap(),
        ));
        // D s_38_26: cast reint s_38_25 -> u8
        let s_38_26: bool = ((s_38_25.value()) != 0);
        // D s_38_27: call decode_adrp_aarch64_instrs_integer_arithmetic_address_pc_rel(s_38_8, s_38_14, s_38_20, s_38_26)
        let s_38_27: () = decode_adrp_aarch64_instrs_integer_arithmetic_address_pc_rel(
            state,
            tracer,
            s_38_8,
            s_38_14,
            s_38_20,
            s_38_26,
        );
        // N s_38_28: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var merge#var.1:struct
        let s_39_0: u32 = fn_state.merge_var._1;
        // D s_39_1: write-var u#22935 <= s_39_0
        fn_state.u_22935 = s_39_0;
        // C s_39_2: const #23s : i
        let s_39_2: i128 = 23;
        // D s_39_3: read-var u#22935:u32
        let s_39_3: u32 = fn_state.u_22935;
        // D s_39_4: cast zx s_39_3 -> bv
        let s_39_4: Bits = Bits::new(s_39_3 as u128, 32u16);
        // C s_39_5: const #1s : i64
        let s_39_5: i64 = 1;
        // C s_39_6: cast zx s_39_5 -> i
        let s_39_6: i128 = (i128::try_from(s_39_5).unwrap());
        // C s_39_7: const #7s : i
        let s_39_7: i128 = 7;
        // C s_39_8: add s_39_7 s_39_6
        let s_39_8: i128 = (s_39_7 + s_39_6);
        // D s_39_9: bit-extract s_39_4 s_39_2 s_39_8
        let s_39_9: Bits = (Bits::new(
            ((s_39_4) >> (s_39_2)).value(),
            u16::try_from(s_39_8).unwrap(),
        ));
        // D s_39_10: cast reint s_39_9 -> u8
        let s_39_10: u8 = (s_39_9.value() as u8);
        // D s_39_11: cast zx s_39_10 -> bv
        let s_39_11: Bits = Bits::new(s_39_10 as u128, 8u16);
        // C s_39_12: const #36u : u8
        let s_39_12: u8 = 36;
        // C s_39_13: cast zx s_39_12 -> bv
        let s_39_13: Bits = Bits::new(s_39_12 as u128, 8u16);
        // D s_39_14: cmp-eq s_39_11 s_39_13
        let s_39_14: bool = ((s_39_11) == (s_39_13));
        // N s_39_15: branch s_39_14 b130 b40
        if s_39_14 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#373313 <= s_40_0
        fn_state.gs_373313 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#373313:u8
        let s_41_0: bool = fn_state.gs_373313;
        // D s_41_1: not s_41_0
        let s_41_1: bool = !s_41_0;
        // N s_41_2: branch s_41_1 b43 b42
        if s_41_1 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #40s : i
        let s_42_0: i128 = 40;
        // C s_42_1: const #14696u : u32
        let s_42_1: u32 = 14696;
        // N s_42_2: write-reg s_42_1 <= s_42_0
        let s_42_2: () = {
            state.write_register::<i128>(s_42_1 as isize, s_42_0);
            tracer.write_register(s_42_1 as isize, s_42_0);
        };
        // C s_42_3: const #0s : i
        let s_42_3: i128 = 0;
        // C s_42_4: const #5s : i
        let s_42_4: i128 = 5;
        // D s_42_5: read-var u#22935:u32
        let s_42_5: u32 = fn_state.u_22935;
        // D s_42_6: cast zx s_42_5 -> bv
        let s_42_6: Bits = Bits::new(s_42_5 as u128, 32u16);
        // D s_42_7: bit-extract s_42_6 s_42_3 s_42_4
        let s_42_7: Bits = (Bits::new(
            ((s_42_6) >> (s_42_3)).value(),
            u16::try_from(s_42_4).unwrap(),
        ));
        // D s_42_8: cast reint s_42_7 -> u8
        let s_42_8: u8 = (s_42_7.value() as u8);
        // C s_42_9: const #5s : i
        let s_42_9: i128 = 5;
        // C s_42_10: const #5s : i
        let s_42_10: i128 = 5;
        // D s_42_11: read-var u#22935:u32
        let s_42_11: u32 = fn_state.u_22935;
        // D s_42_12: cast zx s_42_11 -> bv
        let s_42_12: Bits = Bits::new(s_42_11 as u128, 32u16);
        // D s_42_13: bit-extract s_42_12 s_42_9 s_42_10
        let s_42_13: Bits = (Bits::new(
            ((s_42_12) >> (s_42_9)).value(),
            u16::try_from(s_42_10).unwrap(),
        ));
        // D s_42_14: cast reint s_42_13 -> u8
        let s_42_14: u8 = (s_42_13.value() as u8);
        // C s_42_15: const #10s : i
        let s_42_15: i128 = 10;
        // C s_42_16: const #6s : i
        let s_42_16: i128 = 6;
        // D s_42_17: read-var u#22935:u32
        let s_42_17: u32 = fn_state.u_22935;
        // D s_42_18: cast zx s_42_17 -> bv
        let s_42_18: Bits = Bits::new(s_42_17 as u128, 32u16);
        // D s_42_19: bit-extract s_42_18 s_42_15 s_42_16
        let s_42_19: Bits = (Bits::new(
            ((s_42_18) >> (s_42_15)).value(),
            u16::try_from(s_42_16).unwrap(),
        ));
        // D s_42_20: cast reint s_42_19 -> u8
        let s_42_20: u8 = (s_42_19.value() as u8);
        // C s_42_21: const #16s : i
        let s_42_21: i128 = 16;
        // C s_42_22: const #6s : i
        let s_42_22: i128 = 6;
        // D s_42_23: read-var u#22935:u32
        let s_42_23: u32 = fn_state.u_22935;
        // D s_42_24: cast zx s_42_23 -> bv
        let s_42_24: Bits = Bits::new(s_42_23 as u128, 32u16);
        // D s_42_25: bit-extract s_42_24 s_42_21 s_42_22
        let s_42_25: Bits = (Bits::new(
            ((s_42_24) >> (s_42_21)).value(),
            u16::try_from(s_42_22).unwrap(),
        ));
        // D s_42_26: cast reint s_42_25 -> u8
        let s_42_26: u8 = (s_42_25.value() as u8);
        // C s_42_27: const #22s : i
        let s_42_27: i128 = 22;
        // C s_42_28: const #1s : i
        let s_42_28: i128 = 1;
        // D s_42_29: read-var u#22935:u32
        let s_42_29: u32 = fn_state.u_22935;
        // D s_42_30: cast zx s_42_29 -> bv
        let s_42_30: Bits = Bits::new(s_42_29 as u128, 32u16);
        // D s_42_31: bit-extract s_42_30 s_42_27 s_42_28
        let s_42_31: Bits = (Bits::new(
            ((s_42_30) >> (s_42_27)).value(),
            u16::try_from(s_42_28).unwrap(),
        ));
        // D s_42_32: cast reint s_42_31 -> u8
        let s_42_32: bool = ((s_42_31.value()) != 0);
        // C s_42_33: const #29s : i
        let s_42_33: i128 = 29;
        // C s_42_34: const #2s : i
        let s_42_34: i128 = 2;
        // D s_42_35: read-var u#22935:u32
        let s_42_35: u32 = fn_state.u_22935;
        // D s_42_36: cast zx s_42_35 -> bv
        let s_42_36: Bits = Bits::new(s_42_35 as u128, 32u16);
        // D s_42_37: bit-extract s_42_36 s_42_33 s_42_34
        let s_42_37: Bits = (Bits::new(
            ((s_42_36) >> (s_42_33)).value(),
            u16::try_from(s_42_34).unwrap(),
        ));
        // D s_42_38: cast reint s_42_37 -> u8
        let s_42_38: u8 = (s_42_37.value() as u8);
        // C s_42_39: const #31s : i
        let s_42_39: i128 = 31;
        // C s_42_40: const #1s : i
        let s_42_40: i128 = 1;
        // D s_42_41: read-var u#22935:u32
        let s_42_41: u32 = fn_state.u_22935;
        // D s_42_42: cast zx s_42_41 -> bv
        let s_42_42: Bits = Bits::new(s_42_41 as u128, 32u16);
        // D s_42_43: bit-extract s_42_42 s_42_39 s_42_40
        let s_42_43: Bits = (Bits::new(
            ((s_42_42) >> (s_42_39)).value(),
            u16::try_from(s_42_40).unwrap(),
        ));
        // D s_42_44: cast reint s_42_43 -> u8
        let s_42_44: bool = ((s_42_43.value()) != 0);
        // D s_42_45: call decode_and_log_imm_aarch64_instrs_integer_logical_immediate(s_42_8, s_42_14, s_42_20, s_42_26, s_42_32, s_42_38, s_42_44)
        let s_42_45: () = decode_and_log_imm_aarch64_instrs_integer_logical_immediate(
            state,
            tracer,
            s_42_8,
            s_42_14,
            s_42_20,
            s_42_26,
            s_42_32,
            s_42_38,
            s_42_44,
        );
        // N s_42_46: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var merge#var.1:struct
        let s_43_0: u32 = fn_state.merge_var._1;
        // D s_43_1: write-var u#22940 <= s_43_0
        fn_state.u_22940 = s_43_0;
        // C s_43_2: const #23s : i
        let s_43_2: i128 = 23;
        // D s_43_3: read-var u#22940:u32
        let s_43_3: u32 = fn_state.u_22940;
        // D s_43_4: cast zx s_43_3 -> bv
        let s_43_4: Bits = Bits::new(s_43_3 as u128, 32u16);
        // C s_43_5: const #1s : i64
        let s_43_5: i64 = 1;
        // C s_43_6: cast zx s_43_5 -> i
        let s_43_6: i128 = (i128::try_from(s_43_5).unwrap());
        // C s_43_7: const #7s : i
        let s_43_7: i128 = 7;
        // C s_43_8: add s_43_7 s_43_6
        let s_43_8: i128 = (s_43_7 + s_43_6);
        // D s_43_9: bit-extract s_43_4 s_43_2 s_43_8
        let s_43_9: Bits = (Bits::new(
            ((s_43_4) >> (s_43_2)).value(),
            u16::try_from(s_43_8).unwrap(),
        ));
        // D s_43_10: cast reint s_43_9 -> u8
        let s_43_10: u8 = (s_43_9.value() as u8);
        // D s_43_11: cast zx s_43_10 -> bv
        let s_43_11: Bits = Bits::new(s_43_10 as u128, 8u16);
        // C s_43_12: const #228u : u8
        let s_43_12: u8 = 228;
        // C s_43_13: cast zx s_43_12 -> bv
        let s_43_13: Bits = Bits::new(s_43_12 as u128, 8u16);
        // D s_43_14: cmp-eq s_43_11 s_43_13
        let s_43_14: bool = ((s_43_11) == (s_43_13));
        // N s_43_15: branch s_43_14 b129 b44
        if s_43_14 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#373334 <= s_44_0
        fn_state.gs_373334 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#373334:u8
        let s_45_0: bool = fn_state.gs_373334;
        // D s_45_1: not s_45_0
        let s_45_1: bool = !s_45_0;
        // N s_45_2: branch s_45_1 b47 b46
        if s_45_1 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #41s : i
        let s_46_0: i128 = 41;
        // C s_46_1: const #14696u : u32
        let s_46_1: u32 = 14696;
        // N s_46_2: write-reg s_46_1 <= s_46_0
        let s_46_2: () = {
            state.write_register::<i128>(s_46_1 as isize, s_46_0);
            tracer.write_register(s_46_1 as isize, s_46_0);
        };
        // C s_46_3: const #0s : i
        let s_46_3: i128 = 0;
        // C s_46_4: const #5s : i
        let s_46_4: i128 = 5;
        // D s_46_5: read-var u#22940:u32
        let s_46_5: u32 = fn_state.u_22940;
        // D s_46_6: cast zx s_46_5 -> bv
        let s_46_6: Bits = Bits::new(s_46_5 as u128, 32u16);
        // D s_46_7: bit-extract s_46_6 s_46_3 s_46_4
        let s_46_7: Bits = (Bits::new(
            ((s_46_6) >> (s_46_3)).value(),
            u16::try_from(s_46_4).unwrap(),
        ));
        // D s_46_8: cast reint s_46_7 -> u8
        let s_46_8: u8 = (s_46_7.value() as u8);
        // C s_46_9: const #5s : i
        let s_46_9: i128 = 5;
        // C s_46_10: const #5s : i
        let s_46_10: i128 = 5;
        // D s_46_11: read-var u#22940:u32
        let s_46_11: u32 = fn_state.u_22940;
        // D s_46_12: cast zx s_46_11 -> bv
        let s_46_12: Bits = Bits::new(s_46_11 as u128, 32u16);
        // D s_46_13: bit-extract s_46_12 s_46_9 s_46_10
        let s_46_13: Bits = (Bits::new(
            ((s_46_12) >> (s_46_9)).value(),
            u16::try_from(s_46_10).unwrap(),
        ));
        // D s_46_14: cast reint s_46_13 -> u8
        let s_46_14: u8 = (s_46_13.value() as u8);
        // C s_46_15: const #10s : i
        let s_46_15: i128 = 10;
        // C s_46_16: const #6s : i
        let s_46_16: i128 = 6;
        // D s_46_17: read-var u#22940:u32
        let s_46_17: u32 = fn_state.u_22940;
        // D s_46_18: cast zx s_46_17 -> bv
        let s_46_18: Bits = Bits::new(s_46_17 as u128, 32u16);
        // D s_46_19: bit-extract s_46_18 s_46_15 s_46_16
        let s_46_19: Bits = (Bits::new(
            ((s_46_18) >> (s_46_15)).value(),
            u16::try_from(s_46_16).unwrap(),
        ));
        // D s_46_20: cast reint s_46_19 -> u8
        let s_46_20: u8 = (s_46_19.value() as u8);
        // C s_46_21: const #16s : i
        let s_46_21: i128 = 16;
        // C s_46_22: const #6s : i
        let s_46_22: i128 = 6;
        // D s_46_23: read-var u#22940:u32
        let s_46_23: u32 = fn_state.u_22940;
        // D s_46_24: cast zx s_46_23 -> bv
        let s_46_24: Bits = Bits::new(s_46_23 as u128, 32u16);
        // D s_46_25: bit-extract s_46_24 s_46_21 s_46_22
        let s_46_25: Bits = (Bits::new(
            ((s_46_24) >> (s_46_21)).value(),
            u16::try_from(s_46_22).unwrap(),
        ));
        // D s_46_26: cast reint s_46_25 -> u8
        let s_46_26: u8 = (s_46_25.value() as u8);
        // C s_46_27: const #22s : i
        let s_46_27: i128 = 22;
        // C s_46_28: const #1s : i
        let s_46_28: i128 = 1;
        // D s_46_29: read-var u#22940:u32
        let s_46_29: u32 = fn_state.u_22940;
        // D s_46_30: cast zx s_46_29 -> bv
        let s_46_30: Bits = Bits::new(s_46_29 as u128, 32u16);
        // D s_46_31: bit-extract s_46_30 s_46_27 s_46_28
        let s_46_31: Bits = (Bits::new(
            ((s_46_30) >> (s_46_27)).value(),
            u16::try_from(s_46_28).unwrap(),
        ));
        // D s_46_32: cast reint s_46_31 -> u8
        let s_46_32: bool = ((s_46_31.value()) != 0);
        // C s_46_33: const #29s : i
        let s_46_33: i128 = 29;
        // C s_46_34: const #2s : i
        let s_46_34: i128 = 2;
        // D s_46_35: read-var u#22940:u32
        let s_46_35: u32 = fn_state.u_22940;
        // D s_46_36: cast zx s_46_35 -> bv
        let s_46_36: Bits = Bits::new(s_46_35 as u128, 32u16);
        // D s_46_37: bit-extract s_46_36 s_46_33 s_46_34
        let s_46_37: Bits = (Bits::new(
            ((s_46_36) >> (s_46_33)).value(),
            u16::try_from(s_46_34).unwrap(),
        ));
        // D s_46_38: cast reint s_46_37 -> u8
        let s_46_38: u8 = (s_46_37.value() as u8);
        // C s_46_39: const #31s : i
        let s_46_39: i128 = 31;
        // C s_46_40: const #1s : i
        let s_46_40: i128 = 1;
        // D s_46_41: read-var u#22940:u32
        let s_46_41: u32 = fn_state.u_22940;
        // D s_46_42: cast zx s_46_41 -> bv
        let s_46_42: Bits = Bits::new(s_46_41 as u128, 32u16);
        // D s_46_43: bit-extract s_46_42 s_46_39 s_46_40
        let s_46_43: Bits = (Bits::new(
            ((s_46_42) >> (s_46_39)).value(),
            u16::try_from(s_46_40).unwrap(),
        ));
        // D s_46_44: cast reint s_46_43 -> u8
        let s_46_44: bool = ((s_46_43.value()) != 0);
        // D s_46_45: call decode_ands_log_imm_aarch64_instrs_integer_logical_immediate(s_46_8, s_46_14, s_46_20, s_46_26, s_46_32, s_46_38, s_46_44)
        let s_46_45: () = decode_ands_log_imm_aarch64_instrs_integer_logical_immediate(
            state,
            tracer,
            s_46_8,
            s_46_14,
            s_46_20,
            s_46_26,
            s_46_32,
            s_46_38,
            s_46_44,
        );
        // N s_46_46: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var merge#var.1:struct
        let s_47_0: u32 = fn_state.merge_var._1;
        // D s_47_1: write-var u#22949 <= s_47_0
        fn_state.u_22949 = s_47_0;
        // C s_47_2: const #23s : i
        let s_47_2: i128 = 23;
        // D s_47_3: read-var u#22949:u32
        let s_47_3: u32 = fn_state.u_22949;
        // D s_47_4: cast zx s_47_3 -> bv
        let s_47_4: Bits = Bits::new(s_47_3 as u128, 32u16);
        // C s_47_5: const #1s : i64
        let s_47_5: i64 = 1;
        // C s_47_6: cast zx s_47_5 -> i
        let s_47_6: i128 = (i128::try_from(s_47_5).unwrap());
        // C s_47_7: const #7s : i
        let s_47_7: i128 = 7;
        // C s_47_8: add s_47_7 s_47_6
        let s_47_8: i128 = (s_47_7 + s_47_6);
        // D s_47_9: bit-extract s_47_4 s_47_2 s_47_8
        let s_47_9: Bits = (Bits::new(
            ((s_47_4) >> (s_47_2)).value(),
            u16::try_from(s_47_8).unwrap(),
        ));
        // D s_47_10: cast reint s_47_9 -> u8
        let s_47_10: u8 = (s_47_9.value() as u8);
        // D s_47_11: cast zx s_47_10 -> bv
        let s_47_11: Bits = Bits::new(s_47_10 as u128, 8u16);
        // C s_47_12: const #164u : u8
        let s_47_12: u8 = 164;
        // C s_47_13: cast zx s_47_12 -> bv
        let s_47_13: Bits = Bits::new(s_47_12 as u128, 8u16);
        // D s_47_14: cmp-eq s_47_11 s_47_13
        let s_47_14: bool = ((s_47_11) == (s_47_13));
        // N s_47_15: branch s_47_14 b128 b48
        if s_47_14 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#373355 <= s_48_0
        fn_state.gs_373355 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#373355:u8
        let s_49_0: bool = fn_state.gs_373355;
        // D s_49_1: not s_49_0
        let s_49_1: bool = !s_49_0;
        // N s_49_2: branch s_49_1 b51 b50
        if s_49_1 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #42s : i
        let s_50_0: i128 = 42;
        // C s_50_1: const #14696u : u32
        let s_50_1: u32 = 14696;
        // N s_50_2: write-reg s_50_1 <= s_50_0
        let s_50_2: () = {
            state.write_register::<i128>(s_50_1 as isize, s_50_0);
            tracer.write_register(s_50_1 as isize, s_50_0);
        };
        // C s_50_3: const #0s : i
        let s_50_3: i128 = 0;
        // C s_50_4: const #5s : i
        let s_50_4: i128 = 5;
        // D s_50_5: read-var u#22949:u32
        let s_50_5: u32 = fn_state.u_22949;
        // D s_50_6: cast zx s_50_5 -> bv
        let s_50_6: Bits = Bits::new(s_50_5 as u128, 32u16);
        // D s_50_7: bit-extract s_50_6 s_50_3 s_50_4
        let s_50_7: Bits = (Bits::new(
            ((s_50_6) >> (s_50_3)).value(),
            u16::try_from(s_50_4).unwrap(),
        ));
        // D s_50_8: cast reint s_50_7 -> u8
        let s_50_8: u8 = (s_50_7.value() as u8);
        // C s_50_9: const #5s : i
        let s_50_9: i128 = 5;
        // C s_50_10: const #5s : i
        let s_50_10: i128 = 5;
        // D s_50_11: read-var u#22949:u32
        let s_50_11: u32 = fn_state.u_22949;
        // D s_50_12: cast zx s_50_11 -> bv
        let s_50_12: Bits = Bits::new(s_50_11 as u128, 32u16);
        // D s_50_13: bit-extract s_50_12 s_50_9 s_50_10
        let s_50_13: Bits = (Bits::new(
            ((s_50_12) >> (s_50_9)).value(),
            u16::try_from(s_50_10).unwrap(),
        ));
        // D s_50_14: cast reint s_50_13 -> u8
        let s_50_14: u8 = (s_50_13.value() as u8);
        // C s_50_15: const #10s : i
        let s_50_15: i128 = 10;
        // C s_50_16: const #6s : i
        let s_50_16: i128 = 6;
        // D s_50_17: read-var u#22949:u32
        let s_50_17: u32 = fn_state.u_22949;
        // D s_50_18: cast zx s_50_17 -> bv
        let s_50_18: Bits = Bits::new(s_50_17 as u128, 32u16);
        // D s_50_19: bit-extract s_50_18 s_50_15 s_50_16
        let s_50_19: Bits = (Bits::new(
            ((s_50_18) >> (s_50_15)).value(),
            u16::try_from(s_50_16).unwrap(),
        ));
        // D s_50_20: cast reint s_50_19 -> u8
        let s_50_20: u8 = (s_50_19.value() as u8);
        // C s_50_21: const #16s : i
        let s_50_21: i128 = 16;
        // C s_50_22: const #6s : i
        let s_50_22: i128 = 6;
        // D s_50_23: read-var u#22949:u32
        let s_50_23: u32 = fn_state.u_22949;
        // D s_50_24: cast zx s_50_23 -> bv
        let s_50_24: Bits = Bits::new(s_50_23 as u128, 32u16);
        // D s_50_25: bit-extract s_50_24 s_50_21 s_50_22
        let s_50_25: Bits = (Bits::new(
            ((s_50_24) >> (s_50_21)).value(),
            u16::try_from(s_50_22).unwrap(),
        ));
        // D s_50_26: cast reint s_50_25 -> u8
        let s_50_26: u8 = (s_50_25.value() as u8);
        // C s_50_27: const #22s : i
        let s_50_27: i128 = 22;
        // C s_50_28: const #1s : i
        let s_50_28: i128 = 1;
        // D s_50_29: read-var u#22949:u32
        let s_50_29: u32 = fn_state.u_22949;
        // D s_50_30: cast zx s_50_29 -> bv
        let s_50_30: Bits = Bits::new(s_50_29 as u128, 32u16);
        // D s_50_31: bit-extract s_50_30 s_50_27 s_50_28
        let s_50_31: Bits = (Bits::new(
            ((s_50_30) >> (s_50_27)).value(),
            u16::try_from(s_50_28).unwrap(),
        ));
        // D s_50_32: cast reint s_50_31 -> u8
        let s_50_32: bool = ((s_50_31.value()) != 0);
        // C s_50_33: const #29s : i
        let s_50_33: i128 = 29;
        // C s_50_34: const #2s : i
        let s_50_34: i128 = 2;
        // D s_50_35: read-var u#22949:u32
        let s_50_35: u32 = fn_state.u_22949;
        // D s_50_36: cast zx s_50_35 -> bv
        let s_50_36: Bits = Bits::new(s_50_35 as u128, 32u16);
        // D s_50_37: bit-extract s_50_36 s_50_33 s_50_34
        let s_50_37: Bits = (Bits::new(
            ((s_50_36) >> (s_50_33)).value(),
            u16::try_from(s_50_34).unwrap(),
        ));
        // D s_50_38: cast reint s_50_37 -> u8
        let s_50_38: u8 = (s_50_37.value() as u8);
        // C s_50_39: const #31s : i
        let s_50_39: i128 = 31;
        // C s_50_40: const #1s : i
        let s_50_40: i128 = 1;
        // D s_50_41: read-var u#22949:u32
        let s_50_41: u32 = fn_state.u_22949;
        // D s_50_42: cast zx s_50_41 -> bv
        let s_50_42: Bits = Bits::new(s_50_41 as u128, 32u16);
        // D s_50_43: bit-extract s_50_42 s_50_39 s_50_40
        let s_50_43: Bits = (Bits::new(
            ((s_50_42) >> (s_50_39)).value(),
            u16::try_from(s_50_40).unwrap(),
        ));
        // D s_50_44: cast reint s_50_43 -> u8
        let s_50_44: bool = ((s_50_43.value()) != 0);
        // D s_50_45: call decode_eor_log_imm_aarch64_instrs_integer_logical_immediate(s_50_8, s_50_14, s_50_20, s_50_26, s_50_32, s_50_38, s_50_44)
        let s_50_45: () = decode_eor_log_imm_aarch64_instrs_integer_logical_immediate(
            state,
            tracer,
            s_50_8,
            s_50_14,
            s_50_20,
            s_50_26,
            s_50_32,
            s_50_38,
            s_50_44,
        );
        // N s_50_46: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var merge#var.1:struct
        let s_51_0: u32 = fn_state.merge_var._1;
        // D s_51_1: write-var u#22958 <= s_51_0
        fn_state.u_22958 = s_51_0;
        // C s_51_2: const #23s : i
        let s_51_2: i128 = 23;
        // D s_51_3: read-var u#22958:u32
        let s_51_3: u32 = fn_state.u_22958;
        // D s_51_4: cast zx s_51_3 -> bv
        let s_51_4: Bits = Bits::new(s_51_3 as u128, 32u16);
        // C s_51_5: const #1s : i64
        let s_51_5: i64 = 1;
        // C s_51_6: cast zx s_51_5 -> i
        let s_51_6: i128 = (i128::try_from(s_51_5).unwrap());
        // C s_51_7: const #7s : i
        let s_51_7: i128 = 7;
        // C s_51_8: add s_51_7 s_51_6
        let s_51_8: i128 = (s_51_7 + s_51_6);
        // D s_51_9: bit-extract s_51_4 s_51_2 s_51_8
        let s_51_9: Bits = (Bits::new(
            ((s_51_4) >> (s_51_2)).value(),
            u16::try_from(s_51_8).unwrap(),
        ));
        // D s_51_10: cast reint s_51_9 -> u8
        let s_51_10: u8 = (s_51_9.value() as u8);
        // D s_51_11: cast zx s_51_10 -> bv
        let s_51_11: Bits = Bits::new(s_51_10 as u128, 8u16);
        // C s_51_12: const #100u : u8
        let s_51_12: u8 = 100;
        // C s_51_13: cast zx s_51_12 -> bv
        let s_51_13: Bits = Bits::new(s_51_12 as u128, 8u16);
        // D s_51_14: cmp-eq s_51_11 s_51_13
        let s_51_14: bool = ((s_51_11) == (s_51_13));
        // N s_51_15: branch s_51_14 b127 b52
        if s_51_14 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#373376 <= s_52_0
        fn_state.gs_373376 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#373376:u8
        let s_53_0: bool = fn_state.gs_373376;
        // D s_53_1: not s_53_0
        let s_53_1: bool = !s_53_0;
        // N s_53_2: branch s_53_1 b55 b54
        if s_53_1 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #43s : i
        let s_54_0: i128 = 43;
        // C s_54_1: const #14696u : u32
        let s_54_1: u32 = 14696;
        // N s_54_2: write-reg s_54_1 <= s_54_0
        let s_54_2: () = {
            state.write_register::<i128>(s_54_1 as isize, s_54_0);
            tracer.write_register(s_54_1 as isize, s_54_0);
        };
        // C s_54_3: const #0s : i
        let s_54_3: i128 = 0;
        // C s_54_4: const #5s : i
        let s_54_4: i128 = 5;
        // D s_54_5: read-var u#22958:u32
        let s_54_5: u32 = fn_state.u_22958;
        // D s_54_6: cast zx s_54_5 -> bv
        let s_54_6: Bits = Bits::new(s_54_5 as u128, 32u16);
        // D s_54_7: bit-extract s_54_6 s_54_3 s_54_4
        let s_54_7: Bits = (Bits::new(
            ((s_54_6) >> (s_54_3)).value(),
            u16::try_from(s_54_4).unwrap(),
        ));
        // D s_54_8: cast reint s_54_7 -> u8
        let s_54_8: u8 = (s_54_7.value() as u8);
        // C s_54_9: const #5s : i
        let s_54_9: i128 = 5;
        // C s_54_10: const #5s : i
        let s_54_10: i128 = 5;
        // D s_54_11: read-var u#22958:u32
        let s_54_11: u32 = fn_state.u_22958;
        // D s_54_12: cast zx s_54_11 -> bv
        let s_54_12: Bits = Bits::new(s_54_11 as u128, 32u16);
        // D s_54_13: bit-extract s_54_12 s_54_9 s_54_10
        let s_54_13: Bits = (Bits::new(
            ((s_54_12) >> (s_54_9)).value(),
            u16::try_from(s_54_10).unwrap(),
        ));
        // D s_54_14: cast reint s_54_13 -> u8
        let s_54_14: u8 = (s_54_13.value() as u8);
        // C s_54_15: const #10s : i
        let s_54_15: i128 = 10;
        // C s_54_16: const #6s : i
        let s_54_16: i128 = 6;
        // D s_54_17: read-var u#22958:u32
        let s_54_17: u32 = fn_state.u_22958;
        // D s_54_18: cast zx s_54_17 -> bv
        let s_54_18: Bits = Bits::new(s_54_17 as u128, 32u16);
        // D s_54_19: bit-extract s_54_18 s_54_15 s_54_16
        let s_54_19: Bits = (Bits::new(
            ((s_54_18) >> (s_54_15)).value(),
            u16::try_from(s_54_16).unwrap(),
        ));
        // D s_54_20: cast reint s_54_19 -> u8
        let s_54_20: u8 = (s_54_19.value() as u8);
        // C s_54_21: const #16s : i
        let s_54_21: i128 = 16;
        // C s_54_22: const #6s : i
        let s_54_22: i128 = 6;
        // D s_54_23: read-var u#22958:u32
        let s_54_23: u32 = fn_state.u_22958;
        // D s_54_24: cast zx s_54_23 -> bv
        let s_54_24: Bits = Bits::new(s_54_23 as u128, 32u16);
        // D s_54_25: bit-extract s_54_24 s_54_21 s_54_22
        let s_54_25: Bits = (Bits::new(
            ((s_54_24) >> (s_54_21)).value(),
            u16::try_from(s_54_22).unwrap(),
        ));
        // D s_54_26: cast reint s_54_25 -> u8
        let s_54_26: u8 = (s_54_25.value() as u8);
        // C s_54_27: const #22s : i
        let s_54_27: i128 = 22;
        // C s_54_28: const #1s : i
        let s_54_28: i128 = 1;
        // D s_54_29: read-var u#22958:u32
        let s_54_29: u32 = fn_state.u_22958;
        // D s_54_30: cast zx s_54_29 -> bv
        let s_54_30: Bits = Bits::new(s_54_29 as u128, 32u16);
        // D s_54_31: bit-extract s_54_30 s_54_27 s_54_28
        let s_54_31: Bits = (Bits::new(
            ((s_54_30) >> (s_54_27)).value(),
            u16::try_from(s_54_28).unwrap(),
        ));
        // D s_54_32: cast reint s_54_31 -> u8
        let s_54_32: bool = ((s_54_31.value()) != 0);
        // C s_54_33: const #29s : i
        let s_54_33: i128 = 29;
        // C s_54_34: const #2s : i
        let s_54_34: i128 = 2;
        // D s_54_35: read-var u#22958:u32
        let s_54_35: u32 = fn_state.u_22958;
        // D s_54_36: cast zx s_54_35 -> bv
        let s_54_36: Bits = Bits::new(s_54_35 as u128, 32u16);
        // D s_54_37: bit-extract s_54_36 s_54_33 s_54_34
        let s_54_37: Bits = (Bits::new(
            ((s_54_36) >> (s_54_33)).value(),
            u16::try_from(s_54_34).unwrap(),
        ));
        // D s_54_38: cast reint s_54_37 -> u8
        let s_54_38: u8 = (s_54_37.value() as u8);
        // C s_54_39: const #31s : i
        let s_54_39: i128 = 31;
        // C s_54_40: const #1s : i
        let s_54_40: i128 = 1;
        // D s_54_41: read-var u#22958:u32
        let s_54_41: u32 = fn_state.u_22958;
        // D s_54_42: cast zx s_54_41 -> bv
        let s_54_42: Bits = Bits::new(s_54_41 as u128, 32u16);
        // D s_54_43: bit-extract s_54_42 s_54_39 s_54_40
        let s_54_43: Bits = (Bits::new(
            ((s_54_42) >> (s_54_39)).value(),
            u16::try_from(s_54_40).unwrap(),
        ));
        // D s_54_44: cast reint s_54_43 -> u8
        let s_54_44: bool = ((s_54_43.value()) != 0);
        // D s_54_45: call decode_orr_log_imm_aarch64_instrs_integer_logical_immediate(s_54_8, s_54_14, s_54_20, s_54_26, s_54_32, s_54_38, s_54_44)
        let s_54_45: () = decode_orr_log_imm_aarch64_instrs_integer_logical_immediate(
            state,
            tracer,
            s_54_8,
            s_54_14,
            s_54_20,
            s_54_26,
            s_54_32,
            s_54_38,
            s_54_44,
        );
        // N s_54_46: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var merge#var.1:struct
        let s_55_0: u32 = fn_state.merge_var._1;
        // D s_55_1: write-var u#22967 <= s_55_0
        fn_state.u_22967 = s_55_0;
        // C s_55_2: const #23s : i
        let s_55_2: i128 = 23;
        // D s_55_3: read-var u#22967:u32
        let s_55_3: u32 = fn_state.u_22967;
        // D s_55_4: cast zx s_55_3 -> bv
        let s_55_4: Bits = Bits::new(s_55_3 as u128, 32u16);
        // C s_55_5: const #1s : i64
        let s_55_5: i64 = 1;
        // C s_55_6: cast zx s_55_5 -> i
        let s_55_6: i128 = (i128::try_from(s_55_5).unwrap());
        // C s_55_7: const #7s : i
        let s_55_7: i128 = 7;
        // C s_55_8: add s_55_7 s_55_6
        let s_55_8: i128 = (s_55_7 + s_55_6);
        // D s_55_9: bit-extract s_55_4 s_55_2 s_55_8
        let s_55_9: Bits = (Bits::new(
            ((s_55_4) >> (s_55_2)).value(),
            u16::try_from(s_55_8).unwrap(),
        ));
        // D s_55_10: cast reint s_55_9 -> u8
        let s_55_10: u8 = (s_55_9.value() as u8);
        // D s_55_11: cast zx s_55_10 -> bv
        let s_55_11: Bits = Bits::new(s_55_10 as u128, 8u16);
        // C s_55_12: const #102u : u8
        let s_55_12: u8 = 102;
        // C s_55_13: cast zx s_55_12 -> bv
        let s_55_13: Bits = Bits::new(s_55_12 as u128, 8u16);
        // D s_55_14: cmp-eq s_55_11 s_55_13
        let s_55_14: bool = ((s_55_11) == (s_55_13));
        // N s_55_15: branch s_55_14 b126 b56
        if s_55_14 {
            return block_126(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#373397 <= s_56_0
        fn_state.gs_373397 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#373397:u8
        let s_57_0: bool = fn_state.gs_373397;
        // D s_57_1: not s_57_0
        let s_57_1: bool = !s_57_0;
        // N s_57_2: branch s_57_1 b59 b58
        if s_57_1 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #76s : i
        let s_58_0: i128 = 76;
        // C s_58_1: const #14696u : u32
        let s_58_1: u32 = 14696;
        // N s_58_2: write-reg s_58_1 <= s_58_0
        let s_58_2: () = {
            state.write_register::<i128>(s_58_1 as isize, s_58_0);
            tracer.write_register(s_58_1 as isize, s_58_0);
        };
        // C s_58_3: const #0s : i
        let s_58_3: i128 = 0;
        // C s_58_4: const #5s : i
        let s_58_4: i128 = 5;
        // D s_58_5: read-var u#22967:u32
        let s_58_5: u32 = fn_state.u_22967;
        // D s_58_6: cast zx s_58_5 -> bv
        let s_58_6: Bits = Bits::new(s_58_5 as u128, 32u16);
        // D s_58_7: bit-extract s_58_6 s_58_3 s_58_4
        let s_58_7: Bits = (Bits::new(
            ((s_58_6) >> (s_58_3)).value(),
            u16::try_from(s_58_4).unwrap(),
        ));
        // D s_58_8: cast reint s_58_7 -> u8
        let s_58_8: u8 = (s_58_7.value() as u8);
        // C s_58_9: const #5s : i
        let s_58_9: i128 = 5;
        // C s_58_10: const #5s : i
        let s_58_10: i128 = 5;
        // D s_58_11: read-var u#22967:u32
        let s_58_11: u32 = fn_state.u_22967;
        // D s_58_12: cast zx s_58_11 -> bv
        let s_58_12: Bits = Bits::new(s_58_11 as u128, 32u16);
        // D s_58_13: bit-extract s_58_12 s_58_9 s_58_10
        let s_58_13: Bits = (Bits::new(
            ((s_58_12) >> (s_58_9)).value(),
            u16::try_from(s_58_10).unwrap(),
        ));
        // D s_58_14: cast reint s_58_13 -> u8
        let s_58_14: u8 = (s_58_13.value() as u8);
        // C s_58_15: const #10s : i
        let s_58_15: i128 = 10;
        // C s_58_16: const #6s : i
        let s_58_16: i128 = 6;
        // D s_58_17: read-var u#22967:u32
        let s_58_17: u32 = fn_state.u_22967;
        // D s_58_18: cast zx s_58_17 -> bv
        let s_58_18: Bits = Bits::new(s_58_17 as u128, 32u16);
        // D s_58_19: bit-extract s_58_18 s_58_15 s_58_16
        let s_58_19: Bits = (Bits::new(
            ((s_58_18) >> (s_58_15)).value(),
            u16::try_from(s_58_16).unwrap(),
        ));
        // D s_58_20: cast reint s_58_19 -> u8
        let s_58_20: u8 = (s_58_19.value() as u8);
        // C s_58_21: const #16s : i
        let s_58_21: i128 = 16;
        // C s_58_22: const #6s : i
        let s_58_22: i128 = 6;
        // D s_58_23: read-var u#22967:u32
        let s_58_23: u32 = fn_state.u_22967;
        // D s_58_24: cast zx s_58_23 -> bv
        let s_58_24: Bits = Bits::new(s_58_23 as u128, 32u16);
        // D s_58_25: bit-extract s_58_24 s_58_21 s_58_22
        let s_58_25: Bits = (Bits::new(
            ((s_58_24) >> (s_58_21)).value(),
            u16::try_from(s_58_22).unwrap(),
        ));
        // D s_58_26: cast reint s_58_25 -> u8
        let s_58_26: u8 = (s_58_25.value() as u8);
        // C s_58_27: const #22s : i
        let s_58_27: i128 = 22;
        // C s_58_28: const #1s : i
        let s_58_28: i128 = 1;
        // D s_58_29: read-var u#22967:u32
        let s_58_29: u32 = fn_state.u_22967;
        // D s_58_30: cast zx s_58_29 -> bv
        let s_58_30: Bits = Bits::new(s_58_29 as u128, 32u16);
        // D s_58_31: bit-extract s_58_30 s_58_27 s_58_28
        let s_58_31: Bits = (Bits::new(
            ((s_58_30) >> (s_58_27)).value(),
            u16::try_from(s_58_28).unwrap(),
        ));
        // D s_58_32: cast reint s_58_31 -> u8
        let s_58_32: bool = ((s_58_31.value()) != 0);
        // C s_58_33: const #29s : i
        let s_58_33: i128 = 29;
        // C s_58_34: const #2s : i
        let s_58_34: i128 = 2;
        // D s_58_35: read-var u#22967:u32
        let s_58_35: u32 = fn_state.u_22967;
        // D s_58_36: cast zx s_58_35 -> bv
        let s_58_36: Bits = Bits::new(s_58_35 as u128, 32u16);
        // D s_58_37: bit-extract s_58_36 s_58_33 s_58_34
        let s_58_37: Bits = (Bits::new(
            ((s_58_36) >> (s_58_33)).value(),
            u16::try_from(s_58_34).unwrap(),
        ));
        // D s_58_38: cast reint s_58_37 -> u8
        let s_58_38: u8 = (s_58_37.value() as u8);
        // C s_58_39: const #31s : i
        let s_58_39: i128 = 31;
        // C s_58_40: const #1s : i
        let s_58_40: i128 = 1;
        // D s_58_41: read-var u#22967:u32
        let s_58_41: u32 = fn_state.u_22967;
        // D s_58_42: cast zx s_58_41 -> bv
        let s_58_42: Bits = Bits::new(s_58_41 as u128, 32u16);
        // D s_58_43: bit-extract s_58_42 s_58_39 s_58_40
        let s_58_43: Bits = (Bits::new(
            ((s_58_42) >> (s_58_39)).value(),
            u16::try_from(s_58_40).unwrap(),
        ));
        // D s_58_44: cast reint s_58_43 -> u8
        let s_58_44: bool = ((s_58_43.value()) != 0);
        // D s_58_45: call decode_bfm_aarch64_instrs_integer_bitfield(s_58_8, s_58_14, s_58_20, s_58_26, s_58_32, s_58_38, s_58_44)
        let s_58_45: () = decode_bfm_aarch64_instrs_integer_bitfield(
            state,
            tracer,
            s_58_8,
            s_58_14,
            s_58_20,
            s_58_26,
            s_58_32,
            s_58_38,
            s_58_44,
        );
        // N s_58_46: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var merge#var.1:struct
        let s_59_0: u32 = fn_state.merge_var._1;
        // D s_59_1: write-var u#22976 <= s_59_0
        fn_state.u_22976 = s_59_0;
        // C s_59_2: const #23s : i
        let s_59_2: i128 = 23;
        // D s_59_3: read-var u#22976:u32
        let s_59_3: u32 = fn_state.u_22976;
        // D s_59_4: cast zx s_59_3 -> bv
        let s_59_4: Bits = Bits::new(s_59_3 as u128, 32u16);
        // C s_59_5: const #1s : i64
        let s_59_5: i64 = 1;
        // C s_59_6: cast zx s_59_5 -> i
        let s_59_6: i128 = (i128::try_from(s_59_5).unwrap());
        // C s_59_7: const #7s : i
        let s_59_7: i128 = 7;
        // C s_59_8: add s_59_7 s_59_6
        let s_59_8: i128 = (s_59_7 + s_59_6);
        // D s_59_9: bit-extract s_59_4 s_59_2 s_59_8
        let s_59_9: Bits = (Bits::new(
            ((s_59_4) >> (s_59_2)).value(),
            u16::try_from(s_59_8).unwrap(),
        ));
        // D s_59_10: cast reint s_59_9 -> u8
        let s_59_10: u8 = (s_59_9.value() as u8);
        // D s_59_11: cast zx s_59_10 -> bv
        let s_59_11: Bits = Bits::new(s_59_10 as u128, 8u16);
        // C s_59_12: const #38u : u8
        let s_59_12: u8 = 38;
        // C s_59_13: cast zx s_59_12 -> bv
        let s_59_13: Bits = Bits::new(s_59_12 as u128, 8u16);
        // D s_59_14: cmp-eq s_59_11 s_59_13
        let s_59_14: bool = ((s_59_11) == (s_59_13));
        // N s_59_15: branch s_59_14 b125 b60
        if s_59_14 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#373418 <= s_60_0
        fn_state.gs_373418 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#373418:u8
        let s_61_0: bool = fn_state.gs_373418;
        // D s_61_1: not s_61_0
        let s_61_1: bool = !s_61_0;
        // N s_61_2: branch s_61_1 b63 b62
        if s_61_1 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #77s : i
        let s_62_0: i128 = 77;
        // C s_62_1: const #14696u : u32
        let s_62_1: u32 = 14696;
        // N s_62_2: write-reg s_62_1 <= s_62_0
        let s_62_2: () = {
            state.write_register::<i128>(s_62_1 as isize, s_62_0);
            tracer.write_register(s_62_1 as isize, s_62_0);
        };
        // C s_62_3: const #0s : i
        let s_62_3: i128 = 0;
        // C s_62_4: const #5s : i
        let s_62_4: i128 = 5;
        // D s_62_5: read-var u#22976:u32
        let s_62_5: u32 = fn_state.u_22976;
        // D s_62_6: cast zx s_62_5 -> bv
        let s_62_6: Bits = Bits::new(s_62_5 as u128, 32u16);
        // D s_62_7: bit-extract s_62_6 s_62_3 s_62_4
        let s_62_7: Bits = (Bits::new(
            ((s_62_6) >> (s_62_3)).value(),
            u16::try_from(s_62_4).unwrap(),
        ));
        // D s_62_8: cast reint s_62_7 -> u8
        let s_62_8: u8 = (s_62_7.value() as u8);
        // C s_62_9: const #5s : i
        let s_62_9: i128 = 5;
        // C s_62_10: const #5s : i
        let s_62_10: i128 = 5;
        // D s_62_11: read-var u#22976:u32
        let s_62_11: u32 = fn_state.u_22976;
        // D s_62_12: cast zx s_62_11 -> bv
        let s_62_12: Bits = Bits::new(s_62_11 as u128, 32u16);
        // D s_62_13: bit-extract s_62_12 s_62_9 s_62_10
        let s_62_13: Bits = (Bits::new(
            ((s_62_12) >> (s_62_9)).value(),
            u16::try_from(s_62_10).unwrap(),
        ));
        // D s_62_14: cast reint s_62_13 -> u8
        let s_62_14: u8 = (s_62_13.value() as u8);
        // C s_62_15: const #10s : i
        let s_62_15: i128 = 10;
        // C s_62_16: const #6s : i
        let s_62_16: i128 = 6;
        // D s_62_17: read-var u#22976:u32
        let s_62_17: u32 = fn_state.u_22976;
        // D s_62_18: cast zx s_62_17 -> bv
        let s_62_18: Bits = Bits::new(s_62_17 as u128, 32u16);
        // D s_62_19: bit-extract s_62_18 s_62_15 s_62_16
        let s_62_19: Bits = (Bits::new(
            ((s_62_18) >> (s_62_15)).value(),
            u16::try_from(s_62_16).unwrap(),
        ));
        // D s_62_20: cast reint s_62_19 -> u8
        let s_62_20: u8 = (s_62_19.value() as u8);
        // C s_62_21: const #16s : i
        let s_62_21: i128 = 16;
        // C s_62_22: const #6s : i
        let s_62_22: i128 = 6;
        // D s_62_23: read-var u#22976:u32
        let s_62_23: u32 = fn_state.u_22976;
        // D s_62_24: cast zx s_62_23 -> bv
        let s_62_24: Bits = Bits::new(s_62_23 as u128, 32u16);
        // D s_62_25: bit-extract s_62_24 s_62_21 s_62_22
        let s_62_25: Bits = (Bits::new(
            ((s_62_24) >> (s_62_21)).value(),
            u16::try_from(s_62_22).unwrap(),
        ));
        // D s_62_26: cast reint s_62_25 -> u8
        let s_62_26: u8 = (s_62_25.value() as u8);
        // C s_62_27: const #22s : i
        let s_62_27: i128 = 22;
        // C s_62_28: const #1s : i
        let s_62_28: i128 = 1;
        // D s_62_29: read-var u#22976:u32
        let s_62_29: u32 = fn_state.u_22976;
        // D s_62_30: cast zx s_62_29 -> bv
        let s_62_30: Bits = Bits::new(s_62_29 as u128, 32u16);
        // D s_62_31: bit-extract s_62_30 s_62_27 s_62_28
        let s_62_31: Bits = (Bits::new(
            ((s_62_30) >> (s_62_27)).value(),
            u16::try_from(s_62_28).unwrap(),
        ));
        // D s_62_32: cast reint s_62_31 -> u8
        let s_62_32: bool = ((s_62_31.value()) != 0);
        // C s_62_33: const #29s : i
        let s_62_33: i128 = 29;
        // C s_62_34: const #2s : i
        let s_62_34: i128 = 2;
        // D s_62_35: read-var u#22976:u32
        let s_62_35: u32 = fn_state.u_22976;
        // D s_62_36: cast zx s_62_35 -> bv
        let s_62_36: Bits = Bits::new(s_62_35 as u128, 32u16);
        // D s_62_37: bit-extract s_62_36 s_62_33 s_62_34
        let s_62_37: Bits = (Bits::new(
            ((s_62_36) >> (s_62_33)).value(),
            u16::try_from(s_62_34).unwrap(),
        ));
        // D s_62_38: cast reint s_62_37 -> u8
        let s_62_38: u8 = (s_62_37.value() as u8);
        // C s_62_39: const #31s : i
        let s_62_39: i128 = 31;
        // C s_62_40: const #1s : i
        let s_62_40: i128 = 1;
        // D s_62_41: read-var u#22976:u32
        let s_62_41: u32 = fn_state.u_22976;
        // D s_62_42: cast zx s_62_41 -> bv
        let s_62_42: Bits = Bits::new(s_62_41 as u128, 32u16);
        // D s_62_43: bit-extract s_62_42 s_62_39 s_62_40
        let s_62_43: Bits = (Bits::new(
            ((s_62_42) >> (s_62_39)).value(),
            u16::try_from(s_62_40).unwrap(),
        ));
        // D s_62_44: cast reint s_62_43 -> u8
        let s_62_44: bool = ((s_62_43.value()) != 0);
        // D s_62_45: call decode_sbfm_aarch64_instrs_integer_bitfield(s_62_8, s_62_14, s_62_20, s_62_26, s_62_32, s_62_38, s_62_44)
        let s_62_45: () = decode_sbfm_aarch64_instrs_integer_bitfield(
            state,
            tracer,
            s_62_8,
            s_62_14,
            s_62_20,
            s_62_26,
            s_62_32,
            s_62_38,
            s_62_44,
        );
        // N s_62_46: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var merge#var.1:struct
        let s_63_0: u32 = fn_state.merge_var._1;
        // D s_63_1: write-var u#22985 <= s_63_0
        fn_state.u_22985 = s_63_0;
        // C s_63_2: const #23s : i
        let s_63_2: i128 = 23;
        // D s_63_3: read-var u#22985:u32
        let s_63_3: u32 = fn_state.u_22985;
        // D s_63_4: cast zx s_63_3 -> bv
        let s_63_4: Bits = Bits::new(s_63_3 as u128, 32u16);
        // C s_63_5: const #1s : i64
        let s_63_5: i64 = 1;
        // C s_63_6: cast zx s_63_5 -> i
        let s_63_6: i128 = (i128::try_from(s_63_5).unwrap());
        // C s_63_7: const #7s : i
        let s_63_7: i128 = 7;
        // C s_63_8: add s_63_7 s_63_6
        let s_63_8: i128 = (s_63_7 + s_63_6);
        // D s_63_9: bit-extract s_63_4 s_63_2 s_63_8
        let s_63_9: Bits = (Bits::new(
            ((s_63_4) >> (s_63_2)).value(),
            u16::try_from(s_63_8).unwrap(),
        ));
        // D s_63_10: cast reint s_63_9 -> u8
        let s_63_10: u8 = (s_63_9.value() as u8);
        // D s_63_11: cast zx s_63_10 -> bv
        let s_63_11: Bits = Bits::new(s_63_10 as u128, 8u16);
        // C s_63_12: const #166u : u8
        let s_63_12: u8 = 166;
        // C s_63_13: cast zx s_63_12 -> bv
        let s_63_13: Bits = Bits::new(s_63_12 as u128, 8u16);
        // D s_63_14: cmp-eq s_63_11 s_63_13
        let s_63_14: bool = ((s_63_11) == (s_63_13));
        // N s_63_15: branch s_63_14 b124 b64
        if s_63_14 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #0u : u8
        let s_64_0: bool = false;
        // D s_64_1: write-var gs#373439 <= s_64_0
        fn_state.gs_373439 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#373439:u8
        let s_65_0: bool = fn_state.gs_373439;
        // D s_65_1: not s_65_0
        let s_65_1: bool = !s_65_0;
        // N s_65_2: branch s_65_1 b67 b66
        if s_65_1 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #78s : i
        let s_66_0: i128 = 78;
        // C s_66_1: const #14696u : u32
        let s_66_1: u32 = 14696;
        // N s_66_2: write-reg s_66_1 <= s_66_0
        let s_66_2: () = {
            state.write_register::<i128>(s_66_1 as isize, s_66_0);
            tracer.write_register(s_66_1 as isize, s_66_0);
        };
        // C s_66_3: const #0s : i
        let s_66_3: i128 = 0;
        // C s_66_4: const #5s : i
        let s_66_4: i128 = 5;
        // D s_66_5: read-var u#22985:u32
        let s_66_5: u32 = fn_state.u_22985;
        // D s_66_6: cast zx s_66_5 -> bv
        let s_66_6: Bits = Bits::new(s_66_5 as u128, 32u16);
        // D s_66_7: bit-extract s_66_6 s_66_3 s_66_4
        let s_66_7: Bits = (Bits::new(
            ((s_66_6) >> (s_66_3)).value(),
            u16::try_from(s_66_4).unwrap(),
        ));
        // D s_66_8: cast reint s_66_7 -> u8
        let s_66_8: u8 = (s_66_7.value() as u8);
        // C s_66_9: const #5s : i
        let s_66_9: i128 = 5;
        // C s_66_10: const #5s : i
        let s_66_10: i128 = 5;
        // D s_66_11: read-var u#22985:u32
        let s_66_11: u32 = fn_state.u_22985;
        // D s_66_12: cast zx s_66_11 -> bv
        let s_66_12: Bits = Bits::new(s_66_11 as u128, 32u16);
        // D s_66_13: bit-extract s_66_12 s_66_9 s_66_10
        let s_66_13: Bits = (Bits::new(
            ((s_66_12) >> (s_66_9)).value(),
            u16::try_from(s_66_10).unwrap(),
        ));
        // D s_66_14: cast reint s_66_13 -> u8
        let s_66_14: u8 = (s_66_13.value() as u8);
        // C s_66_15: const #10s : i
        let s_66_15: i128 = 10;
        // C s_66_16: const #6s : i
        let s_66_16: i128 = 6;
        // D s_66_17: read-var u#22985:u32
        let s_66_17: u32 = fn_state.u_22985;
        // D s_66_18: cast zx s_66_17 -> bv
        let s_66_18: Bits = Bits::new(s_66_17 as u128, 32u16);
        // D s_66_19: bit-extract s_66_18 s_66_15 s_66_16
        let s_66_19: Bits = (Bits::new(
            ((s_66_18) >> (s_66_15)).value(),
            u16::try_from(s_66_16).unwrap(),
        ));
        // D s_66_20: cast reint s_66_19 -> u8
        let s_66_20: u8 = (s_66_19.value() as u8);
        // C s_66_21: const #16s : i
        let s_66_21: i128 = 16;
        // C s_66_22: const #6s : i
        let s_66_22: i128 = 6;
        // D s_66_23: read-var u#22985:u32
        let s_66_23: u32 = fn_state.u_22985;
        // D s_66_24: cast zx s_66_23 -> bv
        let s_66_24: Bits = Bits::new(s_66_23 as u128, 32u16);
        // D s_66_25: bit-extract s_66_24 s_66_21 s_66_22
        let s_66_25: Bits = (Bits::new(
            ((s_66_24) >> (s_66_21)).value(),
            u16::try_from(s_66_22).unwrap(),
        ));
        // D s_66_26: cast reint s_66_25 -> u8
        let s_66_26: u8 = (s_66_25.value() as u8);
        // C s_66_27: const #22s : i
        let s_66_27: i128 = 22;
        // C s_66_28: const #1s : i
        let s_66_28: i128 = 1;
        // D s_66_29: read-var u#22985:u32
        let s_66_29: u32 = fn_state.u_22985;
        // D s_66_30: cast zx s_66_29 -> bv
        let s_66_30: Bits = Bits::new(s_66_29 as u128, 32u16);
        // D s_66_31: bit-extract s_66_30 s_66_27 s_66_28
        let s_66_31: Bits = (Bits::new(
            ((s_66_30) >> (s_66_27)).value(),
            u16::try_from(s_66_28).unwrap(),
        ));
        // D s_66_32: cast reint s_66_31 -> u8
        let s_66_32: bool = ((s_66_31.value()) != 0);
        // C s_66_33: const #29s : i
        let s_66_33: i128 = 29;
        // C s_66_34: const #2s : i
        let s_66_34: i128 = 2;
        // D s_66_35: read-var u#22985:u32
        let s_66_35: u32 = fn_state.u_22985;
        // D s_66_36: cast zx s_66_35 -> bv
        let s_66_36: Bits = Bits::new(s_66_35 as u128, 32u16);
        // D s_66_37: bit-extract s_66_36 s_66_33 s_66_34
        let s_66_37: Bits = (Bits::new(
            ((s_66_36) >> (s_66_33)).value(),
            u16::try_from(s_66_34).unwrap(),
        ));
        // D s_66_38: cast reint s_66_37 -> u8
        let s_66_38: u8 = (s_66_37.value() as u8);
        // C s_66_39: const #31s : i
        let s_66_39: i128 = 31;
        // C s_66_40: const #1s : i
        let s_66_40: i128 = 1;
        // D s_66_41: read-var u#22985:u32
        let s_66_41: u32 = fn_state.u_22985;
        // D s_66_42: cast zx s_66_41 -> bv
        let s_66_42: Bits = Bits::new(s_66_41 as u128, 32u16);
        // D s_66_43: bit-extract s_66_42 s_66_39 s_66_40
        let s_66_43: Bits = (Bits::new(
            ((s_66_42) >> (s_66_39)).value(),
            u16::try_from(s_66_40).unwrap(),
        ));
        // D s_66_44: cast reint s_66_43 -> u8
        let s_66_44: bool = ((s_66_43.value()) != 0);
        // D s_66_45: call decode_ubfm_aarch64_instrs_integer_bitfield(s_66_8, s_66_14, s_66_20, s_66_26, s_66_32, s_66_38, s_66_44)
        let s_66_45: () = decode_ubfm_aarch64_instrs_integer_bitfield(
            state,
            tracer,
            s_66_8,
            s_66_14,
            s_66_20,
            s_66_26,
            s_66_32,
            s_66_38,
            s_66_44,
        );
        // N s_66_46: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var merge#var.1:struct
        let s_67_0: u32 = fn_state.merge_var._1;
        // D s_67_1: write-var u#22994 <= s_67_0
        fn_state.u_22994 = s_67_0;
        // C s_67_2: const #23s : i
        let s_67_2: i128 = 23;
        // D s_67_3: read-var u#22994:u32
        let s_67_3: u32 = fn_state.u_22994;
        // D s_67_4: cast zx s_67_3 -> bv
        let s_67_4: Bits = Bits::new(s_67_3 as u128, 32u16);
        // C s_67_5: const #1s : i64
        let s_67_5: i64 = 1;
        // C s_67_6: cast zx s_67_5 -> i
        let s_67_6: i128 = (i128::try_from(s_67_5).unwrap());
        // C s_67_7: const #7s : i
        let s_67_7: i128 = 7;
        // C s_67_8: add s_67_7 s_67_6
        let s_67_8: i128 = (s_67_7 + s_67_6);
        // D s_67_9: bit-extract s_67_4 s_67_2 s_67_8
        let s_67_9: Bits = (Bits::new(
            ((s_67_4) >> (s_67_2)).value(),
            u16::try_from(s_67_8).unwrap(),
        ));
        // D s_67_10: cast reint s_67_9 -> u8
        let s_67_10: u8 = (s_67_9.value() as u8);
        // D s_67_11: cast zx s_67_10 -> bv
        let s_67_11: Bits = Bits::new(s_67_10 as u128, 8u16);
        // C s_67_12: const #39u : u8
        let s_67_12: u8 = 39;
        // C s_67_13: cast zx s_67_12 -> bv
        let s_67_13: Bits = Bits::new(s_67_12 as u128, 8u16);
        // D s_67_14: cmp-eq s_67_11 s_67_13
        let s_67_14: bool = ((s_67_11) == (s_67_13));
        // N s_67_15: branch s_67_14 b123 b68
        if s_67_14 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // D s_68_1: write-var gs#373461 <= s_68_0
        fn_state.gs_373461 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#373461:u8
        let s_69_0: bool = fn_state.gs_373461;
        // N s_69_1: branch s_69_0 b122 b70
        if s_69_0 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #0u : u8
        let s_70_0: bool = false;
        // D s_70_1: write-var gs#373463 <= s_70_0
        fn_state.gs_373463 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#373463:u8
        let s_71_0: bool = fn_state.gs_373463;
        // D s_71_1: not s_71_0
        let s_71_1: bool = !s_71_0;
        // N s_71_2: branch s_71_1 b73 b72
        if s_71_1 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #206s : i
        let s_72_0: i128 = 206;
        // C s_72_1: const #14696u : u32
        let s_72_1: u32 = 14696;
        // N s_72_2: write-reg s_72_1 <= s_72_0
        let s_72_2: () = {
            state.write_register::<i128>(s_72_1 as isize, s_72_0);
            tracer.write_register(s_72_1 as isize, s_72_0);
        };
        // C s_72_3: const #0s : i
        let s_72_3: i128 = 0;
        // C s_72_4: const #5s : i
        let s_72_4: i128 = 5;
        // D s_72_5: read-var u#22994:u32
        let s_72_5: u32 = fn_state.u_22994;
        // D s_72_6: cast zx s_72_5 -> bv
        let s_72_6: Bits = Bits::new(s_72_5 as u128, 32u16);
        // D s_72_7: bit-extract s_72_6 s_72_3 s_72_4
        let s_72_7: Bits = (Bits::new(
            ((s_72_6) >> (s_72_3)).value(),
            u16::try_from(s_72_4).unwrap(),
        ));
        // D s_72_8: cast reint s_72_7 -> u8
        let s_72_8: u8 = (s_72_7.value() as u8);
        // C s_72_9: const #5s : i
        let s_72_9: i128 = 5;
        // C s_72_10: const #5s : i
        let s_72_10: i128 = 5;
        // D s_72_11: read-var u#22994:u32
        let s_72_11: u32 = fn_state.u_22994;
        // D s_72_12: cast zx s_72_11 -> bv
        let s_72_12: Bits = Bits::new(s_72_11 as u128, 32u16);
        // D s_72_13: bit-extract s_72_12 s_72_9 s_72_10
        let s_72_13: Bits = (Bits::new(
            ((s_72_12) >> (s_72_9)).value(),
            u16::try_from(s_72_10).unwrap(),
        ));
        // D s_72_14: cast reint s_72_13 -> u8
        let s_72_14: u8 = (s_72_13.value() as u8);
        // C s_72_15: const #10s : i
        let s_72_15: i128 = 10;
        // C s_72_16: const #6s : i
        let s_72_16: i128 = 6;
        // D s_72_17: read-var u#22994:u32
        let s_72_17: u32 = fn_state.u_22994;
        // D s_72_18: cast zx s_72_17 -> bv
        let s_72_18: Bits = Bits::new(s_72_17 as u128, 32u16);
        // D s_72_19: bit-extract s_72_18 s_72_15 s_72_16
        let s_72_19: Bits = (Bits::new(
            ((s_72_18) >> (s_72_15)).value(),
            u16::try_from(s_72_16).unwrap(),
        ));
        // D s_72_20: cast reint s_72_19 -> u8
        let s_72_20: u8 = (s_72_19.value() as u8);
        // C s_72_21: const #16s : i
        let s_72_21: i128 = 16;
        // C s_72_22: const #5s : i
        let s_72_22: i128 = 5;
        // D s_72_23: read-var u#22994:u32
        let s_72_23: u32 = fn_state.u_22994;
        // D s_72_24: cast zx s_72_23 -> bv
        let s_72_24: Bits = Bits::new(s_72_23 as u128, 32u16);
        // D s_72_25: bit-extract s_72_24 s_72_21 s_72_22
        let s_72_25: Bits = (Bits::new(
            ((s_72_24) >> (s_72_21)).value(),
            u16::try_from(s_72_22).unwrap(),
        ));
        // D s_72_26: cast reint s_72_25 -> u8
        let s_72_26: u8 = (s_72_25.value() as u8);
        // C s_72_27: const #22s : i
        let s_72_27: i128 = 22;
        // C s_72_28: const #1s : i
        let s_72_28: i128 = 1;
        // D s_72_29: read-var u#22994:u32
        let s_72_29: u32 = fn_state.u_22994;
        // D s_72_30: cast zx s_72_29 -> bv
        let s_72_30: Bits = Bits::new(s_72_29 as u128, 32u16);
        // D s_72_31: bit-extract s_72_30 s_72_27 s_72_28
        let s_72_31: Bits = (Bits::new(
            ((s_72_30) >> (s_72_27)).value(),
            u16::try_from(s_72_28).unwrap(),
        ));
        // D s_72_32: cast reint s_72_31 -> u8
        let s_72_32: bool = ((s_72_31.value()) != 0);
        // C s_72_33: const #31s : i
        let s_72_33: i128 = 31;
        // C s_72_34: const #1s : i
        let s_72_34: i128 = 1;
        // D s_72_35: read-var u#22994:u32
        let s_72_35: u32 = fn_state.u_22994;
        // D s_72_36: cast zx s_72_35 -> bv
        let s_72_36: Bits = Bits::new(s_72_35 as u128, 32u16);
        // D s_72_37: bit-extract s_72_36 s_72_33 s_72_34
        let s_72_37: Bits = (Bits::new(
            ((s_72_36) >> (s_72_33)).value(),
            u16::try_from(s_72_34).unwrap(),
        ));
        // D s_72_38: cast reint s_72_37 -> u8
        let s_72_38: bool = ((s_72_37.value()) != 0);
        // D s_72_39: call decode_extr_aarch64_instrs_integer_ins_ext_extract_immediate(s_72_8, s_72_14, s_72_20, s_72_26, s_72_32, s_72_38)
        let s_72_39: () = decode_extr_aarch64_instrs_integer_ins_ext_extract_immediate(
            state,
            tracer,
            s_72_8,
            s_72_14,
            s_72_20,
            s_72_26,
            s_72_32,
            s_72_38,
        );
        // N s_72_40: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var merge#var.1:struct
        let s_73_0: u32 = fn_state.merge_var._1;
        // D s_73_1: write-var u#23001 <= s_73_0
        fn_state.u_23001 = s_73_0;
        // C s_73_2: const #23s : i
        let s_73_2: i128 = 23;
        // D s_73_3: read-var u#23001:u32
        let s_73_3: u32 = fn_state.u_23001;
        // D s_73_4: cast zx s_73_3 -> bv
        let s_73_4: Bits = Bits::new(s_73_3 as u128, 32u16);
        // C s_73_5: const #1s : i64
        let s_73_5: i64 = 1;
        // C s_73_6: cast zx s_73_5 -> i
        let s_73_6: i128 = (i128::try_from(s_73_5).unwrap());
        // C s_73_7: const #7s : i
        let s_73_7: i128 = 7;
        // C s_73_8: add s_73_7 s_73_6
        let s_73_8: i128 = (s_73_7 + s_73_6);
        // D s_73_9: bit-extract s_73_4 s_73_2 s_73_8
        let s_73_9: Bits = (Bits::new(
            ((s_73_4) >> (s_73_2)).value(),
            u16::try_from(s_73_8).unwrap(),
        ));
        // D s_73_10: cast reint s_73_9 -> u8
        let s_73_10: u8 = (s_73_9.value() as u8);
        // D s_73_11: cast zx s_73_10 -> bv
        let s_73_11: Bits = Bits::new(s_73_10 as u128, 8u16);
        // C s_73_12: const #229u : u8
        let s_73_12: u8 = 229;
        // C s_73_13: cast zx s_73_12 -> bv
        let s_73_13: Bits = Bits::new(s_73_12 as u128, 8u16);
        // D s_73_14: cmp-eq s_73_11 s_73_13
        let s_73_14: bool = ((s_73_11) == (s_73_13));
        // N s_73_15: branch s_73_14 b121 b74
        if s_73_14 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var gs#373482 <= s_74_0
        fn_state.gs_373482 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#373482:u8
        let s_75_0: bool = fn_state.gs_373482;
        // D s_75_1: not s_75_0
        let s_75_1: bool = !s_75_0;
        // N s_75_2: branch s_75_1 b77 b76
        if s_75_1 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #696s : i
        let s_76_0: i128 = 696;
        // C s_76_1: const #14696u : u32
        let s_76_1: u32 = 14696;
        // N s_76_2: write-reg s_76_1 <= s_76_0
        let s_76_2: () = {
            state.write_register::<i128>(s_76_1 as isize, s_76_0);
            tracer.write_register(s_76_1 as isize, s_76_0);
        };
        // C s_76_3: const #0s : i
        let s_76_3: i128 = 0;
        // C s_76_4: const #5s : i
        let s_76_4: i128 = 5;
        // D s_76_5: read-var u#23001:u32
        let s_76_5: u32 = fn_state.u_23001;
        // D s_76_6: cast zx s_76_5 -> bv
        let s_76_6: Bits = Bits::new(s_76_5 as u128, 32u16);
        // D s_76_7: bit-extract s_76_6 s_76_3 s_76_4
        let s_76_7: Bits = (Bits::new(
            ((s_76_6) >> (s_76_3)).value(),
            u16::try_from(s_76_4).unwrap(),
        ));
        // D s_76_8: cast reint s_76_7 -> u8
        let s_76_8: u8 = (s_76_7.value() as u8);
        // C s_76_9: const #5s : i
        let s_76_9: i128 = 5;
        // C s_76_10: const #16s : i
        let s_76_10: i128 = 16;
        // D s_76_11: read-var u#23001:u32
        let s_76_11: u32 = fn_state.u_23001;
        // D s_76_12: cast zx s_76_11 -> bv
        let s_76_12: Bits = Bits::new(s_76_11 as u128, 32u16);
        // D s_76_13: bit-extract s_76_12 s_76_9 s_76_10
        let s_76_13: Bits = (Bits::new(
            ((s_76_12) >> (s_76_9)).value(),
            u16::try_from(s_76_10).unwrap(),
        ));
        // D s_76_14: cast reint s_76_13 -> u16
        let s_76_14: u16 = (s_76_13.value() as u16);
        // C s_76_15: const #21s : i
        let s_76_15: i128 = 21;
        // C s_76_16: const #2s : i
        let s_76_16: i128 = 2;
        // D s_76_17: read-var u#23001:u32
        let s_76_17: u32 = fn_state.u_23001;
        // D s_76_18: cast zx s_76_17 -> bv
        let s_76_18: Bits = Bits::new(s_76_17 as u128, 32u16);
        // D s_76_19: bit-extract s_76_18 s_76_15 s_76_16
        let s_76_19: Bits = (Bits::new(
            ((s_76_18) >> (s_76_15)).value(),
            u16::try_from(s_76_16).unwrap(),
        ));
        // D s_76_20: cast reint s_76_19 -> u8
        let s_76_20: u8 = (s_76_19.value() as u8);
        // C s_76_21: const #29s : i
        let s_76_21: i128 = 29;
        // C s_76_22: const #2s : i
        let s_76_22: i128 = 2;
        // D s_76_23: read-var u#23001:u32
        let s_76_23: u32 = fn_state.u_23001;
        // D s_76_24: cast zx s_76_23 -> bv
        let s_76_24: Bits = Bits::new(s_76_23 as u128, 32u16);
        // D s_76_25: bit-extract s_76_24 s_76_21 s_76_22
        let s_76_25: Bits = (Bits::new(
            ((s_76_24) >> (s_76_21)).value(),
            u16::try_from(s_76_22).unwrap(),
        ));
        // D s_76_26: cast reint s_76_25 -> u8
        let s_76_26: u8 = (s_76_25.value() as u8);
        // C s_76_27: const #31s : i
        let s_76_27: i128 = 31;
        // C s_76_28: const #1s : i
        let s_76_28: i128 = 1;
        // D s_76_29: read-var u#23001:u32
        let s_76_29: u32 = fn_state.u_23001;
        // D s_76_30: cast zx s_76_29 -> bv
        let s_76_30: Bits = Bits::new(s_76_29 as u128, 32u16);
        // D s_76_31: bit-extract s_76_30 s_76_27 s_76_28
        let s_76_31: Bits = (Bits::new(
            ((s_76_30) >> (s_76_27)).value(),
            u16::try_from(s_76_28).unwrap(),
        ));
        // D s_76_32: cast reint s_76_31 -> u8
        let s_76_32: bool = ((s_76_31.value()) != 0);
        // D s_76_33: call decode_movk_aarch64_instrs_integer_ins_ext_insert_movewide(s_76_8, s_76_14, s_76_20, s_76_26, s_76_32)
        let s_76_33: () = decode_movk_aarch64_instrs_integer_ins_ext_insert_movewide(
            state,
            tracer,
            s_76_8,
            s_76_14,
            s_76_20,
            s_76_26,
            s_76_32,
        );
        // N s_76_34: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var merge#var.1:struct
        let s_77_0: u32 = fn_state.merge_var._1;
        // D s_77_1: write-var u#23006 <= s_77_0
        fn_state.u_23006 = s_77_0;
        // C s_77_2: const #23s : i
        let s_77_2: i128 = 23;
        // D s_77_3: read-var u#23006:u32
        let s_77_3: u32 = fn_state.u_23006;
        // D s_77_4: cast zx s_77_3 -> bv
        let s_77_4: Bits = Bits::new(s_77_3 as u128, 32u16);
        // C s_77_5: const #1s : i64
        let s_77_5: i64 = 1;
        // C s_77_6: cast zx s_77_5 -> i
        let s_77_6: i128 = (i128::try_from(s_77_5).unwrap());
        // C s_77_7: const #7s : i
        let s_77_7: i128 = 7;
        // C s_77_8: add s_77_7 s_77_6
        let s_77_8: i128 = (s_77_7 + s_77_6);
        // D s_77_9: bit-extract s_77_4 s_77_2 s_77_8
        let s_77_9: Bits = (Bits::new(
            ((s_77_4) >> (s_77_2)).value(),
            u16::try_from(s_77_8).unwrap(),
        ));
        // D s_77_10: cast reint s_77_9 -> u8
        let s_77_10: u8 = (s_77_9.value() as u8);
        // D s_77_11: cast zx s_77_10 -> bv
        let s_77_11: Bits = Bits::new(s_77_10 as u128, 8u16);
        // C s_77_12: const #37u : u8
        let s_77_12: u8 = 37;
        // C s_77_13: cast zx s_77_12 -> bv
        let s_77_13: Bits = Bits::new(s_77_12 as u128, 8u16);
        // D s_77_14: cmp-eq s_77_11 s_77_13
        let s_77_14: bool = ((s_77_11) == (s_77_13));
        // N s_77_15: branch s_77_14 b120 b78
        if s_77_14 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #0u : u8
        let s_78_0: bool = false;
        // D s_78_1: write-var gs#373499 <= s_78_0
        fn_state.gs_373499 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#373499:u8
        let s_79_0: bool = fn_state.gs_373499;
        // D s_79_1: not s_79_0
        let s_79_1: bool = !s_79_0;
        // N s_79_2: branch s_79_1 b81 b80
        if s_79_1 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #697s : i
        let s_80_0: i128 = 697;
        // C s_80_1: const #14696u : u32
        let s_80_1: u32 = 14696;
        // N s_80_2: write-reg s_80_1 <= s_80_0
        let s_80_2: () = {
            state.write_register::<i128>(s_80_1 as isize, s_80_0);
            tracer.write_register(s_80_1 as isize, s_80_0);
        };
        // C s_80_3: const #0s : i
        let s_80_3: i128 = 0;
        // C s_80_4: const #5s : i
        let s_80_4: i128 = 5;
        // D s_80_5: read-var u#23006:u32
        let s_80_5: u32 = fn_state.u_23006;
        // D s_80_6: cast zx s_80_5 -> bv
        let s_80_6: Bits = Bits::new(s_80_5 as u128, 32u16);
        // D s_80_7: bit-extract s_80_6 s_80_3 s_80_4
        let s_80_7: Bits = (Bits::new(
            ((s_80_6) >> (s_80_3)).value(),
            u16::try_from(s_80_4).unwrap(),
        ));
        // D s_80_8: cast reint s_80_7 -> u8
        let s_80_8: u8 = (s_80_7.value() as u8);
        // C s_80_9: const #5s : i
        let s_80_9: i128 = 5;
        // C s_80_10: const #16s : i
        let s_80_10: i128 = 16;
        // D s_80_11: read-var u#23006:u32
        let s_80_11: u32 = fn_state.u_23006;
        // D s_80_12: cast zx s_80_11 -> bv
        let s_80_12: Bits = Bits::new(s_80_11 as u128, 32u16);
        // D s_80_13: bit-extract s_80_12 s_80_9 s_80_10
        let s_80_13: Bits = (Bits::new(
            ((s_80_12) >> (s_80_9)).value(),
            u16::try_from(s_80_10).unwrap(),
        ));
        // D s_80_14: cast reint s_80_13 -> u16
        let s_80_14: u16 = (s_80_13.value() as u16);
        // C s_80_15: const #21s : i
        let s_80_15: i128 = 21;
        // C s_80_16: const #2s : i
        let s_80_16: i128 = 2;
        // D s_80_17: read-var u#23006:u32
        let s_80_17: u32 = fn_state.u_23006;
        // D s_80_18: cast zx s_80_17 -> bv
        let s_80_18: Bits = Bits::new(s_80_17 as u128, 32u16);
        // D s_80_19: bit-extract s_80_18 s_80_15 s_80_16
        let s_80_19: Bits = (Bits::new(
            ((s_80_18) >> (s_80_15)).value(),
            u16::try_from(s_80_16).unwrap(),
        ));
        // D s_80_20: cast reint s_80_19 -> u8
        let s_80_20: u8 = (s_80_19.value() as u8);
        // C s_80_21: const #29s : i
        let s_80_21: i128 = 29;
        // C s_80_22: const #2s : i
        let s_80_22: i128 = 2;
        // D s_80_23: read-var u#23006:u32
        let s_80_23: u32 = fn_state.u_23006;
        // D s_80_24: cast zx s_80_23 -> bv
        let s_80_24: Bits = Bits::new(s_80_23 as u128, 32u16);
        // D s_80_25: bit-extract s_80_24 s_80_21 s_80_22
        let s_80_25: Bits = (Bits::new(
            ((s_80_24) >> (s_80_21)).value(),
            u16::try_from(s_80_22).unwrap(),
        ));
        // D s_80_26: cast reint s_80_25 -> u8
        let s_80_26: u8 = (s_80_25.value() as u8);
        // C s_80_27: const #31s : i
        let s_80_27: i128 = 31;
        // C s_80_28: const #1s : i
        let s_80_28: i128 = 1;
        // D s_80_29: read-var u#23006:u32
        let s_80_29: u32 = fn_state.u_23006;
        // D s_80_30: cast zx s_80_29 -> bv
        let s_80_30: Bits = Bits::new(s_80_29 as u128, 32u16);
        // D s_80_31: bit-extract s_80_30 s_80_27 s_80_28
        let s_80_31: Bits = (Bits::new(
            ((s_80_30) >> (s_80_27)).value(),
            u16::try_from(s_80_28).unwrap(),
        ));
        // D s_80_32: cast reint s_80_31 -> u8
        let s_80_32: bool = ((s_80_31.value()) != 0);
        // D s_80_33: call decode_movn_aarch64_instrs_integer_ins_ext_insert_movewide(s_80_8, s_80_14, s_80_20, s_80_26, s_80_32)
        let s_80_33: () = decode_movn_aarch64_instrs_integer_ins_ext_insert_movewide(
            state,
            tracer,
            s_80_8,
            s_80_14,
            s_80_20,
            s_80_26,
            s_80_32,
        );
        // N s_80_34: return
        return;
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var merge#var.1:struct
        let s_81_0: u32 = fn_state.merge_var._1;
        // D s_81_1: write-var u#23013 <= s_81_0
        fn_state.u_23013 = s_81_0;
        // C s_81_2: const #23s : i
        let s_81_2: i128 = 23;
        // D s_81_3: read-var u#23013:u32
        let s_81_3: u32 = fn_state.u_23013;
        // D s_81_4: cast zx s_81_3 -> bv
        let s_81_4: Bits = Bits::new(s_81_3 as u128, 32u16);
        // C s_81_5: const #1s : i64
        let s_81_5: i64 = 1;
        // C s_81_6: cast zx s_81_5 -> i
        let s_81_6: i128 = (i128::try_from(s_81_5).unwrap());
        // C s_81_7: const #7s : i
        let s_81_7: i128 = 7;
        // C s_81_8: add s_81_7 s_81_6
        let s_81_8: i128 = (s_81_7 + s_81_6);
        // D s_81_9: bit-extract s_81_4 s_81_2 s_81_8
        let s_81_9: Bits = (Bits::new(
            ((s_81_4) >> (s_81_2)).value(),
            u16::try_from(s_81_8).unwrap(),
        ));
        // D s_81_10: cast reint s_81_9 -> u8
        let s_81_10: u8 = (s_81_9.value() as u8);
        // D s_81_11: cast zx s_81_10 -> bv
        let s_81_11: Bits = Bits::new(s_81_10 as u128, 8u16);
        // C s_81_12: const #165u : u8
        let s_81_12: u8 = 165;
        // C s_81_13: cast zx s_81_12 -> bv
        let s_81_13: Bits = Bits::new(s_81_12 as u128, 8u16);
        // D s_81_14: cmp-eq s_81_11 s_81_13
        let s_81_14: bool = ((s_81_11) == (s_81_13));
        // N s_81_15: branch s_81_14 b119 b82
        if s_81_14 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #0u : u8
        let s_82_0: bool = false;
        // D s_82_1: write-var gs#373516 <= s_82_0
        fn_state.gs_373516 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#373516:u8
        let s_83_0: bool = fn_state.gs_373516;
        // D s_83_1: not s_83_0
        let s_83_1: bool = !s_83_0;
        // N s_83_2: branch s_83_1 b85 b84
        if s_83_1 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #698s : i
        let s_84_0: i128 = 698;
        // C s_84_1: const #14696u : u32
        let s_84_1: u32 = 14696;
        // N s_84_2: write-reg s_84_1 <= s_84_0
        let s_84_2: () = {
            state.write_register::<i128>(s_84_1 as isize, s_84_0);
            tracer.write_register(s_84_1 as isize, s_84_0);
        };
        // C s_84_3: const #0s : i
        let s_84_3: i128 = 0;
        // C s_84_4: const #5s : i
        let s_84_4: i128 = 5;
        // D s_84_5: read-var u#23013:u32
        let s_84_5: u32 = fn_state.u_23013;
        // D s_84_6: cast zx s_84_5 -> bv
        let s_84_6: Bits = Bits::new(s_84_5 as u128, 32u16);
        // D s_84_7: bit-extract s_84_6 s_84_3 s_84_4
        let s_84_7: Bits = (Bits::new(
            ((s_84_6) >> (s_84_3)).value(),
            u16::try_from(s_84_4).unwrap(),
        ));
        // D s_84_8: cast reint s_84_7 -> u8
        let s_84_8: u8 = (s_84_7.value() as u8);
        // C s_84_9: const #5s : i
        let s_84_9: i128 = 5;
        // C s_84_10: const #16s : i
        let s_84_10: i128 = 16;
        // D s_84_11: read-var u#23013:u32
        let s_84_11: u32 = fn_state.u_23013;
        // D s_84_12: cast zx s_84_11 -> bv
        let s_84_12: Bits = Bits::new(s_84_11 as u128, 32u16);
        // D s_84_13: bit-extract s_84_12 s_84_9 s_84_10
        let s_84_13: Bits = (Bits::new(
            ((s_84_12) >> (s_84_9)).value(),
            u16::try_from(s_84_10).unwrap(),
        ));
        // D s_84_14: cast reint s_84_13 -> u16
        let s_84_14: u16 = (s_84_13.value() as u16);
        // C s_84_15: const #21s : i
        let s_84_15: i128 = 21;
        // C s_84_16: const #2s : i
        let s_84_16: i128 = 2;
        // D s_84_17: read-var u#23013:u32
        let s_84_17: u32 = fn_state.u_23013;
        // D s_84_18: cast zx s_84_17 -> bv
        let s_84_18: Bits = Bits::new(s_84_17 as u128, 32u16);
        // D s_84_19: bit-extract s_84_18 s_84_15 s_84_16
        let s_84_19: Bits = (Bits::new(
            ((s_84_18) >> (s_84_15)).value(),
            u16::try_from(s_84_16).unwrap(),
        ));
        // D s_84_20: cast reint s_84_19 -> u8
        let s_84_20: u8 = (s_84_19.value() as u8);
        // C s_84_21: const #29s : i
        let s_84_21: i128 = 29;
        // C s_84_22: const #2s : i
        let s_84_22: i128 = 2;
        // D s_84_23: read-var u#23013:u32
        let s_84_23: u32 = fn_state.u_23013;
        // D s_84_24: cast zx s_84_23 -> bv
        let s_84_24: Bits = Bits::new(s_84_23 as u128, 32u16);
        // D s_84_25: bit-extract s_84_24 s_84_21 s_84_22
        let s_84_25: Bits = (Bits::new(
            ((s_84_24) >> (s_84_21)).value(),
            u16::try_from(s_84_22).unwrap(),
        ));
        // D s_84_26: cast reint s_84_25 -> u8
        let s_84_26: u8 = (s_84_25.value() as u8);
        // C s_84_27: const #31s : i
        let s_84_27: i128 = 31;
        // C s_84_28: const #1s : i
        let s_84_28: i128 = 1;
        // D s_84_29: read-var u#23013:u32
        let s_84_29: u32 = fn_state.u_23013;
        // D s_84_30: cast zx s_84_29 -> bv
        let s_84_30: Bits = Bits::new(s_84_29 as u128, 32u16);
        // D s_84_31: bit-extract s_84_30 s_84_27 s_84_28
        let s_84_31: Bits = (Bits::new(
            ((s_84_30) >> (s_84_27)).value(),
            u16::try_from(s_84_28).unwrap(),
        ));
        // D s_84_32: cast reint s_84_31 -> u8
        let s_84_32: bool = ((s_84_31.value()) != 0);
        // D s_84_33: call decode_movz_aarch64_instrs_integer_ins_ext_insert_movewide(s_84_8, s_84_14, s_84_20, s_84_26, s_84_32)
        let s_84_33: () = decode_movz_aarch64_instrs_integer_ins_ext_insert_movewide(
            state,
            tracer,
            s_84_8,
            s_84_14,
            s_84_20,
            s_84_26,
            s_84_32,
        );
        // N s_84_34: return
        return;
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var merge#var.1:struct
        let s_85_0: u32 = fn_state.merge_var._1;
        // D s_85_1: write-var u#23020 <= s_85_0
        fn_state.u_23020 = s_85_0;
        // C s_85_2: const #18s : i
        let s_85_2: i128 = 18;
        // D s_85_3: read-var u#23020:u32
        let s_85_3: u32 = fn_state.u_23020;
        // D s_85_4: cast zx s_85_3 -> bv
        let s_85_4: Bits = Bits::new(s_85_3 as u128, 32u16);
        // C s_85_5: const #1s : i64
        let s_85_5: i64 = 1;
        // C s_85_6: cast zx s_85_5 -> i
        let s_85_6: i128 = (i128::try_from(s_85_5).unwrap());
        // C s_85_7: const #12s : i
        let s_85_7: i128 = 12;
        // C s_85_8: add s_85_7 s_85_6
        let s_85_8: i128 = (s_85_7 + s_85_6);
        // D s_85_9: bit-extract s_85_4 s_85_2 s_85_8
        let s_85_9: Bits = (Bits::new(
            ((s_85_4) >> (s_85_2)).value(),
            u16::try_from(s_85_8).unwrap(),
        ));
        // D s_85_10: cast reint s_85_9 -> u13
        let s_85_10: u16 = (s_85_9.value() as u16);
        // D s_85_11: cast zx s_85_10 -> bv
        let s_85_11: Bits = Bits::new(s_85_10 as u128, 13u16);
        // C s_85_12: const #1136u : u13
        let s_85_12: u16 = 1136;
        // C s_85_13: cast zx s_85_12 -> bv
        let s_85_13: Bits = Bits::new(s_85_12 as u128, 13u16);
        // D s_85_14: cmp-eq s_85_11 s_85_13
        let s_85_14: bool = ((s_85_11) == (s_85_13));
        // N s_85_15: branch s_85_14 b118 b86
        if s_85_14 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#373533 <= s_86_0
        fn_state.gs_373533 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#373533:u8
        let s_87_0: bool = fn_state.gs_373533;
        // D s_87_1: not s_87_0
        let s_87_1: bool = !s_87_0;
        // N s_87_2: branch s_87_1 b89 b88
        if s_87_1 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #832s : i
        let s_88_0: i128 = 832;
        // C s_88_1: const #14696u : u32
        let s_88_1: u32 = 14696;
        // N s_88_2: write-reg s_88_1 <= s_88_0
        let s_88_2: () = {
            state.write_register::<i128>(s_88_1 as isize, s_88_0);
            tracer.write_register(s_88_1 as isize, s_88_0);
        };
        // C s_88_3: const #0s : i
        let s_88_3: i128 = 0;
        // C s_88_4: const #5s : i
        let s_88_4: i128 = 5;
        // D s_88_5: read-var u#23020:u32
        let s_88_5: u32 = fn_state.u_23020;
        // D s_88_6: cast zx s_88_5 -> bv
        let s_88_6: Bits = Bits::new(s_88_5 as u128, 32u16);
        // D s_88_7: bit-extract s_88_6 s_88_3 s_88_4
        let s_88_7: Bits = (Bits::new(
            ((s_88_6) >> (s_88_3)).value(),
            u16::try_from(s_88_4).unwrap(),
        ));
        // D s_88_8: cast reint s_88_7 -> u8
        let s_88_8: u8 = (s_88_7.value() as u8);
        // C s_88_9: const #5s : i
        let s_88_9: i128 = 5;
        // C s_88_10: const #5s : i
        let s_88_10: i128 = 5;
        // D s_88_11: read-var u#23020:u32
        let s_88_11: u32 = fn_state.u_23020;
        // D s_88_12: cast zx s_88_11 -> bv
        let s_88_12: Bits = Bits::new(s_88_11 as u128, 32u16);
        // D s_88_13: bit-extract s_88_12 s_88_9 s_88_10
        let s_88_13: Bits = (Bits::new(
            ((s_88_12) >> (s_88_9)).value(),
            u16::try_from(s_88_10).unwrap(),
        ));
        // D s_88_14: cast reint s_88_13 -> u8
        let s_88_14: u8 = (s_88_13.value() as u8);
        // C s_88_15: const #10s : i
        let s_88_15: i128 = 10;
        // C s_88_16: const #8s : i
        let s_88_16: i128 = 8;
        // D s_88_17: read-var u#23020:u32
        let s_88_17: u32 = fn_state.u_23020;
        // D s_88_18: cast zx s_88_17 -> bv
        let s_88_18: Bits = Bits::new(s_88_17 as u128, 32u16);
        // D s_88_19: bit-extract s_88_18 s_88_15 s_88_16
        let s_88_19: Bits = (Bits::new(
            ((s_88_18) >> (s_88_15)).value(),
            u16::try_from(s_88_16).unwrap(),
        ));
        // D s_88_20: cast reint s_88_19 -> u8
        let s_88_20: u8 = (s_88_19.value() as u8);
        // C s_88_21: const #31s : i
        let s_88_21: i128 = 31;
        // C s_88_22: const #1s : i
        let s_88_22: i128 = 1;
        // D s_88_23: read-var u#23020:u32
        let s_88_23: u32 = fn_state.u_23020;
        // D s_88_24: cast zx s_88_23 -> bv
        let s_88_24: Bits = Bits::new(s_88_23 as u128, 32u16);
        // D s_88_25: bit-extract s_88_24 s_88_21 s_88_22
        let s_88_25: Bits = (Bits::new(
            ((s_88_24) >> (s_88_21)).value(),
            u16::try_from(s_88_22).unwrap(),
        ));
        // D s_88_26: cast reint s_88_25 -> u8
        let s_88_26: bool = ((s_88_25.value()) != 0);
        // D s_88_27: call decode_smax_imm_aarch64_instrs_integer_arithmetic_max_min_smax_imm(s_88_8, s_88_14, s_88_20, s_88_26)
        let s_88_27: () = decode_smax_imm_aarch64_instrs_integer_arithmetic_max_min_smax_imm(
            state,
            tracer,
            s_88_8,
            s_88_14,
            s_88_20,
            s_88_26,
        );
        // N s_88_28: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var merge#var.1:struct
        let s_89_0: u32 = fn_state.merge_var._1;
        // D s_89_1: write-var u#23025 <= s_89_0
        fn_state.u_23025 = s_89_0;
        // C s_89_2: const #18s : i
        let s_89_2: i128 = 18;
        // D s_89_3: read-var u#23025:u32
        let s_89_3: u32 = fn_state.u_23025;
        // D s_89_4: cast zx s_89_3 -> bv
        let s_89_4: Bits = Bits::new(s_89_3 as u128, 32u16);
        // C s_89_5: const #1s : i64
        let s_89_5: i64 = 1;
        // C s_89_6: cast zx s_89_5 -> i
        let s_89_6: i128 = (i128::try_from(s_89_5).unwrap());
        // C s_89_7: const #12s : i
        let s_89_7: i128 = 12;
        // C s_89_8: add s_89_7 s_89_6
        let s_89_8: i128 = (s_89_7 + s_89_6);
        // D s_89_9: bit-extract s_89_4 s_89_2 s_89_8
        let s_89_9: Bits = (Bits::new(
            ((s_89_4) >> (s_89_2)).value(),
            u16::try_from(s_89_8).unwrap(),
        ));
        // D s_89_10: cast reint s_89_9 -> u13
        let s_89_10: u16 = (s_89_9.value() as u16);
        // D s_89_11: cast zx s_89_10 -> bv
        let s_89_11: Bits = Bits::new(s_89_10 as u128, 13u16);
        // C s_89_12: const #1138u : u13
        let s_89_12: u16 = 1138;
        // C s_89_13: cast zx s_89_12 -> bv
        let s_89_13: Bits = Bits::new(s_89_12 as u128, 13u16);
        // D s_89_14: cmp-eq s_89_11 s_89_13
        let s_89_14: bool = ((s_89_11) == (s_89_13));
        // N s_89_15: branch s_89_14 b117 b90
        if s_89_14 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#373548 <= s_90_0
        fn_state.gs_373548 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#373548:u8
        let s_91_0: bool = fn_state.gs_373548;
        // D s_91_1: not s_91_0
        let s_91_1: bool = !s_91_0;
        // N s_91_2: branch s_91_1 b93 b92
        if s_91_1 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #847s : i
        let s_92_0: i128 = 847;
        // C s_92_1: const #14696u : u32
        let s_92_1: u32 = 14696;
        // N s_92_2: write-reg s_92_1 <= s_92_0
        let s_92_2: () = {
            state.write_register::<i128>(s_92_1 as isize, s_92_0);
            tracer.write_register(s_92_1 as isize, s_92_0);
        };
        // C s_92_3: const #0s : i
        let s_92_3: i128 = 0;
        // C s_92_4: const #5s : i
        let s_92_4: i128 = 5;
        // D s_92_5: read-var u#23025:u32
        let s_92_5: u32 = fn_state.u_23025;
        // D s_92_6: cast zx s_92_5 -> bv
        let s_92_6: Bits = Bits::new(s_92_5 as u128, 32u16);
        // D s_92_7: bit-extract s_92_6 s_92_3 s_92_4
        let s_92_7: Bits = (Bits::new(
            ((s_92_6) >> (s_92_3)).value(),
            u16::try_from(s_92_4).unwrap(),
        ));
        // D s_92_8: cast reint s_92_7 -> u8
        let s_92_8: u8 = (s_92_7.value() as u8);
        // C s_92_9: const #5s : i
        let s_92_9: i128 = 5;
        // C s_92_10: const #5s : i
        let s_92_10: i128 = 5;
        // D s_92_11: read-var u#23025:u32
        let s_92_11: u32 = fn_state.u_23025;
        // D s_92_12: cast zx s_92_11 -> bv
        let s_92_12: Bits = Bits::new(s_92_11 as u128, 32u16);
        // D s_92_13: bit-extract s_92_12 s_92_9 s_92_10
        let s_92_13: Bits = (Bits::new(
            ((s_92_12) >> (s_92_9)).value(),
            u16::try_from(s_92_10).unwrap(),
        ));
        // D s_92_14: cast reint s_92_13 -> u8
        let s_92_14: u8 = (s_92_13.value() as u8);
        // C s_92_15: const #10s : i
        let s_92_15: i128 = 10;
        // C s_92_16: const #8s : i
        let s_92_16: i128 = 8;
        // D s_92_17: read-var u#23025:u32
        let s_92_17: u32 = fn_state.u_23025;
        // D s_92_18: cast zx s_92_17 -> bv
        let s_92_18: Bits = Bits::new(s_92_17 as u128, 32u16);
        // D s_92_19: bit-extract s_92_18 s_92_15 s_92_16
        let s_92_19: Bits = (Bits::new(
            ((s_92_18) >> (s_92_15)).value(),
            u16::try_from(s_92_16).unwrap(),
        ));
        // D s_92_20: cast reint s_92_19 -> u8
        let s_92_20: u8 = (s_92_19.value() as u8);
        // C s_92_21: const #31s : i
        let s_92_21: i128 = 31;
        // C s_92_22: const #1s : i
        let s_92_22: i128 = 1;
        // D s_92_23: read-var u#23025:u32
        let s_92_23: u32 = fn_state.u_23025;
        // D s_92_24: cast zx s_92_23 -> bv
        let s_92_24: Bits = Bits::new(s_92_23 as u128, 32u16);
        // D s_92_25: bit-extract s_92_24 s_92_21 s_92_22
        let s_92_25: Bits = (Bits::new(
            ((s_92_24) >> (s_92_21)).value(),
            u16::try_from(s_92_22).unwrap(),
        ));
        // D s_92_26: cast reint s_92_25 -> u8
        let s_92_26: bool = ((s_92_25.value()) != 0);
        // D s_92_27: call decode_smin_imm_aarch64_instrs_integer_arithmetic_max_min_smin_imm(s_92_8, s_92_14, s_92_20, s_92_26)
        let s_92_27: () = decode_smin_imm_aarch64_instrs_integer_arithmetic_max_min_smin_imm(
            state,
            tracer,
            s_92_8,
            s_92_14,
            s_92_20,
            s_92_26,
        );
        // N s_92_28: return
        return;
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var merge#var.1:struct
        let s_93_0: u32 = fn_state.merge_var._1;
        // D s_93_1: write-var u#23031 <= s_93_0
        fn_state.u_23031 = s_93_0;
        // C s_93_2: const #22s : i
        let s_93_2: i128 = 22;
        // D s_93_3: read-var u#23031:u32
        let s_93_3: u32 = fn_state.u_23031;
        // D s_93_4: cast zx s_93_3 -> bv
        let s_93_4: Bits = Bits::new(s_93_3 as u128, 32u16);
        // C s_93_5: const #1s : i64
        let s_93_5: i64 = 1;
        // C s_93_6: cast zx s_93_5 -> i
        let s_93_6: i128 = (i128::try_from(s_93_5).unwrap());
        // C s_93_7: const #9s : i
        let s_93_7: i128 = 9;
        // C s_93_8: add s_93_7 s_93_6
        let s_93_8: i128 = (s_93_7 + s_93_6);
        // D s_93_9: bit-extract s_93_4 s_93_2 s_93_8
        let s_93_9: Bits = (Bits::new(
            ((s_93_4) >> (s_93_2)).value(),
            u16::try_from(s_93_8).unwrap(),
        ));
        // D s_93_10: cast reint s_93_9 -> u10
        let s_93_10: u16 = (s_93_9.value() as u16);
        // D s_93_11: cast zx s_93_10 -> bv
        let s_93_11: Bits = Bits::new(s_93_10 as u128, 10u16);
        // C s_93_12: const #838u : u10
        let s_93_12: u16 = 838;
        // C s_93_13: cast zx s_93_12 -> bv
        let s_93_13: Bits = Bits::new(s_93_12 as u128, 10u16);
        // D s_93_14: cmp-eq s_93_11 s_93_13
        let s_93_14: bool = ((s_93_11) == (s_93_13));
        // N s_93_15: branch s_93_14 b116 b94
        if s_93_14 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #0u : u8
        let s_94_0: bool = false;
        // D s_94_1: write-var gs#373564 <= s_94_0
        fn_state.gs_373564 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#373564:u8
        let s_95_0: bool = fn_state.gs_373564;
        // N s_95_1: branch s_95_0 b115 b96
        if s_95_0 {
            return block_115(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#373566 <= s_96_0
        fn_state.gs_373566 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#373566:u8
        let s_97_0: bool = fn_state.gs_373566;
        // D s_97_1: not s_97_0
        let s_97_1: bool = !s_97_0;
        // N s_97_2: branch s_97_1 b104 b98
        if s_97_1 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #990s : i
        let s_98_0: i128 = 990;
        // C s_98_1: const #14696u : u32
        let s_98_1: u32 = 14696;
        // N s_98_2: write-reg s_98_1 <= s_98_0
        let s_98_2: () = {
            state.write_register::<i128>(s_98_1 as isize, s_98_0);
            tracer.write_register(s_98_1 as isize, s_98_0);
        };
        // C s_98_3: const #0s : i
        let s_98_3: i128 = 0;
        // C s_98_4: const #5s : i
        let s_98_4: i128 = 5;
        // D s_98_5: read-var u#23031:u32
        let s_98_5: u32 = fn_state.u_23031;
        // D s_98_6: cast zx s_98_5 -> bv
        let s_98_6: Bits = Bits::new(s_98_5 as u128, 32u16);
        // D s_98_7: bit-extract s_98_6 s_98_3 s_98_4
        let s_98_7: Bits = (Bits::new(
            ((s_98_6) >> (s_98_3)).value(),
            u16::try_from(s_98_4).unwrap(),
        ));
        // D s_98_8: cast reint s_98_7 -> u8
        let s_98_8: u8 = (s_98_7.value() as u8);
        // D s_98_9: write-var u#23032 <= s_98_8
        fn_state.u_23032 = s_98_8;
        // C s_98_10: const #5s : i
        let s_98_10: i128 = 5;
        // C s_98_11: const #5s : i
        let s_98_11: i128 = 5;
        // D s_98_12: read-var u#23031:u32
        let s_98_12: u32 = fn_state.u_23031;
        // D s_98_13: cast zx s_98_12 -> bv
        let s_98_13: Bits = Bits::new(s_98_12 as u128, 32u16);
        // D s_98_14: bit-extract s_98_13 s_98_10 s_98_11
        let s_98_14: Bits = (Bits::new(
            ((s_98_13) >> (s_98_10)).value(),
            u16::try_from(s_98_11).unwrap(),
        ));
        // D s_98_15: cast reint s_98_14 -> u8
        let s_98_15: u8 = (s_98_14.value() as u8);
        // D s_98_16: write-var u#23033 <= s_98_15
        fn_state.u_23033 = s_98_15;
        // C s_98_17: const #10s : i
        let s_98_17: i128 = 10;
        // C s_98_18: const #4s : i
        let s_98_18: i128 = 4;
        // D s_98_19: read-var u#23031:u32
        let s_98_19: u32 = fn_state.u_23031;
        // D s_98_20: cast zx s_98_19 -> bv
        let s_98_20: Bits = Bits::new(s_98_19 as u128, 32u16);
        // D s_98_21: bit-extract s_98_20 s_98_17 s_98_18
        let s_98_21: Bits = (Bits::new(
            ((s_98_20) >> (s_98_17)).value(),
            u16::try_from(s_98_18).unwrap(),
        ));
        // D s_98_22: cast reint s_98_21 -> u8
        let s_98_22: u8 = (s_98_21.value() as u8);
        // D s_98_23: write-var u#23034 <= s_98_22
        fn_state.u_23034 = s_98_22;
        // C s_98_24: const #14s : i
        let s_98_24: i128 = 14;
        // C s_98_25: const #2s : i
        let s_98_25: i128 = 2;
        // D s_98_26: read-var u#23031:u32
        let s_98_26: u32 = fn_state.u_23031;
        // D s_98_27: cast zx s_98_26 -> bv
        let s_98_27: Bits = Bits::new(s_98_26 as u128, 32u16);
        // D s_98_28: bit-extract s_98_27 s_98_24 s_98_25
        let s_98_28: Bits = (Bits::new(
            ((s_98_27) >> (s_98_24)).value(),
            u16::try_from(s_98_25).unwrap(),
        ));
        // D s_98_29: cast reint s_98_28 -> u8
        let s_98_29: u8 = (s_98_28.value() as u8);
        // D s_98_30: write-var u#23035 <= s_98_29
        fn_state.u_23035 = s_98_29;
        // C s_98_31: const #16s : i
        let s_98_31: i128 = 16;
        // C s_98_32: const #6s : i
        let s_98_32: i128 = 6;
        // D s_98_33: read-var u#23031:u32
        let s_98_33: u32 = fn_state.u_23031;
        // D s_98_34: cast zx s_98_33 -> bv
        let s_98_34: Bits = Bits::new(s_98_33 as u128, 32u16);
        // D s_98_35: bit-extract s_98_34 s_98_31 s_98_32
        let s_98_35: Bits = (Bits::new(
            ((s_98_34) >> (s_98_31)).value(),
            u16::try_from(s_98_32).unwrap(),
        ));
        // D s_98_36: cast reint s_98_35 -> u8
        let s_98_36: u8 = (s_98_35.value() as u8);
        // D s_98_37: write-var u#23036 <= s_98_36
        fn_state.u_23036 = s_98_36;
        // C s_98_38: const #14s : i
        let s_98_38: i128 = 14;
        // D s_98_39: read-var u#23031:u32
        let s_98_39: u32 = fn_state.u_23031;
        // D s_98_40: cast zx s_98_39 -> bv
        let s_98_40: Bits = Bits::new(s_98_39 as u128, 32u16);
        // C s_98_41: const #1u : u64
        let s_98_41: u64 = 1;
        // D s_98_42: bit-extract s_98_40 s_98_38 s_98_41
        let s_98_42: Bits = (Bits::new(
            ((s_98_40) >> (s_98_38)).value(),
            u16::try_from(s_98_41).unwrap(),
        ));
        // D s_98_43: cast reint s_98_42 -> u8
        let s_98_43: bool = ((s_98_42.value()) != 0);
        // C s_98_44: const #0s : i
        let s_98_44: i128 = 0;
        // C s_98_45: const #0u : u64
        let s_98_45: u64 = 0;
        // D s_98_46: cast zx s_98_43 -> u64
        let s_98_46: u64 = (s_98_43 as u64);
        // C s_98_47: const #1u : u64
        let s_98_47: u64 = 1;
        // D s_98_48: and s_98_46 s_98_47
        let s_98_48: u64 = ((s_98_46) & (s_98_47));
        // D s_98_49: cmp-eq s_98_48 s_98_47
        let s_98_49: bool = ((s_98_48) == (s_98_47));
        // D s_98_50: lsl s_98_46 s_98_44
        let s_98_50: u64 = s_98_46 << s_98_44;
        // D s_98_51: or s_98_45 s_98_50
        let s_98_51: u64 = ((s_98_45) | (s_98_50));
        // D s_98_52: cmpl s_98_50
        let s_98_52: u64 = !s_98_50;
        // D s_98_53: and s_98_45 s_98_52
        let s_98_53: u64 = ((s_98_45) & (s_98_52));
        // D s_98_54: select s_98_49 s_98_51 s_98_53
        let s_98_54: u64 = if s_98_49 { s_98_51 } else { s_98_53 };
        // D s_98_55: cast trunc s_98_54 -> u8
        let s_98_55: bool = ((s_98_54) != 0);
        // D s_98_56: cast zx s_98_55 -> bv
        let s_98_56: Bits = Bits::new(s_98_55 as u128, 1u16);
        // C s_98_57: const #0u : u8
        let s_98_57: bool = false;
        // C s_98_58: cast zx s_98_57 -> bv
        let s_98_58: Bits = Bits::new(s_98_57 as u128, 1u16);
        // D s_98_59: cmp-ne s_98_56 s_98_58
        let s_98_59: bool = ((s_98_56) != (s_98_58));
        // N s_98_60: branch s_98_59 b103 b99
        if s_98_59 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #15s : i
        let s_99_0: i128 = 15;
        // D s_99_1: read-var u#23031:u32
        let s_99_1: u32 = fn_state.u_23031;
        // D s_99_2: cast zx s_99_1 -> bv
        let s_99_2: Bits = Bits::new(s_99_1 as u128, 32u16);
        // C s_99_3: const #1u : u64
        let s_99_3: u64 = 1;
        // D s_99_4: bit-extract s_99_2 s_99_0 s_99_3
        let s_99_4: Bits = (Bits::new(
            ((s_99_2) >> (s_99_0)).value(),
            u16::try_from(s_99_3).unwrap(),
        ));
        // D s_99_5: cast reint s_99_4 -> u8
        let s_99_5: bool = ((s_99_4.value()) != 0);
        // C s_99_6: const #0s : i
        let s_99_6: i128 = 0;
        // C s_99_7: const #0u : u64
        let s_99_7: u64 = 0;
        // D s_99_8: cast zx s_99_5 -> u64
        let s_99_8: u64 = (s_99_5 as u64);
        // C s_99_9: const #1u : u64
        let s_99_9: u64 = 1;
        // D s_99_10: and s_99_8 s_99_9
        let s_99_10: u64 = ((s_99_8) & (s_99_9));
        // D s_99_11: cmp-eq s_99_10 s_99_9
        let s_99_11: bool = ((s_99_10) == (s_99_9));
        // D s_99_12: lsl s_99_8 s_99_6
        let s_99_12: u64 = s_99_8 << s_99_6;
        // D s_99_13: or s_99_7 s_99_12
        let s_99_13: u64 = ((s_99_7) | (s_99_12));
        // D s_99_14: cmpl s_99_12
        let s_99_14: u64 = !s_99_12;
        // D s_99_15: and s_99_7 s_99_14
        let s_99_15: u64 = ((s_99_7) & (s_99_14));
        // D s_99_16: select s_99_11 s_99_13 s_99_15
        let s_99_16: u64 = if s_99_11 { s_99_13 } else { s_99_15 };
        // D s_99_17: cast trunc s_99_16 -> u8
        let s_99_17: bool = ((s_99_16) != 0);
        // D s_99_18: cast zx s_99_17 -> bv
        let s_99_18: Bits = Bits::new(s_99_17 as u128, 1u16);
        // C s_99_19: const #0u : u8
        let s_99_19: bool = false;
        // C s_99_20: cast zx s_99_19 -> bv
        let s_99_20: Bits = Bits::new(s_99_19 as u128, 1u16);
        // D s_99_21: cmp-ne s_99_18 s_99_20
        let s_99_21: bool = ((s_99_18) != (s_99_20));
        // D s_99_22: write-var gs#373583 <= s_99_21
        fn_state.gs_373583 = s_99_21;
        // N s_99_23: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#373583:u8
        let s_100_0: bool = fn_state.gs_373583;
        // N s_100_1: branch s_100_0 b102 b101
        if s_100_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var u#23032:u8
        let s_101_0: u8 = fn_state.u_23032;
        // D s_101_1: read-var u#23033:u8
        let s_101_1: u8 = fn_state.u_23033;
        // D s_101_2: read-var u#23034:u8
        let s_101_2: u8 = fn_state.u_23034;
        // D s_101_3: read-var u#23035:u8
        let s_101_3: u8 = fn_state.u_23035;
        // D s_101_4: read-var u#23036:u8
        let s_101_4: u8 = fn_state.u_23036;
        // D s_101_5: call decode_subg_aarch64_instrs_integer_tags_mcsubtag(s_101_0, s_101_1, s_101_2, s_101_3, s_101_4)
        let s_101_5: () = decode_subg_aarch64_instrs_integer_tags_mcsubtag(
            state,
            tracer,
            s_101_0,
            s_101_1,
            s_101_2,
            s_101_3,
            s_101_4,
        );
        // N s_101_6: return
        return;
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_102_0: panic
        panic!("{:?}", ());
        // N s_102_1: return
        return;
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #1u : u8
        let s_103_0: bool = true;
        // D s_103_1: write-var gs#373583 <= s_103_0
        fn_state.gs_373583 = s_103_0;
        // N s_103_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var merge#var.1:struct
        let s_104_0: u32 = fn_state.merge_var._1;
        // D s_104_1: write-var u#23038 <= s_104_0
        fn_state.u_23038 = s_104_0;
        // C s_104_2: const #18s : i
        let s_104_2: i128 = 18;
        // D s_104_3: read-var u#23038:u32
        let s_104_3: u32 = fn_state.u_23038;
        // D s_104_4: cast zx s_104_3 -> bv
        let s_104_4: Bits = Bits::new(s_104_3 as u128, 32u16);
        // C s_104_5: const #1s : i64
        let s_104_5: i64 = 1;
        // C s_104_6: cast zx s_104_5 -> i
        let s_104_6: i128 = (i128::try_from(s_104_5).unwrap());
        // C s_104_7: const #12s : i
        let s_104_7: i128 = 12;
        // C s_104_8: add s_104_7 s_104_6
        let s_104_8: i128 = (s_104_7 + s_104_6);
        // D s_104_9: bit-extract s_104_4 s_104_2 s_104_8
        let s_104_9: Bits = (Bits::new(
            ((s_104_4) >> (s_104_2)).value(),
            u16::try_from(s_104_8).unwrap(),
        ));
        // D s_104_10: cast reint s_104_9 -> u13
        let s_104_10: u16 = (s_104_9.value() as u16);
        // D s_104_11: cast zx s_104_10 -> bv
        let s_104_11: Bits = Bits::new(s_104_10 as u128, 13u16);
        // C s_104_12: const #1137u : u13
        let s_104_12: u16 = 1137;
        // C s_104_13: cast zx s_104_12 -> bv
        let s_104_13: Bits = Bits::new(s_104_12 as u128, 13u16);
        // D s_104_14: cmp-eq s_104_11 s_104_13
        let s_104_14: bool = ((s_104_11) == (s_104_13));
        // N s_104_15: branch s_104_14 b114 b105
        if s_104_14 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #0u : u8
        let s_105_0: bool = false;
        // D s_105_1: write-var gs#373588 <= s_105_0
        fn_state.gs_373588 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#373588:u8
        let s_106_0: bool = fn_state.gs_373588;
        // D s_106_1: not s_106_0
        let s_106_1: bool = !s_106_0;
        // N s_106_2: branch s_106_1 b108 b107
        if s_106_1 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #1018s : i
        let s_107_0: i128 = 1018;
        // C s_107_1: const #14696u : u32
        let s_107_1: u32 = 14696;
        // N s_107_2: write-reg s_107_1 <= s_107_0
        let s_107_2: () = {
            state.write_register::<i128>(s_107_1 as isize, s_107_0);
            tracer.write_register(s_107_1 as isize, s_107_0);
        };
        // C s_107_3: const #0s : i
        let s_107_3: i128 = 0;
        // C s_107_4: const #5s : i
        let s_107_4: i128 = 5;
        // D s_107_5: read-var u#23038:u32
        let s_107_5: u32 = fn_state.u_23038;
        // D s_107_6: cast zx s_107_5 -> bv
        let s_107_6: Bits = Bits::new(s_107_5 as u128, 32u16);
        // D s_107_7: bit-extract s_107_6 s_107_3 s_107_4
        let s_107_7: Bits = (Bits::new(
            ((s_107_6) >> (s_107_3)).value(),
            u16::try_from(s_107_4).unwrap(),
        ));
        // D s_107_8: cast reint s_107_7 -> u8
        let s_107_8: u8 = (s_107_7.value() as u8);
        // C s_107_9: const #5s : i
        let s_107_9: i128 = 5;
        // C s_107_10: const #5s : i
        let s_107_10: i128 = 5;
        // D s_107_11: read-var u#23038:u32
        let s_107_11: u32 = fn_state.u_23038;
        // D s_107_12: cast zx s_107_11 -> bv
        let s_107_12: Bits = Bits::new(s_107_11 as u128, 32u16);
        // D s_107_13: bit-extract s_107_12 s_107_9 s_107_10
        let s_107_13: Bits = (Bits::new(
            ((s_107_12) >> (s_107_9)).value(),
            u16::try_from(s_107_10).unwrap(),
        ));
        // D s_107_14: cast reint s_107_13 -> u8
        let s_107_14: u8 = (s_107_13.value() as u8);
        // C s_107_15: const #10s : i
        let s_107_15: i128 = 10;
        // C s_107_16: const #8s : i
        let s_107_16: i128 = 8;
        // D s_107_17: read-var u#23038:u32
        let s_107_17: u32 = fn_state.u_23038;
        // D s_107_18: cast zx s_107_17 -> bv
        let s_107_18: Bits = Bits::new(s_107_17 as u128, 32u16);
        // D s_107_19: bit-extract s_107_18 s_107_15 s_107_16
        let s_107_19: Bits = (Bits::new(
            ((s_107_18) >> (s_107_15)).value(),
            u16::try_from(s_107_16).unwrap(),
        ));
        // D s_107_20: cast reint s_107_19 -> u8
        let s_107_20: u8 = (s_107_19.value() as u8);
        // C s_107_21: const #31s : i
        let s_107_21: i128 = 31;
        // C s_107_22: const #1s : i
        let s_107_22: i128 = 1;
        // D s_107_23: read-var u#23038:u32
        let s_107_23: u32 = fn_state.u_23038;
        // D s_107_24: cast zx s_107_23 -> bv
        let s_107_24: Bits = Bits::new(s_107_23 as u128, 32u16);
        // D s_107_25: bit-extract s_107_24 s_107_21 s_107_22
        let s_107_25: Bits = (Bits::new(
            ((s_107_24) >> (s_107_21)).value(),
            u16::try_from(s_107_22).unwrap(),
        ));
        // D s_107_26: cast reint s_107_25 -> u8
        let s_107_26: bool = ((s_107_25.value()) != 0);
        // D s_107_27: call decode_umax_imm_aarch64_instrs_integer_arithmetic_max_min_umax_imm(s_107_8, s_107_14, s_107_20, s_107_26)
        let s_107_27: () = decode_umax_imm_aarch64_instrs_integer_arithmetic_max_min_umax_imm(
            state,
            tracer,
            s_107_8,
            s_107_14,
            s_107_20,
            s_107_26,
        );
        // N s_107_28: return
        return;
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var merge#var.1:struct
        let s_108_0: u32 = fn_state.merge_var._1;
        // D s_108_1: write-var u#23044 <= s_108_0
        fn_state.u_23044 = s_108_0;
        // C s_108_2: const #18s : i
        let s_108_2: i128 = 18;
        // D s_108_3: read-var u#23044:u32
        let s_108_3: u32 = fn_state.u_23044;
        // D s_108_4: cast zx s_108_3 -> bv
        let s_108_4: Bits = Bits::new(s_108_3 as u128, 32u16);
        // C s_108_5: const #1s : i64
        let s_108_5: i64 = 1;
        // C s_108_6: cast zx s_108_5 -> i
        let s_108_6: i128 = (i128::try_from(s_108_5).unwrap());
        // C s_108_7: const #12s : i
        let s_108_7: i128 = 12;
        // C s_108_8: add s_108_7 s_108_6
        let s_108_8: i128 = (s_108_7 + s_108_6);
        // D s_108_9: bit-extract s_108_4 s_108_2 s_108_8
        let s_108_9: Bits = (Bits::new(
            ((s_108_4) >> (s_108_2)).value(),
            u16::try_from(s_108_8).unwrap(),
        ));
        // D s_108_10: cast reint s_108_9 -> u13
        let s_108_10: u16 = (s_108_9.value() as u16);
        // D s_108_11: cast zx s_108_10 -> bv
        let s_108_11: Bits = Bits::new(s_108_10 as u128, 13u16);
        // C s_108_12: const #1139u : u13
        let s_108_12: u16 = 1139;
        // C s_108_13: cast zx s_108_12 -> bv
        let s_108_13: Bits = Bits::new(s_108_12 as u128, 13u16);
        // D s_108_14: cmp-eq s_108_11 s_108_13
        let s_108_14: bool = ((s_108_11) == (s_108_13));
        // N s_108_15: branch s_108_14 b113 b109
        if s_108_14 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #0u : u8
        let s_109_0: bool = false;
        // D s_109_1: write-var gs#373603 <= s_109_0
        fn_state.gs_373603 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#373603:u8
        let s_110_0: bool = fn_state.gs_373603;
        // D s_110_1: not s_110_0
        let s_110_1: bool = !s_110_0;
        // N s_110_2: branch s_110_1 b112 b111
        if s_110_1 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #1021s : i
        let s_111_0: i128 = 1021;
        // C s_111_1: const #14696u : u32
        let s_111_1: u32 = 14696;
        // N s_111_2: write-reg s_111_1 <= s_111_0
        let s_111_2: () = {
            state.write_register::<i128>(s_111_1 as isize, s_111_0);
            tracer.write_register(s_111_1 as isize, s_111_0);
        };
        // C s_111_3: const #0s : i
        let s_111_3: i128 = 0;
        // C s_111_4: const #5s : i
        let s_111_4: i128 = 5;
        // D s_111_5: read-var u#23044:u32
        let s_111_5: u32 = fn_state.u_23044;
        // D s_111_6: cast zx s_111_5 -> bv
        let s_111_6: Bits = Bits::new(s_111_5 as u128, 32u16);
        // D s_111_7: bit-extract s_111_6 s_111_3 s_111_4
        let s_111_7: Bits = (Bits::new(
            ((s_111_6) >> (s_111_3)).value(),
            u16::try_from(s_111_4).unwrap(),
        ));
        // D s_111_8: cast reint s_111_7 -> u8
        let s_111_8: u8 = (s_111_7.value() as u8);
        // C s_111_9: const #5s : i
        let s_111_9: i128 = 5;
        // C s_111_10: const #5s : i
        let s_111_10: i128 = 5;
        // D s_111_11: read-var u#23044:u32
        let s_111_11: u32 = fn_state.u_23044;
        // D s_111_12: cast zx s_111_11 -> bv
        let s_111_12: Bits = Bits::new(s_111_11 as u128, 32u16);
        // D s_111_13: bit-extract s_111_12 s_111_9 s_111_10
        let s_111_13: Bits = (Bits::new(
            ((s_111_12) >> (s_111_9)).value(),
            u16::try_from(s_111_10).unwrap(),
        ));
        // D s_111_14: cast reint s_111_13 -> u8
        let s_111_14: u8 = (s_111_13.value() as u8);
        // C s_111_15: const #10s : i
        let s_111_15: i128 = 10;
        // C s_111_16: const #8s : i
        let s_111_16: i128 = 8;
        // D s_111_17: read-var u#23044:u32
        let s_111_17: u32 = fn_state.u_23044;
        // D s_111_18: cast zx s_111_17 -> bv
        let s_111_18: Bits = Bits::new(s_111_17 as u128, 32u16);
        // D s_111_19: bit-extract s_111_18 s_111_15 s_111_16
        let s_111_19: Bits = (Bits::new(
            ((s_111_18) >> (s_111_15)).value(),
            u16::try_from(s_111_16).unwrap(),
        ));
        // D s_111_20: cast reint s_111_19 -> u8
        let s_111_20: u8 = (s_111_19.value() as u8);
        // C s_111_21: const #31s : i
        let s_111_21: i128 = 31;
        // C s_111_22: const #1s : i
        let s_111_22: i128 = 1;
        // D s_111_23: read-var u#23044:u32
        let s_111_23: u32 = fn_state.u_23044;
        // D s_111_24: cast zx s_111_23 -> bv
        let s_111_24: Bits = Bits::new(s_111_23 as u128, 32u16);
        // D s_111_25: bit-extract s_111_24 s_111_21 s_111_22
        let s_111_25: Bits = (Bits::new(
            ((s_111_24) >> (s_111_21)).value(),
            u16::try_from(s_111_22).unwrap(),
        ));
        // D s_111_26: cast reint s_111_25 -> u8
        let s_111_26: bool = ((s_111_25.value()) != 0);
        // D s_111_27: call decode_umin_imm_aarch64_instrs_integer_arithmetic_max_min_umin_imm(s_111_8, s_111_14, s_111_20, s_111_26)
        let s_111_27: () = decode_umin_imm_aarch64_instrs_integer_arithmetic_max_min_umin_imm(
            state,
            tracer,
            s_111_8,
            s_111_14,
            s_111_20,
            s_111_26,
        );
        // N s_111_28: return
        return;
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_112_0: panic
        panic!("{:?}", ());
        // N s_112_1: return
        return;
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #1021s : i
        let s_113_0: i128 = 1021;
        // C s_113_1: const #14696u : u32
        let s_113_1: u32 = 14696;
        // D s_113_2: read-reg s_113_1:i
        let s_113_2: i128 = {
            let value = state.read_register::<i128>(s_113_1 as isize);
            tracer.read_register(s_113_1 as isize, value);
            value
        };
        // D s_113_3: cmp-lt s_113_2 s_113_0
        let s_113_3: bool = ((s_113_2) < (s_113_0));
        // D s_113_4: write-var gs#373603 <= s_113_3
        fn_state.gs_373603 = s_113_3;
        // N s_113_5: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #1018s : i
        let s_114_0: i128 = 1018;
        // C s_114_1: const #14696u : u32
        let s_114_1: u32 = 14696;
        // D s_114_2: read-reg s_114_1:i
        let s_114_2: i128 = {
            let value = state.read_register::<i128>(s_114_1 as isize);
            tracer.read_register(s_114_1 as isize, value);
            value
        };
        // D s_114_3: cmp-lt s_114_2 s_114_0
        let s_114_3: bool = ((s_114_2) < (s_114_0));
        // D s_114_4: write-var gs#373588 <= s_114_3
        fn_state.gs_373588 = s_114_3;
        // N s_114_5: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #990s : i
        let s_115_0: i128 = 990;
        // C s_115_1: const #14696u : u32
        let s_115_1: u32 = 14696;
        // D s_115_2: read-reg s_115_1:i
        let s_115_2: i128 = {
            let value = state.read_register::<i128>(s_115_1 as isize);
            tracer.read_register(s_115_1 as isize, value);
            value
        };
        // D s_115_3: cmp-lt s_115_2 s_115_0
        let s_115_3: bool = ((s_115_2) < (s_115_0));
        // D s_115_4: write-var gs#373566 <= s_115_3
        fn_state.gs_373566 = s_115_3;
        // N s_115_5: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #14s : i
        let s_116_0: i128 = 14;
        // D s_116_1: read-var u#23031:u32
        let s_116_1: u32 = fn_state.u_23031;
        // D s_116_2: cast zx s_116_1 -> bv
        let s_116_2: Bits = Bits::new(s_116_1 as u128, 32u16);
        // C s_116_3: const #1s : i64
        let s_116_3: i64 = 1;
        // C s_116_4: cast zx s_116_3 -> i
        let s_116_4: i128 = (i128::try_from(s_116_3).unwrap());
        // C s_116_5: const #1s : i
        let s_116_5: i128 = 1;
        // C s_116_6: add s_116_5 s_116_4
        let s_116_6: i128 = (s_116_5 + s_116_4);
        // D s_116_7: bit-extract s_116_2 s_116_0 s_116_6
        let s_116_7: Bits = (Bits::new(
            ((s_116_2) >> (s_116_0)).value(),
            u16::try_from(s_116_6).unwrap(),
        ));
        // D s_116_8: cast reint s_116_7 -> u8
        let s_116_8: u8 = (s_116_7.value() as u8);
        // D s_116_9: cast zx s_116_8 -> bv
        let s_116_9: Bits = Bits::new(s_116_8 as u128, 2u16);
        // C s_116_10: const #0u : u8
        let s_116_10: u8 = 0;
        // C s_116_11: cast zx s_116_10 -> bv
        let s_116_11: Bits = Bits::new(s_116_10 as u128, 2u16);
        // D s_116_12: cmp-eq s_116_9 s_116_11
        let s_116_12: bool = ((s_116_9) == (s_116_11));
        // D s_116_13: write-var gs#373564 <= s_116_12
        fn_state.gs_373564 = s_116_12;
        // N s_116_14: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #847s : i
        let s_117_0: i128 = 847;
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
        // D s_117_4: write-var gs#373548 <= s_117_3
        fn_state.gs_373548 = s_117_3;
        // N s_117_5: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #832s : i
        let s_118_0: i128 = 832;
        // C s_118_1: const #14696u : u32
        let s_118_1: u32 = 14696;
        // D s_118_2: read-reg s_118_1:i
        let s_118_2: i128 = {
            let value = state.read_register::<i128>(s_118_1 as isize);
            tracer.read_register(s_118_1 as isize, value);
            value
        };
        // D s_118_3: cmp-lt s_118_2 s_118_0
        let s_118_3: bool = ((s_118_2) < (s_118_0));
        // D s_118_4: write-var gs#373533 <= s_118_3
        fn_state.gs_373533 = s_118_3;
        // N s_118_5: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #698s : i
        let s_119_0: i128 = 698;
        // C s_119_1: const #14696u : u32
        let s_119_1: u32 = 14696;
        // D s_119_2: read-reg s_119_1:i
        let s_119_2: i128 = {
            let value = state.read_register::<i128>(s_119_1 as isize);
            tracer.read_register(s_119_1 as isize, value);
            value
        };
        // D s_119_3: cmp-lt s_119_2 s_119_0
        let s_119_3: bool = ((s_119_2) < (s_119_0));
        // D s_119_4: write-var gs#373516 <= s_119_3
        fn_state.gs_373516 = s_119_3;
        // N s_119_5: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #697s : i
        let s_120_0: i128 = 697;
        // C s_120_1: const #14696u : u32
        let s_120_1: u32 = 14696;
        // D s_120_2: read-reg s_120_1:i
        let s_120_2: i128 = {
            let value = state.read_register::<i128>(s_120_1 as isize);
            tracer.read_register(s_120_1 as isize, value);
            value
        };
        // D s_120_3: cmp-lt s_120_2 s_120_0
        let s_120_3: bool = ((s_120_2) < (s_120_0));
        // D s_120_4: write-var gs#373499 <= s_120_3
        fn_state.gs_373499 = s_120_3;
        // N s_120_5: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #696s : i
        let s_121_0: i128 = 696;
        // C s_121_1: const #14696u : u32
        let s_121_1: u32 = 14696;
        // D s_121_2: read-reg s_121_1:i
        let s_121_2: i128 = {
            let value = state.read_register::<i128>(s_121_1 as isize);
            tracer.read_register(s_121_1 as isize, value);
            value
        };
        // D s_121_3: cmp-lt s_121_2 s_121_0
        let s_121_3: bool = ((s_121_2) < (s_121_0));
        // D s_121_4: write-var gs#373482 <= s_121_3
        fn_state.gs_373482 = s_121_3;
        // N s_121_5: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #206s : i
        let s_122_0: i128 = 206;
        // C s_122_1: const #14696u : u32
        let s_122_1: u32 = 14696;
        // D s_122_2: read-reg s_122_1:i
        let s_122_2: i128 = {
            let value = state.read_register::<i128>(s_122_1 as isize);
            tracer.read_register(s_122_1 as isize, value);
            value
        };
        // D s_122_3: cmp-lt s_122_2 s_122_0
        let s_122_3: bool = ((s_122_2) < (s_122_0));
        // D s_122_4: write-var gs#373463 <= s_122_3
        fn_state.gs_373463 = s_122_3;
        // N s_122_5: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #21s : i
        let s_123_0: i128 = 21;
        // D s_123_1: read-var u#22994:u32
        let s_123_1: u32 = fn_state.u_22994;
        // D s_123_2: cast zx s_123_1 -> bv
        let s_123_2: Bits = Bits::new(s_123_1 as u128, 32u16);
        // C s_123_3: const #1s : i64
        let s_123_3: i64 = 1;
        // C s_123_4: cast zx s_123_3 -> i
        let s_123_4: i128 = (i128::try_from(s_123_3).unwrap());
        // C s_123_5: const #0s : i
        let s_123_5: i128 = 0;
        // C s_123_6: add s_123_5 s_123_4
        let s_123_6: i128 = (s_123_5 + s_123_4);
        // D s_123_7: bit-extract s_123_2 s_123_0 s_123_6
        let s_123_7: Bits = (Bits::new(
            ((s_123_2) >> (s_123_0)).value(),
            u16::try_from(s_123_6).unwrap(),
        ));
        // D s_123_8: cast reint s_123_7 -> u8
        let s_123_8: bool = ((s_123_7.value()) != 0);
        // D s_123_9: cast zx s_123_8 -> bv
        let s_123_9: Bits = Bits::new(s_123_8 as u128, 1u16);
        // C s_123_10: const #0u : u8
        let s_123_10: bool = false;
        // C s_123_11: cast zx s_123_10 -> bv
        let s_123_11: Bits = Bits::new(s_123_10 as u128, 1u16);
        // D s_123_12: cmp-eq s_123_9 s_123_11
        let s_123_12: bool = ((s_123_9) == (s_123_11));
        // D s_123_13: write-var gs#373461 <= s_123_12
        fn_state.gs_373461 = s_123_12;
        // N s_123_14: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #78s : i
        let s_124_0: i128 = 78;
        // C s_124_1: const #14696u : u32
        let s_124_1: u32 = 14696;
        // D s_124_2: read-reg s_124_1:i
        let s_124_2: i128 = {
            let value = state.read_register::<i128>(s_124_1 as isize);
            tracer.read_register(s_124_1 as isize, value);
            value
        };
        // D s_124_3: cmp-lt s_124_2 s_124_0
        let s_124_3: bool = ((s_124_2) < (s_124_0));
        // D s_124_4: write-var gs#373439 <= s_124_3
        fn_state.gs_373439 = s_124_3;
        // N s_124_5: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #77s : i
        let s_125_0: i128 = 77;
        // C s_125_1: const #14696u : u32
        let s_125_1: u32 = 14696;
        // D s_125_2: read-reg s_125_1:i
        let s_125_2: i128 = {
            let value = state.read_register::<i128>(s_125_1 as isize);
            tracer.read_register(s_125_1 as isize, value);
            value
        };
        // D s_125_3: cmp-lt s_125_2 s_125_0
        let s_125_3: bool = ((s_125_2) < (s_125_0));
        // D s_125_4: write-var gs#373418 <= s_125_3
        fn_state.gs_373418 = s_125_3;
        // N s_125_5: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #76s : i
        let s_126_0: i128 = 76;
        // C s_126_1: const #14696u : u32
        let s_126_1: u32 = 14696;
        // D s_126_2: read-reg s_126_1:i
        let s_126_2: i128 = {
            let value = state.read_register::<i128>(s_126_1 as isize);
            tracer.read_register(s_126_1 as isize, value);
            value
        };
        // D s_126_3: cmp-lt s_126_2 s_126_0
        let s_126_3: bool = ((s_126_2) < (s_126_0));
        // D s_126_4: write-var gs#373397 <= s_126_3
        fn_state.gs_373397 = s_126_3;
        // N s_126_5: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #43s : i
        let s_127_0: i128 = 43;
        // C s_127_1: const #14696u : u32
        let s_127_1: u32 = 14696;
        // D s_127_2: read-reg s_127_1:i
        let s_127_2: i128 = {
            let value = state.read_register::<i128>(s_127_1 as isize);
            tracer.read_register(s_127_1 as isize, value);
            value
        };
        // D s_127_3: cmp-lt s_127_2 s_127_0
        let s_127_3: bool = ((s_127_2) < (s_127_0));
        // D s_127_4: write-var gs#373376 <= s_127_3
        fn_state.gs_373376 = s_127_3;
        // N s_127_5: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #42s : i
        let s_128_0: i128 = 42;
        // C s_128_1: const #14696u : u32
        let s_128_1: u32 = 14696;
        // D s_128_2: read-reg s_128_1:i
        let s_128_2: i128 = {
            let value = state.read_register::<i128>(s_128_1 as isize);
            tracer.read_register(s_128_1 as isize, value);
            value
        };
        // D s_128_3: cmp-lt s_128_2 s_128_0
        let s_128_3: bool = ((s_128_2) < (s_128_0));
        // D s_128_4: write-var gs#373355 <= s_128_3
        fn_state.gs_373355 = s_128_3;
        // N s_128_5: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #41s : i
        let s_129_0: i128 = 41;
        // C s_129_1: const #14696u : u32
        let s_129_1: u32 = 14696;
        // D s_129_2: read-reg s_129_1:i
        let s_129_2: i128 = {
            let value = state.read_register::<i128>(s_129_1 as isize);
            tracer.read_register(s_129_1 as isize, value);
            value
        };
        // D s_129_3: cmp-lt s_129_2 s_129_0
        let s_129_3: bool = ((s_129_2) < (s_129_0));
        // D s_129_4: write-var gs#373334 <= s_129_3
        fn_state.gs_373334 = s_129_3;
        // N s_129_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #40s : i
        let s_130_0: i128 = 40;
        // C s_130_1: const #14696u : u32
        let s_130_1: u32 = 14696;
        // D s_130_2: read-reg s_130_1:i
        let s_130_2: i128 = {
            let value = state.read_register::<i128>(s_130_1 as isize);
            tracer.read_register(s_130_1 as isize, value);
            value
        };
        // D s_130_3: cmp-lt s_130_2 s_130_0
        let s_130_3: bool = ((s_130_2) < (s_130_0));
        // D s_130_4: write-var gs#373313 <= s_130_3
        fn_state.gs_373313 = s_130_3;
        // N s_130_5: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #35s : i
        let s_131_0: i128 = 35;
        // C s_131_1: const #14696u : u32
        let s_131_1: u32 = 14696;
        // D s_131_2: read-reg s_131_1:i
        let s_131_2: i128 = {
            let value = state.read_register::<i128>(s_131_1 as isize);
            tracer.read_register(s_131_1 as isize, value);
            value
        };
        // D s_131_3: cmp-lt s_131_2 s_131_0
        let s_131_3: bool = ((s_131_2) < (s_131_0));
        // D s_131_4: write-var gs#373298 <= s_131_3
        fn_state.gs_373298 = s_131_3;
        // N s_131_5: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #24s : i
        let s_132_0: i128 = 24;
        // D s_132_1: read-var u#22929:u32
        let s_132_1: u32 = fn_state.u_22929;
        // D s_132_2: cast zx s_132_1 -> bv
        let s_132_2: Bits = Bits::new(s_132_1 as u128, 32u16);
        // C s_132_3: const #1s : i64
        let s_132_3: i64 = 1;
        // C s_132_4: cast zx s_132_3 -> i
        let s_132_4: i128 = (i128::try_from(s_132_3).unwrap());
        // C s_132_5: const #4s : i
        let s_132_5: i128 = 4;
        // C s_132_6: add s_132_5 s_132_4
        let s_132_6: i128 = (s_132_5 + s_132_4);
        // D s_132_7: bit-extract s_132_2 s_132_0 s_132_6
        let s_132_7: Bits = (Bits::new(
            ((s_132_2) >> (s_132_0)).value(),
            u16::try_from(s_132_6).unwrap(),
        ));
        // D s_132_8: cast reint s_132_7 -> u8
        let s_132_8: u8 = (s_132_7.value() as u8);
        // D s_132_9: cast zx s_132_8 -> bv
        let s_132_9: Bits = Bits::new(s_132_8 as u128, 5u16);
        // C s_132_10: const #16u : u8
        let s_132_10: u8 = 16;
        // C s_132_11: cast zx s_132_10 -> bv
        let s_132_11: Bits = Bits::new(s_132_10 as u128, 5u16);
        // D s_132_12: cmp-eq s_132_9 s_132_11
        let s_132_12: bool = ((s_132_9) == (s_132_11));
        // D s_132_13: write-var gs#373296 <= s_132_12
        fn_state.gs_373296 = s_132_12;
        // N s_132_14: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #34s : i
        let s_133_0: i128 = 34;
        // C s_133_1: const #14696u : u32
        let s_133_1: u32 = 14696;
        // D s_133_2: read-reg s_133_1:i
        let s_133_2: i128 = {
            let value = state.read_register::<i128>(s_133_1 as isize);
            tracer.read_register(s_133_1 as isize, value);
            value
        };
        // D s_133_3: cmp-lt s_133_2 s_133_0
        let s_133_3: bool = ((s_133_2) < (s_133_0));
        // D s_133_4: write-var gs#373280 <= s_133_3
        fn_state.gs_373280 = s_133_3;
        // N s_133_5: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #24s : i
        let s_134_0: i128 = 24;
        // D s_134_1: read-var u#22925:u32
        let s_134_1: u32 = fn_state.u_22925;
        // D s_134_2: cast zx s_134_1 -> bv
        let s_134_2: Bits = Bits::new(s_134_1 as u128, 32u16);
        // C s_134_3: const #1s : i64
        let s_134_3: i64 = 1;
        // C s_134_4: cast zx s_134_3 -> i
        let s_134_4: i128 = (i128::try_from(s_134_3).unwrap());
        // C s_134_5: const #4s : i
        let s_134_5: i128 = 4;
        // C s_134_6: add s_134_5 s_134_4
        let s_134_6: i128 = (s_134_5 + s_134_4);
        // D s_134_7: bit-extract s_134_2 s_134_0 s_134_6
        let s_134_7: Bits = (Bits::new(
            ((s_134_2) >> (s_134_0)).value(),
            u16::try_from(s_134_6).unwrap(),
        ));
        // D s_134_8: cast reint s_134_7 -> u8
        let s_134_8: u8 = (s_134_7.value() as u8);
        // D s_134_9: cast zx s_134_8 -> bv
        let s_134_9: Bits = Bits::new(s_134_8 as u128, 5u16);
        // C s_134_10: const #16u : u8
        let s_134_10: u8 = 16;
        // C s_134_11: cast zx s_134_10 -> bv
        let s_134_11: Bits = Bits::new(s_134_10 as u128, 5u16);
        // D s_134_12: cmp-eq s_134_9 s_134_11
        let s_134_12: bool = ((s_134_9) == (s_134_11));
        // D s_134_13: write-var gs#373278 <= s_134_12
        fn_state.gs_373278 = s_134_12;
        // N s_134_14: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #26s : i
        let s_135_0: i128 = 26;
        // C s_135_1: const #14696u : u32
        let s_135_1: u32 = 14696;
        // D s_135_2: read-reg s_135_1:i
        let s_135_2: i128 = {
            let value = state.read_register::<i128>(s_135_1 as isize);
            tracer.read_register(s_135_1 as isize, value);
            value
        };
        // D s_135_3: cmp-lt s_135_2 s_135_0
        let s_135_3: bool = ((s_135_2) < (s_135_0));
        // D s_135_4: write-var gs#373255 <= s_135_3
        fn_state.gs_373255 = s_135_3;
        // N s_135_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #14s : i
        let s_136_0: i128 = 14;
        // D s_136_1: read-var u#22923:u32
        let s_136_1: u32 = fn_state.u_22923;
        // D s_136_2: cast zx s_136_1 -> bv
        let s_136_2: Bits = Bits::new(s_136_1 as u128, 32u16);
        // C s_136_3: const #1s : i64
        let s_136_3: i64 = 1;
        // C s_136_4: cast zx s_136_3 -> i
        let s_136_4: i128 = (i128::try_from(s_136_3).unwrap());
        // C s_136_5: const #1s : i
        let s_136_5: i128 = 1;
        // C s_136_6: add s_136_5 s_136_4
        let s_136_6: i128 = (s_136_5 + s_136_4);
        // D s_136_7: bit-extract s_136_2 s_136_0 s_136_6
        let s_136_7: Bits = (Bits::new(
            ((s_136_2) >> (s_136_0)).value(),
            u16::try_from(s_136_6).unwrap(),
        ));
        // D s_136_8: cast reint s_136_7 -> u8
        let s_136_8: u8 = (s_136_7.value() as u8);
        // D s_136_9: cast zx s_136_8 -> bv
        let s_136_9: Bits = Bits::new(s_136_8 as u128, 2u16);
        // C s_136_10: const #0u : u8
        let s_136_10: u8 = 0;
        // C s_136_11: cast zx s_136_10 -> bv
        let s_136_11: Bits = Bits::new(s_136_10 as u128, 2u16);
        // D s_136_12: cmp-eq s_136_9 s_136_11
        let s_136_12: bool = ((s_136_9) == (s_136_11));
        // D s_136_13: write-var gs#373253 <= s_136_12
        fn_state.gs_373253 = s_136_12;
        // N s_136_14: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #21s : i
        let s_137_0: i128 = 21;
        // C s_137_1: const #14696u : u32
        let s_137_1: u32 = 14696;
        // D s_137_2: read-reg s_137_1:i
        let s_137_2: i128 = {
            let value = state.read_register::<i128>(s_137_1 as isize);
            tracer.read_register(s_137_1 as isize, value);
            value
        };
        // D s_137_3: cmp-lt s_137_2 s_137_0
        let s_137_3: bool = ((s_137_2) < (s_137_0));
        // D s_137_4: write-var gs#373231 <= s_137_3
        fn_state.gs_373231 = s_137_3;
        // N s_137_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #20s : i
        let s_138_0: i128 = 20;
        // C s_138_1: const #14696u : u32
        let s_138_1: u32 = 14696;
        // D s_138_2: read-reg s_138_1:i
        let s_138_2: i128 = {
            let value = state.read_register::<i128>(s_138_1 as isize);
            tracer.read_register(s_138_1 as isize, value);
            value
        };
        // D s_138_3: cmp-lt s_138_2 s_138_0
        let s_138_3: bool = ((s_138_2) < (s_138_0));
        // D s_138_4: write-var gs#373210 <= s_138_3
        fn_state.gs_373210 = s_138_3;
        // N s_138_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #19s : i
        let s_139_0: i128 = 19;
        // C s_139_1: const #14696u : u32
        let s_139_1: u32 = 14696;
        // D s_139_2: read-reg s_139_1:i
        let s_139_2: i128 = {
            let value = state.read_register::<i128>(s_139_1 as isize);
            tracer.read_register(s_139_1 as isize, value);
            value
        };
        // D s_139_3: cmp-lt s_139_2 s_139_0
        let s_139_3: bool = ((s_139_2) < (s_139_0));
        // D s_139_4: write-var gs#373189 <= s_139_3
        fn_state.gs_373189 = s_139_3;
        // N s_139_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #18s : i
        let s_140_0: i128 = 18;
        // C s_140_1: const #14696u : u32
        let s_140_1: u32 = 14696;
        // D s_140_2: read-reg s_140_1:i
        let s_140_2: i128 = {
            let value = state.read_register::<i128>(s_140_1 as isize);
            tracer.read_register(s_140_1 as isize, value);
            value
        };
        // D s_140_3: cmp-lt s_140_2 s_140_0
        let s_140_3: bool = ((s_140_2) < (s_140_0));
        // D s_140_4: write-var gs#373168 <= s_140_3
        fn_state.gs_373168 = s_140_3;
        // N s_140_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
